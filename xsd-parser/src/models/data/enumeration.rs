use proc_macro2::{Ident as Ident2, Literal, TokenStream};

use crate::models::data::{ConstrainsData, PathData};
use crate::models::meta::{EnumerationMeta, EnumerationMetaVariant};
use crate::pipeline::renderer::ValueRendererBox;

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
    pub variants: Vec<EnumerationDataVariant<'types>>,

    /// List of traits that needs to be implemented by this type.
    pub trait_impls: Vec<TokenStream>,

    /// Simple base type of this enumeration.
    pub simple_base_type: Option<PathData>,
}

/// Represents a enumeration variant used by [`EnumerationData`].
#[derive(Debug)]
pub struct EnumerationDataVariant<'types> {
    /// Reference to the original type information.
    pub meta: &'types EnumerationMetaVariant,

    /// Name of the attribute as string.
    pub s_name: String,

    /// Name of the attribute as byte string literal.
    pub b_name: Literal,

    /// Value renderer to render either a constant or a getter function for
    /// the value of this variant represented at its simple base type.
    pub value: EnumerationVariantValue,

    /// Name of this variant.
    pub variant_ident: Ident2,

    /// Target type of this variant.
    pub target_type: Option<PathData>,

    /// Additional attributes that will be added to the variant.
    pub extra_attributes: Vec<TokenStream>,
}

/// Value renderer to render either a constant or a getter function for
/// the value of this variant represented at its simple base type.
#[derive(Debug)]
pub enum EnumerationVariantValue {
    /// No value renderer, because the simple base type of this enumeration is not supported.
    None,

    /// Value renderer to render a byte string literal for the value of this variant.
    ByteLiteral(Ident2, ValueRendererBox),

    /// Value renderer to render a constant for the value of this variant.
    Constant(Ident2, ValueRendererBox),
}
