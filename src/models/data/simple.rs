use std::ops::{Bound, Range};

use proc_macro2::{Ident as Ident2, TokenStream};

use crate::models::{data::Occurs, meta::SimpleMeta};

use super::PathData;

/// Contains additional information for the rendering process of a
/// [`MetaTypeVariant::SimpleType`](crate::models::meta::MetaTypeVariant::SimpleType)
/// type.
#[derive(Debug)]
pub struct SimpleData<'types> {
    /// Reference to the original type information.
    pub meta: &'types SimpleMeta,

    /// The the value should ne in as token stream literals.
    pub range: Range<Bound<TokenStream>>,

    /// Occurrence of the referenced type within this type.
    pub occurs: Occurs,

    /// The identifier of the rendered type.
    pub type_ident: Ident2,

    /// Actual target type of this referenced type.
    pub target_type: PathData,

    /// List of traits that needs to be implemented by this type.
    pub trait_impls: Vec<TokenStream>,
}

impl SimpleData<'_> {
    /// Returns `true` if this simple type needs value validation, `false` otherwise.
    #[must_use]
    pub fn need_value_validation(&self) -> bool {
        self.meta.range.start != Bound::Unbounded
            || self.meta.range.end != Bound::Unbounded
            || self.meta.min_length.is_some()
            || self.meta.max_length.is_some()
    }

    /// Returns `true` if this simple type needs string validation, `false` otherwise.
    #[must_use]
    pub fn need_string_validation(&self) -> bool {
        self.meta.pattern.is_some()
            || self.meta.total_digits.is_some()
            || self.meta.fraction_digits.is_some()
    }
}
