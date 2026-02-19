use std::borrow::Cow;
use std::collections::{hash_set::Iter as HashSetIter, HashMap, HashSet};
use std::iter::FusedIterator;
use std::slice::Iter as SliceIter;

use crate::models::schema::Dependency;
use crate::{InterpreterError, Name};

use super::{
    schema::{NamespaceId, SchemaId},
    IdentType, TypeIdent,
};

/// Cache that keeps track of different [`TypeIdent`]ifiers.
///
/// The [`IdentCache`] is created by the [`Interpreter`](crate::Interpreter)
/// (see [`exec_interpreter_with_ident_cache`](crate::exec_interpreter_with_ident_cache)).
///
/// It contains all types that are created during the interpretation of the
/// provided schemas. It is able to resolve half qualified types identifiers
/// (identifiers with a missing schema or namespace ID), to the actual identifier
/// that is used to identify a [`MetaType`](crate::models::meta::MetaType) inside
/// the [`MetaTypes`](crate::MetaTypes) structure.
#[derive(Default, Debug)]
pub struct IdentCache {
    schemas: HashMap<SchemaId, SchemaEntry>,
    unknown_schema: HashMap<NamespaceId, SchemaEntry>,

    namespaces: HashMap<NamespaceId, HashSet<SchemaId>>,
    global_namespaces: Vec<NamespaceId>,
}

/// Iterator over all identifiers that are known to the [`IdentCache`] in the
/// context of the specified `schema` set. Not including the global namespaces,
/// the unknown schema and the identifiers referenced by a `xs:include`.
#[derive(Debug)]
pub struct SchemaSetIter<'a> {
    cache: &'a IdentCache,

    visited: HashSet<SchemaId>,
    emitted: HashSet<(IdentType, Cow<'a, str>)>,

    types_iter: Option<TypesIterTuple<'a>>,
    dependencies_iter: Vec<SliceIter<'a, Dependency<SchemaId>>>,
}

#[derive(Debug)]
struct SchemaEntry {
    ns: NamespaceId,
    schema: SchemaId,
    types: HashSet<(IdentType, Cow<'static, str>)>,
    dependencies: Vec<Dependency<SchemaId>>,
}

type TypesIterTuple<'a> = (
    NamespaceId,
    SchemaId,
    HashSetIter<'a, (IdentType, Cow<'static, str>)>,
);

impl IdentCache {
    /// Insert the passed `ident`ifier into the cache.
    #[inline]
    pub fn insert(&mut self, ident: TypeIdent) {
        let entry = if ident.schema.is_unknown() {
            self.unknown_schema
                .entry(ident.ns)
                .or_insert_with(|| SchemaEntry {
                    ns: ident.ns,
                    schema: SchemaId::UNKNOWN,
                    types: HashSet::new(),
                    dependencies: Vec::new(),
                })
        } else {
            self.schemas
                .entry(ident.schema)
                .or_insert_with(|| SchemaEntry {
                    ns: ident.ns,
                    schema: ident.schema,
                    types: HashSet::new(),
                    dependencies: Vec::new(),
                })
        };

        entry.types.insert((ident.type_, ident.name.into()));
    }

    /// Add a schema to the cache.
    ///
    /// This is required to be able to resolve identifiers that are defined in the
    /// schema, and to be able to add dependencies to it.
    pub fn add_schema(&mut self, ns: NamespaceId, schema: SchemaId) {
        self.schemas.entry(schema).or_insert_with(|| SchemaEntry {
            ns,
            schema,
            types: HashSet::new(),
            dependencies: Vec::new(),
        });

        self.namespaces.entry(ns).or_default().insert(schema);
    }

    /// Add a dependency between two schemas.
    ///
    /// This means that when trying to resolve an identifier for `schema`, the
    /// cache will also search for it in `dependency`.
    ///
    /// # Returns
    /// Returns `true` if the dependency was added, or `false` if it already
    /// existed or if `schema` is not known to the cache.
    pub fn add_dependency(&mut self, schema: SchemaId, dependency: Dependency<SchemaId>) -> bool {
        if let Some(entry) = self.schemas.get_mut(&schema) {
            if !entry.dependencies.contains(&dependency) {
                entry.dependencies.push(dependency);
                return true;
            }
        }

        false
    }

    /// Add a namespace that is always searched when trying to resolve an identifier.
    pub fn add_global_namespace(&mut self, ns: NamespaceId) {
        self.namespaces.entry(ns).or_default();
        self.global_namespaces.push(ns);
    }

    /// Try to resolve the passed `ident`ifier to an actual existing identifier.
    ///
    /// This function will lookup the passed `ident`ifier in the global context.
    /// This means that all schemas that are known to the cache will be searched
    /// for a matching type, and if multiple matches are found, an
    /// [`InterpreterError::AmbiguousType`] error will be returned.
    ///
    /// # Errors
    ///
    /// Returns a [`InterpreterError::UnknownType`] if the identifier is not known
    /// to the cache, or [`InterpreterError::AmbiguousType`] if multiple identifiers
    /// matches the passed one.
    pub fn resolve(&self, ident: TypeIdent) -> Result<TypeIdent, InterpreterError> {
        let schemas = match (ident.ns, ident.schema) {
            (NamespaceId::UNKNOWN, SchemaId::UNKNOWN) => self.schemas.keys().copied().collect(),
            (ns, SchemaId::UNKNOWN) => self
                .namespaces
                .get(&ns)
                .into_iter()
                .flatten()
                .copied()
                .collect(),
            (_, schema) => vec![schema],
        };

        let mut ret = None;
        for schema in schemas {
            if let Some(entry) = self.schemas.get(&schema) {
                if entry.matches(&ident) {
                    if ret.is_some() {
                        return Err(InterpreterError::AmbiguousType(ident));
                    }

                    ret = Some(entry.make_ident(ident.clone()));
                }
            }
        }

        if ident.schema.is_unknown() {
            for entry in self.unknown_schema.values() {
                if entry.matches(&ident) {
                    if ret.is_some() {
                        return Err(InterpreterError::AmbiguousType(ident));
                    }

                    ret = Some(entry.make_ident(ident.clone()));
                }
            }
        }

        if let Some(resolved_ident) = ret {
            Ok(resolved_ident)
        } else {
            Err(InterpreterError::UnknownType(ident))
        }
    }

    /// Same as [`resolve`](IdentCache::resolve), but instead of returning a
    /// [`UnknownType`](InterpreterError::UnknownType) error for unknown
    /// identifiers it returns the original identifier.
    ///
    /// # Errors
    ///
    /// Returns [`InterpreterError::AmbiguousType`] if multiple identifiers
    /// matches the passed one.
    pub fn resolve_allow_unknown(&self, ident: TypeIdent) -> Result<TypeIdent, InterpreterError> {
        match self.resolve(ident) {
            Ok(ident) => Ok(ident),
            Err(InterpreterError::UnknownType(ident)) => Ok(ident),
            Err(error) => Err(error),
        }
    }

    /// Get an iterator over all identifiers that are known to the cache in the
    /// context of the specified `schema` set. Not including the global namespaces,
    /// the unknown schema and the identifiers referenced by a `xs:include`.
    #[must_use]
    pub fn schema_set(&self, schema: SchemaId) -> SchemaSetIter<'_> {
        SchemaSetIter::new(self, schema)
    }

    /// Try to resolve the passed `ident`ifier to an actual existing identifier.
    ///
    /// In contrast to [`resolve`](IdentCache::resolve), this function will search
    /// for the passed `ident`ifier in the context of the specified `schema`.
    /// This means that it will try to resolve the type inside `schema`,
    /// and if it is not found, it will try to resolve it in its dependencies.
    /// As soon as a match is found, it will be returned, and the search will
    /// not continue, so no error will be raised if multiple matches are found.
    ///
    /// # Errors
    ///
    /// Returns a [`InterpreterError::UnknownType`] if the identifier is not known
    /// to the cache.
    pub fn resolve_for_schema(
        &self,
        schema: SchemaId,
        ident: TypeIdent,
    ) -> Result<TypeIdent, InterpreterError> {
        let mut visited = HashSet::new();

        if let Some(entry) = self.search_in_schema(&mut visited, schema, &ident) {
            return Ok(entry.make_ident(ident));
        }

        for ns in &self.global_namespaces {
            for schema in self.namespaces.get(ns).into_iter().flatten() {
                if let Some(entry) = self.search_in_schema(&mut visited, *schema, &ident) {
                    return Ok(entry.make_ident(ident));
                }
            }
        }

        for entry in self.unknown_schema.values() {
            if entry.matches(&ident) {
                return Ok(entry.make_ident(ident));
            }
        }

        Err(InterpreterError::UnknownType(ident))
    }

    fn search_in_schema(
        &self,
        visited: &mut HashSet<SchemaId>,
        schema: SchemaId,
        ident: &TypeIdent,
    ) -> Option<&SchemaEntry> {
        if !visited.insert(schema) {
            return None;
        }

        let entry = self.schemas.get(&schema)?;
        if entry.matches(ident) {
            return Some(entry);
        }

        for dep in &entry.dependencies {
            if let Some(found) = self.search_in_schema(visited, **dep, ident) {
                return Some(found);
            }
        }

        None
    }
}

impl SchemaEntry {
    fn matches(&self, ident: &TypeIdent) -> bool {
        let ns_matches = ident.ns.is_unknown() || ident.ns == self.ns;
        let schema_matches = ident.schema.is_unknown() || ident.schema == self.schema;
        let contains_type = self
            .types
            .contains(&(ident.type_, Cow::Borrowed(ident.name.as_str())));

        ns_matches && schema_matches && contains_type
    }

    fn make_ident(&self, ident: TypeIdent) -> TypeIdent {
        TypeIdent {
            ns: ident.ns.or(self.ns),
            schema: ident.schema.or(self.schema),
            type_: ident.type_,
            name: ident.name,
        }
    }
}

impl<'a> SchemaSetIter<'a> {
    fn new(cache: &'a IdentCache, schema: SchemaId) -> Self {
        let schema = cache.schemas.get(&schema);
        let types_iter = schema.map(|x| (x.ns, x.schema, x.types.iter()));
        let dependencies_iter = schema.map(|x| x.dependencies.iter()).into_iter().collect();

        Self {
            cache,
            visited: HashSet::new(),
            emitted: HashSet::new(),
            types_iter,
            dependencies_iter,
        }
    }
}

impl Iterator for SchemaSetIter<'_> {
    type Item = TypeIdent;

    fn next(&mut self) -> Option<Self::Item> {
        #[allow(clippy::redundant_else)]
        loop {
            if let Some((ns, schema, types_iter)) = &mut self.types_iter {
                if let Some((type_, name)) = types_iter.next() {
                    if !self.emitted.insert((*type_, Cow::Borrowed(name.as_ref()))) {
                        continue;
                    }

                    break Some(TypeIdent {
                        ns: *ns,
                        schema: *schema,
                        type_: *type_,
                        name: Name::new_named(name.clone()),
                    });
                } else {
                    self.types_iter = None;
                }
            } else if let Some(dependencies_iter) = self.dependencies_iter.last_mut() {
                if let Some(dep) = dependencies_iter.next() {
                    if self.visited.insert(**dep) {
                        let Some(entry) = self.cache.schemas.get(&**dep) else {
                            continue;
                        };
                        self.types_iter = Some((entry.ns, entry.schema, entry.types.iter()));
                        self.dependencies_iter.push(entry.dependencies.iter());
                    }
                } else {
                    self.dependencies_iter.pop();
                }
            } else {
                break None;
            }
        }
    }
}

impl FusedIterator for SchemaSetIter<'_> {}
