//! Contains the [`ReferenceMeta`] type information and all related types.

use std::hash::{Hash, Hasher};

use crate::models::{
    schema::{MaxOccurs, MinOccurs},
    Ident,
};

use super::{MetaTypes, TypeEq};

/// Type information that defines a reference to another type.
#[derive(Debug, Clone)]
pub struct ReferenceMeta {
    /// Type that is referenced.
    pub type_: Ident,

    /// Whether the referenced type is nillable or not.
    pub nillable: bool,

    /// Minimum occurrence of the referenced type.
    pub min_occurs: MinOccurs,

    /// Maximum occurrence of the referenced type.
    pub max_occurs: MaxOccurs,
}

impl ReferenceMeta {
    /// Create a new [`ReferenceMeta`] instance from the passed `type_`.
    #[must_use]
    pub fn new<T>(type_: T) -> Self
    where
        T: Into<Ident>,
    {
        Self {
            type_: type_.into(),
            nillable: false,
            min_occurs: 1,
            max_occurs: MaxOccurs::Bounded(1),
        }
    }

    /// Returns `true` if this references a single type, `false` otherwise.
    ///
    /// This means that the target type is used exactly once in this reference.
    #[must_use]
    pub fn is_single(&self) -> bool {
        self.min_occurs == 1 && self.max_occurs == MaxOccurs::Bounded(1)
    }

    /// Returns `true` if this references a simple type, `false` otherwise.
    ///
    /// This means that it is more or less just a type definition or renaming of an
    /// existing type.
    #[must_use]
    pub fn is_simple(&self) -> bool {
        self.is_single() && !self.nillable
    }

    /// Sets the minimum occurrence of the referenced type.
    #[must_use]
    pub fn min_occurs(mut self, min: MinOccurs) -> Self {
        self.min_occurs = min;

        self
    }

    /// Sets the maximum occurrence of the referenced type.
    #[must_use]
    pub fn max_occurs(mut self, max: MaxOccurs) -> Self {
        self.max_occurs = max;

        self
    }

    /// Returns `true` if this type is emptiable, `false` otherwise.
    ///
    /// Emptiable means that the type may not have any element.
    #[must_use]
    pub fn is_emptiable(&self, types: &MetaTypes) -> bool {
        if self.min_occurs == 0 {
            return true;
        }

        types
            .items
            .get(&self.type_)
            .is_none_or(|ty| ty.is_emptiable(types))
    }

    /// Returns `true` if this type is mixed, `false` otherwise.
    ///
    /// Mixed means, that the type also accepts text intermixed with it's elements.
    #[must_use]
    pub fn is_mixed(&self, types: &MetaTypes) -> bool {
        types
            .items
            .get(&self.type_)
            .is_some_and(|ty| ty.is_mixed(types))
    }
}

impl TypeEq for ReferenceMeta {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        let Self {
            type_,
            nillable,
            min_occurs,
            max_occurs,
        } = self;

        type_.type_hash(hasher, types);
        nillable.hash(hasher);
        min_occurs.hash(hasher);
        max_occurs.hash(hasher);
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        let Self {
            type_,
            nillable,
            min_occurs,
            max_occurs,
        } = self;

        type_.type_eq(&other.type_, types)
            && nillable.eq(&other.nillable)
            && min_occurs.eq(&other.min_occurs)
            && max_occurs.eq(&other.max_occurs)
    }
}
