use num::BigInt;
use xsd_parser::models::schema::Namespace;
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub mod tns {
    use xsd_parser::quick_xml::{Error, WithDeserializer, WithSerializer};
    pub type Foo = FooType;
    #[derive(Debug)]
    pub struct FooType {
        pub a_int: super::BigInt,
        pub b_int: super::BigInt,
    }
    impl FooType {
        #[must_use]
        pub fn default_a_int() -> super::BigInt {
            use core::str::FromStr;
            num::BigInt::from_str("123").unwrap()
        }
        #[must_use]
        pub fn default_b_int() -> super::BigInt {
            use core::str::FromStr;
            num::BigInt::from_str("456").unwrap()
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
                state: Box::new(quick_xml_serialize::FooTypeSerializerState::Init__),
                name: name.unwrap_or("tns:FooType"),
                is_root,
            })
        }
    }
    impl WithDeserializer for FooType {
        type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
    }
    pub mod quick_xml_deserialize {
        use core::mem::replace;
        use xsd_parser::quick_xml::{
            filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer,
            DeserializerArtifact, DeserializerEvent, DeserializerOutput, DeserializerResult, Error,
            Event,
        };
        #[derive(Debug)]
        pub struct FooTypeDeserializer {
            a_int: super::super::BigInt,
            b_int: super::super::BigInt,
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
                let mut a_int: Option<super::super::BigInt> = None;
                let mut b_int: Option<super::super::BigInt> = None;
                for attrib in filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    if matches!(
                        reader.resolve_local_name(attrib.key, &super::super::NS_TNS),
                        Some(b"a-int")
                    ) {
                        reader.read_attrib(&mut a_int, b"a-int", &attrib.value)?;
                    } else if matches!(
                        reader.resolve_local_name(attrib.key, &super::super::NS_TNS),
                        Some(b"b-int")
                    ) {
                        reader.read_attrib(&mut b_int, b"b-int", &attrib.value)?;
                    } else {
                        reader.raise_unexpected_attrib(attrib)?;
                    }
                }
                Ok(Self {
                    a_int: a_int.unwrap_or_else(super::FooType::default_a_int),
                    b_int: b_int.unwrap_or_else(super::FooType::default_b_int),
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
                    a_int: self.a_int,
                    b_int: self.b_int,
                })
            }
        }
    }
    pub mod quick_xml_serialize {
        use core::iter::Iterator;
        use xsd_parser::quick_xml::{write_attrib, BytesStart, Error, Event};
        #[derive(Debug)]
        pub struct FooTypeSerializer<'ser> {
            pub(super) value: &'ser super::FooType,
            pub(super) state: Box<FooTypeSerializerState<'ser>>,
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
                    match &mut *self.state {
                        FooTypeSerializerState::Init__ => {
                            *self.state = FooTypeSerializerState::Done__;
                            let mut bytes = BytesStart::new(self.name);
                            if self.is_root {
                                bytes
                                    .push_attribute((&b"xmlns:tns"[..], &super::super::NS_TNS[..]));
                            }
                            write_attrib(&mut bytes, "tns:a-int", &self.value.a_int)?;
                            write_attrib(&mut bytes, "tns:b-int", &self.value.b_int)?;
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
                        *self.state = FooTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
    }
}
