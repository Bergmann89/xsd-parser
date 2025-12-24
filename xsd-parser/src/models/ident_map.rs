use std::collections::{btree_map, BTreeMap, HashMap};
use std::hash::Hash;

use indexmap::IndexMap;

use crate::Ident;

/// todo
#[derive(Debug)]
pub struct IdentMap<M> {
    items: M,
    schema_idents: HashMap<Ident, Ident>,
}
impl<M: Default> Default for IdentMap<M> {
    fn default() -> Self {
        Self {
            items: Default::default(),
            schema_idents: Default::default(),
        }
    }
}
impl<T, M: Map<K = Ident, V = T>> IdentMap<M> {
    /// Resolve this Ident to the schema where it was defined.
    /// If the provided Ident is defined in the schema file specified by `Ident::schema`,
    /// the Ident is returned unchanged.
    /// Else an otherwise identical Ident pointing to a schema where the Ident is defined is returned.
    /// In a valid schema this should be unambiguous. In schemas that define duplicate types,
    /// an arbitrary instance is selected.
    #[must_use]
    pub fn find_original_schema<'a>(&'a self, ident: &'a Ident) -> &'a Ident {
        if self.items.contains_key(ident) {
            ident
        } else {
            self.schema_idents
                .get(&ident.clone().with_schema(None))
                .unwrap_or(ident)
        }
    }

    /// Inserts an item into the map.
    ///
    /// If the map already contained an entry for this exact identifier, the value is updated.
    pub fn insert(&mut self, ident: Ident, value: T) {
        self.schema_idents
            .entry(ident.clone().with_schema(None))
            .or_insert_with(|| ident.clone());
        self.items.insert(ident, value);
    }

    /// Returns a reference to the value corresponding to the identifier.
    ///
    /// If this exact identifier is not found in the map, a matching entry from a different schema is returned.
    pub fn get(&self, ident: &Ident) -> Option<&T> {
        self.items.get(ident).or_else(|| {
            self.schema_idents
                .get(&ident.clone().with_schema(None))
                .and_then(|i| self.items.get(i))
        })
    }
    /// Returns a mutable reference to the value corresponding to the identifier.
    ///
    /// If this exact identifier is not found in the map, a matching entry from a different schema is returned.
    pub fn get_mut<'a, 'b>(&'a mut self, mut ident: &'b Ident) -> Option<&'a mut T>
    where
        'a: 'b,
    {
        if !self.items.contains_key(ident) {
            ident = self.schema_idents.get(&ident.clone().with_schema(None))?;
        }
        self.items.get_mut(ident)
    }
    /// Gets an iterator over the entries of the map.
    pub fn iter(&self) -> M::Iter<'_> {
        self.items.iter()
    }
    /// Gets a mutable iterator over the entries of the map.
    pub fn iter_mut(&mut self) -> M::IterMut<'_> {
        self.items.iter_mut()
    }
    /// Gets an iterator over the keys of the map.
    pub fn keys(&self) -> M::Keys<'_> {
        self.items.keys()
    }
    /// Gets a mutable iterator over the values of the map.
    pub fn values_mut(&mut self) -> M::ValuesMut<'_> {
        self.items.values_mut()
    }
    /// Returns `true` only if the map contains this exact identifier.
    ///
    /// Note that if the map contains a matching identifier from a different schema, this returns `false`.
    /// If you need to check for any matching entry, use `.get(x).is_some()` instead.
    pub fn contains_exact(&self, ident: &Ident) -> bool {
        self.items.contains_key(ident)
    }
}
impl<'a, M: Map> IntoIterator for &'a IdentMap<M> {
    type Item = (&'a M::K, &'a M::V);
    type IntoIter = M::Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}
impl<'a, M: Map> IntoIterator for &'a mut IdentMap<M> {
    type Item = (&'a M::K, &'a mut M::V);
    type IntoIter = M::IterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}

#[rustfmt::skip]
pub trait Map {
    type K;
    type V;

    type Iter<'a>: Iterator<Item = (&'a Self::K, &'a Self::V)> where Self: 'a, Self::V: 'a, Self::K: 'a;
    type IterMut<'a>: Iterator<Item = (&'a Self::K, &'a mut Self::V)> where Self: 'a, Self::V: 'a, Self::K: 'a;
    type Keys<'a>: Iterator<Item = &'a Self::K> where Self: 'a, Self::V: 'a, Self::K: 'a;
    type ValuesMut<'a>: Iterator<Item = &'a mut Self::V> where Self: 'a, Self::V: 'a, Self::K: 'a;

    fn insert(&mut self, key: Self::K, value: Self::V);
    fn get(&self, key: &Self::K) -> Option<&Self::V>;
    fn get_mut(&mut self, key: &Self::K) -> Option<&mut Self::V>;
    fn contains_key(&self, key: &Self::K) -> bool;
    fn iter(&self) -> Self::Iter<'_>;
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
    fn keys(&self) -> Self::Keys<'_>;
    fn values_mut(&mut self) -> Self::ValuesMut<'_>;
}
#[rustfmt::skip]
impl<K: Ord, V> Map for BTreeMap<K, V> {
    type K = K;
    type V = V;

    type Iter<'a> = btree_map::Iter<'a, K, V> where K: 'a, V: 'a;
    type IterMut<'a> = btree_map::IterMut<'a, K, V> where K: 'a, V: 'a;
    type Keys<'a> = btree_map::Keys<'a, K, V> where K: 'a, V: 'a;
    type ValuesMut<'a> = btree_map::ValuesMut<'a, K, V> where K: 'a, V: 'a;

    fn insert(&mut self, key: Self::K, value: Self::V) { self.insert(key, value); }
    fn get(&self, key: &Self::K) -> Option<&Self::V> { self.get(key) }
    fn get_mut(&mut self, key: &Self::K) -> Option<&mut Self::V> { self.get_mut(key) }
    fn contains_key(&self, key: &Self::K) -> bool { self.contains_key(key) }
    fn iter(&self) -> Self::Iter<'_> { self.iter() }
    fn iter_mut(&mut self) -> Self::IterMut<'_> { self.iter_mut() }
    fn keys(&self) -> Self::Keys<'_> { self.keys() }
    fn values_mut(&mut self) -> Self::ValuesMut<'_> { self.values_mut() }
}
#[rustfmt::skip]
impl<K: Hash + Eq, V> Map for IndexMap<K, V> {
    type K = K;
    type V = V;

    type Iter<'a> = indexmap::map::Iter<'a, K, V> where K: 'a, V: 'a;
    type IterMut<'a> = indexmap::map::IterMut<'a, K, V> where K: 'a, V: 'a;
    type Keys<'a> = indexmap::map::Keys<'a, K, V> where K: 'a, V: 'a;
    type ValuesMut<'a> = indexmap::map::ValuesMut<'a, K, V> where K: 'a, V: 'a;

    fn insert(&mut self, key: Self::K, value: Self::V) { self.insert(key, value); }
    fn get(&self, key: &Self::K) -> Option<&Self::V> { self.get(key) }
    fn get_mut(&mut self, key: &Self::K) -> Option<&mut Self::V> { self.get_mut(key) }
    fn contains_key(&self, key: &Self::K) -> bool { self.contains_key(key) }
    fn iter(&self) -> Self::Iter<'_> { self.iter() }
    fn iter_mut(&mut self) -> Self::IterMut<'_> { self.iter_mut() }
    fn keys(&self) -> Self::Keys<'_> { self.keys() }
    fn values_mut(&mut self) -> Self::ValuesMut<'_> { self.values_mut() }
}
