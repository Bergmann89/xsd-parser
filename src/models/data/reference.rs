use proc_macro2::{Ident as Ident2, TokenStream};

use crate::config::TypedefMode;
use crate::models::{code::IdentPath, meta::ReferenceMeta};

use super::Occurs;

/// Contains additional information for the rendering process
/// of a [`MetaTypeVariant::Reference`] type.
#[derive(Debug)]
pub struct ReferenceData<'types> {
    /// Reference to the original type information.
    pub meta: &'types ReferenceMeta,

    /// Typedef mode that should be used to render this reference type.
    pub mode: TypedefMode,

    /// Occurrence of the referenced type within this type.
    pub occurs: Occurs,

    /// The identifier of the rendered type.
    pub type_ident: Ident2,

    /// Actual target type of this referenced type.
    pub target_type: IdentPath,

    /// List of traits that needs to be implemented by this type.
    pub trait_impls: Vec<TokenStream>,
}
