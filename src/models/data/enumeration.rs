use proc_macro2::{Ident as Ident2, Literal, TokenStream};

use crate::models::{
    data::{ConstrainsData, PathData},
    meta::{EnumerationMeta, EnumerationMetaVariant},
};

/// Contains additional information for the rendering process of a
/// [`MetaTypeVariant::Enumeration`](crate::models::meta::MetaTypeVariant::Enumeration)
/// type.
#[derive(Debug)]
pub struct EnumerationData<'types> {
    /// Reference to the original type information.
    pub meta: &'types EnumerationMeta,

    /// Code generator data for the constrains of the type.
    pub constrains: ConstrainsData<'types>,

    /// The identifier of the rendered type.
    pub type_ident: Ident2,

    /// List of variants of this enumeration.
    pub variants: Vec<EnumerationTypeVariant<'types>>,

    /// List of traits that needs to be implemented by this type.
    pub trait_impls: Vec<TokenStream>,
}

/// Represents a enumeration variant used by [`EnumerationData`].
#[derive(Debug)]
pub struct EnumerationTypeVariant<'types> {
    /// Reference to the original type information.
    pub meta: &'types EnumerationMetaVariant,

    /// Name of the attribute as string.
    pub s_name: String,

    /// Name of the attribute as byte string literal.
    pub b_name: Literal,

    /// Name of this variant.
    pub variant_ident: Ident2,

    /// Target type of this variant.
    pub target_type: Option<PathData>,

    /// Additional attributes that will be added to the variant.
    pub extra_attributes: Vec<TokenStream>,
}
