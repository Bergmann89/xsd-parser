use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use quick_xml::name::{QName as QuickXmlQName, ResolveResult};

use crate::misc::{format_utf8_slice, Namespace};

#[cfg(feature = "quick-xml")]
use crate::quick_xml::{DeserializeBytes, DeserializeHelper, Error};

/// Type that represents a a resolved [`QName`].
///
/// The namespace of this [`QName`] was resolved during deserialization.
#[derive(Clone, Eq, PartialEq)]
pub struct QName {
    raw: Vec<u8>,
    index: Option<usize>,
    ns: Option<Namespace>,
}

impl QName {
    /// Get the raw bytes of the [`QName`].
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.raw
    }

    /// Create a new [`QName`] instance from the passed `reader` and `raw` data.
    #[must_use]
    #[cfg(feature = "quick-xml")]
    pub fn from_helper(helper: &DeserializeHelper, raw: &[u8]) -> Self {
        let index = raw.iter().position(|x| *x == b':');
        let ns = match helper.resolve(QuickXmlQName(raw), false).0 {
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

impl AsRef<[u8]> for QName {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(feature = "quick-xml")]
impl DeserializeBytes for QName {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self::from_helper(helper, bytes))
    }
}

#[allow(clippy::missing_fields_in_debug)]
impl Debug for QName {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        struct Helper<'a>(&'a [u8]);

        impl Debug for Helper<'_> {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, "b\"")?;
                format_utf8_slice(self.0, f)?;
                write!(f, "\"")?;

                Ok(())
            }
        }

        f.debug_struct("QName")
            .field("raw", &Helper(&self.raw))
            .field("ns", &self.ns)
            .finish()
    }
}

impl Display for QName {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        format_utf8_slice(&self.raw, f)
    }
}
