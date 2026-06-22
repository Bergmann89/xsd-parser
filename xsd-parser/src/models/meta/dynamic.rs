//! Contains the [`SubstitutionMeta`] type information and all related types.

use std::hash::{Hash, Hasher};

use indexmap::IndexMap;

use crate::models::TypeIdent;

use super::{MetaTypes, TypeEq};

/// Type information that contains data about dynamic elements (substitution groups) or types.
#[derive(Default, Debug, Clone)]
pub struct DynamicMeta {
    /// Base type of this dynamic type.
    pub type_: Option<TypeIdent>,

    /// Whether the type was defined as abstract or not.
    pub is_abstract: bool,

    /// Map of the derived types of this dynamic type.
    pub derived_types: IndexMap<TypeIdent, DerivedTypeMeta>,
}

impl TypeEq for DynamicMeta {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        let Self {
            type_,
            is_abstract,
            derived_types,
        } = self;

        type_.type_hash(hasher, types);
        is_abstract.hash(hasher);
        TypeEq::type_hash_iter(derived_types, hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        let Self {
            type_,
            is_abstract,
            derived_types,
        } = self;

        type_.type_eq(&other.type_, types)
            && is_abstract == &other.is_abstract
            && TypeEq::type_eq_iter(derived_types.iter(), other.derived_types.iter(), types)
    }
}

/// Meta information about a single derived type of a dynamic type.
#[derive(Debug, Clone)]
pub struct DerivedTypeMeta {
    /// Identifier of the type this derived type represents.
    pub type_: TypeIdent,

    /// Name of the element to use inside the generated code.
    pub display_name: Option<String>,

    /// Relationship between this derived type and its owning [`DynamicMeta`].
    pub relationship: DeriveRelationship,
}

/// Defines the relationship between a [`DerivedTypeMeta`] and its owning [`DynamicMeta`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeriveRelationship {
    /// The derived type is the concrete type definition of this dynamic type.
    ConcreteType,

    /// The derived type is a direct child of the base type.
    DirectChild,

    /// The derived type is an indirect child of the base type.
    IndirectChild,
}

impl DerivedTypeMeta {
    /// Creates a new [`DerivedTypeMeta`] with the given type.
    #[must_use]
    pub fn new(type_: TypeIdent, relationship: DeriveRelationship) -> Self {
        Self {
            type_,
            relationship,
            display_name: None,
        }
    }

    /// Creates a new [`DerivedTypeMeta`] with the given type as a concrete type.
    #[must_use]
    pub fn new_concrete(type_: TypeIdent) -> Self {
        Self::new(type_, DeriveRelationship::ConcreteType)
    }

    /// Creates a new [`DerivedTypeMeta`] with the given type as a direct child.
    #[must_use]
    pub fn new_direct_child(type_: TypeIdent) -> Self {
        Self::new(type_, DeriveRelationship::DirectChild)
    }

    /// Creates a new [`DerivedTypeMeta`] with the given type as an indirect child.
    #[must_use]
    pub fn new_indirect_child(type_: TypeIdent) -> Self {
        Self::new(type_, DeriveRelationship::IndirectChild)
    }

    /// Creates a new [`DerivedTypeMeta`] with the given type and direct or indirect child relationship.
    #[must_use]
    pub fn new_child(type_: TypeIdent, is_direct: bool) -> Self {
        let relationship = if is_direct {
            DeriveRelationship::DirectChild
        } else {
            DeriveRelationship::IndirectChild
        };

        Self::new(type_, relationship)
    }
}

impl TypeEq for DerivedTypeMeta {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        let Self {
            type_,
            display_name,
            relationship,
        } = self;

        type_.type_hash(hasher, types);
        display_name.hash(hasher);
        relationship.hash(hasher);
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        let Self {
            type_,
            display_name,
            relationship,
        } = self;

        type_.type_eq(&other.type_, types)
            && display_name == &other.display_name
            && relationship == &other.relationship
    }
}
