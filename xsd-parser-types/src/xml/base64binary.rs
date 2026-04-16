use std::borrow::Cow;
use std::ops::Deref;

#[cfg(feature = "quick-xml")]
use crate::quick_xml::{
    DeserializeBytes, DeserializeHelper, Error, SerializeBytes, SerializeHelper,
};

/// Wrapper for base64Binary encoded as a String.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Base64String(pub String);

impl Base64String {
    /// Returns the byte length of the decoded data, not the length of the string.
    #[must_use]
    pub fn len(&self) -> usize {
        let bytes = self.0.trim_end_matches('=');
        bytes.len() * 3 / 4
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

impl From<String> for Base64String {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<Base64String> for String {
    fn from(value: Base64String) -> Self {
        value.0
    }
}

impl Deref for Base64String {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "quick-xml")]
impl SerializeBytes for Base64String {
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        self.0.serialize_bytes(helper)
    }
}

#[cfg(feature = "quick-xml")]
impl DeserializeBytes for Base64String {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let inner = String::deserialize_bytes(helper, bytes)?;
        Ok(Self(inner))
    }
}

/// Wrapper for base64Binary as decoded bytes.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Base64Binary(pub Vec<u8>);

impl Deref for Base64Binary {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
