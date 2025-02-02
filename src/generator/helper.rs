use std::collections::{BTreeMap, HashSet};

use proc_macro2::TokenStream;
use quote::quote;
use tracing::instrument;

use crate::schema::{xs::Use, NamespaceId};
use crate::types::{BuildInInfo, ComplexInfo, Ident, Type, Types};

use super::misc::{format_type_ref, format_variant_ident, ContentMode, Occurs, TypeMode, TypeRef};
use super::{Error, GenerateFlags, Generator};

/* Helpers */

impl Generator<'_> {
    pub(super) fn check_generate_flags(&self, mode: GenerateFlags) -> bool {
        self.generate_flags.intersects(mode)
    }

    pub(super) fn target_mode(&self, mode: TypeMode) -> TypeMode {
        match self.content_mode {
            ContentMode::Auto => mode,
            ContentMode::Enum => TypeMode::Choice,
            ContentMode::Struct => TypeMode::All,
        }
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn get_default(
        &mut self,
        current_ns: Option<NamespaceId>,
        default: &str,
        ident: &Ident,
    ) -> Result<TokenStream, Error> {
        let ty = self
            .types
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

        match ty {
            Type::BuildIn(BuildInInfo::U8) => build_in!(u8),
            Type::BuildIn(BuildInInfo::U16) => build_in!(u16),
            Type::BuildIn(BuildInInfo::U32) => build_in!(u32),
            Type::BuildIn(BuildInInfo::U64) => build_in!(u64),
            Type::BuildIn(BuildInInfo::U128) => build_in!(u128),
            Type::BuildIn(BuildInInfo::Usize) => build_in!(usize),

            Type::BuildIn(BuildInInfo::I8) => build_in!(i8),
            Type::BuildIn(BuildInInfo::I16) => build_in!(i16),
            Type::BuildIn(BuildInInfo::I32) => build_in!(i32),
            Type::BuildIn(BuildInInfo::I64) => build_in!(i64),
            Type::BuildIn(BuildInInfo::I128) => build_in!(i128),
            Type::BuildIn(BuildInInfo::Isize) => build_in!(isize),

            Type::BuildIn(BuildInInfo::F32) => build_in!(f32),
            Type::BuildIn(BuildInInfo::F64) => build_in!(f64),

            Type::BuildIn(BuildInInfo::Bool) => match default.to_ascii_lowercase().as_str() {
                "true" | "yes" | "1" => return Ok(quote!(true)),
                "false" | "no" | "0" => return Ok(quote!(false)),
                _ => (),
            },
            Type::BuildIn(BuildInInfo::String) => {
                return Ok(quote!(String::from(#default)));
            }
            Type::BuildIn(BuildInInfo::Custom(x)) => {
                if let Some(x) = x.default(default) {
                    return Ok(x);
                }
            }

            Type::Enumeration(ei) => {
                let target_type = format_type_ref(current_ns, type_ref);

                for var in &*ei.variants {
                    if var.type_.is_none()
                        && matches!(var.ident.name.as_str(), Some(x) if x == default)
                    {
                        let variant_ident =
                            format_variant_ident(&var.ident.name, var.display_name.as_deref());

                        return Ok(quote!(#target_type :: #variant_ident));
                    }

                    if let Some(target_ident) = &var.type_ {
                        if let Ok(default) = self.get_default(current_ns, default, target_ident) {
                            let variant_ident = match self.cache.get(target_ident) {
                                Some(type_ref) if var.ident.name.is_unnamed() => {
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

            Type::Union(ui) => {
                let target_type = format_type_ref(current_ns, type_ref);

                for ty in &*ui.types {
                    if let Ok(code) = self.get_default(current_ns, default, &ty.type_) {
                        let variant_ident =
                            format_variant_ident(&ty.type_.name, ty.display_name.as_deref());

                        return Ok(quote! {
                            #target_type :: #variant_ident ( #code )
                        });
                    }
                }
            }

            Type::Reference(ti) => match Occurs::from_occurs(ti.min_occurs, ti.max_occurs) {
                Occurs::Single => return self.get_default(current_ns, default, &ti.type_),
                Occurs::DynamicList if default.is_empty() => {
                    let type_ident = format_type_ref(current_ns, type_ref);

                    return Ok(quote! { #type_ident(Vec::new()) });
                }
                _ => (),
            },

            _ => (),
        }

        Err(Error::InvalidDefaultValue(
            ident.clone(),
            default.to_owned(),
        ))
    }
}

/* Walk */

pub(super) struct Walk<'a> {
    types: &'a Types,
    cache: &'a BTreeMap<Ident, TypeRef>,
    visit: HashSet<Ident>,
}

impl<'a> Walk<'a> {
    pub(super) fn new(types: &'a Types, cache: &'a BTreeMap<Ident, TypeRef>) -> Self {
        Self {
            types,
            cache,
            visit: HashSet::new(),
        }
    }

    pub(super) fn is_loop(&mut self, origin: &Ident, current: &Ident) -> bool {
        if !self.visit.insert(current.clone()) {
            return false;
        }

        if origin == current {
            return true;
        }

        let mut ret = false;

        match self.types.get(current) {
            Some(Type::Reference(x)) => {
                let occurs = Occurs::from_occurs(x.min_occurs, x.max_occurs);

                ret = occurs.is_direct() && self.is_loop(origin, &x.type_);
            }
            Some(Type::Union(x)) => {
                for var in x.types.iter() {
                    if self.is_loop(origin, &var.type_) {
                        ret = true;
                        break;
                    }
                }
            }
            Some(Type::Enumeration(x)) => {
                for var in x.variants.iter() {
                    if let Some(type_) = &var.type_ {
                        if var.use_ != Use::Prohibited && self.is_loop(origin, type_) {
                            ret = true;
                            break;
                        }
                    }
                }
            }
            Some(Type::ComplexType(ComplexInfo {
                content: Some(content),
                min_occurs,
                max_occurs,
                ..
            })) => {
                let occurs = Occurs::from_occurs(*min_occurs, *max_occurs);

                ret = occurs.is_direct() && self.is_loop(origin, content);
            }
            Some(Type::All(x) | Type::Choice(x) | Type::Sequence(x)) => {
                for f in x.elements.iter() {
                    let already_boxed = self
                        .cache
                        .get(current)
                        .is_some_and(|x| x.boxed_elements.contains(&f.ident));
                    if already_boxed {
                        break;
                    }

                    let occurs = Occurs::from_occurs(f.min_occurs, f.max_occurs);
                    if occurs.is_direct() && self.is_loop(origin, &f.type_) {
                        ret = true;
                        break;
                    }
                }
            }
            _ => (),
        }

        self.visit.remove(current);

        ret
    }
}
