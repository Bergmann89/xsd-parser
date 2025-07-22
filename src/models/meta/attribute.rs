//! Contains the [`AttributeMeta`] type information and all related types.

use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use crate::models::{
    meta::{MetaTypes, TypeEq},
    schema::xs::{
        BasicNamespaceListType, FormChoiceType, NamespaceListType, ProcessContentsType,
        QnameListAType, Use,
    },
    Ident,
};

/// Type information that contains data about attribute definitions.
#[derive(Debug, Clone)]
pub struct AttributeMeta {
    /// Identifier of the attribute.
    pub ident: Ident,

    /// Type of the attribute.
    pub variant: AttributeMetaVariant,

    /// Usage of the attribute.
    pub use_: Use,

    /// The form of this attribute.
    pub form: FormChoiceType,

    /// Default value of the attribute.
    pub default: Option<String>,

    /// Name of the attribute to use inside the generated code.
    pub display_name: Option<String>,

    /// Documentation of the element extracted from `xs:documentation` nodes.
    pub documentation: Vec<String>,
}

/// Variant of a [`AttributeMeta`]
///
/// Either it's any attribute or a specific type.
#[derive(Debug, Clone)]
pub enum AttributeMetaVariant {
    /// The attribute is a `xs:anyAttribute`.
    Any(AnyAttributeMeta),

    /// The attribute has a specific type.
    Type(Ident),
}

/// Contains information about attributes that may occur in the XML file that
/// are not explicitly defined by the schema.
#[allow(missing_docs)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AnyAttributeMeta {
    pub id: Option<String>,
    pub namespace: Option<NamespaceListType>,
    pub not_q_name: Option<QnameListAType>,
    pub not_namespace: Option<BasicNamespaceListType>,
    pub process_contents: ProcessContentsType,
}

/// Type information that represents a list of [`AttributeMeta`] instances.
#[derive(Default, Debug, Clone)]
pub struct AttributesMeta(Vec<AttributeMeta>);

/* AttributeMeta */

impl AttributeMeta {
    /// Create a new [`AttributeMeta`] instance from the passed `name` and `type_`.
    #[must_use]
    pub fn new(ident: Ident, type_: Ident, form: FormChoiceType) -> Self {
        Self {
            ident,
            variant: AttributeMetaVariant::Type(type_),
            use_: Use::Optional,
            form,
            default: None,
            display_name: None,
            documentation: Vec::new(),
        }
    }

    /// Create a new [`AttributeMeta`] instance from for `xs:anyAttribute` attributes.
    #[must_use]
    pub fn any(ident: Ident, any: AnyAttributeMeta) -> Self {
        Self {
            ident,
            variant: AttributeMetaVariant::Any(any),
            use_: Use::Required,
            form: FormChoiceType::Unqualified,
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
        matches!(&self.variant, AttributeMetaVariant::Any(_))
    }

    /// Returns the [`AnyAttributeMeta`] if this attribute is a `xs:anyAttribute`.
    #[must_use]
    pub fn as_any(&self) -> Option<&AnyAttributeMeta> {
        if let AttributeMetaVariant::Any(any) = &self.variant {
            Some(any)
        } else {
            None
        }
    }
}

impl TypeEq for AttributeMeta {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        let Self {
            ident,
            variant,
            use_,
            form,
            default,
            display_name,
            documentation,
        } = self;

        ident.hash(hasher);
        variant.type_hash(hasher, types);
        use_.hash(hasher);
        form.hash(hasher);
        default.hash(hasher);
        display_name.hash(hasher);
        documentation.hash(hasher);
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        let Self {
            ident,
            variant: type_,
            use_,
            form,
            default,
            display_name,
            documentation,
        } = self;

        ident.eq(&other.ident)
            && type_.type_eq(&other.variant, types)
            && use_.eq(&other.use_)
            && form.eq(&other.form)
            && default.eq(&other.default)
            && display_name.eq(&other.display_name)
            && documentation.eq(&other.documentation)
    }
}

/* AttributesMeta */

impl Deref for AttributesMeta {
    type Target = Vec<AttributeMeta>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AttributesMeta {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TypeEq for AttributesMeta {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        TypeEq::type_hash_slice(&self.0, hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        TypeEq::type_eq_iter(self.0.iter(), other.0.iter(), types)
    }
}

/* AttributeType */

impl TypeEq for AttributeMetaVariant {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
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

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        match (self, other) {
            (Self::Any(a), Self::Any(b)) => a.eq(b),
            (Self::Type(a), Self::Type(b)) => a.type_eq(b, types),
            (_, _) => false,
        }
    }
}

/* AnyAttributeMeta */

impl Hash for AnyAttributeMeta {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        // HACK: We currently only hash the id, because the types from the `xs`
        // module do not implement `Hash`.

        self.id.hash(hasher);
    }
}
