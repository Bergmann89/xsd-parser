use std::fmt::{Debug, Formatter, Result as FmtResult};

#[cfg(feature = "quick-xml")]
use quick_xml::events::{BytesCData, BytesText};

use crate::misc::format_utf8_slice;

use super::Element;

#[cfg(not(feature = "quick-xml"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BytesText<'a>(std::borrow::Cow<'a, str>);

#[cfg(not(feature = "quick-xml"))]
impl<'a> std::ops::Deref for BytesText<'a> {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

#[cfg(not(feature = "quick-xml"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BytesCData<'a>(std::borrow::Cow<'a, str>);

#[cfg(not(feature = "quick-xml"))]
impl<'a> std::ops::Deref for BytesCData<'a> {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

/// Represents unstructured XML data.
///
/// This is mainly used to store the data contained by an [`Element`].
#[derive(Clone, Eq, PartialEq)]
pub enum Value<'a> {
    /// A child [`Element`].
    Element(Element<'a>),

    /// A comment in the XML code.
    Comment(BytesText<'a>),

    /// A CDATA value.
    CData(BytesCData<'a>),

    /// A simple text value.
    Text(BytesText<'a>),
}

impl Debug for Value<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Element(element) => element.fmt(f)?,
            Self::Comment(comment) => {
                write!(f, "Comment(\"")?;
                format_utf8_slice(comment.as_bytes(), f)?;
                write!(f, "\")")?;
            }
            Self::CData(cdata) => {
                write!(f, "CData(\"")?;
                format_utf8_slice(cdata.as_bytes(), f)?;
                write!(f, "\")")?;
            }
            Self::Text(text) => {
                write!(f, "Text(\"")?;
                format_utf8_slice(text.as_bytes(), f)?;
                write!(f, "\")")?;
            }
        }

        Ok(())
    }
}
