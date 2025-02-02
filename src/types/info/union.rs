//! Contains the [`UnionInfo`] type information and all related types.

use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use crate::types::{Ident, TypeEq, Types};

use super::Base;

/// Type information that defines a union type.
#[derive(Default, Debug, Clone)]
pub struct UnionInfo {
    /// Base type of the union type.
    pub base: Base,

    /// Types that are unified in this union type.
    pub types: UnionTypesInfo,
}

/// Type information that represents one type unified by a [`UnionInfo`].
#[derive(Debug, Clone)]
pub struct UnionTypeInfo {
    /// Target type of this type variant.
    pub type_: Ident,

    /// Name of the variant to use inside the generated code.
    pub display_name: Option<String>,
}

/// Type information that represents a list of [`UnionTypeInfo`] instances.
#[derive(Default, Debug, Clone)]
pub struct UnionTypesInfo(pub Vec<UnionTypeInfo>);

/* UnionInfo */

impl TypeEq for UnionInfo {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, ctx: &Types) {
        let Self { base, types } = self;

        base.type_hash(hasher, ctx);
        types.type_hash(hasher, ctx);
    }

    fn type_eq(&self, other: &Self, ctx: &Types) -> bool {
        let Self { base, types } = self;

        base.type_eq(&other.base, ctx) && types.type_eq(&other.types, ctx)
    }
}

/* UnionTypeInfo */

impl UnionTypeInfo {
    /// Create a new [`UnionTypeInfo`] from the passed `type_`.
    #[must_use]
    pub fn new(type_: Ident) -> Self {
        Self {
            type_,
            display_name: None,
        }
    }
}

impl TypeEq for UnionTypeInfo {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        let Self {
            type_,
            display_name,
        } = self;

        type_.type_hash(hasher, types);
        display_name.hash(hasher);
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        let Self {
            type_,
            display_name,
        } = self;

        type_.type_eq(&other.type_, types) && display_name.eq(&other.display_name)
    }
}

/* UnionTypesInfo */

impl Deref for UnionTypesInfo {
    type Target = Vec<UnionTypeInfo>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UnionTypesInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TypeEq for UnionTypesInfo {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        TypeEq::type_hash_slice(&self.0, hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        TypeEq::type_eq_iter(self.0.iter(), other.0.iter(), types)
    }
}
