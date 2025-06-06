//! Contains the [`ComplexInfo`] type information and all related types.

use std::hash::{Hash, Hasher};

use crate::models::{
    schema::{MaxOccurs, MinOccurs},
    types::{TypeEq, TypeVariant, Types},
    Ident,
};

use super::{AttributesInfo, Base, ElementsInfo};

/// Type information that contains data about a complex type.
#[derive(Debug, Clone)]
pub struct ComplexInfo {
    /// Base type of the complex type.
    pub base: Base,

    /// Content type information of the complex type that contains the actual
    /// information about the elements that are defined for this type.
    pub content: Option<Ident>,

    /// Minimum occurrence of this complex types content type.
    pub min_occurs: MinOccurs,

    /// Maximum occurrence of this complex types content type.
    pub max_occurs: MaxOccurs,

    /// Whether the type is dynamic or not.
    pub is_dynamic: bool,

    /// List of attributes defined for this complex type.
    pub attributes: AttributesInfo,
}

/// Represents a group of elements.
///
/// This is usually a `xs:all`, `xs:choice` or `xs:sequence`.
#[derive(Default, Debug, Clone)]
pub struct GroupInfo {
    /// List of elements defined in this group.
    pub elements: ElementsInfo,
}

/* GroupInfo */

impl TypeEq for GroupInfo {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        let Self { elements } = self;

        elements.type_hash(hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        let Self { elements } = self;

        elements.type_eq(&other.elements, types)
    }
}

/* ComplexInfo */

impl ComplexInfo {
    /// Returns `true` if the content of this complex type information
    /// is a [`TypeVariant::Choice`], `false` otherwise.
    #[must_use]
    pub fn has_complex_choice_content(&self, types: &Types) -> bool {
        matches!(
            self.content
                .as_ref()
                .and_then(|ident| types.get_resolved_type(ident))
                .map(|ty| &ty.variant),
            Some(TypeVariant::Choice(_))
        )
    }

    /// Returns `true` if the content of this complex type information
    /// is a [`TypeVariant::All`], [`TypeVariant::Choice`] or [`TypeVariant::Sequence`],
    /// `false` otherwise.
    #[must_use]
    pub fn has_complex_content(&self, types: &Types) -> bool {
        matches!(
            self.content
                .as_ref()
                .and_then(|ident| types.get_resolved_type(ident))
                .map(|ty| &ty.variant),
            Some(TypeVariant::All(_) | TypeVariant::Choice(_) | TypeVariant::Sequence(_))
        )
    }

    /// Returns `true` if the content of this complex type information
    /// is a [`TypeVariant::BuildIn`], [`TypeVariant::Union`] or [`TypeVariant::Enumeration`],
    /// `false` otherwise.
    #[must_use]
    pub fn has_simple_content(&self, types: &Types) -> bool {
        matches!(
            self.content
                .as_ref()
                .and_then(|ident| types.get_resolved_type(ident))
                .map(|ty| &ty.variant),
            Some(
                TypeVariant::Reference(_)
                    | TypeVariant::BuildIn(_)
                    | TypeVariant::Union(_)
                    | TypeVariant::Enumeration(_)
            )
        )
    }
}

impl Default for ComplexInfo {
    fn default() -> Self {
        Self {
            base: Base::None,
            content: None,
            min_occurs: 1,
            max_occurs: MaxOccurs::Bounded(1),
            is_dynamic: false,
            attributes: AttributesInfo::default(),
        }
    }
}

impl TypeEq for ComplexInfo {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        let Self {
            base,
            content,
            min_occurs,
            max_occurs,
            is_dynamic,
            attributes,
        } = self;

        base.type_hash(hasher, types);
        content.type_hash(hasher, types);
        min_occurs.hash(hasher);
        max_occurs.hash(hasher);
        is_dynamic.hash(hasher);
        attributes.type_hash(hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        let Self {
            base,
            content,
            min_occurs,
            max_occurs,
            is_dynamic,
            attributes,
        } = self;

        base.type_eq(&other.base, types)
            && content.type_eq(&other.content, types)
            && min_occurs.eq(&other.min_occurs)
            && max_occurs.eq(&other.max_occurs)
            && is_dynamic.eq(&other.is_dynamic)
            && attributes.type_eq(&other.attributes, types)
    }
}
