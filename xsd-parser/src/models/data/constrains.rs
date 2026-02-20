use std::ops::{Bound, Range};

use crate::models::meta::Constrains;
use crate::pipeline::renderer::ValueRendererBox;

/// Code generator data for the constrains of a specific type.
#[derive(Debug)]
pub struct ConstrainsData<'types> {
    /// Reference to the original constrains information.
    pub meta: &'types Constrains,

    /// The the value should ne in as token stream literals.
    pub range: Range<Bound<ValueRendererBox>>,
}
