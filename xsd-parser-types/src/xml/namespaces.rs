use std::borrow::{Borrow, Cow};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use quick_xml::name::{Namespace, PrefixDeclaration};

use crate::misc::format_utf8_slice;

/// Represents a list of namespaces.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Namespaces<'a>(pub HashMap<Key<'a>, Value<'a>>);

impl<'a> Namespaces<'a> {
    /// Create a new [`Namespaces`] instance.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Convert this list into a shared version ([`NamespacesShared`]).
    #[must_use]
    pub fn into_shared(self) -> NamespacesShared<'a> {
        NamespacesShared::new(self)
    }
}

impl<'a> Deref for Namespaces<'a> {
    type Target = HashMap<Key<'a>, Value<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Namespaces<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, K, V> FromIterator<(K, V)> for Namespaces<'a>
where
    K: Into<Cow<'a, [u8]>>,
    V: Into<Cow<'a, [u8]>>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut namespaces = Namespaces::default();

        for (k, v) in iter {
            let key = Key(k.into());
            let value = Value(v.into());

            namespaces.insert(key, value);
        }

        namespaces
    }
}

/// Wrapper type that is used as key for the [`Namespaces`] map.
#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Key<'a>(pub Cow<'a, [u8]>);

impl Key<'_> {
    /// Get the key value as [`PrefixDeclaration`].
    #[must_use]
    pub fn prefix_decl(&self) -> PrefixDeclaration<'_> {
        if self.0.is_empty() {
            PrefixDeclaration::Default
        } else {
            PrefixDeclaration::Named(&self.0)
        }
    }
}

impl From<Vec<u8>> for Key<'static> {
    fn from(value: Vec<u8>) -> Self {
        Self(Cow::Owned(value))
    }
}

impl<'a> From<&'a [u8]> for Key<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl<'a> From<Cow<'a, [u8]>> for Key<'a> {
    fn from(value: Cow<'a, [u8]>) -> Self {
        Self(value)
    }
}

impl Deref for Key<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for Key<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Key(\"")?;
        format_utf8_slice(&self.0, f)?;
        write!(f, "\")")?;

        Ok(())
    }
}

impl AsRef<[u8]> for Key<'_> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Borrow<[u8]> for Key<'_> {
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}

/// Wrapper type that is used as value for the [`Namespaces`] map.
#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Value<'a>(pub Cow<'a, [u8]>);

impl Value<'_> {
    /// Get the namespace as [`Namespace`].
    #[inline]
    #[must_use]
    pub fn namespace(&self) -> Namespace<'_> {
        Namespace(&self.0)
    }
}

impl Debug for Value<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Value(\"")?;
        format_utf8_slice(&self.0, f)?;
        write!(f, "\")")?;

        Ok(())
    }
}

impl AsRef<[u8]> for Value<'_> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Borrow<[u8]> for Value<'_> {
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for Value<'static> {
    fn from(value: Vec<u8>) -> Self {
        Self(Cow::Owned(value))
    }
}

impl<'a> From<&'a [u8]> for Value<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl<'a> From<Cow<'a, [u8]>> for Value<'a> {
    fn from(value: Cow<'a, [u8]>) -> Self {
        Self(value)
    }
}

/// Represents a shared list of namespaces.
///
/// This is useful to not store the same map again and again. Is uses an [`Arc`]
/// under the hood, to maintain a reference to a shared list. If the list needs
/// to be manipulated, a copy is created (like using [`Cow`]).
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NamespacesShared<'a>(pub Arc<Namespaces<'a>>);

impl<'a> NamespacesShared<'a> {
    /// Create a new [`NamespacesShared`] instance from the passed `namespaces`.
    #[must_use]
    pub fn new(namespaces: Namespaces<'a>) -> Self {
        Self(Arc::new(namespaces))
    }

    /// Create a new cloned copy of this shared list.
    #[must_use]
    pub fn to_owned(&self) -> Namespaces<'a> {
        (*self.0).clone()
    }

    /// Convert this shared list into a unique list.
    ///
    /// This will clone the underlying [`Namespaces`] object if this shared list
    /// is owned more then once.
    #[must_use]
    pub fn into_owned(self) -> Namespaces<'a> {
        Arc::try_unwrap(self.0).unwrap_or_else(|x| (*x).clone())
    }
}

impl<'a> Deref for NamespacesShared<'a> {
    type Target = Namespaces<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NamespacesShared<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::make_mut(&mut self.0)
    }
}

impl Debug for NamespacesShared<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}
