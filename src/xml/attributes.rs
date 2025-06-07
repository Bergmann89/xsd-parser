use std::borrow::{Borrow, Cow};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ops::{Deref, DerefMut};

use encoding_rs::{Encoding, UTF_8};
use indexmap::map::Entry;
use quick_xml::{
    encoding::decode,
    escape::{resolve_predefined_entity, unescape_with},
    events::attributes::Attribute,
};

use indexmap::IndexMap;
use quick_xml::name::QName;

use crate::models::{format_utf8_slice, RawByteStr};
use crate::quick_xml::{Error, ErrorKind};

/// Represents a list of unstructured XML attributes.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Attributes<'a>(pub IndexMap<Key<'a>, Value<'a>>);

/// Helper type for attributes with static lifetime
pub type AnyAttributes = Attributes<'static>;

impl<'a> Attributes<'a> {
    /// Push a attribute to the list of attributes.
    ///
    /// This will push a new owned (static lifetime) attribute to the list of
    /// attributes. If the attribute already exists, an error is raised.
    ///
    /// # Errors
    ///
    /// Raises a [`DuplicateAttribute`](ErrorKind::DuplicateAttribute) error if
    /// the passed attribute is already part of the list.
    pub fn push(&mut self, attrib: Attribute<'_>) -> Result<(), Error> {
        let key = Key(Cow::Owned(attrib.key.0.to_owned()));

        match self.0.entry(key) {
            Entry::Vacant(e) => {
                e.insert(Value(Cow::Owned(attrib.value.into_owned())));

                Ok(())
            }
            Entry::Occupied(e) => {
                Err(ErrorKind::DuplicateAttribute(RawByteStr::from_slice(e.key())).into())
            }
        }
    }

    /// Insert a new attribute into the list.
    ///
    /// Returns the value that already exists for the specified `key`.
    pub fn insert<K, V>(&mut self, key: K, value: V) -> Option<Value<'a>>
    where
        K: Into<Cow<'a, [u8]>>,
        V: Into<Cow<'a, [u8]>>,
    {
        let key = Key(key.into());
        let value = Value(value.into());

        self.0.insert(key, value)
    }

    /// Create an iterator that yields [`Attribute`] items.
    pub fn attributes(&self) -> impl Iterator<Item = Attribute<'_>> {
        self.iter().map(|(k, v)| Attribute {
            key: QName(k.as_ref()),
            value: Cow::Borrowed(v.as_ref()),
        })
    }
}

impl<'a> Deref for Attributes<'a> {
    type Target = IndexMap<Key<'a>, Value<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Attributes<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> Extend<Attribute<'a>> for Attributes<'a> {
    fn extend<T: IntoIterator<Item = Attribute<'a>>>(&mut self, iter: T) {
        self.0.extend(iter.into_iter().map(|a| {
            let key = Key(Cow::Borrowed(a.key.0));
            let value = Value(a.value.clone());

            (key, value)
        }));
    }
}

impl<'a> Extend<(Key<'a>, Value<'a>)> for Attributes<'a> {
    fn extend<T: IntoIterator<Item = (Key<'a>, Value<'a>)>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl<'a> FromIterator<(Key<'a>, Value<'a>)> for Attributes<'a> {
    fn from_iter<T: IntoIterator<Item = (Key<'a>, Value<'a>)>>(iter: T) -> Self {
        Self(FromIterator::from_iter(iter))
    }
}

/// Wrapper type that is used as key for the [`Attributes`] map.
#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Key<'a>(pub Cow<'a, [u8]>);

impl Key<'_> {
    /// Return the key as [`QName`].
    #[must_use]
    pub fn qname(&self) -> QName<'_> {
        QName(&self.0)
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

/// Wrapper type that is used as value for the [`Attributes`] map.
#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Value<'a>(pub Cow<'a, [u8]>);

impl Value<'_> {
    /// Return the unescaped value using UTF-8 encoding.
    ///
    /// # Errors
    ///
    /// Throws an error if the value could not be unescaped.
    #[inline]
    pub fn unescape(&self) -> Result<Cow<'_, str>, Error> {
        self.unescape_with(resolve_predefined_entity)
    }

    /// Return the unescaped value using UTF-8 encoding and the passed function
    /// to resolve the escaped string.
    ///
    /// For details see [`unescape_with`].
    ///
    /// # Errors
    ///
    /// Throws an error if the value could not be unescaped.
    #[inline]
    pub fn unescape_with<'entity>(
        &self,
        resolve_entity: impl FnMut(&str) -> Option<&'entity str>,
    ) -> Result<Cow<'_, str>, Error> {
        self.decode_and_unescape_with(UTF_8, resolve_entity)
    }

    /// Decodes and unescape the value using the passed `encoding`.
    ///
    /// # Errors
    ///
    /// Throws an error if the value could not be unescaped.
    #[inline]
    pub fn decode_and_unescape(&self, encoding: &'static Encoding) -> Result<Cow<'_, str>, Error> {
        self.decode_and_unescape_with(encoding, resolve_predefined_entity)
    }

    /// Return the unescaped value using the passed `encoding` and the passed
    /// function to resolve the escaped string.
    ///
    /// For details see [`unescape_with`].
    ///
    /// # Errors
    ///
    /// Throws an error if the value could not be unescaped.
    #[inline]
    pub fn decode_and_unescape_with<'entity>(
        &self,
        encoding: &'static Encoding,
        resolve_entity: impl FnMut(&str) -> Option<&'entity str>,
    ) -> Result<Cow<'_, str>, Error> {
        let decoded = decode(&self.0, encoding)?;

        match unescape_with(&decoded, resolve_entity)? {
            Cow::Borrowed(_) => Ok(decoded),
            Cow::Owned(s) => Ok(s.into()),
        }
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
