use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::mem::replace;
use std::ops::{Deref, DerefMut};
use std::str::from_utf8;

use quick_xml::escape::{escape, unescape};
use quick_xml::events::{BytesText, Event};

use crate::quick_xml::{
    Deserializer, DeserializerArtifact, DeserializerEvent, DeserializerOutput, DeserializerResult,
    Error, ErrorKind, WithDeserializer, WithSerializer, XmlReader,
};

/// Type to represent text values inside a XML.
#[derive(
    Default,
    Debug,
    Clone,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct Text(pub String);

impl Text {
    /// Create a new [`Text`] instance from the passed `value`.
    #[must_use]
    pub fn new<S: Into<String>>(value: S) -> Self {
        Self(value.into())
    }

    /// Return the content of this text object as string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}

impl<X> From<X> for Text
where
    X: Into<String>,
{
    fn from(value: X) -> Self {
        Self::new(value)
    }
}

impl Deref for Text {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Text {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl WithSerializer for Text {
    type Serializer<'x> = TextSerializer<'x>;

    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, crate::quick_xml::Error> {
        let _name = name;
        let _is_root = is_root;

        Ok(TextSerializer::Emit { value: self })
    }
}

impl WithDeserializer for Text {
    type Deserializer = TextDeserializer;
}

/// Implemented the [`Serializer`](crate::quick_xml::Serializer) trait for [`Text`].
#[derive(Debug)]
pub enum TextSerializer<'ser> {
    /// Emit events for the contained text value.
    Emit {
        /// Value to emit events for.
        value: &'ser Text,
    },

    /// Serialization is done.
    Done,
}

impl<'ser> Iterator for TextSerializer<'ser> {
    type Item = Result<Event<'ser>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match replace(self, Self::Done) {
            Self::Emit { value } => {
                Some(Ok(Event::Text(BytesText::from_escaped(escape(&value.0)))))
            }
            Self::Done => None,
        }
    }
}

/// Implemented the [`Deserializer`] trait for [`Text`].
#[derive(Debug)]
pub enum TextDeserializer {
    /// Init the deserializer
    Init,

    /// Deserialize text elements.
    Text {
        /// Already deserialized text elements.
        value: Text,
    },
}

impl<'de> Deserializer<'de, Text> for TextDeserializer {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, Text>
    where
        R: XmlReader,
    {
        Self::Init.next(reader, event)
    }

    fn next<R>(self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, Text>
    where
        R: XmlReader,
    {
        let _reader = reader;

        let text = match event {
            Event::Text(x) => x.decode()?,
            Event::CData(x) => x.decode()?,
            Event::GeneralRef(x) => {
                let x = from_utf8(x.as_ref())?;

                Cow::Owned(format!("&{x};"))
            }
            event @ (Event::Start(_) | Event::Empty(_) | Event::End(_)) => {
                let artifact = match self {
                    Self::Init => DeserializerArtifact::None,
                    Self::Text { value } => DeserializerArtifact::Data(value),
                };

                return Ok(DeserializerOutput {
                    event: DeserializerEvent::Continue(event),
                    artifact,
                    allow_any: false,
                });
            }
            event => {
                let artifact = match self {
                    Self::Init => DeserializerArtifact::None,
                    Self::Text { value } => {
                        DeserializerArtifact::Deserializer(Self::Text { value })
                    }
                };

                return Ok(DeserializerOutput {
                    event: DeserializerEvent::Break(event),
                    artifact,
                    allow_any: false,
                });
            }
        };

        let mut value = match self {
            Self::Init => Text::default(),
            Self::Text { value } => value,
        };

        let text = unescape(&text)?;
        value.0.push_str(&text);

        Ok(DeserializerOutput {
            event: DeserializerEvent::None,
            artifact: DeserializerArtifact::Deserializer(Self::Text { value }),
            allow_any: false,
        })
    }

    fn finish<R>(self, reader: &R) -> Result<Text, Error>
    where
        R: XmlReader,
    {
        let _reader = reader;

        match self {
            Self::Init => Err(ErrorKind::MissingContent.into()),
            Self::Text { value } => Ok(value),
        }
    }
}
