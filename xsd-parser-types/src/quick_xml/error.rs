use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::mem::take;
use std::str::Utf8Error;

use quick_xml::{
    encoding::EncodingError,
    escape::EscapeError,
    events::{attributes::AttrError, Event},
    Error as XmlError,
};
use thiserror::Error;

use crate::misc::RawByteStr;

/// Quick XML Error
#[derive(Debug)]
pub struct Error {
    /// Detailed information about the actual error.
    pub kind: Kind,

    /// Cursor position inside the XML document where the error occurred.
    pub position: Option<u64>,

    /// Path of XML tags where the error occurred.
    pub elements: Option<Vec<String>>,
}

/// Quick XML error kind.
#[derive(Debug, Error)]
pub enum Kind {
    /// Error forwarded from the [`quick_xml`] crate.
    #[error("IO Error: message={0}")]
    IoError(
        #[from]
        #[source]
        IoError,
    ),

    /// Error forwarded from the [`quick_xml`] crate.
    #[error("XML Error: message={0}")]
    XmlError(
        #[from]
        #[source]
        XmlError,
    ),

    /// Attribute error forwarded from the [`quick_xml`] crate.
    #[error("Attribute Error: message={0}")]
    AttrError(
        #[from]
        #[source]
        AttrError,
    ),

    /// Invalid UTF-8 string.
    #[error("UTF-8 Error: message={0}")]
    InvalidUtf8(
        #[from]
        #[source]
        Utf8Error,
    ),

    /// Escape Error
    #[error("Escape Error: message={0}")]
    EscapeError(
        #[from]
        #[source]
        EscapeError,
    ),

    /// Encoding Error
    #[error("Encoding Error: message={0}")]
    EncodingError(
        #[from]
        #[source]
        EncodingError,
    ),

    /// Duplicate attribute.
    ///
    /// The attribute was expected only once.
    #[error("Duplicated attribute: name={0}")]
    DuplicateAttribute(RawByteStr),

    /// Duplicate element.
    ///
    /// The element was expected only once.
    #[error("Duplicated element: name={0}")]
    DuplicateElement(RawByteStr),

    /// Unexpected attribute.
    ///
    /// The attribute was not expected for the current element.
    #[error("Unexpected attribute: name={0}")]
    UnexpectedAttribute(RawByteStr),

    /// Missing attribute.
    ///
    /// The attribute was expected to be present, but it was not.
    #[error("Missing attribute: name={0}")]
    MissingAttribute(RawByteStr),

    /// Missing element.
    ///
    /// The element was expected to be present, but it was not.
    #[error("Missing element: name={0}")]
    MissingElement(RawByteStr),

    /// Invalid data.
    #[error("Invalid data: `{0}`")]
    InvalidData(RawByteStr),

    /// Invalid value.
    #[error("Invalid value `{0}`: {1}")]
    InvalidValue(RawByteStr, ValidateError),

    /// Missing content.
    ///
    /// The element was expected to have some content, but it haven't.
    #[error("Missing content")]
    MissingContent,

    /// Duplicate content.
    ///
    /// The content was expected only once.
    #[error("Duplicate content")]
    DuplicateContent,

    /// Missing any element.
    ///
    /// The element was expected to have at least one `xs:any` element, but it haven't.
    #[error("Missing any element")]
    MissingAnyElement,

    /// Duplicate content.
    ///
    /// The `xs:any` element is only expected once.
    #[error("Duplicate any element")]
    DuplicateAnyElement,

    /// Missing name.
    ///
    /// The serializer is not able to set a default name for the specified value.
    #[error("Missing name")]
    MissingName,

    /// Unknown or invalid value.
    #[error("Unknown or invalid value: {0}")]
    UnknownOrInvalidValue(RawByteStr),

    /// Insufficient size.
    ///
    /// The element or attribute contains less items then expected.
    #[error("Insufficient size (min={min}, max={max}, actual={actual})")]
    InsufficientSize {
        /// Smallest expected index.
        min: usize,

        /// Largest expected index.
        max: usize,

        /// Actual index.
        actual: usize,
    },

    /// Invalid union.
    #[error("Invalid union: {0}")]
    InvalidUnion(UnionError),

    /// Custom error.
    ///
    /// Can store any kind of error.
    #[error("Custom Error: message={0}")]
    Custom(Box<dyn StdError + Send + Sync>),

    /// Unexpected [`quick_xml`] event.
    #[error("Unexpected event: {0:#?}!")]
    UnexpectedEvent(Event<'static>),

    /// Unexpected end of file.
    #[error("Unexpected EoF!")]
    UnexpectedEoF,

    /// Unexpected end of element.
    #[error("Unexpected EoL!")]
    UnexpectedEoL,
}

/// Error raised by different XML value validation functions.
#[derive(Debug, Error)]
pub enum ValidateError {
    /// Value is not a valid decimal.
    #[error("Value is not a valid decimal!")]
    InvalidDecimalValue,

    /// The string value is too short.
    #[error("Value is shorter then {0} characters!")]
    MinLength(usize),

    /// The string value is too large.
    #[error("Value is longer than {0} characters!")]
    MaxLength(usize),

    /// The string value does not match the expected pattern.
    #[error("Value does not match the expected pattern `{0}`!")]
    Pattern(&'static str),

    /// The decimal value has too much total digits.
    #[error("Value has more then {0} total digits!")]
    TotalDigits(usize),

    /// The decimal value has too much fraction digits.
    #[error("Value has more then {0} fraction digits!")]
    FractionDigits(usize),

    /// The decimal value is less than the expected range.
    ///
    /// Range is defined by `xs:minInclusive`.
    #[error("Value is less then `{0}`!")]
    LessThan(&'static str),

    /// The decimal value is less or equal than the expected range.
    ///
    /// Range is defined by `xs:minExclusive`.
    #[error("Value is less or equal then `{0}`!")]
    LessEqualThan(&'static str),

    /// The decimal value is greater than the expected range.
    ///
    /// Range is defined by `xs:maxInclusive`.
    #[error("Value is greater then `{0}`!")]
    GraterThan(&'static str),

    /// The decimal value is greater or equal than the expected range.
    ///
    /// Range is defined by `xs:maxExclusive`.
    #[error("Value is greater equal then `{0}`!")]
    GraterEqualThan(&'static str),
}

impl Error {
    /// Create a new error that uses [`Kind::Custom`] to store the passed `error`.
    pub fn custom<E: StdError + Send + Sync + 'static>(error: E) -> Self {
        Kind::Custom(Box::new(error)).into()
    }

    pub(super) fn new<E>(error: E) -> Self
    where
        Kind: From<E>,
    {
        Self {
            kind: Kind::from(error),
            position: None,
            elements: None,
        }
    }

    pub(super) fn with_pos(mut self, position: u64) -> Self {
        self.position = Some(position);

        self
    }

    pub(super) fn with_error_info(mut self, info: &ErrorInfo) -> Self {
        self.elements = Some(info.elements.clone());

        self
    }
}

impl<E> From<E> for Error
where
    Kind: From<E>,
{
    fn from(error: E) -> Self {
        Self::new(error)
    }
}

impl<'a> From<(&'a [u8], ValidateError)> for Error {
    fn from((value, error): (&'a [u8], ValidateError)) -> Self {
        Self::new(Kind::InvalidValue(RawByteStr::from_slice(value), error))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", &self.kind)?;

        if let Some(pos) = &self.position {
            write!(f, "; position={}", pos)?;
        }

        if let Some(elements) = &self.elements {
            let mut first = true;

            for element in elements {
                if take(&mut first) {
                    write!(f, "; element={}", element)?;
                } else {
                    write!(f, ">{}", element)?;
                }
            }
        }

        Ok(())
    }
}

impl StdError for Error {}

/// Contains the different errors that occurred when deserializing a union.
pub struct UnionError(Vec<Box<dyn StdError + Send + Sync + 'static>>);

impl<X> From<X> for UnionError
where
    X: IntoIterator,
    X::Item: StdError + Send + Sync + 'static,
{
    fn from(value: X) -> Self {
        Self(
            value
                .into_iter()
                .map(|err| -> Box<dyn StdError + Send + Sync + 'static> { Box::new(err) })
                .collect(),
        )
    }
}

impl Debug for UnionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for err in &self.0 {
            write!(f, "- {err}")?;
        }

        Ok(())
    }
}

impl Display for UnionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for err in &self.0 {
            write!(f, "- {err}")?;
        }

        Ok(())
    }
}

#[derive(Default, Debug)]
pub(super) struct ErrorInfo {
    elements: Vec<String>,
}

impl ErrorInfo {
    pub(super) fn update(&mut self, event: &Event<'_>) {
        match event {
            Event::Start(x) => {
                self.elements
                    .push(String::from_utf8_lossy(x.name().0).to_string());
            }
            Event::End(_) => {
                self.elements.pop();
            }
            _ => (),
        }
    }
}
