//! The `quick_xml` module contains helper types for serializing and deserializing
//! generated code using the [`quick_xml`] crate.

pub mod reader;

mod deserialize;
mod error;
mod serialize;

use std::sync::LazyLock;

pub use std::io::Write as XmlWrite;

pub use quick_xml::{
    events::{BytesCData, BytesDecl, BytesEnd, BytesPI, BytesStart, BytesText, Event},
    name::{LocalName, Namespace, QName, ResolveResult},
    Writer,
};
use regex::Regex;

pub use crate::misc::RawByteStr;

pub use self::deserialize::{
    ContentDeserializer, DeserializeBytes, DeserializeBytesFromStr, DeserializeHelper,
    DeserializeStrError, DeserializeSync, Deserializer, DeserializerArtifact, DeserializerEvent,
    DeserializerOutput, DeserializerResult, ElementHandlerOutput, WithDeserializer,
};
pub use self::error::{Error, Kind as ErrorKind, UnionError, ValidateError};
pub use self::reader::{ErrorReader, IoReader, SliceReader, XmlReader, XmlReaderSync};
pub use self::serialize::{
    BoxedSerializer, ContentSerializer, DerefIter, IterSerializer, SerializeBytes,
    SerializeBytesToString, SerializeHelper, SerializeSync, Serializer, WithBoxedSerializer,
    WithSerializer,
};

#[cfg(feature = "async")]
pub use tokio::io::AsyncWrite as XmlWriteAsync;

#[cfg(feature = "async")]
pub use self::serialize::SerializeAsync;

#[cfg(feature = "async")]
pub use self::deserialize::DeserializeAsync;

#[cfg(feature = "async")]
pub use self::reader::XmlReaderAsync;

/// Helper method to replace whitespaces in a string value.
#[must_use]
pub fn whitespace_replace(s: &str) -> String {
    s.replace(['\t', '\n', '\r'], " ")
}

/// Helper method to collapse whitespaces in a string value.
pub fn whitespace_collapse(s: &str) -> String {
    RX_WHITESPACE_COLLAPSE.replace_all(s, " ").trim().to_owned()
}

/// Helper method the get the total number of digits for a decimal string value.
///
/// # Errors
///
/// Returns [`ValidateError::InvalidDecimalValue`] if the passed string is
/// not a valid decimal value, or [`ValidateError::TotalDigits`] if it exceeds
/// the `expected` amount or total digits.
pub fn total_digits(s: &str, expected: usize) -> Result<(), ValidateError> {
    let m = RX_DECIMAL
        .captures(s)
        .ok_or(ValidateError::InvalidDecimalValue)?;

    let actual = match (m.get(1), m.get(2)) {
        (None, _) => unreachable!("Capture group 1 should be always present"),
        (Some(a), None) => a.len(),
        (Some(a), Some(b)) => a.len() + b.len(),
    };

    if actual <= expected {
        Ok(())
    } else {
        Err(ValidateError::TotalDigits(expected))
    }
}

/// Helper method the get the number of fraction digits for a decimal string value.
///
/// # Errors
///
/// Returns [`ValidateError::InvalidDecimalValue`] if the passed string is
/// not a valid decimal value, or [`ValidateError::FractionDigits`] if it exceeds
/// the `expected` amount or fraction digits.
pub fn fraction_digits(s: &str, expected: usize) -> Result<(), ValidateError> {
    let m = RX_DECIMAL
        .captures(s)
        .ok_or(ValidateError::InvalidDecimalValue)?;

    let actual = m.get(2).map(|s| s.len()).unwrap_or_default();

    if actual <= expected {
        Ok(())
    } else {
        Err(ValidateError::FractionDigits(expected))
    }
}

static RX_DECIMAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^-?([0-9]+)(?:\.([0-9]*))?$").unwrap());
static RX_WHITESPACE_COLLAPSE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());
