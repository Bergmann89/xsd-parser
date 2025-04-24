//! Contains the [`ComplexInfo`] type information and all related types.

use std::hash::{Hash, Hasher};

use crate::schema::xs::{
    Annotation, BasicNamespaceListType, NamespaceListType, ProcessContentsType, QnameListAType,
    QnameListType,
};
use crate::schema::{MaxOccurs, MinOccurs};
use crate::types::type_::ComplexTypeVariant;
use crate::types::{Ident, TypeEq, Types};

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

    /// If this complex type accepts any other attribute, that is not defined by
    /// this type, this contains the information for these attributes, otherwise
    /// it is set to `None`.
    pub any_attribute: Option<AnyAttributeInfo>,
}

/// Represents a group of elements.
///
/// This is usually a `xs:all`, `xs:choice` or `xs:sequence`.
#[derive(Default, Debug, Clone)]
pub struct GroupInfo {
    /// If this group accepts any other element, that is not defined by this group,
    /// this field contains the information for these elements, otherwise it is
    /// set to `None`.
    pub any: Option<AnyInfo>,

    /// List of elements defined in this group.
    pub elements: ElementsInfo,
}

/// Contains information about elements that may occur in the XML file that
/// are not explicitly defined by the schema.
#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct AnyInfo {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<BasicNamespaceListType>,
    pub process_contents: Option<ProcessContentsType>,
    pub not_q_name: Option<QnameListType>,
    pub min_occurs: Option<MinOccurs>,
    pub max_occurs: Option<MaxOccurs>,
    pub annotation: Option<Annotation>,
}

/// Contains information about attributes that may occur in the XML file that
/// are not explicitly defined by the schema.
#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct AnyAttributeInfo {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<BasicNamespaceListType>,
    pub process_contents: Option<ProcessContentsType>,
    pub not_q_name: Option<QnameListAType>,
    pub annotation: Option<Annotation>,
}

/* GroupInfo */

impl TypeEq for GroupInfo {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        let Self { any, elements } = self;

        // HACK: We currently only evaluate if any is set or not, so just hashing
        // the result from `is_some` does work, but this might break in the future!
        any.is_some().hash(hasher);
        elements.type_hash(hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        let Self { any, elements } = self;

        any.eq(&other.any) && elements.type_eq(&other.elements, types)
    }
}

/* ComplexInfo */

impl ComplexInfo {
    /// Returns `true` if the content of this complex type information
    /// is a [`ComplexTypeVariant::Choice`], `false` otherwise.
    #[must_use]
    pub fn has_complex_choice_content(&self, types: &Types) -> bool {
        matches!(
            self.content
                .as_ref()
                .and_then(|ident| types.get_resolved_complex_type(ident))
                .map(|ty| &ty.variant),
            Some(ComplexTypeVariant::Choice(_))
        )
    }

    /// Returns `true` if the content of this complex type information
    /// is a [`ComplexTypeVariant::All`], [`ComplexTypeVariant::Choice`] or [`ComplexTypeVariant::Sequence`],
    /// `false` otherwise.
    #[must_use]
    pub fn has_complex_content(&self, types: &Types) -> bool {
        matches!(
            self.content
                .as_ref()
                .and_then(|ident| types.get_resolved_complex_type(ident))
                .map(|ty| &ty.variant),
            Some(
                ComplexTypeVariant::All(_)
                    | ComplexTypeVariant::Choice(_)
                    | ComplexTypeVariant::Sequence(_)
            )
        )
    }

    /// Returns `true` if the content of this complex type information
    /// is a [`ComplexTypeVariant::BuildIn`], [`ComplexTypeVariant::Union`] or [`ComplexTypeVariant::Enumeration`],
    /// `false` otherwise.
    #[must_use]
    pub fn has_simple_content(&self, types: &Types) -> bool {
        self.content
            .as_ref()
            .and_then(|ident| types.get_resolved_simple_type(ident))
            .is_some()
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
            any_attribute: None,
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
            any_attribute,
        } = self;

        base.type_hash(hasher, types);
        content.type_hash(hasher, types);
        min_occurs.hash(hasher);
        max_occurs.hash(hasher);
        is_dynamic.hash(hasher);
        attributes.type_hash(hasher, types);

        // HACK: We currently only evaluate if any is set or not, so just hashing
        // the result from `is_some` does work, but this might break in the future!
        any_attribute.is_some().hash(hasher);
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        let Self {
            base,
            content,
            min_occurs,
            max_occurs,
            is_dynamic,
            attributes,
            any_attribute,
        } = self;

        base.type_eq(&other.base, types)
            && content.type_eq(&other.content, types)
            && min_occurs.eq(&other.min_occurs)
            && max_occurs.eq(&other.max_occurs)
            && is_dynamic.eq(&other.is_dynamic)
            && attributes.type_eq(&other.attributes, types)
            && any_attribute.eq(&other.any_attribute)
    }
}
