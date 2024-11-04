//! The `schema` module contains the XML schema types.

pub mod xs;

mod namespace;
mod namespace_prefix;
mod occurs;
mod qname;

use std::borrow::Cow;
use std::collections::btree_map::{BTreeMap, Entry, Iter};

use self::xs::Schema;

pub use namespace::Namespace;
pub use namespace_prefix::NamespacePrefix;
pub use occurs::{MaxOccurs, MinOccurs};
pub use qname::QName;

/// Represents the XML schema information load from different sources.
///
/// This structure is usually created by the [`Parser`](crate::parser::Parser).
/// It is the used by the [`Interpreter`](crate::interpreter::Interpreter) to
/// generate the more common [`Types`](crate::types::Types) structure out of it.
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
    /// First used /known prefix of the namespace.
    pub prefix: Option<NamespacePrefix>,

    /// URI of the namespace.
    pub namespace: Namespace,

    /// Schema files associated with this namespace.
    pub schemas: Vec<SchemaId>,
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
pub type Namespaces = BTreeMap<Namespace, NamespaceId>;

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
        namespace: Namespace,
        schema: Schema,
    ) {
        let schema_id = SchemaId(self.next_schema_id);
        let schema_info = self.get_or_create_namespace_info_mut(prefix, namespace);
        schema_info.schemas.push(schema_id);
        self.next_schema_id = self.next_schema_id.wrapping_add(1);

        match self.schemas.entry(schema_id) {
            Entry::Vacant(e) => e.insert(schema),
            Entry::Occupied(_) => unreachable!(),
        };
    }

    /// Get a mutable reference to a [`NamespaceInfo`] or create a new one if needed.
    pub fn get_or_create_namespace_info_mut(
        &mut self,
        prefix: Option<NamespacePrefix>,
        namespace: Namespace,
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
                    Entry::Occupied(_) => unreachable!(),
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

    /// Returns a reference to a specific namespace information instance by using
    /// the namespace id, or `None` if the schema is not known.
    #[must_use]
    pub fn get_namespace_info(&self, id: &NamespaceId) -> Option<&NamespaceInfo> {
        self.namespace_infos.get(id)
    }

    /// Returns a reference to a specific namespace information instance by using
    /// the namespace URI, or `None` if the schema is not known.
    #[must_use]
    pub fn get_namespace_info_by_namespace(&self, ns: &Namespace) -> Option<&NamespaceInfo> {
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
    pub fn resolve_namespace(&self, ns: &Namespace) -> Option<NamespaceId> {
        Some(*self.known_namespaces.get(ns)?)
    }
}

/* NamespaceInfo */

impl NamespaceInfo {
    /// Create a new [`NamespaceInfo`] instance from the passed `namespace`.
    #[must_use]
    pub fn new(namespace: Namespace) -> Self {
        Self {
            prefix: None,
            namespace,
            schemas: Vec::new(),
        }
    }
}
