use crate::models::{meta::MetaTypeVariant, schema::MaxOccurs, TypeIdent};

use super::{Error, Optimizer};

impl Optimizer {
    /// This will merge the cardinality of each element of the complex choice
    /// type identified by `ident` with the cardinality of the types content.
    ///
    /// # Errors
    ///
    /// Returns an error if the passed `ident` could not be found, the referenced
    /// type is not complex type or the complex types content is not a choice.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema.
    /// ```xml
    #[doc = include_str!("../../../tests/optimizer/complex_choice.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../../tests/optimizer/expected0/merge_choice_cardinalities.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../../tests/optimizer/expected1/merge_choice_cardinalities.rs")]
    /// ```
    pub fn merge_choice_cardinality(mut self, ident: TypeIdent) -> Result<Self, Error> {
        tracing::debug!("merge_choice_cardinality(ident={ident:?})");

        let Some(ty) = self.types.get_variant(&ident) else {
            return Err(Error::UnknownType(ident));
        };

        let MetaTypeVariant::ComplexType(ci) = ty else {
            return Err(Error::ExpectedComplexType(ident));
        };

        let Some(content_ident) = ci.content.clone() else {
            return Err(Error::MissingContentType(ident));
        };

        let Some(MetaTypeVariant::Choice(ci)) = self.types.get_variant_mut(&content_ident) else {
            return Err(Error::ExpectedComplexChoice(ident));
        };

        let mut min = 1;
        let mut max = MaxOccurs::Bounded(1);

        for element in &mut *ci.elements {
            min = min.min(element.min_occurs);
            max = max.max(element.max_occurs);

            element.min_occurs = 1;
            element.max_occurs = MaxOccurs::Bounded(1);
        }

        let Some(MetaTypeVariant::ComplexType(ci)) = self.types.get_variant_mut(&ident) else {
            unreachable!();
        };

        ci.min_occurs = min.min(ci.min_occurs);
        ci.max_occurs = max.max(ci.max_occurs);

        Ok(self)
    }

    /// This merge the cardinality of all elements of a choice with the content of the choice for
    /// all choice types.
    ///
    /// For details see [`merge_choice_cardinality`](Self::merge_choice_cardinality).
    pub fn merge_choice_cardinalities(mut self) -> Self {
        tracing::debug!("merge_choice_cardinalities");

        let idents = self
            .types
            .items
            .iter()
            .filter_map(|(ident, type_)| {
                if matches!(&type_.variant, MetaTypeVariant::ComplexType(ci) if ci.has_complex_choice_content(&self.types)) {
                    Some(ident)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        for ident in idents {
            self = self.merge_choice_cardinality(ident).unwrap();
        }

        self
    }
}
