pub type FooOption = FooOptionType;
#[derive(Debug, Clone)]
pub struct FooOptionType {
    pub content: Option<FooOptionTypeContent>,
}
#[derive(Debug, Clone)]
pub enum FooOptionTypeContent {
    Bar(String),
    Baz(i32),
}
impl xsd_parser::quick_xml::WithSerializer for FooOptionType {
    type Serializer<'x> = quick_xml_serialize::FooOptionTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::FooOptionTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for FooOptionType {
    type Deserializer = quick_xml_deserialize::FooOptionTypeDeserializer;
}
pub type FooList = FooListType;
#[derive(Debug, Clone)]
pub struct FooListType {
    pub content: Vec<FooListTypeContent>,
}
#[derive(Debug, Clone)]
pub enum FooListTypeContent {
    Bar(String),
    Baz(i32),
}
impl xsd_parser::quick_xml::WithSerializer for FooListType {
    type Serializer<'x> = quick_xml_serialize::FooListTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::FooListTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for FooListType {
    type Deserializer = quick_xml_deserialize::FooListTypeDeserializer;
}
pub mod quick_xml_serialize {
    use super::*;
    #[derive(Debug)]
    pub struct FooOptionTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::FooOptionType,
        is_root: bool,
        state: FooOptionTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum FooOptionTypeSerializerState<'ser> {
        Init__,
        Bar(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        Baz(xsd_parser::quick_xml::ContentSerializer<'ser, i32>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooOptionTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::FooOptionType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("tns:FooOptionType");
            Ok(Self {
                name,
                value,
                is_root,
                state: FooOptionTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for FooOptionTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    FooOptionTypeSerializerState::Init__ => {
                        match &self.value.content {
                            FooOptionTypeContent::Bar(x) => {
                                self.state = FooOptionTypeSerializerState::Bar(
                                    xsd_parser::quick_xml::ContentSerializer::new(
                                        x,
                                        Some("tns:Bar"),
                                        false,
                                    ),
                                )
                            }
                            FooOptionTypeContent::Baz(x) => {
                                self.state = FooOptionTypeSerializerState::Baz(
                                    xsd_parser::quick_xml::ContentSerializer::new(
                                        x,
                                        Some("tns:Baz"),
                                        false,
                                    ),
                                )
                            }
                        }
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                        }
                        return Some(Ok(Event::Start(bytes)));
                    }
                    FooOptionTypeSerializerState::Bar(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooOptionTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = FooOptionTypeSerializerState::End__,
                    },
                    FooOptionTypeSerializerState::Baz(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooOptionTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = FooOptionTypeSerializerState::End__,
                    },
                    FooOptionTypeSerializerState::End__ => {
                        self.state = FooOptionTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    FooOptionTypeSerializerState::Done__ => return None,
                    FooOptionTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct FooListTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::FooListType,
        is_root: bool,
        state: FooListTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum FooListTypeSerializerState<'ser> {
        Init__,
        Bar(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        Baz(xsd_parser::quick_xml::ContentSerializer<'ser, i32>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooListTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::FooListType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("tns:FooListType");
            Ok(Self {
                name,
                value,
                is_root,
                state: FooListTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for FooListTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    FooListTypeSerializerState::Init__ => {
                        match &self.value.content {
                            FooListTypeContent::Bar(x) => {
                                self.state = FooListTypeSerializerState::Bar(
                                    xsd_parser::quick_xml::ContentSerializer::new(
                                        x,
                                        Some("tns:Bar"),
                                        false,
                                    ),
                                )
                            }
                            FooListTypeContent::Baz(x) => {
                                self.state = FooListTypeSerializerState::Baz(
                                    xsd_parser::quick_xml::ContentSerializer::new(
                                        x,
                                        Some("tns:Baz"),
                                        false,
                                    ),
                                )
                            }
                        }
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                        }
                        return Some(Ok(Event::Start(bytes)));
                    }
                    FooListTypeSerializerState::Bar(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooListTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = FooListTypeSerializerState::End__,
                    },
                    FooListTypeSerializerState::Baz(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = FooListTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = FooListTypeSerializerState::End__,
                    },
                    FooListTypeSerializerState::End__ => {
                        self.state = FooListTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    FooListTypeSerializerState::Done__ => return None,
                    FooListTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
}
pub mod quick_xml_deserialize {
    use super::*;
    #[derive(Debug)]
    pub struct FooOptionTypeDeserializer {
        content: Option<super::FooOptionTypeContent>,
        state: Box<FooOptionTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooOptionTypeDeserializerState {
        Next__,
        Bar(<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Baz(<i32 as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl FooOptionTypeDeserializer {
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
                content: None,
                state: Box::new(FooOptionTypeDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::FooOptionType>
        for FooOptionTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FooOptionType, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FooOptionType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_TNS: &[u8] = b"http://example.com";
            match (
                core::mem::replace(&mut *self.state, FooOptionTypeDeserializerState::Next__),
                &event,
            ) {
                (FooOptionTypeDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(reader.resolve_local_name(x.name(), NS_TNS), Some(b"Bar")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Bar")))?;
                            }
                            self.content = Some(FooOptionTypeContent::Bar(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FooOptionTypeDeserializerState::Bar(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_TNS), Some(b"Baz")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Baz")))?;
                            }
                            self.content = Some(FooOptionTypeContent::Baz(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FooOptionTypeDeserializerState::Baz(deserializer);
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
                (FooOptionTypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (FooOptionTypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (FooOptionTypeDeserializerState::Bar(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Bar")))?;
                        }
                        self.content = Some(FooOptionTypeContent::Bar(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FooOptionTypeDeserializerState::Bar(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FooOptionTypeDeserializerState::Baz(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Baz")))?;
                        }
                        self.content = Some(FooOptionTypeContent::Baz(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FooOptionTypeDeserializerState::Baz(deserializer);
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
        ) -> Result<super::FooOptionType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::FooOptionType {
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct FooListTypeDeserializer {
        content: Vec<super::FooListTypeContent>,
        state: Box<FooListTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooListTypeDeserializerState {
        Next__,
        Bar(<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Baz(<i32 as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl FooListTypeDeserializer {
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
                content: Vec::new(),
                state: Box::new(FooListTypeDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::FooListType> for FooListTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FooListType, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FooListType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_TNS: &[u8] = b"http://example.com";
            match (
                core::mem::replace(&mut *self.state, FooListTypeDeserializerState::Next__),
                &event,
            ) {
                (FooListTypeDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(reader.resolve_local_name(x.name(), NS_TNS), Some(b"Bar")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(FooListTypeContent::Bar(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FooListTypeDeserializerState::Bar(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_TNS), Some(b"Baz")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(FooListTypeContent::Baz(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = FooListTypeDeserializerState::Baz(deserializer);
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
                (FooListTypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (FooListTypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (FooListTypeDeserializerState::Bar(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(FooListTypeContent::Bar(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FooListTypeDeserializerState::Bar(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (FooListTypeDeserializerState::Baz(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(FooListTypeContent::Baz(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = FooListTypeDeserializerState::Baz(deserializer);
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
        fn finish<R>(self, _reader: &R) -> Result<super::FooListType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::FooListType {
                content: self.content,
            })
        }
    }
}
