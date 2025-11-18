use proc_macro2::{Ident as Ident2, TokenStream};

use crate::models::{
    data::{ConstrainsData, Occurs},
    meta::SimpleMeta,
};

use super::PathData;

/// Contains additional information for the rendering process of a
/// [`MetaTypeVariant::SimpleType`](crate::models::meta::MetaTypeVariant::SimpleType)
/// type.
#[derive(Debug)]
pub struct SimpleData<'types> {
    /// Reference to the original type information.
    pub meta: &'types SimpleMeta,

    /// Occurrence of the referenced type within this type.
    pub occurs: Occurs,

    /// Code generator data for the constrains of the type.
    pub constrains: ConstrainsData<'types>,

    /// The identifier of the rendered type.
    pub type_ident: Ident2,

    /// Actual target type of this referenced type.
    pub target_type: PathData,

    /// List of traits that needs to be implemented by this type.
    pub trait_impls: Vec<TokenStream>,
}
