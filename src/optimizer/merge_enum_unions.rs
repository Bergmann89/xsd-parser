use crate::types::{
    type_::SimpleTypeVariant, EnumerationInfo, Ident, Types, UnionInfo, UnionTypeInfo, VariantInfo,
    VecHelper,
};

use super::{Error, TypeTransformer};

/// This will flatten all enumeration and union types.
///
/// For details see [`merge_enum_union`](Self::merge_enum_union).
#[derive(Debug)]
pub struct MergeEnumUnions;

impl TypeTransformer for MergeEnumUnions {
    type Error = super::Error;

    fn transform(&self, types: &mut Types) -> Result<(), Error> {
        tracing::debug!("merge_enum_unions");

        let idents = types
            .simple_types_iter()
            .filter_map(|(ident, type_)| {
                if matches!(&type_.variant, SimpleTypeVariant::Union(_)) {
                    Some(ident)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        for ident in idents {
            Self::merge_enum_union(types, ident)?;
        }

        Ok(())
    }
}

impl MergeEnumUnions {
    /// This will flatten the union identified by `ident` to one single union.
    ///
    /// This will merge the nested union and enum types of the union identified
    /// by `ident` to one single enum type.
    ///
    /// # Errors
    ///
    /// Returns an error if the passed `ident` could not be found,
    /// or is not an union.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema.
    /// ```xml
    #[doc = include_str!("../../tests/optimizer/union_flatten.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected0/merge_enum_unions.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected1/merge_enum_unions.rs")]
    /// ```
    pub fn merge_enum_union(types: &mut Types, ident: Ident) -> Result<(), Error> {
        tracing::debug!("merge_enum_union(ident={ident:?})");

        let Some(variant) = types.get_simple_type(&ident).map(|x| &x.variant) else {
            return Err(Error::UnknownType(ident));
        };

        let SimpleTypeVariant::Union(_) = variant else {
            return Err(Error::ExpectedUnion(ident));
        };

        let mut next = None;

        Self::merge_enum_union_impl(types, &ident, None, &mut next);

        if let Some(next) = next {
            let ty = types.get_simple_type_mut(&ident).unwrap();
            ty.variant = next;
        }

        Ok(())
    }

    fn merge_enum_union_impl(
        types: &Types,
        ident: &Ident,
        display_name: Option<&str>,
        next: &mut Option<SimpleTypeVariant>,
    ) {
        let Some(type_) = types.get_simple_type(&ident).map(|x| &x.variant) else {
            return;
        };

        match type_ {
            SimpleTypeVariant::Union(x) => {
                for t in &*x.types {
                    Self::merge_enum_union_impl(types, &t.type_, t.display_name.as_deref(), next);
                }
            }
            SimpleTypeVariant::Enumeration(x) => {
                *next = match next.take() {
                    None => Some(SimpleTypeVariant::Enumeration(EnumerationInfo::default())),
                    Some(SimpleTypeVariant::Enumeration(ei)) => {
                        Some(SimpleTypeVariant::Enumeration(ei))
                    }
                    Some(SimpleTypeVariant::Union(ui)) => {
                        let mut ei = EnumerationInfo::default();

                        for t in ui.types.0 {
                            let var = ei.variants.find_or_insert(t.type_.clone(), |ident| {
                                VariantInfo::new(ident).with_type(Some(t.type_.clone()))
                            });
                            var.display_name = t.display_name;
                        }

                        Some(SimpleTypeVariant::Enumeration(ei))
                    }
                    _ => crate::unreachable!(),
                };

                let Some(SimpleTypeVariant::Enumeration(ei)) = next else {
                    crate::unreachable!();
                };

                for var in &*x.variants {
                    let new_var = ei.variants.find_or_insert(var.ident.clone(), |ident| {
                        VariantInfo::new(ident).with_type(var.type_.clone())
                    });
                    new_var.display_name.clone_from(&var.display_name);
                }
            }
            SimpleTypeVariant::Reference(x) if x.is_single() => {
                Self::merge_enum_union_impl(types, &x.type_, display_name, next);
            }
            _ => {
                if next.is_none() {
                    *next = Some(SimpleTypeVariant::Union(UnionInfo::default()));
                }

                match next {
                    Some(SimpleTypeVariant::Union(ui)) => {
                        let mut ti = UnionTypeInfo::new(ident.clone());
                        ti.display_name = display_name.map(ToOwned::to_owned);

                        ui.types.push(ti);
                    }
                    Some(SimpleTypeVariant::Enumeration(ei)) => {
                        let var = ei.variants.find_or_insert(ident.clone(), |x| {
                            VariantInfo::new(x).with_type(Some(ident.clone()))
                        });
                        var.display_name = display_name.map(ToOwned::to_owned);
                    }
                    _ => crate::unreachable!(),
                }
            }
        }
    }
}
