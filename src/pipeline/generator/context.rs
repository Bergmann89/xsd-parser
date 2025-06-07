use std::ops::Deref;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::config::GeneratorFlags;
use crate::models::{
    code::{format_variant_ident, ModulePath},
    data::Occurs,
    meta::{BuildInMeta, MetaTypeVariant},
    schema::NamespaceId,
    Ident,
};

use super::{Error, MetaData, State, TraitInfos, TypeRef};

/// Helper type that is used to request the code generation for a specific type.
#[derive(Debug)]
pub struct Context<'a, 'types> {
    /// Meta data with different information of the generate process.
    pub meta: &'a MetaData<'types>,

    /// Identifier of the type that is currently processed.
    pub ident: &'a Ident,

    state: &'a mut State<'types>,
}

impl<'a, 'types> Context<'a, 'types> {
    pub(super) fn new(
        meta: &'a MetaData<'types>,
        ident: &'a Ident,
        state: &'a mut State<'types>,
    ) -> Self {
        Self { meta, ident, state }
    }

    pub(super) fn current_module(&self) -> Option<NamespaceId> {
        self.check_generator_flags(GeneratorFlags::USE_MODULES)
            .then_some(self.ident.ns)
            .flatten()
    }

    pub(super) fn current_type_ref(&self) -> &TypeRef {
        self.state.cache.get(self.ident).unwrap()
    }

    pub(super) fn get_trait_infos(&mut self) -> &TraitInfos {
        self.state
            .trait_infos
            .get_or_insert_with(|| TraitInfos::new(self.meta.types))
    }

    pub(super) fn get_or_create_type_ref(&mut self, ident: Ident) -> Result<&TypeRef, Error> {
        self.state.get_or_create_type_ref(self.meta, ident)
    }

    pub(super) fn make_trait_impls(&mut self) -> Result<Vec<TokenStream>, Error> {
        let ident = self.ident.clone();
        let current_ns = self.current_module();
        let module_path = ModulePath::from_namespace(current_ns, self.types);

        self.get_trait_infos()
            .get(&ident)
            .into_iter()
            .flat_map(|info| &info.traits_all)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|ident| {
                let type_ref = self.get_or_create_type_ref(ident.clone())?;
                let ident = format_ident!("{}Trait", type_ref.type_ident);
                let trait_type = type_ref.to_ident_path().with_ident(ident);
                let trait_ident = trait_type.relative_to(&module_path);

                Ok(trait_ident)
            })
            .collect::<Result<Vec<_>, _>>()
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn get_default(
        &mut self,
        current_ns: Option<NamespaceId>,
        default: &str,
        ident: &Ident,
    ) -> Result<TokenStream, Error> {
        let types = self.types;
        let ty = types
            .items
            .get(ident)
            .ok_or_else(|| Error::UnknownType(ident.clone()))?;
        let type_ref = self.get_or_create_type_ref(ident.clone())?;

        macro_rules! build_in {
            ($ty:ty) => {
                if let Ok(val) = default.parse::<$ty>() {
                    return Ok(quote!(#val));
                }
            };
        }

        match &ty.variant {
            MetaTypeVariant::BuildIn(BuildInMeta::U8) => build_in!(u8),
            MetaTypeVariant::BuildIn(BuildInMeta::U16) => build_in!(u16),
            MetaTypeVariant::BuildIn(BuildInMeta::U32) => build_in!(u32),
            MetaTypeVariant::BuildIn(BuildInMeta::U64) => build_in!(u64),
            MetaTypeVariant::BuildIn(BuildInMeta::U128) => build_in!(u128),
            MetaTypeVariant::BuildIn(BuildInMeta::Usize) => build_in!(usize),

            MetaTypeVariant::BuildIn(BuildInMeta::I8) => build_in!(i8),
            MetaTypeVariant::BuildIn(BuildInMeta::I16) => build_in!(i16),
            MetaTypeVariant::BuildIn(BuildInMeta::I32) => build_in!(i32),
            MetaTypeVariant::BuildIn(BuildInMeta::I64) => build_in!(i64),
            MetaTypeVariant::BuildIn(BuildInMeta::I128) => build_in!(i128),
            MetaTypeVariant::BuildIn(BuildInMeta::Isize) => build_in!(isize),

            MetaTypeVariant::BuildIn(BuildInMeta::F32) => build_in!(f32),
            MetaTypeVariant::BuildIn(BuildInMeta::F64) => build_in!(f64),

            MetaTypeVariant::BuildIn(BuildInMeta::Bool) => {
                match default.to_ascii_lowercase().as_str() {
                    "true" | "yes" | "1" => return Ok(quote!(true)),
                    "false" | "no" | "0" => return Ok(quote!(false)),
                    _ => (),
                }
            }
            MetaTypeVariant::BuildIn(BuildInMeta::String) => {
                return Ok(quote!(String::from(#default)));
            }

            MetaTypeVariant::Custom(x) => {
                if let Some(x) = x.default(default) {
                    return Ok(x);
                }
            }

            MetaTypeVariant::Enumeration(ei) => {
                let module_path = ModulePath::from_namespace(current_ns, types);
                let target_type = type_ref.to_ident_path().relative_to(&module_path);

                for var in &*ei.variants {
                    if var.type_.is_none() && var.ident.name.as_str() == default {
                        let variant_ident =
                            format_variant_ident(&var.ident.name, var.display_name.as_deref());

                        return Ok(quote!(#target_type :: #variant_ident));
                    }

                    if let Some(target_ident) = &var.type_ {
                        if let Ok(default) = self.get_default(current_ns, default, target_ident) {
                            let variant_ident = match self.state.cache.get(target_ident) {
                                Some(type_ref) if var.ident.name.is_generated() => {
                                    type_ref.type_ident.clone()
                                }
                                _ => format_variant_ident(
                                    &var.ident.name,
                                    var.display_name.as_deref(),
                                ),
                            };

                            return Ok(quote!(#target_type :: #variant_ident(#default)));
                        }
                    }
                }
            }

            MetaTypeVariant::Union(ui) => {
                let module_path = ModulePath::from_namespace(current_ns, types);
                let target_type = type_ref.to_ident_path().relative_to(&module_path);

                for ty in &*ui.types {
                    if let Ok(code) = self.get_default(current_ns, default, &ty.type_) {
                        let variant_ident = match self.state.cache.get(&ty.type_) {
                            Some(type_ref) if ty.type_.name.is_generated() => {
                                type_ref.type_ident.clone()
                            }
                            _ => format_variant_ident(&ty.type_.name, ty.display_name.as_deref()),
                        };

                        return Ok(quote! {
                            #target_type :: #variant_ident ( #code )
                        });
                    }
                }
            }

            MetaTypeVariant::Reference(ti) => {
                match Occurs::from_occurs(ti.min_occurs, ti.max_occurs) {
                    Occurs::Single => return self.get_default(current_ns, default, &ti.type_),
                    Occurs::DynamicList if default.is_empty() => {
                        let module_path = ModulePath::from_namespace(current_ns, types);
                        let target_type = type_ref.to_ident_path().relative_to(&module_path);

                        return Ok(quote! { #target_type(Vec::new()) });
                    }
                    _ => (),
                }
            }

            _ => (),
        }

        Err(Error::InvalidDefaultValue(
            ident.clone(),
            default.to_owned(),
        ))
    }
}

impl<'types> Deref for Context<'_, 'types> {
    type Target = MetaData<'types>;

    fn deref(&self) -> &Self::Target {
        self.meta
    }
}
