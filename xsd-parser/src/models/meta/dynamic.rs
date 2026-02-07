//! Contains the [`DynamicMeta`] type information and all related types.

use std::hash::{Hash, Hasher};

use crate::models::TypeIdent;

use super::{MetaTypes, TypeEq};

/// Type information that contains data about dynamic types.
#[derive(Default, Debug, Clone)]
pub struct DynamicMeta {
    /// Base type of the dynamic type.
    pub type_: Option<TypeIdent>,

    /// List of derived types.
    pub derived_types: Vec<DerivedTypeMeta>,
}

/// Meta^ information about a derived type.
#[derive(Debug, Clone)]
pub struct DerivedTypeMeta {
    /// Identifier of the derived type.
    pub type_: TypeIdent,

    /// Name of the element to use inside the generated code.
    pub display_name: Option<String>,
}

impl TypeEq for DynamicMeta {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        let Self {
            type_,
            derived_types,
        } = self;

        type_.type_hash(hasher, types);
        TypeEq::type_hash_slice(derived_types, hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        let Self {
            type_,
            derived_types,
        } = self;

        type_.type_eq(&other.type_, types)
            && TypeEq::type_eq_iter(derived_types.iter(), other.derived_types.iter(), types)
    }
}

impl DerivedTypeMeta {
    /// Creates a new [`DerivedTypeMeta`] instance with the given `type_`.
    #[must_use]
    pub fn new(type_: TypeIdent) -> Self {
        Self {
            type_,
            display_name: None,
        }
    }
}

impl TypeEq for DerivedTypeMeta {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        let Self {
            type_,
            display_name,
        } = self;

        type_.type_hash(hasher, types);
        display_name.hash(hasher);
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        let Self {
            type_,
            display_name,
        } = self;

        type_.type_eq(&other.type_, types) && display_name == &other.display_name
    }
}
