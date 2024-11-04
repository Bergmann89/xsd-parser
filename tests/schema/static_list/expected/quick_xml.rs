pub type Array = ArrayType;
#[derive(Debug, Clone)]
pub struct ArrayType {
    pub item: [IntType; 5usize],
}
impl xsd_parser::quick_xml::WithSerializer for ArrayType {
    type Serializer<'x> = quick_xml_serialize::ArrayTypeSerializer<'x>;
}
impl xsd_parser::quick_xml::WithDeserializer for ArrayType {
    type Deserializer = quick_xml_deserialize::ArrayTypeDeserializer;
}
pub type IntType = i32;
pub mod quick_xml_serialize {
    use super::*;
    #[derive(Debug)]
    pub struct ArrayTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::ArrayType,
        is_root: bool,
        state: ArrayTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum ArrayTypeSerializerState<'ser> {
        Init__,
        Item(xsd_parser::quick_xml::IterSerializer<'ser, [IntType; 5usize], IntType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> xsd_parser::quick_xml::Serializer<'ser, super::ArrayType> for ArrayTypeSerializer<'ser> {
        fn init(
            value: &'ser super::ArrayType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ArrayType");
            Ok(Self {
                name,
                value,
                is_root,
                state: ArrayTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for ArrayTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{BytesEnd, BytesStart, Error, Event, Serializer};
            loop {
                match &mut self.state {
                    ArrayTypeSerializerState::Init__ => {
                        match Serializer::init(&self.value.item, Some("tns:Item"), false) {
                            Ok(serializer) => {
                                self.state = ArrayTypeSerializerState::Item(serializer)
                            }
                            Err(error) => {
                                self.state = ArrayTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                        }
                        return Some(Ok(Event::Start(bytes)));
                    }
                    ArrayTypeSerializerState::Item(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ArrayTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = ArrayTypeSerializerState::End__,
                    },
                    ArrayTypeSerializerState::End__ => {
                        self.state = ArrayTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    ArrayTypeSerializerState::Done__ => return None,
                    ArrayTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
}
pub mod quick_xml_deserialize {
    use super::*;
    #[derive(Debug)]
    pub struct ArrayTypeDeserializer {
        item: Vec<super::IntType>,
        state: Box<ArrayTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ArrayTypeDeserializerState {
        Next__,
        Item(<IntType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl ArrayTypeDeserializer {
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
                item: Vec::new(),
                state: Box::new(ArrayTypeDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::ArrayType> for ArrayTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ArrayType, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ArrayType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_TNS: &[u8] = b"http://example.com";
            match (
                core::mem::replace(&mut *self.state, ArrayTypeDeserializerState::Next__),
                &event,
            ) {
                (ArrayTypeDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(reader.resolve_local_name(x.name(), NS_TNS), Some(b"Item")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <IntType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.item.push(data);
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = ArrayTypeDeserializerState::Item(deserializer);
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
                (ArrayTypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (ArrayTypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (ArrayTypeDeserializerState::Item(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.item.push(data);
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = ArrayTypeDeserializerState::Item(deserializer);
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
        fn finish<R>(self, _reader: &R) -> Result<super::ArrayType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::ArrayType {
                item: self
                    .item
                    .try_into()
                    .map_err(|vec: Vec<_>| ErrorKind::InsufficientSize {
                        min: 5usize,
                        max: 5usize,
                        actual: vec.len(),
                    })?,
            })
        }
    }
}
