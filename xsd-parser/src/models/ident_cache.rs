use std::{collections::HashMap, ops::Not};

use smallvec::SmallVec;

use crate::InterpreterError;

use super::{
    schema::{NamespaceId, SchemaId},
    TypeIdent,
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
pub struct IdentCache(HashMap<TypeIdent, IdentCacheEntry>);

/// Cache entry that is used in [`IdentCache`].
///
/// This cache entry contains all ([`NamespaceId`], [`SchemaId`]) combinations
/// that are known for a given [`TypeIdent`]. This can be used to resolve a
/// half qualified [`TypeIdent`] to a real existing one.
#[derive(Default, Debug)]
pub struct IdentCacheEntry(SmallVec<[(NamespaceId, SchemaId); 1]>);

impl IdentCache {
    /// Get a reference to the cache entry for the provided `ident`.
    #[inline]
    #[must_use]
    pub fn get(&self, ident: &TypeIdent) -> Option<&IdentCacheEntry> {
        self.0.get(ident)
    }

    /// Try to resolve the passed `ident`ifier to an actual existing identifier.
    ///
    /// # Errors
    ///
    /// Returns a [`InterpreterError::UnknownType`] if the identifier is not known
    /// to the cache, or [`InterpreterError::AmbiguousType`] if multiple identifiers
    /// matches the passed one.
    pub fn resolve(&self, ident: TypeIdent) -> Result<TypeIdent, InterpreterError> {
        if let Some(entry) = self.0.get(&ident) {
            entry.resolve(ident)
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

    /// Insert a new entry to the cache.
    ///
    /// This adds all alternatives (with schema, without schema, with namespace,
    /// without namespace) of the passed `ident` to the cache.
    #[inline]
    pub fn insert(&mut self, ident: TypeIdent, ns: NamespaceId, schema: SchemaId) {
        for key in alternative_idents(ident) {
            self.0.entry(key).or_default().add(ns, schema);
        }
    }
}

impl IdentCacheEntry {
    pub fn resolve(&self, mut ident: TypeIdent) -> Result<TypeIdent, InterpreterError> {
        if ident.is_fully_qualified() {
            Ok(ident)
        } else if self.0.len() == 1 {
            ident.ns = self.0[0].0;
            ident.schema = self.0[0].1;

            Ok(ident)
        } else {
            Err(InterpreterError::AmbiguousType(ident))
        }
    }

    fn add(&mut self, ns: NamespaceId, schema: SchemaId) {
        let entry = (ns, schema);
        if !self.0.contains(&entry) {
            self.0.push(entry);
        }
    }
}

fn alternative_idents(ident: TypeIdent) -> impl Iterator<Item = TypeIdent> {
    let alt0 = ident
        .ns
        .is_unknown()
        .not()
        .then(|| ident.clone().with_ns(NamespaceId::UNKNOWN));
    let alt1 = ident
        .schema
        .is_unknown()
        .not()
        .then(|| ident.clone().with_schema(SchemaId::UNKNOWN));
    let alt2 = ident.is_fully_qualified().then(|| {
        ident
            .clone()
            .with_ns(NamespaceId::UNKNOWN)
            .with_schema(SchemaId::UNKNOWN)
    });

    Some(ident).into_iter().chain(alt0).chain(alt1).chain(alt2)
}
