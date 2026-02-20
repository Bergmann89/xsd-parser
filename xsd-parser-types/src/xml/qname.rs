use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};

use quick_xml::name::{QName as QuickXmlQName, ResolveResult};

use crate::misc::{format_utf8_slice, Namespace};

#[cfg(feature = "quick-xml")]
use crate::quick_xml::{
    DeserializeBytes, DeserializeHelper, Error, SerializeBytes, SerializeHelper,
};

/// Type that represents a a resolved [`QName`].
///
/// The namespace of this [`QName`] was resolved during deserialization.
#[derive(Clone)]
pub struct QName {
    raw: Cow<'static, [u8]>,
    index: Option<usize>,
    ns: Option<Namespace>,
}

impl QName {
    /// Create a new [`QName`] instance from the passed `raw` data, `index` and `ns`.
    ///
    /// Caution! This is a const function and can be used during compile time, but
    /// the `raw` data must be valid UTF-8 and the `index` must be correct, otherwise
    /// the `QName` is not valid.
    #[must_use]
    pub const fn new_const(
        raw: &'static [u8],
        index: Option<usize>,
        ns: Option<Namespace>,
    ) -> Self {
        Self {
            raw: Cow::Borrowed(raw),
            index,
            ns,
        }
    }

    /// Create a new [`QName`] instance from the passed `bytes`.
    #[must_use]
    pub fn from_bytes<X>(bytes: X) -> Self
    where
        X: Into<Cow<'static, [u8]>>,
    {
        let raw = bytes.into();
        let index = raw.iter().position(|x| *x == b':');

        Self {
            raw,
            index,
            ns: None,
        }
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
        let raw = Cow::Owned(raw.to_owned());

        Self { raw, index, ns }
    }

    /// Get the raw bytes of the [`QName`].
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.raw
    }

    /// Set the namespace of the [`QName`] to `ns`.
    #[must_use]
    pub fn with_namespace(mut self, ns: Namespace) -> Self {
        self.ns = Some(ns);

        self
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

    /// Get the index of the [`QName`].
    #[must_use]
    pub fn index(&self) -> Option<usize> {
        self.index
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
impl SerializeBytes for QName {
    fn serialize_bytes(
        &self,
        helper: &mut SerializeHelper,
    ) -> Result<Option<Cow<'static, str>>, Error> {
        use std::str::from_utf8;

        let prefix = if let Some(ns) = &self.ns.as_ref() {
            helper.get_namespace_prefix(ns)?
        } else {
            use crate::misc::NamespacePrefix;

            self.prefix().map(|x| NamespacePrefix::new(x.to_vec()))
        };
        let name = from_utf8(self.local_name())?;

        let value = if let Some(prefix) = prefix {
            format!("{prefix}:{name}")
        } else {
            name.to_string()
        };

        Ok(Some(Cow::Owned(value)))
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

impl Eq for QName {}

impl PartialEq for QName {
    fn eq(&self, other: &Self) -> bool {
        if let (Some(ns1), Some(ns2)) = (&self.ns, &other.ns) {
            ns1 == ns2 && self.local_name() == other.local_name()
        } else {
            self.raw == other.raw
        }
    }
}

impl Hash for QName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(ns) = &self.ns {
            ns.hash(state);
            self.local_name().hash(state);
        } else {
            self.raw.hash(state);
        }
    }
}
