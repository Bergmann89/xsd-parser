use std::path::PathBuf;

use bitflags::bitflags;
use url::Url;

use super::{Namespace, NamespacePrefix};

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
            flags: ParserFlags::RESOLVE_INCLUDES | ParserFlags::DEFAULT_NAMESPACES,
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
    }
}
