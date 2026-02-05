use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{Error, WithDeserializer, WithSerializer},
    xml::Nillable,
};
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://test.example.com");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub type TestElement = TestType;
#[derive(Debug)]
pub struct TestType {
    pub value: Nillable<String>,
    pub count: Option<i32>,
}
impl WithDeserializer for TestType {
    type Deserializer = quick_xml_deserialize::TestTypeDeserializer;
}
impl WithSerializer for TestType {
    type Serializer<'x> = quick_xml_serialize::TestTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TestTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TestTypeSerializerState::Init__),
            name: name.unwrap_or("tns:TestType"),
            is_root,
        })
    }
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::{
        quick_xml::{
            BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
            RawByteStr, WithDeserializer,
        },
        xml::Nillable,
    };
    #[derive(Debug)]
    pub struct TestTypeDeserializer {
        value: Option<Nillable<String>>,
        count: Option<i32>,
        state__: Box<TestTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TestTypeDeserializerState {
        Init__,
        Value(Option<<Nillable<String> as WithDeserializer>::Deserializer>),
        Count(Option<<i32 as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TestTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                value: None,
                count: None,
                state__: Box::new(TestTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TestTypeDeserializerState,
        ) -> Result<(), Error> {
            use TestTypeDeserializerState as S;
            match state {
                S::Value(Some(deserializer)) => self.store_value(deserializer.finish(helper)?)?,
                S::Count(Some(deserializer)) => self.store_count(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_value(&mut self, value: Nillable<String>) -> Result<(), Error> {
            if self.value.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"value",
                )))?;
            }
            self.value = Some(value);
            Ok(())
        }
        fn store_count(&mut self, value: i32) -> Result<(), Error> {
            if self.count.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"count",
                )))?;
            }
            self.count = Some(value);
            Ok(())
        }
        fn handle_value<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, Nillable<String>>,
            fallback: &mut Option<TestTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TestTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Value(None));
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
                    self.store_value(data)?;
                    *self.state__ = S::Count(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Value(Some(deserializer)));
                    *self.state__ = S::Count(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_count<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<TestTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TestTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Count(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_count(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Count(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TestType> for TestTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TestType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TestType> {
            use TestTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Value(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_value(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Count(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_count(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Value(None);
                        event
                    }
                    (S::Value(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"value",
                            false,
                        )?;
                        match self.handle_value(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Count(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"count",
                            false,
                        )?;
                        match self.handle_count(helper, output, &mut fallback)? {
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
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::TestType, Error> {
            let state = replace(&mut *self.state__, TestTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::TestType {
                value: helper.finish_element("value", self.value)?,
                count: self.count,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::{
        quick_xml::{
            BytesEnd, BytesStart, Error, Event, IterSerializer, SerializeHelper, Serializer,
            WithSerializer,
        },
        xml::Nillable,
    };
    #[derive(Debug)]
    pub struct TestTypeSerializer<'ser> {
        pub(super) value: &'ser super::TestType,
        pub(super) state: Box<TestTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TestTypeSerializerState<'ser> {
        Init__,
        Value(<Nillable<String> as WithSerializer>::Serializer<'ser>),
        Count(IterSerializer<'ser, Option<&'ser i32>, i32>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TestTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TestTypeSerializerState::Init__ => {
                        *self.state = TestTypeSerializerState::Value(WithSerializer::serializer(
                            &self.value.value,
                            Some("tns:value"),
                            false,
                        )?);
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_XSI),
                                &super::NS_XSI,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TestTypeSerializerState::Value(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = TestTypeSerializerState::Count(IterSerializer::new(
                                self.value.count.as_ref(),
                                Some("tns:count"),
                                false,
                            ))
                        }
                    },
                    TestTypeSerializerState::Count(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = TestTypeSerializerState::End__,
                    },
                    TestTypeSerializerState::End__ => {
                        *self.state = TestTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TestTypeSerializerState::Done__ => return Ok(None),
                    TestTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TestTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TestTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
