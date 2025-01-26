use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use crate::misc::format_utf8_slice;

/// Represents a XML namespace.
#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Namespace(pub Cow<'static, [u8]>);

impl Namespace {
    /// The XML schema namespace
    pub const XS: Self = Self(Cow::Borrowed(b"http://www.w3.org/2001/XMLSchema"));

    /// The XML schema instance namespace
    pub const XSI: Self = Self(Cow::Borrowed(b"http://www.w3.org/2001/XMLSchema-instance"));

    /// The XML namespace.
    pub const XML: Self = Self(Cow::Borrowed(b"http://www.w3.org/XML/1998/namespace"));
}

impl Namespace {
    /// Create a new [`Namespace`] instance from the passed `value`.
    #[must_use]
    pub fn new<X>(value: X) -> Self
    where
        X: Into<Cow<'static, [u8]>>,
    {
        Self(value.into())
    }
}

impl<X> From<X> for Namespace
where
    X: Into<Cow<'static, [u8]>>,
{
    fn from(value: X) -> Self {
        Self(value.into())
    }
}

impl<'x> From<&'x Self> for Namespace {
    fn from(value: &'x Self) -> Self {
        value.clone()
    }
}

impl Debug for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Namespace(b\"")?;

        format_utf8_slice(&self.0, f)?;

        write!(f, "\")")?;

        Ok(())
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        format_utf8_slice(&self.0, f)
    }
}
