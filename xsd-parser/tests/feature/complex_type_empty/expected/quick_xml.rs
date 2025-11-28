use xsd_parser_types::{
    misc::Namespace,
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_UNNAMED_2: Namespace = Namespace::new_const(b"http://www.iata.org/IATA/2007/00");
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
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, Error, Event,
    };
    #[derive(Debug)]
    pub struct SuccessTypeDeserializer {
        state__: Box<SuccessTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SuccessTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl SuccessTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                state__: Box::new(SuccessTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SuccessTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::SuccessType> for SuccessTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SuccessType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SuccessType> {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(helper)?),
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::SuccessType, Error> {
            let state = replace(&mut *self.state__, SuccessTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::SuccessType {})
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{BytesStart, Error, Event};
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
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_UNNAMED_2[..]));
                        }
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
