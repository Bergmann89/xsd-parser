pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub messages: FooTypeMessages,
}
impl xsd_parser::quick_xml::WithSerializer for FooType {
    type Serializer<'x> = quick_xml_serialize::FooTypeSerializer<'x>;
}
impl xsd_parser::quick_xml::WithDeserializer for FooType {
    type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct FooTypeMessages {
    pub aa: IntType,
    pub bb: StringType,
    pub a: StringType,
}
impl xsd_parser::quick_xml::WithSerializer for FooTypeMessages {
    type Serializer<'x> = quick_xml_serialize::FooTypeMessagesSerializer<'x>;
}
impl xsd_parser::quick_xml::WithDeserializer for FooTypeMessages {
    type Deserializer = quick_xml_deserialize::FooTypeMessagesDeserializer;
}
pub type IntType = i32;
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
        Messages(<FooTypeMessages as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
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
                        match Serializer::init(&self.value.messages, Some("tns:Messages"), false) {
                            Ok(serializer) => {
                                self.state = FooTypeSerializerState::Messages(serializer)
                            }
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
                    FooTypeSerializerState::Messages(x) => match x.next() {
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
    #[derive(Debug)]
    pub struct FooTypeMessagesSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::FooTypeMessages,
        is_root: bool,
        state: FooTypeMessagesSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum FooTypeMessagesSerializerState<'ser> {
        Init__,
        Aa(xsd_parser::quick_xml::ContentSerializer<'ser, IntType>),
        Bb(xsd_parser::quick_xml::ContentSerializer<'ser, StringType>),
        A(xsd_parser::quick_xml::ContentSerializer<'ser, StringType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> xsd_parser::quick_xml::Serializer<'ser, super::FooTypeMessages>
        for FooTypeMessagesSerializer<'ser>
    {
        fn init(
            value: &'ser super::FooTypeMessages,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("FooTypeMessages");
            Ok(Self {
                name,
                value,
                is_root,
                state: FooTypeMessagesSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for FooTypeMessagesSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{BytesEnd, BytesStart, Error, Event, Serializer};
            loop {
                match &mut self.state {
                    FooTypeMessagesSerializerState::Init__ => {
                        match Serializer::init(&self.value.aa, Some("tns:aa"), false) {
                            Ok(serializer) => {
                                self.state = FooTypeMessagesSerializerState::Aa(serializer)
                            }
                            Err(error) => {
                                self.state = FooTypeMessagesSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                        }
                        return Some(Ok(Event::Start(bytes)));
                    }
                    FooTypeMessagesSerializerState::Aa(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooTypeMessagesSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => match Serializer::init(&self.value.bb, Some("tns:bb"), false) {
                            Ok(serializer) => {
                                self.state = FooTypeMessagesSerializerState::Bb(serializer)
                            }
                            Err(error) => {
                                self.state = FooTypeMessagesSerializerState::Done__;
                                return Some(Err(error));
                            }
                        },
                    },
                    FooTypeMessagesSerializerState::Bb(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooTypeMessagesSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => match Serializer::init(&self.value.a, Some("tns:a"), false) {
                            Ok(serializer) => {
                                self.state = FooTypeMessagesSerializerState::A(serializer)
                            }
                            Err(error) => {
                                self.state = FooTypeMessagesSerializerState::Done__;
                                return Some(Err(error));
                            }
                        },
                    },
                    FooTypeMessagesSerializerState::A(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooTypeMessagesSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = FooTypeMessagesSerializerState::End__,
                    },
                    FooTypeMessagesSerializerState::End__ => {
                        self.state = FooTypeMessagesSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    FooTypeMessagesSerializerState::Done__ => return None,
                    FooTypeMessagesSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
}
pub mod quick_xml_deserialize {
    use super::*;
    #[derive(Debug)]
    pub struct FooTypeDeserializer {
        messages: Option<super::FooTypeMessages>,
        state: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Messages(
            Option<<FooTypeMessages as xsd_parser::quick_xml::WithDeserializer>::Deserializer>,
        ),
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
                messages: None,
                state: Box::new(FooTypeDeserializerState::Messages(None)),
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
                    (FooTypeDeserializerState::Messages(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.messages.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Messages",
                                )))?;
                            }
                            self.messages = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = FooTypeDeserializerState::Messages(deserializer);
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
                                .get_or_insert(FooTypeDeserializerState::Messages(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.messages.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Messages",
                                )))?;
                            }
                            self.messages = Some(data);
                        }
                        *self.state = FooTypeDeserializerState::Messages(None);
                        event
                    }
                    (FooTypeDeserializerState::Messages(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_TNS),
                                Some(b"Messages")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <FooTypeMessages as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.messages.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"Messages",
                                    )))?;
                                }
                                self.messages = Some(data);
                            }
                            *self.state = FooTypeDeserializerState::Messages(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = FooTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            FooTypeDeserializerState::Messages(None),
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
                            *self.state = FooTypeDeserializerState::Messages(None);
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
                messages: self
                    .messages
                    .ok_or_else(|| ErrorKind::MissingElement("Messages".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct FooTypeMessagesDeserializer {
        aa: Option<super::IntType>,
        bb: Option<super::StringType>,
        a: Option<super::StringType>,
        state: Box<FooTypeMessagesDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeMessagesDeserializerState {
        Aa(Option<<IntType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Bb(Option<<StringType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        A(Option<<StringType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl FooTypeMessagesDeserializer {
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
                aa: None,
                bb: None,
                a: None,
                state: Box::new(FooTypeMessagesDeserializerState::Aa(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::FooTypeMessages>
        for FooTypeMessagesDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FooTypeMessages, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FooTypeMessages, Self>
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
                    core::mem::replace(&mut *self.state, FooTypeMessagesDeserializerState::Done__),
                    event,
                ) {
                    (FooTypeMessagesDeserializerState::Aa(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.aa.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"aa")))?;
                            }
                            self.aa = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = FooTypeMessagesDeserializerState::Aa(deserializer);
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
                                .get_or_insert(FooTypeMessagesDeserializerState::Aa(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.aa.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"aa")))?;
                            }
                            self.aa = Some(data);
                        }
                        *self.state = FooTypeMessagesDeserializerState::Aa(None);
                        event
                    }
                    (FooTypeMessagesDeserializerState::Aa(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_TNS),
                                Some(b"aa")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <IntType as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.aa.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"aa",
                                    )))?;
                                }
                                self.aa = Some(data);
                            }
                            *self.state = FooTypeMessagesDeserializerState::Aa(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = FooTypeMessagesDeserializerState::Bb(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            FooTypeMessagesDeserializerState::Aa(None),
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
                            *self.state = FooTypeMessagesDeserializerState::Bb(None);
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
                            *self.state = FooTypeMessagesDeserializerState::Aa(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (FooTypeMessagesDeserializerState::Bb(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.bb.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"bb")))?;
                            }
                            self.bb = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = FooTypeMessagesDeserializerState::Bb(deserializer);
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
                                .get_or_insert(FooTypeMessagesDeserializerState::Bb(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.bb.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"bb")))?;
                            }
                            self.bb = Some(data);
                        }
                        *self.state = FooTypeMessagesDeserializerState::Bb(None);
                        event
                    }
                    (FooTypeMessagesDeserializerState::Bb(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_TNS),
                                Some(b"bb")
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
                                if self.bb.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"bb",
                                    )))?;
                                }
                                self.bb = Some(data);
                            }
                            *self.state = FooTypeMessagesDeserializerState::Bb(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = FooTypeMessagesDeserializerState::A(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            FooTypeMessagesDeserializerState::Bb(None),
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
                            *self.state = FooTypeMessagesDeserializerState::A(None);
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
                            *self.state = FooTypeMessagesDeserializerState::Bb(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (FooTypeMessagesDeserializerState::A(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.a.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"a")))?;
                            }
                            self.a = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = FooTypeMessagesDeserializerState::A(deserializer);
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
                                .get_or_insert(FooTypeMessagesDeserializerState::A(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.a.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"a")))?;
                            }
                            self.a = Some(data);
                        }
                        *self.state = FooTypeMessagesDeserializerState::A(None);
                        event
                    }
                    (FooTypeMessagesDeserializerState::A(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_TNS),
                                Some(b"a")
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
                                if self.a.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"a")))?;
                                }
                                self.a = Some(data);
                            }
                            *self.state = FooTypeMessagesDeserializerState::A(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = FooTypeMessagesDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            FooTypeMessagesDeserializerState::A(None),
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
                            *self.state = FooTypeMessagesDeserializerState::Done__;
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
                            *self.state = FooTypeMessagesDeserializerState::A(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (FooTypeMessagesDeserializerState::Done__, event) => {
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
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::FooTypeMessages, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::FooTypeMessages {
                aa: self
                    .aa
                    .ok_or_else(|| ErrorKind::MissingElement("aa".into()))?,
                bb: self
                    .bb
                    .ok_or_else(|| ErrorKind::MissingElement("bb".into()))?,
                a: self
                    .a
                    .ok_or_else(|| ErrorKind::MissingElement("a".into()))?,
            })
        }
    }
}
