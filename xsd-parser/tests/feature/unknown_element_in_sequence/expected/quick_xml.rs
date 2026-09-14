use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub type Root = RootType;
#[derive(Debug)]
pub struct RootType {
    pub a: String,
    pub b: Option<String>,
}
impl WithSerializer for RootType {
    type Serializer<'x> = quick_xml_serialize::RootTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::RootTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::RootTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Root"),
            is_root,
        })
    }
}
impl WithDeserializer for RootType {
    type Deserializer = quick_xml_deserialize::RootTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct RootTypeDeserializer {
        a: Option<String>,
        b: Option<String>,
        state__: Box<RootTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RootTypeDeserializerState {
        Init__,
        A(Option<<String as WithDeserializer>::Deserializer>),
        B(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl RootTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                a: None,
                b: None,
                state__: Box::new(RootTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: RootTypeDeserializerState,
        ) -> Result<(), Error> {
            use RootTypeDeserializerState as S;
            match state {
                S::A(Some(deserializer)) => self.store_a(deserializer.finish(helper)?)?,
                S::B(Some(deserializer)) => self.store_b(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_a(&mut self, value: String) -> Result<(), Error> {
            if self.a.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"A")))?;
            }
            self.a = Some(value);
            Ok(())
        }
        fn store_b(&mut self, value: String) -> Result<(), Error> {
            if self.b.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"B")))?;
            }
            self.b = Some(value);
            Ok(())
        }
        fn handle_a<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<RootTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RootTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::A(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_a(data)?;
                    *self.state__ = S::B(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::A(Some(deserializer)));
                    *self.state__ = S::B(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_b<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<RootTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RootTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_b(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::B(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RootType> for RootTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RootType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RootType> {
            use RootTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let entry_state__ = match &*self.state__ {
                S::Init__ => Some(S::Init__),
                S::A(None) => Some(S::A(None)),
                S::B(None) => Some(S::B(None)),
                _ => None,
            };
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::A(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_a(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::B(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_b(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::A(None);
                        event
                    }
                    (S::A(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"A",
                            false,
                        )?;
                        match self.handle_a(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::B(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"B",
                            false,
                        )?;
                        match self.handle_b(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            } else if !matches!(event, DeserializerEvent::None) {
                if let Some(entry_state) = entry_state__ {
                    *self.state__ = entry_state;
                }
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::RootType, Error> {
            let state = replace(&mut *self.state__, RootTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::RootType {
                a: helper.finish_element("A", self.a)?,
                b: self.b,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            let _ = helper;
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"A")
            ) {
                return true;
            }
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"B")
            ) {
                return true;
            }
            false
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, Error, Event, IterSerializer, SerializeHelper, Serializer,
        WithSerializer,
    };
    #[derive(Debug)]
    pub struct RootTypeSerializer<'ser> {
        pub(super) value: &'ser super::RootType,
        pub(super) state: Box<RootTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum RootTypeSerializerState<'ser> {
        Init__,
        A(<String as WithSerializer>::Serializer<'ser>),
        B(IterSerializer<'ser, Option<&'ser String>, String>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RootTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    RootTypeSerializerState::Init__ => {
                        *self.state = RootTypeSerializerState::A(WithSerializer::serializer(
                            &self.value.a,
                            Some("tns:A"),
                            false,
                        )?);
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    RootTypeSerializerState::A(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = RootTypeSerializerState::B(IterSerializer::new(
                                self.value.b.as_ref(),
                                Some("tns:B"),
                                false,
                            ))
                        }
                    },
                    RootTypeSerializerState::B(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = RootTypeSerializerState::End__,
                    },
                    RootTypeSerializerState::End__ => {
                        *self.state = RootTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    RootTypeSerializerState::Done__ => return Ok(None),
                    RootTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for RootTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = RootTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
