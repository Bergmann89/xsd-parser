use std::borrow::Cow;

use proc_macro2::{Ident as Ident2, Literal, TokenStream};

use crate::models::{
    data::{PathData, TagName},
    meta::{DerivedTypeMeta, DynamicMeta},
    TypeIdent,
};

/// Contains additional information for the rendering process of a
/// [`MetaTypeVariant::Dynamic`](crate::models::meta::MetaTypeVariant::Dynamic)
/// type.
#[derive(Debug)]
pub struct DynamicData<'types> {
    /// Reference to the original type information.
    pub meta: Cow<'types, DynamicMeta>,

    /// The identifier of the rendered type.
    pub type_ident: Ident2,

    /// The identifier of the trait that needs to be implemented
    /// by the derived types.
    pub trait_ident: Ident2,

    /// Identifier of the deserializer for this type.
    pub deserializer_ident: Ident2,

    /// List of traits that needs to be implemented by this type.
    pub trait_impls: Vec<TokenStream>,

    /// Name of the XML tag of the element.
    pub tag_name: TagName<'types>,

    /// List of additional traits that need to be implemented by the derived
    /// types (if this type was inherited from another dynamic type).
    pub sub_traits: Option<Vec<PathData>>,

    /// List of derived types.
    pub derived_types: Vec<DerivedType<'types>>,
}

/// Represents a derived type used by [`DynamicData`]
#[derive(Debug)]
pub struct DerivedType<'types> {
    /// Identifier of the derived type (the key of the derived type in the original [`DynamicMeta`]).
    pub key: TypeIdent,

    /// Reference to the original type information.
    pub meta: Cow<'types, DerivedTypeMeta>,

    /// Name of the derived type as byte string literal.
    pub b_name: Literal,

    /// Name of the XML tag of the element.
    pub tag_name: TagName<'types>,

    /// The actual target type of this derived type information.
    pub target_type: PathData,

    /// Name of the variant used for this derived type information.
    pub variant_ident: Ident2,
}
