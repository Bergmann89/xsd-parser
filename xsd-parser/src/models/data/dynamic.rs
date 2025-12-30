use proc_macro2::{Ident as Ident2, Literal};

use crate::models::{data::PathData, meta::DynamicMeta, TypeIdent};

/// Contains additional information for the rendering process of a
/// [`MetaTypeVariant::Dynamic`](crate::models::meta::MetaTypeVariant::Dynamic)
/// type.
#[derive(Debug)]
pub struct DynamicData<'types> {
    /// Reference to the original type information.
    pub meta: &'types DynamicMeta,

    /// The identifier of the rendered type.
    pub type_ident: Ident2,

    /// The identifier of the trait that needs to be implemented
    /// by the derived types.
    pub trait_ident: Ident2,

    /// Identifier of the deserializer for this type.
    pub deserializer_ident: Ident2,

    /// List of additional traits that need to be implemented by the derived
    /// types (if this type was inherited from another dynamic type).
    pub sub_traits: Option<Vec<PathData>>,

    /// List of derived types.
    pub derived_types: Vec<DerivedType>,
}

/// Represents a derived type used by [`DynamicData`]
#[derive(Debug)]
pub struct DerivedType {
    /// Identifier of the derived type.
    pub ident: TypeIdent,

    /// Name of the derived type as byte string literal.
    pub b_name: Literal,

    /// The actual target type of this derived type information.
    pub target_type: PathData,

    /// Name of the variant used for this derived type information.
    pub variant_ident: Ident2,
}
