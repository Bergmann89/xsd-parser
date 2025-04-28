use crate::types::{Name, ReferenceInfo, TypeVariant, Types};

use super::TypeTransformer;

/// This will remove any enum variant that has an empty string as name.
///
/// # Examples
///
/// Consider the following XML schema:
/// ```xml
#[doc = include_str!("../../tests/optimizer/enum_empty_variant.xsd")]
/// ```
///
/// Without this optimization this will result in the following code:
/// ```rust
#[doc = include_str!("../../tests/optimizer/expected0/remove_empty_enum_variants.rs")]
/// ```
///
/// With this optimization the following code is generated:
/// ```rust
#[doc = include_str!("../../tests/optimizer/expected1/remove_empty_enum_variants.rs")]
/// ```
#[derive(Debug)]
pub struct RemoveEmptyEnumVariants;

impl TypeTransformer for RemoveEmptyEnumVariants {
    type Error = super::Error;

    fn transform(self, types: &mut Types) -> Result<(), super::Error> {
        tracing::debug!("remove_empty_enum_variants");

        for type_ in types.types.values_mut() {
            if let TypeVariant::Enumeration(x) = &mut type_.variant {
                x.variants
                    .retain(|x| !matches!(&x.ident.name, Name::Named(x) if x.is_empty()));
            }
        }

        Ok(())
    }
}

/// This will replace the enum with a type reference to the enums base type
/// if the enum does not have any variant.
///
/// This optimization is usually used in combination with
/// [`RemoveEmptyEnumVariants`].
///
/// # Examples
///
/// Consider the following XML schema:
/// ```xml
#[doc = include_str!("../../tests/optimizer/enum_empty.xsd")]
/// ```
///
/// Without this optimization this will result in the following code:
/// ```rust
#[doc = include_str!("../../tests/optimizer/expected0/remove_empty_enums.rs")]
/// ```
///
/// With this optimization (and the [`RemoveEmptyEnumVariants`])
/// the following code is generated:
/// ```rust
#[doc = include_str!("../../tests/optimizer/expected1/remove_empty_enums.rs")]
/// ```
#[derive(Debug)]
pub struct RemoveEmptyEnums;

impl TypeTransformer for RemoveEmptyEnums {
    type Error = super::Error;

    fn transform(self, types: &mut Types) -> Result<(), super::Error> {
        tracing::debug!("remove_empty_enums");

        for type_ in types.types.values_mut() {
            if let TypeVariant::Enumeration(x) = &mut type_.variant {
                if x.variants.is_empty() {
                    if let Some(base) = x.base.as_ident() {
                        type_.variant = TypeVariant::Reference(ReferenceInfo::new(base.clone()));
                    }
                }
            }
        }

        Ok(())
    }
}
