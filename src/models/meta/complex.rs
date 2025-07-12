//! Contains the [`ComplexMeta`] type information and all related types.

use std::hash::{Hash, Hasher};

use crate::models::{
    schema::{MaxOccurs, MinOccurs},
    Ident,
};

use super::{AttributesMeta, Base, ElementsMeta, MetaTypeVariant, MetaTypes, TypeEq};

/// Type information that contains data about a complex type.
#[derive(Debug, Clone)]
pub struct ComplexMeta {
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

    /// Wether the content of this type is mixed (contains also text) or not.
    pub is_mixed: bool,

    /// List of attributes defined for this complex type.
    pub attributes: AttributesMeta,
}

/// Represents a group of elements.
///
/// This is usually a `xs:all`, `xs:choice` or `xs:sequence`.
#[derive(Default, Debug, Clone)]
pub struct GroupMeta {
    /// List of elements defined in this group.
    pub elements: ElementsMeta,
}

/* GroupMeta */

impl TypeEq for GroupMeta {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        let Self { elements } = self;

        elements.type_hash(hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        let Self { elements } = self;

        elements.type_eq(&other.elements, types)
    }
}

/* ComplexMeta */

impl ComplexMeta {
    /// Returns `true` if the content of this complex type information
    /// is a [`MetaTypeVariant::All`], `false` otherwise.
    #[must_use]
    pub fn has_complex_all_content(&self, types: &MetaTypes) -> bool {
        matches!(
            self.content
                .as_ref()
                .and_then(|ident| types.get_resolved_type(ident))
                .map(|ty| &ty.variant),
            Some(MetaTypeVariant::All(_))
        )
    }

    /// Returns `true` if the content of this complex type information
    /// is a [`MetaTypeVariant::Choice`], `false` otherwise.
    #[must_use]
    pub fn has_complex_choice_content(&self, types: &MetaTypes) -> bool {
        matches!(
            self.content
                .as_ref()
                .and_then(|ident| types.get_resolved_type(ident))
                .map(|ty| &ty.variant),
            Some(MetaTypeVariant::Choice(_))
        )
    }

    /// Returns `true` if the content of this complex type information
    /// is a [`MetaTypeVariant::Sequence`], `false` otherwise.
    #[must_use]
    pub fn has_complex_sequence_content(&self, types: &MetaTypes) -> bool {
        matches!(
            self.content
                .as_ref()
                .and_then(|ident| types.get_resolved_type(ident))
                .map(|ty| &ty.variant),
            Some(MetaTypeVariant::Sequence(_))
        )
    }

    /// Returns `true` if the content of this complex type information
    /// is a [`MetaTypeVariant::All`], [`MetaTypeVariant::Choice`] or [`MetaTypeVariant::Sequence`],
    /// `false` otherwise.
    #[must_use]
    pub fn has_complex_content(&self, types: &MetaTypes) -> bool {
        matches!(
            self.content
                .as_ref()
                .and_then(|ident| types.get_resolved_type(ident))
                .map(|ty| &ty.variant),
            Some(
                MetaTypeVariant::All(_) | MetaTypeVariant::Choice(_) | MetaTypeVariant::Sequence(_)
            )
        )
    }

    /// Returns `true` if the content of this complex type information
    /// is a [`MetaTypeVariant::BuildIn`], [`MetaTypeVariant::Union`] or [`MetaTypeVariant::Enumeration`],
    /// `false` otherwise.
    #[must_use]
    pub fn has_simple_content(&self, types: &MetaTypes) -> bool {
        matches!(
            self.content
                .as_ref()
                .and_then(|ident| types.get_resolved_type(ident))
                .map(|ty| &ty.variant),
            Some(
                MetaTypeVariant::Reference(_)
                    | MetaTypeVariant::BuildIn(_)
                    | MetaTypeVariant::Union(_)
                    | MetaTypeVariant::Enumeration(_)
            )
        )
    }
}

impl Default for ComplexMeta {
    fn default() -> Self {
        Self {
            base: Base::None,
            content: None,
            min_occurs: 1,
            max_occurs: MaxOccurs::Bounded(1),
            is_dynamic: false,
            is_mixed: false,
            attributes: AttributesMeta::default(),
        }
    }
}

impl TypeEq for ComplexMeta {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        let Self {
            base,
            content,
            min_occurs,
            max_occurs,
            is_dynamic,
            is_mixed: mixed_content,
            attributes,
        } = self;

        base.type_hash(hasher, types);
        content.type_hash(hasher, types);
        min_occurs.hash(hasher);
        max_occurs.hash(hasher);
        is_dynamic.hash(hasher);
        mixed_content.hash(hasher);
        attributes.type_hash(hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        let Self {
            base,
            content,
            min_occurs,
            max_occurs,
            is_dynamic,
            is_mixed: mixed_content,
            attributes,
        } = self;

        base.type_eq(&other.base, types)
            && content.type_eq(&other.content, types)
            && min_occurs.eq(&other.min_occurs)
            && max_occurs.eq(&other.max_occurs)
            && is_dynamic.eq(&other.is_dynamic)
            && mixed_content.eq(&other.is_mixed)
            && attributes.type_eq(&other.attributes, types)
    }
}
