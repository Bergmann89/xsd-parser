//! Contains the [`ElementInfo`] type information and all related types.

use std::ops::{Deref, DerefMut};

use crate::schema::{MaxOccurs, MinOccurs};
use crate::types::{Ident, TypeEq, Types};

/// Type information that contains data about a element.
#[derive(Debug, Clone)]
pub struct ElementInfo {
    /// Identifier of the element.
    pub ident: Ident,

    /// Type of the element.
    pub type_: Ident,

    /// Minimum occurrence of the field.
    pub min_occurs: MinOccurs,

    /// Maximum occurrence of the field.
    pub max_occurs: MaxOccurs,

    /// Mode of the element.
    pub element_mode: ElementMode,
}

/// Type information that represents a list of [`ElementInfo`] instances.
#[derive(Default, Debug, Clone)]
pub struct ElementsInfo(pub Vec<ElementInfo>);

/// Defines the type of an [`ElementInfo`]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ElementMode {
    /// Represents an actual XML element.
    Element,

    /// Represents another group of elements.
    Group,
}

/* ElementInfo */

impl ElementInfo {
    /// Create a new [`ElementInfo`] instance from the passed `name`, `type_`
    /// and `element_mode`.
    #[must_use]
    pub fn new(ident: Ident, type_: Ident, element_mode: ElementMode) -> Self {
        Self {
            ident,
            type_,
            element_mode,
            min_occurs: 1,
            max_occurs: MaxOccurs::Bounded(1),
        }
    }
}

impl TypeEq for ElementInfo {
    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        let Self {
            ident,
            type_,
            element_mode,
            min_occurs,
            max_occurs,
        } = self;

        ident.eq(&other.ident)
            && type_.type_eq(&other.type_, types)
            && element_mode.eq(&other.element_mode)
            && min_occurs.eq(&other.min_occurs)
            && max_occurs.eq(&other.max_occurs)
    }
}

/* ElementsInfo */

impl Deref for ElementsInfo {
    type Target = Vec<ElementInfo>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ElementsInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TypeEq for ElementsInfo {
    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        TypeEq::type_eq_iter(self.0.iter(), other.0.iter(), types)
    }
}
