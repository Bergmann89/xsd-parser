use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_UNNAMED_5: Namespace = Namespace::new_const(b"http://example.com");
pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub bar: String,
    pub baz: i32,
}
impl WithDeserializer for FooType {
    type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
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
            name: name.unwrap_or("FooType"),
            is_root,
        })
    }
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct FooTypeDeserializer {
        bar: Option<String>,
        baz: Option<i32>,
        state__: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Init__,
        Bar(Option<<String as WithDeserializer>::Deserializer>),
        Baz(Option<<i32 as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl FooTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                bar: None,
                baz: None,
                state__: Box::new(FooTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: FooTypeDeserializerState,
        ) -> Result<(), Error> {
            use FooTypeDeserializerState as S;
            match state {
                S::Bar(Some(deserializer)) => self.store_bar(deserializer.finish(helper)?)?,
                S::Baz(Some(deserializer)) => self.store_baz(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_bar(&mut self, value: String) -> Result<(), Error> {
            if self.bar.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Bar")))?;
            }
            self.bar = Some(value);
            Ok(())
        }
        fn store_baz(&mut self, value: i32) -> Result<(), Error> {
            if self.baz.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Baz")))?;
            }
            self.baz = Some(value);
            Ok(())
        }
        fn handle_bar<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<FooTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FooTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Bar(None));
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
                    self.store_bar(data)?;
                    *self.state__ = S::Baz(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Bar(Some(deserializer)));
                    *self.state__ = S::Baz(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_baz<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<FooTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FooTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Baz(None));
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
                    self.store_baz(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Baz(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::FooType> for FooTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooType> {
            use FooTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Bar(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_bar(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Baz(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_baz(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Bar(None);
                        event
                    }
                    (S::Bar(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_5),
                            b"Bar",
                            false,
                        )?;
                        match self.handle_bar(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Baz(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_5),
                            b"Baz",
                            false,
                        )?;
                        match self.handle_baz(helper, output, &mut fallback)? {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::FooType, Error> {
            let state = replace(&mut *self.state__, FooTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::FooType {
                bar: helper.finish_element("Bar", self.bar)?,
                baz: helper.finish_element("Baz", self.baz)?,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, Error, Event, SerializeHelper, Serializer, WithSerializer,
    };
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
        Bar(<String as WithSerializer>::Serializer<'ser>),
        Baz(<i32 as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FooTypeSerializerState::Init__ => {
                        *self.state = FooTypeSerializerState::Bar(WithSerializer::serializer(
                            &self.value.bar,
                            Some("Bar"),
                            false,
                        )?);
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns_for_tag(&mut bytes, self.name, &super::NS_UNNAMED_5);
                        return Ok(Some(Event::Start(bytes)));
                    }
                    FooTypeSerializerState::Bar(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = FooTypeSerializerState::Baz(WithSerializer::serializer(
                                &self.value.baz,
                                Some("Baz"),
                                false,
                            )?)
                        }
                    },
                    FooTypeSerializerState::Baz(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooTypeSerializerState::End__,
                    },
                    FooTypeSerializerState::End__ => {
                        *self.state = FooTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    FooTypeSerializerState::Done__ => return Ok(None),
                    FooTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for FooTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
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
