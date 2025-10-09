use proc_macro2::{Ident as Ident2, Literal, TokenStream};

use crate::models::{
    data::{ConstrainsData, PathData},
    meta::{UnionMeta, UnionMetaType},
};

/// Contains additional information for the rendering process of a
/// [`MetaTypeVariant::Union`](crate::models::meta::MetaTypeVariant::Union) type.
#[derive(Debug)]
pub struct UnionData<'types> {
    /// Reference to the original type information.
    pub meta: &'types UnionMeta,

    /// Code generator data for the constrains of the type.
    pub constrains: ConstrainsData<'types>,

    /// The identifier of the rendered type.
    pub type_ident: Ident2,

    /// List of variants contained in this union.
    pub variants: Vec<UnionTypeVariant<'types>>,

    /// List of traits that needs to be implemented by this type.
    pub trait_impls: Vec<TokenStream>,
}

/// Type variant used in [`UnionData`].
#[derive(Debug)]
pub struct UnionTypeVariant<'types> {
    /// Reference to the original type information.
    pub meta: &'types UnionMetaType,

    /// Name of the attribute as string.
    pub s_name: String,

    /// Name of the attribute as byte string literal.
    pub b_name: Literal,

    /// The type that is stored by the this variant.
    pub target_type: PathData,

    /// Identifier of this variant.
    pub variant_ident: Ident2,
}
