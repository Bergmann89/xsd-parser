use std::borrow::Cow;

use quick_xml::name::{QName as QuickXmlQName, ResolveResult};

use crate::quick_xml::{DeserializeBytes, Error, XmlReader};

use super::Namespace;

/// Type that represents a a resolved [`QName`].
///
/// The namespace of this [`QName`] was resolved during deserialization.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QName {
    raw: Vec<u8>,
    index: Option<usize>,
    ns: Option<Namespace>,
}

impl QName {
    /// Create a new [`QName`] instance from the passed `reader` and `raw` data.
    pub fn from_reader<R>(reader: &R, raw: &[u8]) -> Self
    where
        R: XmlReader,
    {
        let index = raw.iter().position(|x| *x == b':');
        let ns = match reader.resolve(QuickXmlQName(raw), true).0 {
            ResolveResult::Unbound | ResolveResult::Unknown(_) => None,
            ResolveResult::Bound(ns) => Some(Namespace(Cow::Owned(ns.0.to_owned()))),
        };
        let raw = raw.to_owned();

        Self { raw, index, ns }
    }

    /// Get the namespace of the [`QName`].
    #[must_use]
    pub fn namespace(&self) -> Option<&Namespace> {
        self.ns.as_ref()
    }

    /// Get the prefix of the [`QName`].
    #[must_use]
    pub fn prefix(&self) -> Option<&[u8]> {
        let index = self.index?;

        Some(&self.raw[0..index])
    }

    /// Get the local name of the [`QName`].
    #[must_use]
    pub fn local_name(&self) -> &[u8] {
        let index = self.index.map(|index| index + 1).unwrap_or_default();

        &self.raw[index..]
    }
}

impl DeserializeBytes for QName {
    fn deserialize_bytes<R: XmlReader>(reader: &R, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self::from_reader(reader, bytes))
    }
}
