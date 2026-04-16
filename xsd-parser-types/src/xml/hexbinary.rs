use std::borrow::Cow;
use std::ops::Deref;

#[cfg(feature = "quick-xml")]
use crate::quick_xml::{
    DeserializeBytes, DeserializeHelper, Error, SerializeBytes, SerializeHelper,
};

/// Wrapper for hexBinary encoded as String.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HexString(pub String);

impl HexString {
    /// Returns the byte length of the decoded data, and not the length of the string.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len() / 2
    }

    /// Check emptyness
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get the inner string as a &str.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for HexString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<HexString> for String {
    fn from(value: HexString) -> Self {
        value.0
    }
}

impl Deref for HexString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "quick-xml")]
impl SerializeBytes for HexString {
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        self.0.serialize_bytes(helper)
    }
}

#[cfg(feature = "quick-xml")]
impl DeserializeBytes for HexString {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let inner = String::deserialize_bytes(helper, bytes)?;
        Ok(Self(inner))
    }
}

/// Wrapper for hexBinary as decoded bytes.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HexBinary(pub Vec<u8>);

impl Deref for HexBinary {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
