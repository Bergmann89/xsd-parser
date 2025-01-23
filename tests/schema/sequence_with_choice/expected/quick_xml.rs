pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub content_2: FooContent2Type,
    pub content_5: FooContent5Type,
}
impl xsd_parser::quick_xml::WithSerializer for FooType {
    type Serializer<'x> = quick_xml_serialize::FooTypeSerializer<'x>;
}
impl xsd_parser::quick_xml::WithDeserializer for FooType {
    type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
}
#[derive(Debug, Clone)]
pub enum FooContent2Type {
    Element1(i32),
    Element2(String),
}
impl xsd_parser::quick_xml::WithSerializer for FooContent2Type {
    type Serializer<'x> = quick_xml_serialize::FooContent2TypeSerializer<'x>;
}
impl xsd_parser::quick_xml::WithDeserializer for FooContent2Type {
    type Deserializer = quick_xml_deserialize::FooContent2TypeDeserializer;
}
#[derive(Debug, Clone)]
pub enum FooContent5Type {
    Element3(i32),
    Element4(String),
}
impl xsd_parser::quick_xml::WithSerializer for FooContent5Type {
    type Serializer<'x> = quick_xml_serialize::FooContent5TypeSerializer<'x>;
}
impl xsd_parser::quick_xml::WithDeserializer for FooContent5Type {
    type Deserializer = quick_xml_deserialize::FooContent5TypeDeserializer;
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
        Content2(<FooContent2Type as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        Content5(<FooContent5Type as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
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
                        match Serializer::init(&self.value.content_2, Some("tns:Content2"), false) {
                            Ok(serializer) => {
                                self.state = FooTypeSerializerState::Content2(serializer)
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
                    FooTypeSerializerState::Content2(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => match Serializer::init(
                            &self.value.content_5,
                            Some("tns:Content5"),
                            false,
                        ) {
                            Ok(serializer) => {
                                self.state = FooTypeSerializerState::Content5(serializer)
                            }
                            Err(error) => {
                                self.state = FooTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        },
                    },
                    FooTypeSerializerState::Content5(x) => match x.next() {
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
    pub struct FooContent2TypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::FooContent2Type,
        is_root: bool,
        state: FooContent2TypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum FooContent2TypeSerializerState<'ser> {
        Init__,
        Element1(xsd_parser::quick_xml::ContentSerializer<'ser, i32>),
        Element2(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> xsd_parser::quick_xml::Serializer<'ser, super::FooContent2Type>
        for FooContent2TypeSerializer<'ser>
    {
        fn init(
            value: &'ser super::FooContent2Type,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("FooContent2");
            Ok(Self {
                name,
                value,
                is_root,
                state: FooContent2TypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for FooContent2TypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{BytesEnd, BytesStart, Error, Event, Serializer};
            loop {
                match &mut self.state {
                    FooContent2TypeSerializerState::Init__ => match &self.value {
                        FooContent2Type::Element1(x) => {
                            match Serializer::init(x, Some("tns:Element1"), false) {
                                Ok(serializer) => {
                                    self.state =
                                        FooContent2TypeSerializerState::Element1(serializer)
                                }
                                Err(error) => {
                                    self.state = FooContent2TypeSerializerState::Done__;
                                    return Some(Err(error));
                                }
                            }
                        }
                        FooContent2Type::Element2(x) => {
                            match Serializer::init(x, Some("tns:Element2"), false) {
                                Ok(serializer) => {
                                    self.state =
                                        FooContent2TypeSerializerState::Element2(serializer)
                                }
                                Err(error) => {
                                    self.state = FooContent2TypeSerializerState::Done__;
                                    return Some(Err(error));
                                }
                            }
                        }
                    },
                    FooContent2TypeSerializerState::Element1(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooContent2TypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = FooContent2TypeSerializerState::Done__,
                    },
                    FooContent2TypeSerializerState::Element2(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooContent2TypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = FooContent2TypeSerializerState::Done__,
                    },
                    FooContent2TypeSerializerState::Done__ => return None,
                    FooContent2TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct FooContent5TypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::FooContent5Type,
        is_root: bool,
        state: FooContent5TypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum FooContent5TypeSerializerState<'ser> {
        Init__,
        Element3(xsd_parser::quick_xml::ContentSerializer<'ser, i32>),
        Element4(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> xsd_parser::quick_xml::Serializer<'ser, super::FooContent5Type>
        for FooContent5TypeSerializer<'ser>
    {
        fn init(
            value: &'ser super::FooContent5Type,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("FooContent5");
            Ok(Self {
                name,
                value,
                is_root,
                state: FooContent5TypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for FooContent5TypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{BytesEnd, BytesStart, Error, Event, Serializer};
            loop {
                match &mut self.state {
                    FooContent5TypeSerializerState::Init__ => match &self.value {
                        FooContent5Type::Element3(x) => {
                            match Serializer::init(x, Some("tns:Element3"), false) {
                                Ok(serializer) => {
                                    self.state =
                                        FooContent5TypeSerializerState::Element3(serializer)
                                }
                                Err(error) => {
                                    self.state = FooContent5TypeSerializerState::Done__;
                                    return Some(Err(error));
                                }
                            }
                        }
                        FooContent5Type::Element4(x) => {
                            match Serializer::init(x, Some("tns:Element4"), false) {
                                Ok(serializer) => {
                                    self.state =
                                        FooContent5TypeSerializerState::Element4(serializer)
                                }
                                Err(error) => {
                                    self.state = FooContent5TypeSerializerState::Done__;
                                    return Some(Err(error));
                                }
                            }
                        }
                    },
                    FooContent5TypeSerializerState::Element3(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooContent5TypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = FooContent5TypeSerializerState::Done__,
                    },
                    FooContent5TypeSerializerState::Element4(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooContent5TypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = FooContent5TypeSerializerState::Done__,
                    },
                    FooContent5TypeSerializerState::Done__ => return None,
                    FooContent5TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
}
pub mod quick_xml_deserialize {
    use super::*;
    #[derive(Debug)]
    pub struct FooTypeDeserializer {
        content_2: Option<super::FooContent2Type>,
        content_5: Option<super::FooContent5Type>,
        state: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Content2(
            Option<<FooContent2Type as xsd_parser::quick_xml::WithDeserializer>::Deserializer>,
        ),
        Content5(
            Option<<FooContent5Type as xsd_parser::quick_xml::WithDeserializer>::Deserializer>,
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
                content_2: None,
                content_5: None,
                state: Box::new(FooTypeDeserializerState::Content2(None)),
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
                    (FooTypeDeserializerState::Content2(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.content_2.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Content2",
                                )))?;
                            }
                            self.content_2 = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = FooTypeDeserializerState::Content2(deserializer);
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
                                .get_or_insert(FooTypeDeserializerState::Content2(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.content_2.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Content2",
                                )))?;
                            }
                            self.content_2 = Some(data);
                        }
                        *self.state = FooTypeDeserializerState::Content2(None);
                        event
                    }
                    (FooTypeDeserializerState::Content2(None), event) => match &event {
                        Event::Start(_) | Event::Empty(_) => {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <FooContent2Type as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.content_2.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"Content2",
                                    )))?;
                                }
                                self.content_2 = Some(data);
                            }
                            *self.state = FooTypeDeserializerState::Content2(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = FooTypeDeserializerState::Content5(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            FooTypeDeserializerState::Content2(None),
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
                            *self.state = FooTypeDeserializerState::Content2(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (FooTypeDeserializerState::Content5(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.content_5.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Content5",
                                )))?;
                            }
                            self.content_5 = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = FooTypeDeserializerState::Content5(deserializer);
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
                                .get_or_insert(FooTypeDeserializerState::Content5(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.content_5.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Content5",
                                )))?;
                            }
                            self.content_5 = Some(data);
                        }
                        *self.state = FooTypeDeserializerState::Content5(None);
                        event
                    }
                    (FooTypeDeserializerState::Content5(None), event) => match &event {
                        Event::Start(_) | Event::Empty(_) => {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <FooContent5Type as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.content_5.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"Content5",
                                    )))?;
                                }
                                self.content_5 = Some(data);
                            }
                            *self.state = FooTypeDeserializerState::Content5(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = FooTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            FooTypeDeserializerState::Content5(None),
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
                            *self.state = FooTypeDeserializerState::Content5(None);
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
                content_2: self
                    .content_2
                    .ok_or_else(|| ErrorKind::MissingElement("Content2".into()))?,
                content_5: self
                    .content_5
                    .ok_or_else(|| ErrorKind::MissingElement("Content5".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct FooContent2TypeDeserializer {
        content: Option<super::FooContent2Type>,
        state: Box<FooContent2TypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooContent2TypeDeserializerState {
        Next__,
        Element1(<i32 as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Element2(<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl FooContent2TypeDeserializer {}
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::FooContent2Type>
        for FooContent2TypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FooContent2Type, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            let deserializer = Self {
                content: None,
                state: Box::new(FooContent2TypeDeserializerState::Next__),
            };
            let is_empty = matches!(event, Event::Empty(_));
            let mut out = deserializer.next(reader, event)?;
            if out.event.is_some() {
                out.deserializer = None;
            } else if is_empty && out.data.is_none() {
                if let Some(deserializer) = out.deserializer.take() {
                    out.data = Some(deserializer.finish(reader)?);
                }
            }
            Ok(out)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FooContent2Type, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_TNS: &[u8] = b"http://example.com";
            match (
                core::mem::replace(&mut *self.state, FooContent2TypeDeserializerState::Next__),
                &event,
            ) {
                (FooContent2TypeDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(
                        reader.resolve_local_name(x.name(), NS_TNS),
                        Some(b"Element1")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Element1",
                                )))?;
                            }
                            self.content = Some(FooContent2Type::Element1(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FooContent2TypeDeserializerState::Element1(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_TNS),
                        Some(b"Element2")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Element2",
                                )))?;
                            }
                            self.content = Some(FooContent2Type::Element2(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FooContent2TypeDeserializerState::Element2(deserializer);
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
                (FooContent2TypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: Some(event),
                        allow_any: false,
                    })
                }
                (FooContent2TypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (FooContent2TypeDeserializerState::Element1(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"Element1",
                            )))?;
                        }
                        self.content = Some(FooContent2Type::Element1(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FooContent2TypeDeserializerState::Element1(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FooContent2TypeDeserializerState::Element2(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"Element2",
                            )))?;
                        }
                        self.content = Some(FooContent2Type::Element2(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FooContent2TypeDeserializerState::Element2(deserializer);
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
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::FooContent2Type, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(self
                .content
                .ok_or(xsd_parser::quick_xml::ErrorKind::MissingContent)?)
        }
    }
    #[derive(Debug)]
    pub struct FooContent5TypeDeserializer {
        content: Option<super::FooContent5Type>,
        state: Box<FooContent5TypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooContent5TypeDeserializerState {
        Next__,
        Element3(<i32 as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Element4(<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl FooContent5TypeDeserializer {}
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::FooContent5Type>
        for FooContent5TypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FooContent5Type, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            let deserializer = Self {
                content: None,
                state: Box::new(FooContent5TypeDeserializerState::Next__),
            };
            let is_empty = matches!(event, Event::Empty(_));
            let mut out = deserializer.next(reader, event)?;
            if out.event.is_some() {
                out.deserializer = None;
            } else if is_empty && out.data.is_none() {
                if let Some(deserializer) = out.deserializer.take() {
                    out.data = Some(deserializer.finish(reader)?);
                }
            }
            Ok(out)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FooContent5Type, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_TNS: &[u8] = b"http://example.com";
            match (
                core::mem::replace(&mut *self.state, FooContent5TypeDeserializerState::Next__),
                &event,
            ) {
                (FooContent5TypeDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(
                        reader.resolve_local_name(x.name(), NS_TNS),
                        Some(b"Element3")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Element3",
                                )))?;
                            }
                            self.content = Some(FooContent5Type::Element3(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FooContent5TypeDeserializerState::Element3(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_TNS),
                        Some(b"Element4")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Element4",
                                )))?;
                            }
                            self.content = Some(FooContent5Type::Element4(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FooContent5TypeDeserializerState::Element4(deserializer);
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
                (FooContent5TypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: Some(event),
                        allow_any: false,
                    })
                }
                (FooContent5TypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (FooContent5TypeDeserializerState::Element3(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"Element3",
                            )))?;
                        }
                        self.content = Some(FooContent5Type::Element3(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FooContent5TypeDeserializerState::Element3(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FooContent5TypeDeserializerState::Element4(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"Element4",
                            )))?;
                        }
                        self.content = Some(FooContent5Type::Element4(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FooContent5TypeDeserializerState::Element4(deserializer);
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
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::FooContent5Type, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(self
                .content
                .ok_or(xsd_parser::quick_xml::ErrorKind::MissingContent)?)
        }
    }
}
