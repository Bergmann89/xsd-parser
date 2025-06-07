use std::ops::DerefMut;

use crate::models::{
    meta::{AttributeMeta, ElementMeta, EnumerationMetaVariant},
    Ident,
};

/// Trait that is used to get the [`Any`](core::any::Any) trait for a specific type.
pub trait AsAny: core::any::Any {
    /// Get a reference to the current object as [`Any`](core::any::Any).
    fn as_any(&self) -> &dyn core::any::Any;

    /// Get a mutable reference to the current object as [`Any`](core::any::Any).
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any;
}

impl<X: 'static> AsAny for X {
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}

/// Trait that adds namespace information to a type.
pub trait WithNamespace {
    /// The default namespace prefix for this type.
    fn prefix() -> Option<&'static str>;

    /// The namespace for this type.
    fn namespace() -> Option<&'static str>;
}

/// Helper trait that implements additional methods for vectors.
pub trait VecHelper {
    /// Item stored in the vector.
    type Item;

    /// Try to find the object with the passed `ident` in the vector. If it was
    /// not found `None` is returned.
    fn find(&mut self, ident: Ident) -> Option<&mut Self::Item>;

    /// Try to find the object with the passed `ident` in the vector. If it was
    /// not found, a new one is created by invoking `f`.
    ///
    /// Returns a mutable reference either to the found or the newly created object.
    fn find_or_insert<F>(&mut self, ident: Ident, f: F) -> &mut Self::Item
    where
        F: FnOnce(Ident) -> Self::Item;
}

impl<X, T> VecHelper for X
where
    X: DerefMut<Target = Vec<T>>,
    T: WithIdent,
{
    type Item = T;

    fn find(&mut self, ident: Ident) -> Option<&mut Self::Item> {
        self.iter_mut().find(|x| x.ident() == &ident)
    }

    fn find_or_insert<F>(&mut self, ident: Ident, f: F) -> &mut Self::Item
    where
        F: FnOnce(Ident) -> Self::Item,
    {
        let vec = &mut **self;

        if let Some(i) = vec.iter().position(|x| x.ident() == &ident) {
            &mut vec[i]
        } else {
            vec.push(f(ident));

            vec.last_mut().unwrap()
        }
    }
}

/// Helper trait that adds name information to the implementing object.
pub trait WithIdent {
    /// Returns the name of the object.
    fn ident(&self) -> &Ident;
}

impl WithIdent for EnumerationMetaVariant {
    fn ident(&self) -> &Ident {
        &self.ident
    }
}

impl WithIdent for ElementMeta {
    fn ident(&self) -> &Ident {
        &self.ident
    }
}

impl WithIdent for AttributeMeta {
    fn ident(&self) -> &Ident {
        &self.ident
    }
}
