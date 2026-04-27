use std::convert::Infallible;
use std::ops::Deref;
use std::str::FromStr;
use std::{borrow::Cow, ops::DerefMut};

#[cfg(feature = "quick-xml")]
use crate::quick_xml::{
    DeserializeBytes, DeserializeHelper, Error, SerializeBytes, SerializeHelper,
    WithDeserializerFromBytes, WithSerializeToBytes,
};

#[cfg(all(feature = "base64", any(feature = "quick-xml", feature = "serde")))]
use base64::{engine::general_purpose, Engine as _};

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

impl FromStr for Base64String {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_owned()))
    }
}

impl Deref for Base64String {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Base64String {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

#[cfg(feature = "quick-xml")]
impl WithSerializeToBytes for Base64String {}

#[cfg(feature = "quick-xml")]
impl WithDeserializerFromBytes for Base64String {}

/// Wrapper for base64Binary as decoded bytes.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Base64Binary(pub Vec<u8>);

impl From<Vec<u8>> for Base64Binary {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<Base64Binary> for Vec<u8> {
    fn from(value: Base64Binary) -> Self {
        value.0
    }
}

impl Deref for Base64Binary {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Base64Binary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(all(feature = "base64", feature = "quick-xml"))]
impl SerializeBytes for Base64Binary {
    fn serialize_bytes(
        &self,
        _helper: &mut SerializeHelper,
    ) -> Result<Option<Cow<'_, str>>, Error> {
        let base64_string = general_purpose::STANDARD.encode(&self.0);
        Ok(Some(Cow::Owned(base64_string)))
    }
}

#[cfg(all(feature = "base64", feature = "quick-xml"))]
impl DeserializeBytes for Base64Binary {
    fn deserialize_bytes(_helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let inner = general_purpose::STANDARD
            .decode(bytes)
            .map_err(Error::custom)?;
        Ok(Self(inner))
    }
}

#[cfg(all(feature = "base64", feature = "quick-xml"))]
impl WithSerializeToBytes for Base64Binary {}

#[cfg(all(feature = "base64", feature = "quick-xml"))]
impl WithDeserializerFromBytes for Base64Binary {}

#[cfg(all(feature = "base64", feature = "serde"))]
impl serde::Serialize for Base64Binary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let base64_string = general_purpose::STANDARD.encode(&self.0);
        serializer.serialize_str(&base64_string)
    }
}

#[cfg(all(feature = "base64", feature = "serde"))]
impl<'de> serde::Deserialize<'de> for Base64Binary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let base64_string = String::deserialize(deserializer)?;
        let bytes = general_purpose::STANDARD
            .decode(base64_string.as_bytes())
            .map_err(|e| serde::de::Error::custom(format!("Invalid base64 string: {}", e)))?;
        Ok(Self(bytes))
    }
}
