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

use crate::misc::{format_utf8_slice, RawByteStr};
use crate::quick_xml::{Error, ErrorKind};

/// Represents a list of unstructured XML attributes.
#[derive(Default, Debug, Clone)]
pub struct Attributes<'a>(IndexMap<Key<'a>, Value<'a>>);

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

#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Key<'a>(pub Cow<'a, [u8]>);

impl Key<'_> {
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

#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Value<'a>(pub Cow<'a, [u8]>);

impl Value<'_> {
    #[inline]
    pub fn unescape(&self) -> Result<Cow<'_, str>, Error> {
        self.unescape_with(resolve_predefined_entity)
    }

    #[inline]
    pub fn unescape_with<'entity>(
        &self,
        resolve_entity: impl FnMut(&str) -> Option<&'entity str>,
    ) -> Result<Cow<'_, str>, Error> {
        self.decode_and_unescape_with(UTF_8, resolve_entity)
    }

    #[inline]
    pub fn decode_and_unescape(&self, encoding: &'static Encoding) -> Result<Cow<'_, str>, Error> {
        self.decode_and_unescape_with(encoding, resolve_predefined_entity)
    }

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
