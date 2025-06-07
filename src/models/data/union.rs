use proc_macro2::{Ident as Ident2, TokenStream};

use crate::models::{
    code::IdentPath,
    meta::{UnionMeta, UnionMetaType},
};

/// Contains additional information for the rendering process
/// of a [`MetaTypeVariant::Union`] type.
#[derive(Debug)]
pub struct UnionData<'types> {
    /// Reference to the original type information.
    pub meta: &'types UnionMeta,

    /// The identifier of the rendered type.
    pub type_ident: Ident2,

    /// List of variants contained in this union.
    pub variants: Vec<UnionTypeVariant<'types>>,

    /// List of traits that needs to be implemented by this type.
    pub trait_impls: Vec<TokenStream>,
}

/// Type variant used in [`UnionType`].
#[derive(Debug)]
pub struct UnionTypeVariant<'types> {
    /// Reference to the original type information.
    pub info: &'types UnionMetaType,

    /// The type that is stored by the this variant.
    pub target_type: IdentPath,

    /// Identifier of this variant.
    pub variant_ident: Ident2,
}
