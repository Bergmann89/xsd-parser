use std::borrow::Cow;

use proc_macro2::{Ident as Ident2, TokenStream};

use crate::config::TypedefMode;
use crate::models::{
    data::{Occurs, PathData},
    meta::ReferenceMeta,
};

/// Contains additional information for the rendering process of a
/// [`MetaTypeVariant::Reference`](crate::models::meta::MetaTypeVariant::Reference)
/// type or an alias to a native Rust type.
#[derive(Debug)]
pub struct ReferenceData<'types> {
    /// Reference to the original type information.
    ///
    /// Native custom-type aliases use synthetic placeholder metadata that points
    /// to the original [`CustomMeta`](crate::models::meta::CustomMeta) in
    /// [`MetaTypes`](crate::models::meta::MetaTypes), not to a separate generated
    /// target. Custom renderers must use [`target_type`](Self::target_type) for
    /// the Rust alias target rather than follow `meta.type_` as another generated
    /// reference.
    pub meta: Cow<'types, ReferenceMeta>,

    /// Typedef mode that should be used to render this reference type.
    pub mode: TypedefMode,

    /// Occurrence of the referenced type within this type.
    pub occurs: Occurs,

    /// The identifier of the rendered type.
    pub type_ident: Ident2,

    /// Actual target type of this referenced type.
    pub target_type: PathData,

    /// List of traits that needs to be implemented by this type.
    pub trait_impls: Vec<TokenStream>,
}
