use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ops::{Deref, DerefMut};

#[cfg(feature = "quick-xml")]
use quick_xml::events::Event;

#[cfg(feature = "quick-xml")]
use crate::quick_xml::{
    Deserializer, DeserializerArtifact, DeserializerEvent, DeserializerOutput, DeserializerResult,
    Error, IterSerializer, WithDeserializer, WithSerializer, XmlReader,
};

use super::text::{Text, TextDeserializer};

/// Used to represent xml elements with mixed content
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mixed<T> {
    /// The actual element
    pub value: T,

    /// The text after the element
    pub text_after: Option<Text>,
}

impl<T> Mixed<T> {
    /// Create a new [`Mixed`] instance from the passed `value`.
    #[must_use]
    pub fn new(value: T) -> Self {
        Self {
            value,
            text_after: None,
        }
    }

    /// Set the text after the actual XML element.
    #[must_use]
    pub fn with_text_after(mut self, text: Text) -> Self {
        self.text_after = Some(text);

        self
    }
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

#[cfg(feature = "quick-xml")]
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

#[cfg(feature = "quick-xml")]
impl<T> WithDeserializer for Mixed<T>
where
    T: WithDeserializer,
{
    type Deserializer = MixedDeserializer<T>;
}

/// Implements the [`Serializer`](crate::quick_xml::Serializer) trait for the [`Mixed`] types.
#[cfg(feature = "quick-xml")]
pub enum MixedSerializer<'ser, T>
where
    T: WithSerializer,
{
    /// Serialize the inner element.
    Inner {
        /// Value that needs to be serialized.
        value: &'ser Mixed<T>,

        /// Current serializer of the inner value.
        serializer: T::Serializer<'ser>,
    },

    /// Serialize the text after the element.
    TextAfter {
        /// Current serializer of the text after.
        serializer: IterSerializer<'ser, Option<&'ser Text>, Text>,
    },

    /// Serialization is done
    Done,
}

#[cfg(feature = "quick-xml")]
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
            Self::TextAfter { serializer, .. } => f
                .debug_struct("TextAfter")
                .field("serializer", serializer)
                .finish_non_exhaustive(),
            Self::Done => write!(f, "Done"),
        }
    }
}

#[cfg(feature = "quick-xml")]
impl<'ser, T> Iterator for MixedSerializer<'ser, T>
where
    T: WithSerializer,
{
    type Item = Result<Event<'ser>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self {
                Self::Inner { value, serializer } => match serializer.next() {
                    Some(Ok(event)) => return Some(Ok(event)),
                    Some(Err(error)) => {
                        *self = Self::Done;

                        return Some(Err(error));
                    }
                    None => {
                        let serializer =
                            IterSerializer::new(value.text_after.as_ref(), None, false);

                        *self = Self::TextAfter { serializer };
                    }
                },
                Self::TextAfter { serializer } => match serializer.next() {
                    Some(Ok(event)) => return Some(Ok(event)),
                    Some(Err(error)) => {
                        *self = Self::Done;

                        return Some(Err(error));
                    }
                    None => {
                        *self = Self::Done;

                        return None;
                    }
                },
                Self::Done => return None,
            }
        }
    }
}

/// Implements the [`Deserializer`] trait for the [`Mixed`] types.
#[cfg(feature = "quick-xml")]
pub enum MixedDeserializer<T>
where
    T: WithDeserializer,
{
    /// Initialize the deserialization.
    Init,

    /// Deserialize the inner value.
    Inner {
        /// Current deserializer of the inner value.
        deserializer: T::Deserializer,
    },

    /// Deserialize the text after the inner value.
    TextAfter {
        /// Already deserialized inner value.
        value: T,

        /// Wether any elements are allowed or not.
        allow_any: bool,

        /// Current deserializer of the text value.
        deserializer: Option<TextDeserializer>,
    },
}

#[cfg(feature = "quick-xml")]
impl<T> Debug for MixedDeserializer<T>
where
    T: WithDeserializer,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Init => write!(f, "Init"),
            Self::Inner { deserializer } => f
                .debug_struct("Inner")
                .field("deserializer", deserializer)
                .finish(),
            Self::TextAfter { deserializer, .. } => f
                .debug_struct("TextAfter")
                .field("deserializer", deserializer)
                .finish_non_exhaustive(),
        }
    }
}

#[cfg(feature = "quick-xml")]
impl<'de, T> Deserializer<'de, Mixed<T>> for MixedDeserializer<T>
where
    T: WithDeserializer,
{
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, Mixed<T>>
    where
        R: XmlReader,
    {
        Self::Init.next(reader, event)
    }

    #[allow(clippy::too_many_lines)]
    fn next<R>(mut self, reader: &R, mut event: Event<'de>) -> DeserializerResult<'de, Mixed<T>>
    where
        R: XmlReader,
    {
        loop {
            let (next_event, next_state) = match self {
                x @ (Self::Init | Self::Inner { .. }) => {
                    let output = match x {
                        Self::Init => T::Deserializer::init(reader, event),
                        Self::Inner { deserializer } => deserializer.next(reader, event),
                        Self::TextAfter { .. } => unreachable!(),
                    }?;

                    let DeserializerOutput {
                        event,
                        artifact,
                        allow_any,
                    } = output;

                    match artifact {
                        DeserializerArtifact::None => {
                            return Ok(DeserializerOutput {
                                event,
                                artifact: DeserializerArtifact::None,
                                allow_any,
                            })
                        }
                        DeserializerArtifact::Deserializer(deserializer) => {
                            return Ok(DeserializerOutput {
                                event,
                                artifact: DeserializerArtifact::Deserializer(Self::Inner {
                                    deserializer,
                                }),
                                allow_any,
                            })
                        }
                        DeserializerArtifact::Data(value) => match event {
                            DeserializerEvent::Continue(event) => (
                                event,
                                Self::TextAfter {
                                    value,
                                    allow_any,
                                    deserializer: None,
                                },
                            ),
                            event => {
                                return Ok(DeserializerOutput {
                                    event,
                                    artifact: DeserializerArtifact::Deserializer(Self::TextAfter {
                                        value,
                                        allow_any,
                                        deserializer: None,
                                    }),
                                    allow_any,
                                })
                            }
                        },
                    }
                }
                Self::TextAfter {
                    value,
                    allow_any,
                    deserializer,
                } => {
                    let output = if let Some(deserializer) = deserializer {
                        deserializer.next(reader, event)
                    } else {
                        TextDeserializer::init(reader, event)
                    }?;

                    let DeserializerOutput {
                        event,
                        artifact,
                        allow_any: _,
                    } = output;

                    match artifact {
                        DeserializerArtifact::None => {
                            return Ok(DeserializerOutput {
                                event,
                                artifact: DeserializerArtifact::Deserializer(Self::TextAfter {
                                    value,
                                    allow_any,
                                    deserializer: None,
                                }),
                                allow_any,
                            })
                        }
                        DeserializerArtifact::Data(text_after) => {
                            return Ok(DeserializerOutput {
                                event,
                                artifact: DeserializerArtifact::Data(Mixed {
                                    value,
                                    text_after: Some(text_after),
                                }),
                                allow_any,
                            })
                        }
                        DeserializerArtifact::Deserializer(deserializer) => {
                            return Ok(DeserializerOutput {
                                artifact: DeserializerArtifact::Deserializer(Self::TextAfter {
                                    value,
                                    allow_any,
                                    deserializer: Some(deserializer),
                                }),
                                event,
                                allow_any,
                            })
                        }
                    }
                }
            };

            self = next_state;
            event = next_event;
        }
    }

    fn finish<R>(self, reader: &R) -> Result<Mixed<T>, Error>
    where
        R: XmlReader,
    {
        match self {
            Self::Init => unreachable!(),
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
                deserializer,
            } => {
                let text_after = deserializer.map(|x| x.finish(reader)).transpose()?;

                Ok(Mixed { value, text_after })
            }
        }
    }
}
