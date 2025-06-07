use xsd_parser::{
    models::schema::Namespace,
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_DEFAULT: Namespace = Namespace::new_const(b"http://www.iata.org/IATA/2007/00");
#[derive(Debug)]
pub struct SuccessType;
impl WithSerializer for SuccessType {
    type Serializer<'x> = quick_xml_serialize::SuccessTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::SuccessTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SuccessTypeSerializerState::Init__),
            name: name.unwrap_or("SuccessType"),
            is_root,
        })
    }
}
impl WithDeserializer for SuccessType {
    type Deserializer = quick_xml_deserialize::SuccessTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, Error, Event,
    };
    #[derive(Debug)]
    pub struct SuccessTypeDeserializer {
        state: Box<SuccessTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SuccessTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl SuccessTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                state: Box::new(SuccessTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SuccessTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::SuccessType> for SuccessTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::SuccessType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SuccessType>
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
        fn finish<R>(mut self, reader: &R) -> Result<super::SuccessType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, SuccessTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::SuccessType {})
        }
    }
}
pub mod quick_xml_serialize {
    use core::iter::Iterator;
    use xsd_parser::quick_xml::{BytesStart, Error, Event};
    #[derive(Debug)]
    pub struct SuccessTypeSerializer<'ser> {
        pub(super) value: &'ser super::SuccessType,
        pub(super) state: Box<SuccessTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SuccessTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SuccessTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SuccessTypeSerializerState::Init__ => {
                        *self.state = SuccessTypeSerializerState::Done__;
                        let bytes = BytesStart::new(self.name);
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    SuccessTypeSerializerState::Done__ => return Ok(None),
                    SuccessTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for SuccessTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SuccessTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
