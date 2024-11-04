pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub name: StringType,
}
impl xsd_parser::quick_xml::WithSerializer for FooType {
    type Serializer<'x> = quick_xml_serialize::FooTypeSerializer<'x>;
}
impl xsd_parser::quick_xml::WithDeserializer for FooType {
    type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
}
pub type StringType = String;
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
        Name(xsd_parser::quick_xml::ContentSerializer<'ser, StringType>),
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
                        match Serializer::init(&self.value.name, Some("tns:Name"), false) {
                            Ok(serializer) => self.state = FooTypeSerializerState::Name(serializer),
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
                    FooTypeSerializerState::Name(x) => match x.next() {
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
        name: Option<super::StringType>,
        state: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Name(Option<<StringType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
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
                name: None,
                state: Box::new(FooTypeDeserializerState::Name(None)),
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
                    (FooTypeDeserializerState::Name(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.name.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Name")))?;
                            }
                            self.name = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = FooTypeDeserializerState::Name(deserializer);
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
                                .get_or_insert(FooTypeDeserializerState::Name(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.name.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Name")))?;
                            }
                            self.name = Some(data);
                        }
                        *self.state = FooTypeDeserializerState::Name(None);
                        event
                    }
                    (FooTypeDeserializerState::Name(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_TNS),
                                Some(b"Name")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <StringType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.name.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"Name",
                                    )))?;
                                }
                                self.name = Some(data);
                            }
                            *self.state = FooTypeDeserializerState::Name(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = FooTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback
                                            .get_or_insert(FooTypeDeserializerState::Name(None));
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
                            allow_any_fallback.get_or_insert(FooTypeDeserializerState::Name(None));
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
                            *self.state = FooTypeDeserializerState::Name(None);
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
                name: self
                    .name
                    .ok_or_else(|| ErrorKind::MissingElement("Name".into()))?,
            })
        }
    }
}
