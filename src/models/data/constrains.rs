use std::ops::{Bound, Range};

use proc_macro2::TokenStream;

use crate::models::meta::Constrains;

/// Code generator data for the constrains of a specific type.
#[derive(Debug)]
pub struct ConstrainsData<'types> {
    /// Reference to the original constrains information.
    pub meta: &'types Constrains,

    /// The the value should ne in as token stream literals.
    pub range: Range<Bound<TokenStream>>,
}
