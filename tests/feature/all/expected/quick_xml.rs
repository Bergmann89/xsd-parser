pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub once: i32,
    pub optional: Option<i32>,
    pub once_specify: i32,
    pub twice_or_more: Vec<i32>,
}
impl xsd_parser::quick_xml::WithSerializer for FooType {
    type Serializer<'x> = quick_xml_serialize::FooTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::FooTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for FooType {
    type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
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
        Once(xsd_parser::quick_xml::ContentSerializer<'ser, i32>),
        Optional(xsd_parser::quick_xml::IterSerializer<'ser, Option<i32>, i32>),
        OnceSpecify(xsd_parser::quick_xml::ContentSerializer<'ser, i32>),
        TwiceOrMore(xsd_parser::quick_xml::IterSerializer<'ser, Vec<i32>, i32>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::FooType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("tns:FooType");
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
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    FooTypeSerializerState::Init__ => {
                        self.state = FooTypeSerializerState::Once(
                            xsd_parser::quick_xml::ContentSerializer::new(
                                &self.value.once,
                                Some("tns:Once"),
                                false,
                            ),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                        }
                        return Some(Ok(Event::Start(bytes)));
                    }
                    FooTypeSerializerState::Once(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = FooTypeSerializerState::Optional(
                                xsd_parser::quick_xml::IterSerializer::new(
                                    &self.value.optional,
                                    Some("tns:Optional"),
                                    false,
                                ),
                            )
                        }
                    },
                    FooTypeSerializerState::Optional(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = FooTypeSerializerState::OnceSpecify(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    &self.value.once_specify,
                                    Some("tns:OnceSpecify"),
                                    false,
                                ),
                            )
                        }
                    },
                    FooTypeSerializerState::OnceSpecify(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = FooTypeSerializerState::TwiceOrMore(
                                xsd_parser::quick_xml::IterSerializer::new(
                                    &self.value.twice_or_more,
                                    Some("tns:TwiceOrMore"),
                                    false,
                                ),
                            )
                        }
                    },
                    FooTypeSerializerState::TwiceOrMore(x) => match x.next() {
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
        once: Option<i32>,
        optional: Option<i32>,
        once_specify: Option<i32>,
        twice_or_more: Vec<i32>,
        state: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Next__,
        Once(<i32 as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Optional(<i32 as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        OnceSpecify(<i32 as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        TwiceOrMore(<i32 as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
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
                once: None,
                optional: None,
                once_specify: None,
                twice_or_more: Vec::new(),
                state: Box::new(FooTypeDeserializerState::Next__),
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
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_TNS: &[u8] = b"http://example.com";
            match (
                core::mem::replace(&mut *self.state, FooTypeDeserializerState::Next__),
                &event,
            ) {
                (FooTypeDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(reader.resolve_local_name(x.name(), NS_TNS), Some(b"Once")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.once.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Once")))?;
                            }
                            self.once = Some(data);
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FooTypeDeserializerState::Once(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_TNS),
                        Some(b"Optional")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.optional.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Optional",
                                )))?;
                            }
                            self.optional = Some(data);
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FooTypeDeserializerState::Optional(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_TNS),
                        Some(b"OnceSpecify")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.once_specify.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"OnceSpecify",
                                )))?;
                            }
                            self.once_specify = Some(data);
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FooTypeDeserializerState::OnceSpecify(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_TNS),
                        Some(b"TwiceOrMore")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.twice_or_more.push(data);
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FooTypeDeserializerState::TwiceOrMore(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else {
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any: false,
                        })
                    }
                }
                (FooTypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (FooTypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (FooTypeDeserializerState::Once(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.once.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Once")))?;
                        }
                        self.once = Some(data);
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FooTypeDeserializerState::Once(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FooTypeDeserializerState::Optional(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.optional.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"Optional",
                            )))?;
                        }
                        self.optional = Some(data);
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FooTypeDeserializerState::Optional(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FooTypeDeserializerState::OnceSpecify(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.once_specify.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"OnceSpecify",
                            )))?;
                        }
                        self.once_specify = Some(data);
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FooTypeDeserializerState::OnceSpecify(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FooTypeDeserializerState::TwiceOrMore(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.twice_or_more.push(data);
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FooTypeDeserializerState::TwiceOrMore(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
        fn finish<R>(self, _reader: &R) -> Result<super::FooType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::FooType {
                once: self
                    .once
                    .ok_or_else(|| ErrorKind::MissingElement("Once".into()))?,
                optional: self.optional,
                once_specify: self
                    .once_specify
                    .ok_or_else(|| ErrorKind::MissingElement("OnceSpecify".into()))?,
                twice_or_more: self.twice_or_more,
            })
        }
    }
}
