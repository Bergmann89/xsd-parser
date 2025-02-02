pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub content_3: FooContent3Type,
    pub content_6: FooContent6Type,
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
#[derive(Debug, Clone)]
pub enum FooContent3Type {
    Element1(i32),
    Element2(String),
}
impl xsd_parser::quick_xml::WithSerializer for FooContent3Type {
    type Serializer<'x> = quick_xml_serialize::FooContent3TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::FooContent3TypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for FooContent3Type {
    type Deserializer = quick_xml_deserialize::FooContent3TypeDeserializer;
}
#[derive(Debug, Clone)]
pub enum FooContent6Type {
    Element3(i32),
    Element4(String),
}
impl xsd_parser::quick_xml::WithSerializer for FooContent6Type {
    type Serializer<'x> = quick_xml_serialize::FooContent6TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::FooContent6TypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for FooContent6Type {
    type Deserializer = quick_xml_deserialize::FooContent6TypeDeserializer;
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
        Content3(<FooContent3Type as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        Content6(<FooContent6Type as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
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
                        match WithSerializer::serializer(
                            &self.value.content_3,
                            Some("tns:Content3"),
                            false,
                        ) {
                            Ok(serializer) => {
                                self.state = FooTypeSerializerState::Content3(serializer)
                            }
                            Err(error) => {
                                self.state = FooTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        };
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                        }
                        return Some(Ok(Event::Start(bytes)));
                    }
                    FooTypeSerializerState::Content3(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => match WithSerializer::serializer(
                            &self.value.content_6,
                            Some("tns:Content6"),
                            false,
                        ) {
                            Ok(serializer) => {
                                self.state = FooTypeSerializerState::Content6(serializer)
                            }
                            Err(error) => {
                                self.state = FooTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        },
                    },
                    FooTypeSerializerState::Content6(x) => match x.next() {
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
    pub struct FooContent3TypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::FooContent3Type,
        is_root: bool,
        state: FooContent3TypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum FooContent3TypeSerializerState<'ser> {
        Init__,
        Element1(xsd_parser::quick_xml::ContentSerializer<'ser, i32>),
        Element2(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooContent3TypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::FooContent3Type,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("tns:FooContent3");
            Ok(Self {
                name,
                value,
                is_root,
                state: FooContent3TypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for FooContent3TypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    FooContent3TypeSerializerState::Init__ => match &self.value {
                        FooContent3Type::Element1(x) => {
                            self.state = FooContent3TypeSerializerState::Element1(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    x,
                                    Some("tns:Element1"),
                                    false,
                                ),
                            )
                        }
                        FooContent3Type::Element2(x) => {
                            self.state = FooContent3TypeSerializerState::Element2(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    x,
                                    Some("tns:Element2"),
                                    false,
                                ),
                            )
                        }
                    },
                    FooContent3TypeSerializerState::Element1(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooContent3TypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = FooContent3TypeSerializerState::Done__,
                    },
                    FooContent3TypeSerializerState::Element2(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooContent3TypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = FooContent3TypeSerializerState::Done__,
                    },
                    FooContent3TypeSerializerState::Done__ => return None,
                    FooContent3TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct FooContent6TypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::FooContent6Type,
        is_root: bool,
        state: FooContent6TypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum FooContent6TypeSerializerState<'ser> {
        Init__,
        Element3(xsd_parser::quick_xml::ContentSerializer<'ser, i32>),
        Element4(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooContent6TypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::FooContent6Type,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("tns:FooContent6");
            Ok(Self {
                name,
                value,
                is_root,
                state: FooContent6TypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for FooContent6TypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    FooContent6TypeSerializerState::Init__ => match &self.value {
                        FooContent6Type::Element3(x) => {
                            self.state = FooContent6TypeSerializerState::Element3(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    x,
                                    Some("tns:Element3"),
                                    false,
                                ),
                            )
                        }
                        FooContent6Type::Element4(x) => {
                            self.state = FooContent6TypeSerializerState::Element4(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    x,
                                    Some("tns:Element4"),
                                    false,
                                ),
                            )
                        }
                    },
                    FooContent6TypeSerializerState::Element3(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooContent6TypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = FooContent6TypeSerializerState::Done__,
                    },
                    FooContent6TypeSerializerState::Element4(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooContent6TypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = FooContent6TypeSerializerState::Done__,
                    },
                    FooContent6TypeSerializerState::Done__ => return None,
                    FooContent6TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
}
pub mod quick_xml_deserialize {
    use super::*;
    #[derive(Debug)]
    pub struct FooTypeDeserializer {
        content_3: Option<super::FooContent3Type>,
        content_6: Option<super::FooContent6Type>,
        state: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Content3(
            Option<<FooContent3Type as xsd_parser::quick_xml::WithDeserializer>::Deserializer>,
        ),
        Content6(
            Option<<FooContent6Type as xsd_parser::quick_xml::WithDeserializer>::Deserializer>,
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
                content_3: None,
                content_6: None,
                state: Box::new(FooTypeDeserializerState::Content3(None)),
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
                    (FooTypeDeserializerState::Content3(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.content_3.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Content3",
                                )))?;
                            }
                            self.content_3 = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = FooTypeDeserializerState::Content3(deserializer);
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
                                .get_or_insert(FooTypeDeserializerState::Content3(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.content_3.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Content3",
                                )))?;
                            }
                            self.content_3 = Some(data);
                        }
                        *self.state = FooTypeDeserializerState::Content3(None);
                        event
                    }
                    (FooTypeDeserializerState::Content3(None), event) => match &event {
                        Event::Start(_) | Event::Empty(_) => {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <FooContent3Type as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.content_3.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"Content3",
                                    )))?;
                                }
                                self.content_3 = Some(data);
                            }
                            *self.state = FooTypeDeserializerState::Content3(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = FooTypeDeserializerState::Content6(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            FooTypeDeserializerState::Content3(None),
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
                            *self.state = FooTypeDeserializerState::Content3(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (FooTypeDeserializerState::Content6(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.content_6.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Content6",
                                )))?;
                            }
                            self.content_6 = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = FooTypeDeserializerState::Content6(deserializer);
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
                                .get_or_insert(FooTypeDeserializerState::Content6(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.content_6.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Content6",
                                )))?;
                            }
                            self.content_6 = Some(data);
                        }
                        *self.state = FooTypeDeserializerState::Content6(None);
                        event
                    }
                    (FooTypeDeserializerState::Content6(None), event) => match &event {
                        Event::Start(_) | Event::Empty(_) => {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <FooContent6Type as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.content_6.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"Content6",
                                    )))?;
                                }
                                self.content_6 = Some(data);
                            }
                            *self.state = FooTypeDeserializerState::Content6(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = FooTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            FooTypeDeserializerState::Content6(None),
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
                            *self.state = FooTypeDeserializerState::Content6(None);
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
                content_3: self
                    .content_3
                    .ok_or_else(|| ErrorKind::MissingElement("Content3".into()))?,
                content_6: self
                    .content_6
                    .ok_or_else(|| ErrorKind::MissingElement("Content6".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct FooContent3TypeDeserializer {
        content: Option<super::FooContent3Type>,
        state: Box<FooContent3TypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooContent3TypeDeserializerState {
        Next__,
        Element1(<i32 as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Element2(<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl FooContent3TypeDeserializer {}
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::FooContent3Type>
        for FooContent3TypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FooContent3Type, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            let deserializer = Self {
                content: None,
                state: Box::new(FooContent3TypeDeserializerState::Next__),
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FooContent3Type, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_TNS: &[u8] = b"http://example.com";
            match (
                core::mem::replace(&mut *self.state, FooContent3TypeDeserializerState::Next__),
                &event,
            ) {
                (FooContent3TypeDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
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
                            self.content = Some(FooContent3Type::Element1(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FooContent3TypeDeserializerState::Element1(deserializer);
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
                            self.content = Some(FooContent3Type::Element2(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FooContent3TypeDeserializerState::Element2(deserializer);
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
                (FooContent3TypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: Some(event),
                        allow_any: false,
                    })
                }
                (FooContent3TypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (FooContent3TypeDeserializerState::Element1(deserializer), _) => {
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
                        self.content = Some(FooContent3Type::Element1(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FooContent3TypeDeserializerState::Element1(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FooContent3TypeDeserializerState::Element2(deserializer), _) => {
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
                        self.content = Some(FooContent3Type::Element2(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FooContent3TypeDeserializerState::Element2(deserializer);
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
        ) -> Result<super::FooContent3Type, xsd_parser::quick_xml::Error>
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
    pub struct FooContent6TypeDeserializer {
        content: Option<super::FooContent6Type>,
        state: Box<FooContent6TypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooContent6TypeDeserializerState {
        Next__,
        Element3(<i32 as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Element4(<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl FooContent6TypeDeserializer {}
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::FooContent6Type>
        for FooContent6TypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FooContent6Type, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            let deserializer = Self {
                content: None,
                state: Box::new(FooContent6TypeDeserializerState::Next__),
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FooContent6Type, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_TNS: &[u8] = b"http://example.com";
            match (
                core::mem::replace(&mut *self.state, FooContent6TypeDeserializerState::Next__),
                &event,
            ) {
                (FooContent6TypeDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
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
                            self.content = Some(FooContent6Type::Element3(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FooContent6TypeDeserializerState::Element3(deserializer);
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
                            self.content = Some(FooContent6Type::Element4(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FooContent6TypeDeserializerState::Element4(deserializer);
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
                (FooContent6TypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: Some(event),
                        allow_any: false,
                    })
                }
                (FooContent6TypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (FooContent6TypeDeserializerState::Element3(deserializer), _) => {
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
                        self.content = Some(FooContent6Type::Element3(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FooContent6TypeDeserializerState::Element3(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FooContent6TypeDeserializerState::Element4(deserializer), _) => {
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
                        self.content = Some(FooContent6Type::Element4(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FooContent6TypeDeserializerState::Element4(deserializer);
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
        ) -> Result<super::FooContent6Type, xsd_parser::quick_xml::Error>
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
