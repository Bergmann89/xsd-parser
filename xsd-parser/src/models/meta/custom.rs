//! Contains the [`CustomMeta`] type information and all related types.

use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};

use proc_macro2::TokenStream;
use xsd_parser_types::misc::Namespace;

use crate::config::NamespaceId;

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
    pub default: Option<Box<dyn CustomDefaultImpl>>,

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

    /// Try to get the default value (as code) for the given string.
    ///
    /// This is used to translate default values specified in the XSD schema,
    /// to suitable rust code.
    #[must_use]
    pub fn default(&self, s: &str) -> Option<TokenStream> {
        self.default.as_ref()?.exec(s)
    }

    /// Set the handler for the default values for this custom defined type.
    #[must_use]
    pub fn with_default<X: CustomDefaultImpl>(mut self, x: X) -> Self {
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
            default: self
                .default
                .as_ref()
                .map(|x| CustomDefaultImpl::clone(&**x)),
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

/// Trait that converts the default value of a element specified in the XML
/// schema to actual default code.
///
/// You can add a custom default implementation to your custom type using
/// [`CustomMeta::with_default`].
pub trait CustomDefaultImpl: Send + Sync + 'static {
    /// Try to convert the passed string `s` that contains the default value from
    /// the XML schema to actual default code. If the value could not be converted
    /// to code `None` is returned.
    fn exec(&self, s: &str) -> Option<TokenStream>;

    /// Clone this instance and return it as a box.
    fn clone(&self) -> Box<dyn CustomDefaultImpl>;
}

impl<X> CustomDefaultImpl for X
where
    X: Fn(&str) -> Option<TokenStream> + Clone + Send + Sync + 'static,
{
    fn exec(&self, s: &str) -> Option<TokenStream> {
        (*self)(s)
    }

    fn clone(&self) -> Box<dyn CustomDefaultImpl> {
        Box::new(self.clone())
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
