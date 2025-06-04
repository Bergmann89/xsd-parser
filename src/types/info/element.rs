//! Contains the [`ElementInfo`] type information and all related types.

use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use crate::schema::xs::{
    BasicNamespaceListType, NamespaceListType, ProcessContentsType, QnameListType,
};
use crate::schema::{MaxOccurs, MinOccurs};
use crate::types::{Ident, TypeEq, Types};

/// Type information that contains data about a element.
#[derive(Debug, Clone)]
pub struct ElementInfo {
    /// Identifier of the element.
    pub ident: Ident,

    /// Type of the element.
    pub type_: ElementType,

    /// Minimum occurrence of the field.
    pub min_occurs: MinOccurs,

    /// Maximum occurrence of the field.
    pub max_occurs: MaxOccurs,

    /// Mode of the element.
    pub element_mode: ElementMode,

    /// Name of the element to use inside the generated code.
    pub display_name: Option<String>,

    /// Documentation of the element extracted from `xs:documentation` nodes.
    pub documentation: Vec<String>,
}

/// Type of a [`ElementInfo`]
#[derive(Debug, Clone)]
pub enum ElementType {
    /// The element is a `xs:any`.
    Any(AnyInfo),

    /// The element has a specific type.
    Type(Ident),
}

/// Contains information about elements that may occur in the XML file that
/// are not explicitly defined by the schema.
#[allow(missing_docs)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AnyInfo {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_q_name: Option<QnameListType>,
    pub not_namespace: Option<BasicNamespaceListType>,
    pub process_contents: ProcessContentsType,
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
            type_: ElementType::Type(type_),
            element_mode,
            min_occurs: 1,
            max_occurs: MaxOccurs::Bounded(1),
            display_name: None,
            documentation: Vec::new(),
        }
    }

    /// Create a new [`ElementInfo`] instance for an `xs:any` element.
    #[must_use]
    pub fn any(ident: Ident, any: AnyInfo) -> Self {
        Self {
            ident,
            type_: ElementType::Any(any),
            element_mode: ElementMode::Element,
            min_occurs: 1,
            max_occurs: MaxOccurs::Bounded(1),
            display_name: None,
            documentation: Vec::new(),
        }
    }

    /// Returns `true` if this element represents an `xs:any` element, `false` otherwise.
    #[must_use]
    pub fn is_any(&self) -> bool {
        matches!(&self.type_, ElementType::Any(_))
    }

    /// Returns the [`AnyInfo`] if this element is a `xs:any`.
    #[must_use]
    pub fn as_any(&self) -> Option<&AnyInfo> {
        if let ElementType::Any(any) = &self.type_ {
            Some(any)
        } else {
            None
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
            documentation,
        } = self;

        ident.hash(hasher);
        type_.type_hash(hasher, types);
        element_mode.hash(hasher);
        min_occurs.hash(hasher);
        max_occurs.hash(hasher);
        display_name.hash(hasher);
        documentation.hash(hasher);
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        let Self {
            ident,
            type_,
            element_mode,
            min_occurs,
            max_occurs,
            display_name,
            documentation,
        } = self;

        ident.eq(&other.ident)
            && type_.type_eq(&other.type_, types)
            && element_mode.eq(&other.element_mode)
            && min_occurs.eq(&other.min_occurs)
            && max_occurs.eq(&other.max_occurs)
            && display_name.eq(&other.display_name)
            && documentation.eq(&other.documentation)
    }
}

/* ElementsInfo */

impl ElementsInfo {
    /// Push a new `xs:any` element to the list.
    ///
    /// This will update the names of all `xs:any` elements, if more than
    /// one element was added.
    pub fn push_any(&mut self, mut el: ElementInfo) {
        let mut i = 0;
        for element in &mut self.0 {
            if element.is_any() && element.ident.name.is_generated() {
                element.display_name = Some(format!("any_{i}"));
                i += 1;
            }
        }

        el.display_name = Some(if i > 0 {
            format!("any_{i}")
        } else {
            "any".into()
        });

        self.0.push(el);
    }
}

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

/* ElementType */

impl TypeEq for ElementType {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        match self {
            Self::Any(x) => {
                hasher.write_u8(0);
                x.hash(hasher);
            }
            Self::Type(x) => {
                hasher.write_u8(1);
                x.type_hash(hasher, types);
            }
        }
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        match (self, other) {
            (Self::Any(a), Self::Any(b)) => a.eq(b),
            (Self::Type(a), Self::Type(b)) => a.type_eq(b, types),
            (_, _) => false,
        }
    }
}

/* AnyInfo */

impl Hash for AnyInfo {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        // HACK: We currently only hash the id, because the types from the `xs`
        // module do not implement `Hash`.

        self.id.hash(hasher);
    }
}
