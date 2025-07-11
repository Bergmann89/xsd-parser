use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ops::{Deref, DerefMut};

use quick_xml::events::{BytesText, Event};

use crate::quick_xml::{DeserializerArtifact, DeserializerEvent, DeserializerOutput};

use super::{Deserializer, DeserializerResult, Error, WithDeserializer, WithSerializer, XmlReader};

/// Used to represent xml elements with mixed content
#[derive(Debug)]
pub struct Mixed<T> {
    /// The actual element
    pub value: T,

    /// The text after the element
    pub text_after: Option<String>,
}

impl<T> Deref for Mixed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Mixed<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> WithSerializer for Mixed<T>
where
    T: WithSerializer,
{
    type Serializer<'x>
        = MixedSerializer<'x, T>
    where
        T: 'x;

    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let serializer = self.value.serializer(name, is_root)?;

        Ok(MixedSerializer::Inner {
            value: self,
            serializer,
        })
    }
}

impl<T> WithDeserializer for Mixed<T>
where
    T: WithDeserializer,
{
    type Deserializer = MixedDeserializer<T>;
}

/// Implements the [`Serializer`](super::Serializer) trait for the [`Mixed`] types.
pub enum MixedSerializer<'ser, T>
where
    T: WithSerializer,
{
    Inner {
        value: &'ser Mixed<T>,
        serializer: T::Serializer<'ser>,
    },
    Done,
}

impl<T> Debug for MixedSerializer<'_, T>
where
    T: WithSerializer,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Inner { serializer, .. } => f
                .debug_struct("Inner")
                .field("serializer", serializer)
                .finish_non_exhaustive(),
            Self::Done => write!(f, "Done"),
        }
    }
}

impl<'ser, T> Iterator for MixedSerializer<'ser, T>
where
    T: WithSerializer,
{
    type Item = Result<Event<'ser>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Done => None,
            Self::Inner { value, serializer } => match serializer.next() {
                None => {
                    let event = value
                        .text_after
                        .as_ref()
                        .map(|text| Ok(Event::Text(BytesText::from_escaped(text))));

                    *self = Self::Done;

                    event
                }
                Some(Ok(event)) => Some(Ok(event)),
                Some(Err(error)) => {
                    *self = Self::Done;

                    Some(Err(error))
                }
            },
        }
    }
}

/// Implements the [`Deserializer`] trait for the [`Mixed`] types.
pub enum MixedDeserializer<T>
where
    T: WithDeserializer,
{
    Inner {
        deserializer: T::Deserializer,
    },
    TextAfter {
        value: T,
        allow_any: bool,
        text_after: Option<String>,
    },
}

impl<T> MixedDeserializer<T>
where
    T: WithDeserializer,
{
    fn handle_inner_output<'de, R>(
        reader: &R,
        output: DeserializerOutput<'de, T>,
    ) -> DeserializerResult<'de, Mixed<T>>
    where
        R: XmlReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        let deserializer = match artifact {
            DeserializerArtifact::None => {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                })
            }
            DeserializerArtifact::Deserializer(deserializer) => Self::Inner { deserializer },
            DeserializerArtifact::Data(value) => Self::TextAfter {
                value,
                allow_any,
                text_after: None,
            },
        };

        match (deserializer, event) {
            (deserializer @ Self::TextAfter { .. }, DeserializerEvent::Continue(event)) => {
                deserializer.next(reader, event)
            }
            (deserializer, event) => Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(deserializer),
                event,
                allow_any,
            }),
        }
    }
}

impl<T> Debug for MixedDeserializer<T>
where
    T: WithDeserializer,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Inner { deserializer } => f
                .debug_struct("Inner")
                .field("deserializer", deserializer)
                .finish(),
            Self::TextAfter { text_after, .. } => f
                .debug_struct("TextAfter")
                .field("text_after", text_after)
                .finish_non_exhaustive(),
        }
    }
}

impl<'de, T> Deserializer<'de, Mixed<T>> for MixedDeserializer<T>
where
    T: WithDeserializer,
{
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, Mixed<T>>
    where
        R: XmlReader,
    {
        let output = T::Deserializer::init(reader, event)?;

        Self::handle_inner_output(reader, output)
    }

    fn next<R>(self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, Mixed<T>>
    where
        R: XmlReader,
    {
        match self {
            Self::Inner { deserializer } => {
                let output = deserializer.next(reader, event)?;

                Self::handle_inner_output(reader, output)
            }
            Self::TextAfter {
                value,
                allow_any,
                mut text_after,
            } => match event {
                event @ (Event::Start(_) | Event::Empty(_) | Event::End(_)) => {
                    let data = Mixed { value, text_after };
                    let artifact = DeserializerArtifact::Data(data);

                    Ok(DeserializerOutput {
                        artifact,
                        event: DeserializerEvent::Continue(event),
                        allow_any,
                    })
                }
                Event::Text(text) => {
                    let text = text.decode()?;

                    text_after.get_or_insert_default().push_str(&text);

                    let artifact = DeserializerArtifact::Deserializer(Self::TextAfter {
                        value,
                        allow_any,
                        text_after,
                    });

                    Ok(DeserializerOutput {
                        artifact,
                        event: DeserializerEvent::None,
                        allow_any,
                    })
                }
                event => {
                    let artifact = DeserializerArtifact::Deserializer(Self::TextAfter {
                        value,
                        allow_any,
                        text_after,
                    });

                    Ok(DeserializerOutput {
                        artifact,
                        event: DeserializerEvent::Continue(event),
                        allow_any,
                    })
                }
            },
        }
    }

    fn finish<R>(self, reader: &R) -> Result<Mixed<T>, Error>
    where
        R: XmlReader,
    {
        match self {
            Self::Inner { deserializer } => {
                let value = deserializer.finish(reader)?;

                Ok(Mixed {
                    value,
                    text_after: None,
                })
            }
            Self::TextAfter {
                value,
                allow_any: _,
                text_after,
            } => Ok(Mixed { value, text_after }),
        }
    }
}
