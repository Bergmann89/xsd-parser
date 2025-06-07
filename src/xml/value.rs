use std::fmt::{Debug, Formatter, Result as FmtResult};

use quick_xml::events::{BytesCData, BytesText};

use crate::models::format_utf8_slice;

use super::Element;

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
                format_utf8_slice(comment, f)?;
                write!(f, "\")")?;
            }
            Self::CData(cdata) => {
                write!(f, "CData(\"")?;
                format_utf8_slice(cdata, f)?;
                write!(f, "\")")?;
            }
            Self::Text(text) => {
                write!(f, "Text(\"")?;
                format_utf8_slice(text, f)?;
                write!(f, "\")")?;
            }
        }

        Ok(())
    }
}
