//! The `schema` module contains internal representations of XML Schema (XSD) structures.
//!
//! This module defines types that model loaded XML schemas, namespaces, and their relationships,
//! serving as the output of the [`Parser`](crate::Parser) and input to the
//! [`Interpreter`](crate::Interpreter).
//!
//! It manages the resolution of namespaces and tracks schema documents across multiple sources.

mod namespace;
mod namespace_prefix;
mod occurs;
mod qname;
mod xs_extra;

#[allow(
    unused_mut,
    missing_docs,
    unused_variables,
    clippy::len_zero,
    clippy::single_match,
    clippy::needless_pass_by_value,
    clippy::unused_self,
    clippy::unnecessary_wraps,
    clippy::redundant_else,
    clippy::redundant_field_names,
    clippy::too_many_lines,
    clippy::large_enum_variant,
    clippy::semicolon_if_nothing_returned
)]
mod generated;

use std::borrow::Cow;
use std::collections::btree_map::{BTreeMap, Entry, Iter, IterMut};

pub use generated::*;

pub use self::namespace::Namespace;
pub use self::namespace_prefix::NamespacePrefix;
pub use self::occurs::{MaxOccurs, MinOccurs};
pub use self::qname::QName;
pub use self::sch::Schema as Schematron;
pub use self::xs::{AttributeUseType as Use, Schema};

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
#[derive(Default, Debug)]
pub struct Schemas {
    schemas: SchemaFiles,
    namespace_infos: NamespaceInfos,

    known_prefixes: NamespacePrefixes,
    known_namespaces: Namespaces,

    next_schema_id: usize,
    next_namespace_id: usize,
}

/// Contians the information for a specific namespace.
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

/// Represents an unique id for a XML schema.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SchemaId(pub usize);

/// Represents a unique id for a XML namespace.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NamespaceId(pub usize);

/// Map of [`SchemaId`] to [`Schema`]
pub type SchemaFiles = BTreeMap<SchemaId, Schema>;

/// Map of [`NamespaceId`] to [`NamespaceInfo`]
pub type NamespaceInfos = BTreeMap<NamespaceId, NamespaceInfo>;

/// Map of [`Namespace`] to [`NamespaceId`]
pub type Namespaces = BTreeMap<Option<Namespace>, NamespaceId>;

/// Map of [`NamespacePrefix`] to [`NamespaceId`]
pub type NamespacePrefixes = BTreeMap<NamespacePrefix, NamespaceId>;

/* Schemas */

impl Schemas {
    /// Add a new schema to the schemas structure.
    ///
    /// # Errors
    ///
    /// Will just forward the errors from [`get_or_create_namespace_info_mut`](Self::get_or_create_namespace_info_mut).
    pub fn add_schema(
        &mut self,
        prefix: Option<NamespacePrefix>,
        namespace: Option<Namespace>,
        schema: Schema,
    ) {
        let schema_id = SchemaId(self.next_schema_id);
        let schema_info = self.get_or_create_namespace_info_mut(prefix, namespace);
        schema_info.schemas.push(schema_id);
        self.next_schema_id = self.next_schema_id.wrapping_add(1);

        match self.schemas.entry(schema_id) {
            Entry::Vacant(e) => e.insert(schema),
            Entry::Occupied(_) => crate::unreachable!(),
        };
    }

    /// Get a mutable reference to a [`NamespaceInfo`] or create a new one if needed.
    pub fn get_or_create_namespace_info_mut(
        &mut self,
        prefix: Option<NamespacePrefix>,
        namespace: Option<Namespace>,
    ) -> &mut NamespaceInfo {
        let (ns, id) = match self.known_namespaces.entry(namespace) {
            Entry::Occupied(e) => {
                let id = *e.get();
                let ns = self.namespace_infos.get_mut(&id).unwrap();

                (ns, id)
            }
            Entry::Vacant(e) => {
                let id = NamespaceId(self.next_namespace_id);
                self.next_namespace_id = self.next_namespace_id.wrapping_add(1);

                let namespace = e.key().clone();
                e.insert(id);

                let ns = match self.namespace_infos.entry(id) {
                    Entry::Vacant(e) => e.insert(NamespaceInfo::new(namespace)),
                    Entry::Occupied(_) => crate::unreachable!(),
                };

                (ns, id)
            }
        };

        if let Some(mut prefix) = prefix {
            if matches!(self.known_prefixes.get(&prefix), Some(x) if *x != id) {
                let ext = format!("_{}", id.0);
                let ext = ext.as_bytes();

                let mut p = prefix.0.into_owned();
                p.extend_from_slice(ext);

                prefix = NamespacePrefix(Cow::Owned(p));
            }

            self.known_prefixes.insert(prefix.clone(), id);

            if ns.prefix.is_none() {
                ns.prefix = Some(prefix);
            }
        }

        ns
    }

    /// Returns an iterator over all schemas stored in this structure.
    pub fn schemas(&self) -> Iter<'_, SchemaId, Schema> {
        self.schemas.iter()
    }

    /// Returns a mutable iterator over all schemas stored in this structure.
    pub fn schemas_mut(&mut self) -> IterMut<'_, SchemaId, Schema> {
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
    pub fn get_schema(&self, id: &SchemaId) -> Option<&Schema> {
        self.schemas.get(id)
    }

    /// Returns a mutable reference to a specific schema by using the schema id,
    /// or `None` if the schema is not known.
    #[must_use]
    pub fn get_schema_mut(&mut self, id: &SchemaId) -> Option<&mut Schema> {
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
}
