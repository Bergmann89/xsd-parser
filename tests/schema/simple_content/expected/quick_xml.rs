pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub value: Option<String>,
    pub content: EnumType,
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
        Content(xsd_parser::quick_xml::ContentSerializer<'ser, EnumType>),
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
            fn build_attributes<'a>(
                mut bytes: BytesStart<'a>,
                value: &'a super::FooType,
            ) -> Result<BytesStart<'a>, Error> {
                use xsd_parser::quick_xml::SerializeBytes;
                if let Some(val) = value
                    .value
                    .as_ref()
                    .map(SerializeBytes::serialize_bytes)
                    .transpose()?
                    .flatten()
                {
                    bytes.push_attribute(("tns:value", val));
                }
                Ok(bytes)
            }
            loop {
                match &mut self.state {
                    FooTypeSerializerState::Init__ => {
                        self.state = FooTypeSerializerState::Content(
                            xsd_parser::quick_xml::ContentSerializer::new(
                                &self.value.content,
                                None,
                                false,
                            ),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                        }
                        match build_attributes(bytes, &self.value) {
                            Ok(bytes) => return Some(Ok(Event::Start(bytes))),
                            Err(error) => {
                                self.state = FooTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                    }
                    FooTypeSerializerState::Content(x) => match x.next() {
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
        value: Option<String>,
        content: Option<super::EnumType>,
        state: Option<xsd_parser::quick_xml::ContentDeserializer<EnumType>>,
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
            const NS_TNS: &[u8] = b"http://example.com";
            let mut value: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(
                    reader.resolve_local_name(attrib.key, NS_TNS),
                    Some(b"value")
                ) {
                    reader.read_attrib(&mut value, b"value", &attrib.value)?;
                } else {
                    reader.err(ErrorKind::UnexpectedAttribute(
                        xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                    ))?;
                }
            }
            Ok(Self {
                value: value,
                content: None,
                state: None,
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
            use xsd_parser::quick_xml::{ContentDeserializer, DeserializerOutput, Event};
            let (Event::Start(start) | Event::Empty(start)) = &event else {
                return Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                });
            };
            let mut this = Self::from_bytes_start(reader, &start)?;
            let DeserializerOutput {
                data,
                deserializer,
                event,
                allow_any,
            } = ContentDeserializer::init(reader, event)?;
            if let Some(data) = data {
                this.content = Some(data);
                let data = this.finish(reader)?;
                Ok(DeserializerOutput {
                    data: Some(data),
                    deserializer: None,
                    event,
                    allow_any,
                })
            } else if let Some(state) = deserializer {
                this.state = Some(state);
                Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(this),
                    event,
                    allow_any,
                })
            } else {
                Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event,
                    allow_any,
                })
            }
        }
        fn next<R>(
            self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FooType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::DeserializerOutput;
            let mut this = self;
            let DeserializerOutput {
                data,
                deserializer,
                event,
                allow_any,
            } = this.state.take().unwrap().next(reader, event)?;
            if let Some(data) = data {
                this.content = Some(data);
                let data = this.finish(reader)?;
                Ok(DeserializerOutput {
                    data: Some(data),
                    deserializer: None,
                    event,
                    allow_any,
                })
            } else if let Some(state) = deserializer {
                this.state = Some(state);
                Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(this),
                    event,
                    allow_any,
                })
            } else {
                Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event,
                    allow_any,
                })
            }
        }
        fn finish<R>(self, _reader: &R) -> Result<super::FooType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::FooType {
                value: self.value,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
}
