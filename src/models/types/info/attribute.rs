//! Contains the [`AttributeInfo`] type information and all related types.

use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use crate::models::{
    schema::xs::{
        BasicNamespaceListType, NamespaceListType, ProcessContentsType, QnameListAType, Use,
    },
    types::{TypeEq, Types},
    Ident,
};

use super::use_hash;

/// Type information that contains data about attribute definitions.
#[derive(Debug, Clone)]
pub struct AttributeInfo {
    /// Identifier of the attribute.
    pub ident: Ident,

    /// Type of the attribute.
    pub type_: AttributeType,

    /// Usage of the attribute.
    pub use_: Use,

    /// Default value of the attribute.
    pub default: Option<String>,

    /// Name of the attribute to use inside the generated code.
    pub display_name: Option<String>,

    /// Documentation of the element extracted from `xs:documentation` nodes.
    pub documentation: Vec<String>,
}

/// Type of a [`AttributeInfo`]
#[derive(Debug, Clone)]
pub enum AttributeType {
    /// The attribute is a `xs:anyAttribute`.
    Any(AnyAttributeInfo),

    /// The attribute has a specific type.
    Type(Ident),
}

/// Contains information about attributes that may occur in the XML file that
/// are not explicitly defined by the schema.
#[allow(missing_docs)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AnyAttributeInfo {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_q_name: Option<QnameListAType>,
    pub not_namespace: Option<BasicNamespaceListType>,
    pub process_contents: ProcessContentsType,
}

/// Type information that represents a list of [`AttributeInfo`] instances.
#[derive(Default, Debug, Clone)]
pub struct AttributesInfo(Vec<AttributeInfo>);

/* AttributeInfo */

impl AttributeInfo {
    /// Create a new [`AttributeInfo`] instance from the passed `name` and `type_`.
    #[must_use]
    pub fn new(ident: Ident, type_: Ident) -> Self {
        Self {
            ident,
            type_: AttributeType::Type(type_),
            use_: Use::Optional,
            default: None,
            display_name: None,
            documentation: Vec::new(),
        }
    }

    /// Create a new [`AttributeInfo`] instance from for `xs:anyAttribute` attributes.
    #[must_use]
    pub fn any(ident: Ident, any: AnyAttributeInfo) -> Self {
        Self {
            ident,
            type_: AttributeType::Any(any),
            use_: Use::Required,
            default: None,
            display_name: None,
            documentation: Vec::new(),
        }
    }

    /// Set the [`Use`] value of the attribute.
    #[must_use]
    pub fn with_use(mut self, use_: Use) -> Self {
        self.use_ = use_;

        self
    }

    /// Returns `true` if this attribute represents an `xs:anyAttribute`, `false` otherwise.
    #[must_use]
    pub fn is_any(&self) -> bool {
        matches!(&self.type_, AttributeType::Any(_))
    }

    /// Returns the [`AnyAttributeInfo`] if this attribute is a `xs:anyAttribute`.
    #[must_use]
    pub fn as_any(&self) -> Option<&AnyAttributeInfo> {
        if let AttributeType::Any(any) = &self.type_ {
            Some(any)
        } else {
            None
        }
    }
}

impl TypeEq for AttributeInfo {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        let Self {
            ident,
            type_,
            use_,
            default,
            display_name,
            documentation,
        } = self;

        ident.hash(hasher);
        type_.type_hash(hasher, types);
        use_hash(use_, hasher);
        default.hash(hasher);
        display_name.hash(hasher);
        documentation.hash(hasher);
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        let Self {
            ident,
            type_,
            use_,
            default,
            display_name,
            documentation,
        } = self;

        ident.eq(&other.ident)
            && type_.type_eq(&other.type_, types)
            && use_.eq(&other.use_)
            && default.eq(&other.default)
            && display_name.eq(&other.display_name)
            && documentation.eq(&other.documentation)
    }
}

/* AttributesInfo */

impl Deref for AttributesInfo {
    type Target = Vec<AttributeInfo>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AttributesInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TypeEq for AttributesInfo {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        TypeEq::type_hash_slice(&self.0, hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        TypeEq::type_eq_iter(self.0.iter(), other.0.iter(), types)
    }
}

/* AttributeType */

impl TypeEq for AttributeType {
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

/* AnyAttributeInfo */

impl Hash for AnyAttributeInfo {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        // HACK: We currently only hash the id, because the types from the `xs`
        // module do not implement `Hash`.

        self.id.hash(hasher);
    }
}
