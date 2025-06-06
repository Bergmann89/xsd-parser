use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// Helper type that implements [`Debug`] and [`Display`] for a byte slice.
pub struct RawByteStr(Cow<'static, [u8]>);

impl RawByteStr {
    /// Create a new [`RawByteStr`] instance from the passed byte slice.
    #[must_use]
    pub fn from_slice(value: &[u8]) -> Self {
        Self(Cow::Owned(value.to_owned()))
    }
}

impl From<Vec<u8>> for RawByteStr {
    fn from(value: Vec<u8>) -> Self {
        Self(Cow::Owned(value))
    }
}

impl From<&'static [u8]> for RawByteStr {
    fn from(value: &'static [u8]) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl From<&'static str> for RawByteStr {
    fn from(value: &'static str) -> Self {
        Self(Cow::Borrowed(value.as_bytes()))
    }
}

impl Debug for RawByteStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "\"")?;
        format_utf8_slice(&self.0, f)?;
        write!(f, "\"")?;

        Ok(())
    }
}

impl Display for RawByteStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "\"")?;
        format_utf8_slice(&self.0, f)?;
        write!(f, "\"")?;

        Ok(())
    }
}

pub(crate) fn format_utf8_slice(bytes: &[u8], f: &mut Formatter<'_>) -> FmtResult {
    for byte in bytes {
        if byte.is_ascii_control() {
            write!(f, r"\x{byte:02X}")?;
        } else {
            write!(f, "{}", *byte as char)?;
        }
    }

    Ok(())
}
