pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
use std::borrow::Cow;
use xsd_parser::{
    quick_xml::{
        deserialize_new::{DeserializeBytes, DeserializeReader, WithDeserializer},
        Error, SerializeBytes, WithSerializer,
    },
    schema::Namespace,
};
pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub a_list: ListType,
}
impl FooType {
    #[must_use]
    pub fn default_a_list() -> ListType {
        ListType(Vec::new())
    }
}
impl WithSerializer for FooType {
    type Serializer<'x> = quick_xml_serialize::FooTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::FooTypeSerializer {
            value: self,
            state: quick_xml_serialize::FooTypeSerializerState::Init__,
            name: name.unwrap_or("tns:FooType"),
            is_root,
        })
    }
}
impl WithDeserializer for FooType {
    type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
}
#[derive(Debug, Clone, Default)]
pub struct ListType(pub Vec<StringType>);
impl SerializeBytes for ListType {
    fn serialize_bytes(&self) -> Result<Option<Cow<'_, str>>, Error> {
        if self.0.is_empty() {
            return Ok(None);
        }
        let mut data = String::new();
        for item in &self.0 {
            if let Some(bytes) = item.serialize_bytes()? {
                if !data.is_empty() {
                    data.push(' ');
                }
                data.push_str(&bytes);
            }
        }
        Ok(Some(Cow::Owned(data)))
    }
}
impl DeserializeBytes for ListType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        Ok(Self(
            bytes
                .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                .map(|bytes| StringType::deserialize_bytes(reader, bytes))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}
pub type StringType = String;
pub mod quick_xml_serialize {
    use core::iter::Iterator;
    use xsd_parser::quick_xml::{write_attrib, BytesStart, Error, Event};
    #[derive(Debug)]
    pub struct FooTypeSerializer<'ser> {
        pub(super) value: &'ser super::FooType,
        pub(super) state: FooTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum FooTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    FooTypeSerializerState::Init__ => {
                        self.state = FooTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        write_attrib(&mut bytes, "tns:a-list", &self.value.a_list)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    FooTypeSerializerState::Done__ => return Ok(None),
                    FooTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for FooTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = FooTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        deserialize_new::{
            DeserializeReader, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult,
        },
        filter_xmlns_attributes, BytesStart, Error, Event,
    };
    #[derive(Debug)]
    pub struct FooTypeDeserializer {
        a_list: super::ListType,
        state: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl FooTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut a_list: Option<super::ListType> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"a-list")
                ) {
                    reader.read_attrib(&mut a_list, b"a-list", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
            Ok(Self {
                a_list: a_list.unwrap_or_else(super::FooType::default_a_list),
                state: Box::new(FooTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: FooTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::FooType> for FooTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::FooType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooType>
        where
            R: DeserializeReader,
        {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(reader)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                })
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::FooType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, FooTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::FooType {
                a_list: self.a_list,
            })
        }
    }
}
