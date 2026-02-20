//! Contains the [`CustomMeta`] type information and all related types.

use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};

use xsd_parser_types::misc::Namespace;

use crate::config::NamespaceId;
use crate::pipeline::generator::{ValueGenerator, ValueGeneratorBox};

/// Type information for a custom defined type.
pub struct CustomMeta {
    /// Name of the custom defined type.
    pub name: String,

    /// The path the type should be included from.
    ///
    /// The path should be absolute, or relative to the root of the generated code.
    pub include: Option<String>,

    /// The handler for the default values for this custom defined type.
    ///
    /// This is used to translate default values specified in the XSD schema,
    /// to suitable rust code.
    pub default: Option<ValueGeneratorBox>,

    /// The namespaces needed by this custom type.
    pub namespaces: Vec<CustomMetaNamespace>,

    /// Wether this custom type contains `xs:any` elements or not.
    pub allow_any: bool,
}

impl CustomMeta {
    /// Create a new custom type information with the passed `name`.
    #[must_use]
    pub fn new<X>(name: X) -> Self
    where
        X: Into<String>,
    {
        Self {
            name: name.into(),
            include: None,
            default: None,
            namespaces: Vec::new(),
            allow_any: false,
        }
    }

    /// Get the name of the custom defined type.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the include path of the custom defined type.
    #[must_use]
    pub fn include(&self) -> Option<&str> {
        self.include.as_deref()
    }

    /// The the path the type should be included from.
    ///
    /// The path should be absolute, or relative to the root of the generated code.
    #[must_use]
    pub fn include_from<X>(mut self, include: X) -> Self
    where
        X: Into<String>,
    {
        self.include = Some(include.into());

        self
    }

    /// Set the handler for the default values for this custom defined type.
    #[must_use]
    pub fn with_default<X: ValueGenerator>(mut self, x: X) -> Self {
        self.default = Some(Box::new(x));

        self
    }

    /// Add a namespace that is needed by this custom type.
    ///
    /// The namespace may be added to the root element during serialization.
    #[must_use]
    pub fn with_namespace<N>(mut self, ns: N) -> Self
    where
        N: Into<CustomMetaNamespace>,
    {
        self.namespaces.push(ns.into());

        self
    }

    /// Returns the namespaces needed by this custom type.
    #[must_use]
    pub fn namespaces(&self) -> &[CustomMetaNamespace] {
        &self.namespaces
    }

    /// Returns `true` if this type contains `xs:any` elements, `false` otherwise.
    #[must_use]
    pub fn allow_any(&self) -> bool {
        self.allow_any
    }

    /// Set wether this custom type contains`xs:any` elements or not.
    #[must_use]
    pub fn with_allow_any(mut self, value: bool) -> Self {
        self.allow_any = value;

        self
    }
}

impl Clone for CustomMeta {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            include: self.include.clone(),
            default: self.default.as_ref().map(|x| ValueGenerator::clone(&**x)),
            namespaces: self.namespaces.clone(),
            allow_any: self.allow_any,
        }
    }
}

impl Debug for CustomMeta {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("CustomType")
            .field("name", &self.name)
            .field("include", &self.include)
            .field("default", &self.default.is_some())
            .field("namespaces", &self.namespaces)
            .field("allow_any", &self.allow_any)
            .finish()
    }
}

impl Eq for CustomMeta {}

impl PartialEq for CustomMeta {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Hash for CustomMeta {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

/// Namespace information for a custom defined type.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum CustomMetaNamespace {
    /// A namespace that is identified by its id.
    Id(NamespaceId),

    /// A namespace that is identified by the namespace information itself.
    Namespace(Namespace),
}

impl From<NamespaceId> for CustomMetaNamespace {
    fn from(value: NamespaceId) -> Self {
        Self::Id(value)
    }
}

impl From<Namespace> for CustomMetaNamespace {
    fn from(value: Namespace) -> Self {
        Self::Namespace(value)
    }
}
