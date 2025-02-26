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
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    FooTypeSerializerState::Init__ => {
                        self.state = FooTypeSerializerState::Once(
                            xsd_parser::quick_xml::ContentSerializer::new(
                                &self.value.once,
                                Some("Once"),
                                false,
                            ),
                        );
                        let bytes = BytesStart::new(self.name);
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
                                    Some("Optional"),
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
                                    Some("OnceSpecify"),
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
                                    Some("TwiceOrMore"),
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
        Once(Option<<i32 as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Optional(Option<<i32 as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        OnceSpecify(Option<<i32 as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        TwiceOrMore(Option<<i32 as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
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
                once: None,
                optional: None,
                once_specify: None,
                twice_or_more: Vec::new(),
                state: Box::new(FooTypeDeserializerState::Once(None)),
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
            const NS_DEFAULT: &[u8] = b"http://example.com";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, FooTypeDeserializerState::Done__),
                    event,
                ) {
                    (FooTypeDeserializerState::Once(Some(deserializer)), event) => {
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
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = FooTypeDeserializerState::Once(deserializer);
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
                                .get_or_insert(FooTypeDeserializerState::Once(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.once.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Once")))?;
                            }
                            self.once = Some(data);
                        }
                        *self.state = FooTypeDeserializerState::Once(None);
                        event
                    }
                    (FooTypeDeserializerState::Once(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DEFAULT),
                                Some(b"Once")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.once.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"Once",
                                    )))?;
                                }
                                self.once = Some(data);
                            }
                            *self.state = FooTypeDeserializerState::Once(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = FooTypeDeserializerState::Optional(None);
                                    if allow_any {
                                        allow_any_fallback
                                            .get_or_insert(FooTypeDeserializerState::Once(None));
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
                            *self.state = FooTypeDeserializerState::Optional(None);
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
                            *self.state = FooTypeDeserializerState::Once(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (FooTypeDeserializerState::Optional(Some(deserializer)), event) => {
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
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = FooTypeDeserializerState::Optional(deserializer);
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
                                .get_or_insert(FooTypeDeserializerState::Optional(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.optional.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Optional",
                                )))?;
                            }
                            self.optional = Some(data);
                        }
                        *self.state = FooTypeDeserializerState::Optional(None);
                        event
                    }
                    (FooTypeDeserializerState::Optional(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DEFAULT),
                                Some(b"Optional")
                            ) =>
                        {
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
                            *self.state = FooTypeDeserializerState::Optional(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = FooTypeDeserializerState::OnceSpecify(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            FooTypeDeserializerState::Optional(None),
                                        );
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
                            *self.state = FooTypeDeserializerState::OnceSpecify(None);
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
                            *self.state = FooTypeDeserializerState::Optional(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (FooTypeDeserializerState::OnceSpecify(Some(deserializer)), event) => {
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
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = FooTypeDeserializerState::OnceSpecify(deserializer);
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
                                .get_or_insert(FooTypeDeserializerState::OnceSpecify(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.once_specify.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"OnceSpecify",
                                )))?;
                            }
                            self.once_specify = Some(data);
                        }
                        *self.state = FooTypeDeserializerState::OnceSpecify(None);
                        event
                    }
                    (FooTypeDeserializerState::OnceSpecify(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DEFAULT),
                                Some(b"OnceSpecify")
                            ) =>
                        {
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
                            *self.state = FooTypeDeserializerState::OnceSpecify(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = FooTypeDeserializerState::TwiceOrMore(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            FooTypeDeserializerState::OnceSpecify(None),
                                        );
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
                            *self.state = FooTypeDeserializerState::TwiceOrMore(None);
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
                            *self.state = FooTypeDeserializerState::OnceSpecify(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (FooTypeDeserializerState::TwiceOrMore(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            self.twice_or_more.push(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = FooTypeDeserializerState::TwiceOrMore(deserializer);
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
                                .get_or_insert(FooTypeDeserializerState::TwiceOrMore(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            self.twice_or_more.push(data);
                        }
                        *self.state = FooTypeDeserializerState::TwiceOrMore(None);
                        event
                    }
                    (FooTypeDeserializerState::TwiceOrMore(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DEFAULT),
                                Some(b"TwiceOrMore")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                self.twice_or_more.push(data);
                            }
                            *self.state = FooTypeDeserializerState::TwiceOrMore(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = FooTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            FooTypeDeserializerState::TwiceOrMore(None),
                                        );
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
                            *self.state = FooTypeDeserializerState::TwiceOrMore(None);
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
