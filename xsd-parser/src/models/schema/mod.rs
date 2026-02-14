//! The `schema` module contains internal representations of XML Schema (XSD) structures.
//!
//! This module defines types that model loaded XML schemas, namespaces, and their relationships,
//! serving as the output of the [`Parser`](crate::Parser) and input to the
//! [`Interpreter`](crate::Interpreter).
//!
//! It manages the resolution of namespaces and tracks schema documents across multiple sources.

pub mod xs;

mod occurs;

use std::collections::btree_map::{BTreeMap, Iter, IterMut};

use url::Url;

use xsd_parser_types::misc::{Namespace, NamespacePrefix};

use self::xs::Schema;

pub use xsd_parser_types::xml::QName;

pub use self::occurs::{MaxOccurs, MinOccurs};

/// Top-level structure for managing loaded XML schema files and associated namespaces.
///
/// This type is created and populated by the [`Parser`](crate::Parser), and used
/// by the [`Interpreter`](crate::Interpreter) to resolve schema components into
/// meaningful Rust types.
///
/// It tracks all loaded schemas, the namespaces they belong to, and which prefixes
/// are associated with which namespace URIs. Each namespace and schema is assigned
/// a unique ID (`NamespaceId`, `SchemaId`) to allow efficient lookup and association.
///
/// This type supports iterating over loaded schemas and namespaces,
/// as well as resolving prefixes and namespaces during interpretation.
#[derive(Debug)]
pub struct Schemas {
    pub(crate) schemas: SchemaFiles,
    pub(crate) namespace_infos: NamespaceInfos,

    pub(crate) known_prefixes: NamespacePrefixes,
    pub(crate) known_namespaces: Namespaces,

    pub(crate) last_schema_id: usize,
    pub(crate) last_namespace_id: usize,
}

/// Contains the information for a specific namespace.
#[derive(Debug)]
pub struct NamespaceInfo {
    /// First used/known prefix of the namespace or `None` if it is unknown.
    pub prefix: Option<NamespacePrefix>,

    /// URI of the namespace or `None` if it is the global or no namespace.
    pub namespace: Option<Namespace>,

    /// Schema files associated with this namespace.
    pub schemas: Vec<SchemaId>,

    /// User defined name to use for module generation for this namespace.
    pub module_name: Option<String>,
}

/// Contains information for a specific schema
#[derive(Debug)]
pub struct SchemaInfo {
    /// Name of the schema.
    pub name: Option<String>,

    /// The actual schema data.
    pub schema: Schema,

    /// Location the schema was load from.
    pub location: Option<Url>,

    /// Id of the namespace this schema belongs to.
    pub(crate) namespace_id: NamespaceId,

    /// Dependencies of this schema, mapping schema paths to their IDs.
    pub(crate) dependencies: BTreeMap<String, SchemaId>,
}

/// Represents an unique id for a XML schema.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SchemaId(pub usize);

/// Represents a unique id for a XML namespace.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NamespaceId(pub usize);

/// Map of [`SchemaId`] to [`Schema`]
pub type SchemaFiles = BTreeMap<SchemaId, SchemaInfo>;

/// Map of [`NamespaceId`] to [`NamespaceInfo`]
pub type NamespaceInfos = BTreeMap<NamespaceId, NamespaceInfo>;

/// Map of [`Namespace`] to [`NamespaceId`]
pub type Namespaces = BTreeMap<Option<Namespace>, NamespaceId>;

/// Map of [`NamespacePrefix`] to [`NamespaceId`]
pub type NamespacePrefixes = BTreeMap<NamespacePrefix, NamespaceId>;

/* Schemas */

impl Schemas {
    /// Returns an iterator over all schemas stored in this structure.
    pub fn schemas(&self) -> Iter<'_, SchemaId, SchemaInfo> {
        self.schemas.iter()
    }

    /// Returns a mutable iterator over all schemas stored in this structure.
    pub fn schemas_mut(&mut self) -> IterMut<'_, SchemaId, SchemaInfo> {
        self.schemas.iter_mut()
    }

    /// Returns an iterator over all namespace information instances stored
    /// in this structure.
    pub fn namespaces(&self) -> Iter<'_, NamespaceId, NamespaceInfo> {
        self.namespace_infos.iter()
    }

    /// Returns a reference to a specific schema by using the schema id,
    /// or `None` if the schema is not known.
    #[must_use]
    pub fn get_schema(&self, id: &SchemaId) -> Option<&SchemaInfo> {
        self.schemas.get(id)
    }

    /// Returns a mutable reference to a specific schema by using the schema id,
    /// or `None` if the schema is not known.
    #[must_use]
    pub fn get_schema_mut(&mut self, id: &SchemaId) -> Option<&mut SchemaInfo> {
        self.schemas.get_mut(id)
    }

    /// Returns a reference to a specific namespace information instance by using
    /// the namespace id.
    #[must_use]
    pub fn get_namespace_info(&self, id: &NamespaceId) -> Option<&NamespaceInfo> {
        self.namespace_infos.get(id)
    }

    /// Returns a mutable reference to a specific namespace information instance
    /// by using the namespace id.
    #[must_use]
    pub fn get_namespace_info_mut(&mut self, id: &NamespaceId) -> Option<&mut NamespaceInfo> {
        self.namespace_infos.get_mut(id)
    }

    /// Returns a reference to a specific namespace information instance by using
    /// the namespace URI, or `None` if the schema is not known.
    #[must_use]
    pub fn get_namespace_info_by_namespace(
        &self,
        ns: &Option<Namespace>,
    ) -> Option<&NamespaceInfo> {
        let id = self.resolve_namespace(ns)?;

        self.get_namespace_info(&id)
    }

    /// Try to resolve the namespace prefix to a namespace id.
    ///
    /// Returns the namespace id of the given namespace `prefix`, or `None`.
    #[must_use]
    pub fn resolve_prefix(&self, prefix: &NamespacePrefix) -> Option<NamespaceId> {
        Some(*self.known_prefixes.get(prefix)?)
    }

    /// Try to resolve the namespace to a namespace id.
    ///
    /// Returns the namespace id of the given namespace `ns`, or `None`.
    #[must_use]
    pub fn resolve_namespace(&self, ns: &Option<Namespace>) -> Option<NamespaceId> {
        Some(*self.known_namespaces.get(ns)?)
    }

    #[must_use]
    pub(crate) fn next_schema_id(&mut self) -> SchemaId {
        self.last_schema_id = self.last_schema_id.wrapping_add(1);

        SchemaId(self.last_schema_id)
    }
}

impl Default for Schemas {
    fn default() -> Self {
        Self {
            schemas: SchemaFiles::default(),
            namespace_infos: NamespaceInfos::default(),

            known_prefixes: NamespacePrefixes::default(),
            known_namespaces: Namespaces::default(),

            last_schema_id: SchemaId::UNKNOWN.0,
            last_namespace_id: NamespaceId::ANONYMOUS.0,
        }
    }
}

/* NamespaceInfo */

impl NamespaceInfo {
    /// Create a new [`NamespaceInfo`] instance from the passed `namespace`.
    #[must_use]
    pub fn new(namespace: Option<Namespace>) -> Self {
        Self {
            prefix: None,
            namespace,
            schemas: Vec::new(),
            module_name: None,
        }
    }

    /// Return the name of the namespace.
    ///
    /// This is either the [custom name](Self::module_name) or the namespace
    /// [`prefix`](Self::prefix).
    #[must_use]
    pub fn name(&self) -> Option<String> {
        self.module_name
            .clone()
            .or_else(|| self.prefix.as_ref().map(ToString::to_string))
    }
}

/* SchemaInfo */

impl SchemaInfo {
    /// Get the id of the namespace this schema belongs to.
    #[must_use]
    pub fn namespace_id(&self) -> NamespaceId {
        self.namespace_id
    }

    /// Get the dependencies of this schema, mapping schema paths to their IDs.
    #[must_use]
    pub fn dependencies(&self) -> &BTreeMap<String, SchemaId> {
        &self.dependencies
    }

    /// Returns `true` if this schema depends on the given schema id, `false` otherwise.
    #[must_use]
    pub fn depends_on(&self, schema_id: &SchemaId) -> bool {
        self.dependencies.values().any(|id| id == schema_id)
    }
}

/* SchemaId */

impl SchemaId {
    /// Represents the unknown [`SchemaId`]
    pub const UNKNOWN: Self = Self(0);

    /// Returns `true` if this schema id is unknown, `false` otherwise.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.eq(&Self::UNKNOWN)
    }

    /// Returns `other` if this schema id is unknown, `self` otherwise.
    #[inline]
    #[must_use]
    pub fn or(self, other: Self) -> Self {
        if self.is_unknown() {
            other
        } else {
            self
        }
    }
}

/* NamespaceId */

impl NamespaceId {
    /// Represents the unknown [`NamespaceId`]
    pub const UNKNOWN: Self = Self(0);

    /// Represents the anonymous [`NamespaceId`]
    pub const ANONYMOUS: Self = Self(1);

    /// Returns `true` if this namespace id is unknown, `false` otherwise.
    #[inline]
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.eq(&Self::UNKNOWN)
    }

    /// Returns `true` if this namespace id is anonymous, `false` otherwise.
    #[inline]
    #[must_use]
    pub fn is_anonymous(&self) -> bool {
        self.eq(&Self::ANONYMOUS)
    }

    /// Returns `other` if this namespace id is unknown, `self` otherwise.
    #[inline]
    #[must_use]
    pub fn or(self, other: Self) -> Self {
        if self.is_unknown() {
            other
        } else {
            self
        }
    }
}
