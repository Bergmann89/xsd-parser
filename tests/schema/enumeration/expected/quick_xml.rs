pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub enum_: EnumType,
}
impl xsd_parser::quick_xml::WithSerializer for FooType {
    type Serializer<'x> = quick_xml_serialize::FooTypeSerializer<'x>;
}
impl xsd_parser::quick_xml::WithDeserializer for FooType {
    type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
}
#[derive(Debug, Clone)]
pub enum EnumType {
    Off,
    On,
    Auto,
}
impl xsd_parser::quick_xml::SerializeBytes for EnumType {
    fn serialize_bytes(
        &self,
    ) -> Result<Option<std::borrow::Cow<'_, str>>, xsd_parser::quick_xml::Error> {
        match self {
            Self::Off => Ok(Some(std::borrow::Cow::Borrowed("OFF"))),
            Self::On => Ok(Some(std::borrow::Cow::Borrowed("ON"))),
            Self::Auto => Ok(Some(std::borrow::Cow::Borrowed("AUTO"))),
        }
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for EnumType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        match bytes {
            b"OFF" => Ok(Self::Off),
            b"ON" => Ok(Self::On),
            b"AUTO" => Ok(Self::Auto),
            x => {
                use xsd_parser::quick_xml::{ErrorKind, RawByteStr};
                Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x))))
            }
        }
    }
}
pub mod quick_xml_serialize {
    use super::*;
    #[derive(Debug)]
    pub struct FooTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::FooType,
        is_root: bool,
        state: FooTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum FooTypeSerializerState<'ser> {
        Init__,
        Enum(xsd_parser::quick_xml::ContentSerializer<'ser, EnumType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> xsd_parser::quick_xml::Serializer<'ser, super::FooType> for FooTypeSerializer<'ser> {
        fn init(
            value: &'ser super::FooType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("FooType");
            Ok(Self {
                name,
                value,
                is_root,
                state: FooTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for FooTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{BytesEnd, BytesStart, Error, Event, Serializer};
            loop {
                match &mut self.state {
                    FooTypeSerializerState::Init__ => {
                        match Serializer::init(&self.value.enum_, Some("tns:Enum"), false) {
                            Ok(serializer) => self.state = FooTypeSerializerState::Enum(serializer),
                            Err(error) => {
                                self.state = FooTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                        }
                        return Some(Ok(Event::Start(bytes)));
                    }
                    FooTypeSerializerState::Enum(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = FooTypeSerializerState::End__,
                    },
                    FooTypeSerializerState::End__ => {
                        self.state = FooTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    FooTypeSerializerState::Done__ => return None,
                    FooTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
}
pub mod quick_xml_deserialize {
    use super::*;
    #[derive(Debug)]
    pub struct FooTypeDeserializer {
        enum_: Option<super::EnumType>,
        state: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Enum(Option<<EnumType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl FooTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                reader.err(ErrorKind::UnexpectedAttribute(
                    xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                ))?;
            }
            Ok(Self {
                enum_: None,
                state: Box::new(FooTypeDeserializerState::Enum(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::FooType> for FooTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FooType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FooType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_TNS: &[u8] = b"http://example.com";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, FooTypeDeserializerState::Done__),
                    event,
                ) {
                    (FooTypeDeserializerState::Enum(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.enum_.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Enum")))?;
                            }
                            self.enum_ = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = FooTypeDeserializerState::Enum(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback
                                .get_or_insert(FooTypeDeserializerState::Enum(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.enum_.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Enum")))?;
                            }
                            self.enum_ = Some(data);
                        }
                        *self.state = FooTypeDeserializerState::Enum(None);
                        event
                    }
                    (FooTypeDeserializerState::Enum(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_TNS),
                                Some(b"Enum")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <EnumType as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.enum_.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"Enum",
                                    )))?;
                                }
                                self.enum_ = Some(data);
                            }
                            *self.state = FooTypeDeserializerState::Enum(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = FooTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback
                                            .get_or_insert(FooTypeDeserializerState::Enum(None));
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = FooTypeDeserializerState::Done__;
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = FooTypeDeserializerState::Enum(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (FooTypeDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(self, _reader: &R) -> Result<super::FooType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::FooType {
                enum_: self
                    .enum_
                    .ok_or_else(|| ErrorKind::MissingElement("Enum".into()))?,
            })
        }
    }
}
