use std::path::PathBuf;

use bitflags::bitflags;
use url::Url;

use xsd_parser_types::misc::{Namespace, NamespacePrefix};

/// Configuration for the schema parser.
#[derive(Debug, Clone)]
pub struct ParserConfig {
    /// List of resolvers to use for resolving referenced schemas.
    pub resolver: Vec<Resolver>,

    /// List of namespaces to add to the parser before the schemas are loaded.
    ///
    /// See [`with_namespace`](crate::Parser::with_namespace) for more details.
    pub namespaces: Vec<(NamespacePrefix, Namespace)>,

    /// List of schemas to load.
    pub schemas: Vec<Schema>,

    /// Additional flags to control the parser.
    pub flags: ParserFlags,

    /// Wether to enable the debug output and where to write it to.
    pub debug_output: Option<PathBuf>,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            resolver: vec![Resolver::File],
            schemas: vec![],
            namespaces: vec![],
            flags: ParserFlags::RESOLVE_INCLUDES
                | ParserFlags::GENERATE_PREFIXES
                | ParserFlags::DEFAULT_NAMESPACES
                | ParserFlags::ALTERNATIVE_PREFIXES,
            debug_output: None,
        }
    }
}

/// Configuration for the [`Resolver`](crate::pipeline::parser::Resolver)s used in [`ParserConfig`].
#[derive(Debug, Clone)]
pub enum Resolver {
    /// Resolver that is used to resolve ewb resources (like `http://...` or `https://...`).
    #[cfg(feature = "web-resolver")]
    Web,

    /// Resolver that is used to resolve local resources from disk (like `./local-schema.xsd` or `file://...`).
    File,
}

/// Configuration for the schemas to load used in [`ParserConfig`].
#[derive(Debug, Clone)]
pub enum Schema {
    /// Load a schema from the provided URL.
    Url(Url),

    /// Load a schema from the provided file path.
    File(PathBuf),

    /// Load the schema from the provided string.
    Schema(String),

    /// Load the schema from the provided strings: `(name, schema)`.
    NamedSchema(String, String),
}

impl Schema {
    /// Create a [`Schema::Url`] from the passed `value`.
    #[inline]
    pub fn url<T>(value: T) -> Self
    where
        T: Into<Url>,
    {
        Self::Url(value.into())
    }

    /// Create a [`Schema::File`] from the passed `value`.
    #[inline]
    pub fn file<T>(value: T) -> Self
    where
        T: Into<PathBuf>,
    {
        Self::File(value.into())
    }

    /// Create a [`Schema::Schema`] from the passed `value`.
    #[inline]
    #[allow(clippy::self_named_constructors)]
    pub fn schema<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self::Schema(value.into())
    }

    /// Create a [`Schema::NamedSchema`] from the passed `name` and `value`.
    #[inline]
    #[allow(clippy::self_named_constructors)]
    pub fn named_schema<S, T>(name: S, value: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        Self::NamedSchema(name.into(), value.into())
    }
}

impl From<Url> for Schema {
    fn from(value: Url) -> Self {
        Self::Url(value)
    }
}

impl From<PathBuf> for Schema {
    fn from(value: PathBuf) -> Self {
        Self::File(value)
    }
}

impl From<String> for Schema {
    fn from(value: String) -> Self {
        Self::Schema(value)
    }
}

impl From<(String, String)> for Schema {
    fn from((name, schema): (String, String)) -> Self {
        Self::NamedSchema(name, schema)
    }
}

bitflags! {
    /// Flags to control the [`Parser`](crate::Parser).
    #[derive(Debug, Clone)]
    pub struct ParserFlags: u32 {
        /// Whether the parser should resolve `xs:include` and `xs:import` elements
        /// or not.
        ///
        /// See [`resolve_includes`](crate::Parser::resolve_includes) for details.
        const RESOLVE_INCLUDES = 1 << 0;

        /// Whether to add the default namespaces to the parser or not.
        ///
        /// See [`with_default_namespaces`](crate::Parser::with_default_namespaces) for details.
        const DEFAULT_NAMESPACES = 1 << 1;

        /// Allow the parser to assign prefixes known from other schemas to a certain
        /// namespace if the actual prefix is unknown or already in use.
        ///
        /// See [`alternative_prefixes`](crate::Parser::alternative_prefixes) for details.
        const ALTERNATIVE_PREFIXES = 1 << 2;

        /// Allow the parser to generate suitable prefixes for a certain namespace,
        /// if the actual prefix is already used.
        ///
        /// See [`generate_prefixes`](crate::Parser::generate_prefixes) for details.
        const GENERATE_PREFIXES = 1 << 3;
    }
}
