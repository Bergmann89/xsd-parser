//! Contains the [`ReferenceInfo`] type information and all related types.

use std::hash::{Hash, Hasher};

use crate::schema::{MaxOccurs, MinOccurs};
use crate::types::{Ident, TypeEq, Types};

/// Type information that defines a reference to another type.
#[derive(Debug, Clone)]
pub struct ReferenceInfo {
    /// Type that is referenced.
    pub type_: Ident,

    /// Minimum occurrence of the referenced type.
    pub min_occurs: MinOccurs,

    /// Maximum occurrence of the referenced type.
    pub max_occurs: MaxOccurs,
}

impl ReferenceInfo {
    /// Create a new [`ReferenceInfo`] instance from the passed `type_`.
    #[must_use]
    pub fn new<T>(type_: T) -> Self
    where
        T: Into<Ident>,
    {
        Self {
            type_: type_.into(),
            min_occurs: 1,
            max_occurs: MaxOccurs::Bounded(1),
        }
    }

    /// Returns `true` if this is a reference references a single type, `false` otherwise.
    ///
    /// This means that it is more or less just a type definition or renaming of an
    /// existing type.
    #[must_use]
    pub fn is_single(&self) -> bool {
        self.min_occurs == 1 && self.max_occurs == MaxOccurs::Bounded(1)
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
}

impl TypeEq for ReferenceInfo {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        let Self {
            type_,
            min_occurs,
            max_occurs,
        } = self;

        type_.type_hash(hasher, types);
        min_occurs.hash(hasher);
        max_occurs.hash(hasher);
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        let Self {
            type_,
            min_occurs,
            max_occurs,
        } = self;

        type_.type_eq(&other.type_, types)
            && min_occurs.eq(&other.min_occurs)
            && max_occurs.eq(&other.max_occurs)
    }
}
