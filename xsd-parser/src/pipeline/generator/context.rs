use std::collections::BTreeMap;
use std::ops::Deref;

use bit_set::BitSet;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

use crate::config::GeneratorFlags;
use crate::models::{
    code::{ModuleIdent, ModulePath},
    data::Occurs,
    meta::{BuildInMeta, MetaTypeVariant},
    schema::xs::Use,
    TypeIdent,
};
use crate::pipeline::generator::ValueGeneratorMode;
use crate::pipeline::renderer::{Context as RendererContext, ValueRendererBox};

use super::{Error, MetaData, State, TraitInfos, TypeRef};

/// Helper type that is used to request the code generation for a specific type.
#[derive(Debug)]
pub struct Context<'a, 'types> {
    /// Meta data with different information of the generate process.
    pub meta: &'a MetaData<'types>,

    /// Identifier of the type that is currently processed.
    pub ident: &'a TypeIdent,

    state: &'a mut State<'types>,
    reachable: BitSet<u64>,
}

impl<'a, 'types> Context<'a, 'types> {
    pub(super) fn new(
        meta: &'a MetaData<'types>,
        ident: &'a TypeIdent,
        state: &'a mut State<'types>,
    ) -> Self {
        let reachable = state.loop_detection.get_reachable(&state.cache, ident);

        Self {
            meta,
            ident,
            state,
            reachable,
        }
    }

    pub(super) fn current_module(&self) -> ModuleIdent {
        ModuleIdent::new(
            self.meta.types,
            self.ident,
            self.check_generator_flags(GeneratorFlags::USE_NAMESPACE_MODULES),
            self.check_generator_flags(GeneratorFlags::USE_SCHEMA_MODULES),
        )
    }

    pub(super) fn current_type_ref(&self) -> &TypeRef {
        self.state.cache.get(self.ident).unwrap()
    }

    pub(super) fn get_trait_infos(&mut self) -> &TraitInfos {
        self.state
            .trait_infos
            .get_or_insert_with(|| TraitInfos::new(self.meta.types))
    }

    pub(super) fn get_or_create_type_ref(&mut self, ident: &TypeIdent) -> Result<&TypeRef, Error> {
        let type_ref = self.state.get_or_create_type_ref_mut(self.meta, ident)?;

        Ok(type_ref)
    }

    pub(super) fn get_or_create_type_ref_for_value(
        &mut self,
        ident: &TypeIdent,
        by_value: bool,
    ) -> Result<&TypeRef, Error> {
        let type_ref = self.state.get_or_create_type_ref_mut(self.meta, ident)?;

        if by_value {
            type_ref.reachable.union_with(&self.reachable);
        }

        Ok(type_ref)
    }

    pub(super) fn get_or_create_type_ref_for_element(
        &mut self,
        ident: &TypeIdent,
        by_value: bool,
    ) -> Result<(&TypeRef, bool), Error> {
        let boxed = by_value && need_box(&mut self.reachable, &self.state.cache, self.meta, ident);
        let type_ref = self.state.get_or_create_type_ref_mut(self.meta, ident)?;

        if !boxed {
            type_ref.reachable.union_with(&self.reachable);
        }

        Ok((type_ref, boxed))
    }

    pub(super) fn make_trait_impls(&mut self) -> Result<Vec<TokenStream>, Error> {
        let ident = self.ident.clone();
        let current_module = self.current_module();
        let module_path = ModulePath::from_ident(self.types, current_module);

        self.get_trait_infos()
            .get(&ident)
            .into_iter()
            .flat_map(|info| &info.traits_all)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|ident| {
                let type_ref = self.get_or_create_type_ref(&ident)?;
                let ident = format_ident!("{}Trait", type_ref.path.ident());
                let trait_type = (*type_ref.path).clone().with_ident(ident);
                let trait_ident = trait_type.relative_to(&module_path);

                Ok(trait_ident)
            })
            .collect::<Result<Vec<_>, _>>()
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn make_value_renderer(
        &mut self,
        ident: &TypeIdent,
        value: &str,
        mode: ValueGeneratorMode,
    ) -> Result<ValueRendererBox, Error> {
        use ValueGeneratorMode::Constant as C;
        use ValueGeneratorMode::Value as V;

        let types = self.types;
        let ty = types
            .items
            .get(ident)
            .ok_or_else(|| Error::UnknownType(ident.clone()))?;
        let type_ref = self.get_or_create_type_ref(ident)?;

        macro_rules! build_in {
            ($ty:ty) => {
                if let Ok(val) = value.parse::<$ty>() {
                    return Ok(Box::new(if mode == ValueGeneratorMode::Literal {
                        let val = Literal::byte_string(value.as_bytes());

                        quote!(#val)
                    } else {
                        quote!(#val)
                    }));
                }
            };
        }

        match (mode, &ty.variant) {
            (_, MetaTypeVariant::BuildIn(BuildInMeta::U8)) => build_in!(u8),
            (_, MetaTypeVariant::BuildIn(BuildInMeta::U16)) => build_in!(u16),
            (_, MetaTypeVariant::BuildIn(BuildInMeta::U32)) => build_in!(u32),
            (_, MetaTypeVariant::BuildIn(BuildInMeta::U64)) => build_in!(u64),
            (_, MetaTypeVariant::BuildIn(BuildInMeta::U128)) => build_in!(u128),
            (_, MetaTypeVariant::BuildIn(BuildInMeta::Usize)) => build_in!(usize),

            (_, MetaTypeVariant::BuildIn(BuildInMeta::I8)) => build_in!(i8),
            (_, MetaTypeVariant::BuildIn(BuildInMeta::I16)) => build_in!(i16),
            (_, MetaTypeVariant::BuildIn(BuildInMeta::I32)) => build_in!(i32),
            (_, MetaTypeVariant::BuildIn(BuildInMeta::I64)) => build_in!(i64),
            (_, MetaTypeVariant::BuildIn(BuildInMeta::I128)) => build_in!(i128),
            (_, MetaTypeVariant::BuildIn(BuildInMeta::Isize)) => build_in!(isize),

            (C | V, MetaTypeVariant::BuildIn(BuildInMeta::F32)) => build_in!(f32),
            (C | V, MetaTypeVariant::BuildIn(BuildInMeta::F64)) => build_in!(f64),

            (C | V, MetaTypeVariant::BuildIn(BuildInMeta::Bool)) => {
                match value.to_ascii_lowercase().as_str() {
                    "true" | "yes" | "1" => return Ok(Box::new(quote!(true))),
                    "false" | "no" | "0" => return Ok(Box::new(quote!(false))),
                    _ => (),
                }
            }
            (_, MetaTypeVariant::BuildIn(BuildInMeta::Str)) => {
                return Ok(Box::new(if mode == ValueGeneratorMode::Literal {
                    let val = Literal::byte_string(value.as_bytes());

                    quote!(#val)
                } else {
                    quote!(#value)
                }));
            }
            (V, MetaTypeVariant::BuildIn(BuildInMeta::String)) => {
                let value = value.to_string();
                let string = self
                    .get_or_create_type_ref(&TypeIdent::STRING)?
                    .path
                    .clone();

                return Ok(Box::new(move |ctx: &RendererContext<'_, '_>| {
                    let string = ctx.resolve_type_for_module(&string);

                    quote!(#string::from(#value))
                }));
            }

            (mode, MetaTypeVariant::Custom(x)) => {
                if let Some(default) = &x.default {
                    return default.exec(self, value, mode);
                }
            }

            (mode, MetaTypeVariant::Enumeration(ei)) => {
                let target_type = type_ref.path.clone();

                for var in &*ei.variants {
                    if var.type_.is_none() && var.ident.name.as_str() == value {
                        let variant_ident = self
                            .types
                            .naming
                            .format_variant_ident(&var.ident.name, var.display_name.as_deref());

                        return Ok(Box::new(move |ctx: &RendererContext<'_, '_>| {
                            let target_type = ctx.resolve_type_for_module(&target_type);

                            quote!(#target_type :: #variant_ident)
                        }));
                    }

                    if let Some(target_ident) = &var.type_ {
                        if let Ok(inner) = self.make_value_renderer(target_ident, value, mode) {
                            let variant_ident = match self.state.cache.get(target_ident) {
                                Some(type_ref) if var.ident.name.is_generated() => {
                                    type_ref.path.ident().clone()
                                }
                                _ => self.types.naming.format_variant_ident(
                                    &var.ident.name,
                                    var.display_name.as_deref(),
                                ),
                            };

                            return Ok(Box::new(move |ctx: &RendererContext<'_, '_>| {
                                let inner = inner.render(ctx);
                                let target_type = ctx.resolve_type_for_module(&target_type);

                                quote!(#target_type :: #variant_ident(#inner))
                            }));
                        }
                    }
                }
            }

            (mode, MetaTypeVariant::Union(ui)) => {
                let target_type = type_ref.path.clone();

                for ty in &*ui.types {
                    if let Ok(inner) = self.make_value_renderer(&ty.type_, value, mode) {
                        let variant_ident = match self.state.cache.get(&ty.type_) {
                            Some(type_ref) if ty.type_.name.is_generated() => {
                                type_ref.path.ident().clone()
                            }
                            _ => self
                                .types
                                .naming
                                .format_variant_ident(&ty.type_.name, ty.display_name.as_deref()),
                        };

                        return Ok(Box::new(move |ctx: &RendererContext<'_, '_>| {
                            let inner = inner.render(ctx);
                            let target_type = ctx.resolve_type_for_module(&target_type);

                            quote!(#target_type :: #variant_ident ( #inner ))
                        }));
                    }
                }
            }

            (mode, MetaTypeVariant::Reference(ti)) => {
                match Occurs::from_occurs(ti.min_occurs, ti.max_occurs) {
                    Occurs::Single => return self.make_value_renderer(&ti.type_, value, mode),
                    Occurs::DynamicList
                        if value.is_empty() && mode == ValueGeneratorMode::Value =>
                    {
                        let target_type = type_ref.path.clone();

                        return Ok(Box::new(move |ctx: &RendererContext<'_, '_>| {
                            let vec = ctx.resolve_build_in("::alloc::vec::Vec");
                            let target_type = ctx.resolve_type_for_module(&target_type);

                            quote! { #target_type(#vec::new()) }
                        }));
                    }
                    _ => (),
                }
            }

            (mode, MetaTypeVariant::SimpleType(si)) => {
                let target_type = type_ref.path.clone();
                let inner = self.make_value_renderer(&si.base, value, mode)?;

                return Ok(Box::new(move |ctx: &RendererContext<'_, '_>| {
                    let inner = inner.render(ctx);
                    let target_type = ctx.resolve_type_for_module(&target_type);

                    quote! { #target_type(#inner) }
                }));
            }

            _ => (),
        }

        Err(Error::InvalidDefaultValue {
            ident: ident.clone(),
            value: value.to_owned(),
            mode,
        })
    }
}

impl<'types> Deref for Context<'_, 'types> {
    type Target = MetaData<'types>;

    fn deref(&self) -> &Self::Target {
        self.meta
    }
}

fn need_box(
    reachable: &mut BitSet<u64>,
    cache: &BTreeMap<TypeIdent, TypeRef>,
    meta: &MetaData<'_>,
    ident: &TypeIdent,
) -> bool {
    let Some(ty) = meta.types.items.get(ident) else {
        return false;
    };

    let Some(type_ref) = cache.get(ident) else {
        return false;
    };

    if !reachable.insert(type_ref.id) {
        return true;
    }

    let mut ret = false;

    match &ty.variant {
        MetaTypeVariant::Reference(x) => {
            let occurs = Occurs::from_occurs(x.min_occurs, x.max_occurs);

            if occurs.is_direct() {
                ret = need_box(reachable, cache, meta, &x.type_);
            }
        }
        MetaTypeVariant::Union(x) => {
            for var in x.types.iter() {
                ret = ret || need_box(reachable, cache, meta, &var.type_);
            }
        }
        MetaTypeVariant::Enumeration(x) => {
            for var in x.variants.iter() {
                if let Some(type_) = &var.type_ {
                    if var.use_ != Use::Prohibited {
                        ret = ret || need_box(reachable, cache, meta, type_);
                    }
                }
            }
        }
        MetaTypeVariant::All(_)
        | MetaTypeVariant::Choice(_)
        | MetaTypeVariant::Sequence(_)
        | MetaTypeVariant::ComplexType(_)
        | MetaTypeVariant::Dynamic(_)
        | MetaTypeVariant::SimpleType(_)
        | MetaTypeVariant::BuildIn(_)
        | MetaTypeVariant::Custom(_) => (),
    }

    reachable.remove(type_ref.id);

    ret
}
