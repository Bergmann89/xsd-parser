use crate::types::{Name, ReferenceInfo, TypeVariant};

use super::Optimizer;

impl Optimizer {
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
    pub fn remove_empty_enum_variants(mut self) -> Self {
        tracing::debug!("remove_empty_enum_variants");

        for type_ in self.types.types.values_mut() {
            if let TypeVariant::Enumeration(x) = &mut type_.variant {
                x.variants
                    .retain(|x| !matches!(&x.ident.name, Name::Named(x) if x.is_empty()));
            }
        }

        self
    }

    /// This will replace the enum with a type reference to the enums base type
    /// if the enum does not have any variant.
    ///
    /// This optimization is usually used in combination with
    /// [`remove_empty_enum_variants`](Self::remove_empty_enum_variants).
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
    /// With this optimization (and the [`remove_empty_enum_variants`](Self::remove_empty_enum_variants))
    /// the following code is generated:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected1/remove_empty_enums.rs")]
    /// ```
    pub fn remove_empty_enums(mut self) -> Self {
        tracing::debug!("remove_empty_enums");

        for type_ in self.types.types.values_mut() {
            if let TypeVariant::Enumeration(x) = &mut type_.variant {
                if x.variants.is_empty() {
                    if let Some(base) = x.base.as_ident() {
                        self.typedefs = None;
                        type_.variant = TypeVariant::Reference(ReferenceInfo::new(base.clone()));
                    }
                }
            }
        }

        self
    }
}
