use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use crate::misc::format_utf8_slice;

/// Represents a XML namespace prefix.
#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NamespacePrefix(pub Cow<'static, [u8]>);

impl NamespacePrefix {
    /// Common used XML schema namespace prefix.
    pub const XS: Self = Self(Cow::Borrowed(b"xs"));

    /// Common used XML schema instance namespace prefix.
    pub const XSI: Self = Self(Cow::Borrowed(b"xsi"));

    /// Common used XML namespace prefix.
    pub const XML: Self = Self(Cow::Borrowed(b"xml"));
}

impl NamespacePrefix {
    /// Create a new [`NamespacePrefix`] instance from the passed `value`.
    #[must_use]
    pub fn new<X>(value: X) -> Self
    where
        X: Into<Cow<'static, [u8]>>,
    {
        Self(value.into())
    }

    /// Create a new [`NamespacePrefix`] instance from the passed `value`.
    ///
    /// In contrast to [`new`](Self::new) this is a const function
    /// and can be used during compile time.
    #[must_use]
    pub const fn new_const(value: &'static [u8]) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl<X> From<X> for NamespacePrefix
where
    X: Into<Cow<'static, [u8]>>,
{
    fn from(value: X) -> Self {
        Self(value.into())
    }
}

impl<'x> From<&'x Self> for NamespacePrefix {
    fn from(value: &'x Self) -> Self {
        value.clone()
    }
}

impl Debug for NamespacePrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "NamespacePrefix(b\"")?;

        format_utf8_slice(&self.0, f)?;

        write!(f, "\")")?;

        Ok(())
    }
}

impl Display for NamespacePrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        format_utf8_slice(&self.0, f)
    }
}
