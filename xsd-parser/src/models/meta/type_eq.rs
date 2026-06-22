use std::hash::{Hash, Hasher};

use crate::models::TypeIdent;

use super::MetaTypes;

/// Trait to check if two types are equal to each other or not.
///
/// This trait will automatically resolve type definitions to its target
/// type before it compares the two instances. This means a type is considered
/// as equal, if all type identifiers point to the same type and all normal
/// values are equal.
pub trait TypeEq: Sized {
    /// Feeds this value into the given [`Hasher`].
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes);

    /// Feeds a slice of this value into the given [`Hasher`].
    fn type_hash_iter<T: IntoIterator<Item = Self>, H: Hasher>(
        slice: T,
        hasher: &mut H,
        types: &MetaTypes,
    ) {
        let mut count = 0;
        for item in slice {
            item.type_hash(hasher, types);
            count += 1;
        }
        hasher.write_usize(count);
    }

    /// Check if this instance is equal to the `other` instance using the passed
    /// `types` to resolve identifiers.
    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool;

    /// Check if the two passed iterators contain type equal elements.
    fn type_eq_iter<X, Y>(x: X, y: Y, types: &MetaTypes) -> bool
    where
        X: IntoIterator<Item = Self>,
        Y: IntoIterator<Item = Self>,
    {
        let mut x = x.into_iter();
        let mut y = y.into_iter();

        loop {
            match (x.next(), y.next()) {
                (None, None) => return true,
                (Some(x), Some(y)) => {
                    if !x.type_eq(&y, types) {
                        return false;
                    }
                }
                (_, _) => return false,
            }
        }
    }
}

impl TypeEq for TypeIdent {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        types.get_resolved_ident(self).unwrap_or(self).hash(hasher);
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        let a = types.get_resolved_ident(self).unwrap_or(self);
        let b = types.get_resolved_ident(other).unwrap_or(other);

        a.eq(b)
    }
}

impl<T> TypeEq for &T
where
    T: TypeEq,
{
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        (*self).type_hash(hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        (*self).type_eq(*other, types)
    }
}

impl<T1, T2> TypeEq for (T1, T2)
where
    T1: TypeEq,
    T2: TypeEq,
{
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        self.0.type_hash(hasher, types);
        self.1.type_hash(hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        self.0.type_eq(&other.0, types) && self.1.type_eq(&other.1, types)
    }
}

impl<T> TypeEq for Option<T>
where
    T: TypeEq,
{
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        if let Some(inner) = self {
            hasher.write_u8(1);
            inner.type_hash(hasher, types);
        } else {
            hasher.write_u8(0);
        }
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        match (self, other) {
            (Some(x), Some(y)) => x.type_eq(y, types),
            (None, None) => true,
            (_, _) => false,
        }
    }
}
