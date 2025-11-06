use xsd_parser::{
    models::schema::Namespace,
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub type MyName = AnyType;
#[derive(Debug)]
pub struct AnyType;
impl WithSerializer for AnyType {
    type Serializer<'x> = quick_xml_serialize::AnyTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::AnyTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::AnyTypeSerializerState::Init__),
            name: name.unwrap_or("anyType"),
            is_root,
        })
    }
}
impl WithDeserializer for AnyType {
    type Deserializer = quick_xml_deserialize::AnyTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        BytesStart, DeserializeReader, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, Error, Event,
    };
    #[derive(Debug)]
    pub struct AnyTypeDeserializer {
        state__: Box<AnyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AnyTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl AnyTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            Ok(Self {
                state__: Box::new(AnyTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: AnyTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::AnyType> for AnyTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::AnyType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnyType>
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
                    allow_any: true,
                })
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::AnyType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state__, AnyTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::AnyType {})
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser::quick_xml::{BytesStart, Error, Event};
    #[derive(Debug)]
    pub struct AnyTypeSerializer<'ser> {
        pub(super) value: &'ser super::AnyType,
        pub(super) state: Box<AnyTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum AnyTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> AnyTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    AnyTypeSerializerState::Init__ => {
                        *self.state = AnyTypeSerializerState::Done__;
                        let bytes = BytesStart::new(self.name);
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    AnyTypeSerializerState::Done__ => return Ok(None),
                    AnyTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for AnyTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = AnyTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
