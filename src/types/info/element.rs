//! Contains the [`ElementInfo`] type information and all related types.

use std::hash::{Hash, Hasher};
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

    /// Name of the element to use inside the generated code.
    pub display_name: Option<String>,
}

/// Type information that represents a list of [`ElementInfo`] instances.
#[derive(Default, Debug, Clone)]
pub struct ElementsInfo(pub Vec<ElementInfo>);

/// Defines the type of an [`ElementInfo`]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
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
            display_name: None,
        }
    }
}

impl TypeEq for ElementInfo {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        let Self {
            ident,
            type_,
            element_mode,
            min_occurs,
            max_occurs,
            display_name,
        } = self;

        ident.hash(hasher);
        type_.type_hash(hasher, types);
        element_mode.hash(hasher);
        min_occurs.hash(hasher);
        max_occurs.hash(hasher);
        display_name.hash(hasher);
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        let Self {
            ident,
            type_,
            element_mode,
            min_occurs,
            max_occurs,
            display_name,
        } = self;

        ident.eq(&other.ident)
            && type_.type_eq(&other.type_, types)
            && element_mode.eq(&other.element_mode)
            && min_occurs.eq(&other.min_occurs)
            && max_occurs.eq(&other.max_occurs)
            && display_name.eq(&other.display_name)
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
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        TypeEq::type_hash_slice(&self.0, hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        TypeEq::type_eq_iter(self.0.iter(), other.0.iter(), types)
    }
}
