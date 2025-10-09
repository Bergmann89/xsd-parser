//! Contains the [`UnionMeta`] type information and all related types.

use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use crate::models::Ident;

use super::{Base, Constrains, MetaTypes, TypeEq};

/// Type information that defines a union type.
#[derive(Default, Debug, Clone)]
pub struct UnionMeta {
    /// Base type of the union type.
    pub base: Base,

    /// Types that are unified in this union type.
    pub types: UnionMetaTypes,

    /// Constraining facets defined for this type.
    pub constrains: Constrains,
}

/// Type information that represents one type unified by a [`UnionMeta`].
#[derive(Debug, Clone)]
pub struct UnionMetaType {
    /// Target type of this type variant.
    pub type_: Ident,

    /// Name of the variant to use inside the generated code.
    pub display_name: Option<String>,
}

/// Type information that represents a list of [`UnionMetaType`] instances.
#[derive(Default, Debug, Clone)]
pub struct UnionMetaTypes(pub Vec<UnionMetaType>);

/* UnionMeta */

impl TypeEq for UnionMeta {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, ctx: &MetaTypes) {
        let Self {
            base,
            types,
            constrains,
        } = self;

        base.type_hash(hasher, ctx);
        types.type_hash(hasher, ctx);
        constrains.hash(hasher);
    }

    fn type_eq(&self, other: &Self, ctx: &MetaTypes) -> bool {
        let Self {
            base,
            types,
            constrains,
        } = self;

        base.type_eq(&other.base, ctx)
            && types.type_eq(&other.types, ctx)
            && constrains.eq(&other.constrains)
    }
}

/* UnionMetaType */

impl UnionMetaType {
    /// Create a new [`UnionMetaType`] from the passed `type_`.
    #[must_use]
    pub fn new(type_: Ident) -> Self {
        Self {
            type_,
            display_name: None,
        }
    }
}

impl TypeEq for UnionMetaType {
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

        type_.type_eq(&other.type_, types) && display_name.eq(&other.display_name)
    }
}

/* UnionMetaTypes */

impl Deref for UnionMetaTypes {
    type Target = Vec<UnionMetaType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UnionMetaTypes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TypeEq for UnionMetaTypes {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        TypeEq::type_hash_slice(&self.0, hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        TypeEq::type_eq_iter(self.0.iter(), other.0.iter(), types)
    }
}
