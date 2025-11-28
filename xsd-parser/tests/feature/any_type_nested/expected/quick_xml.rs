use xsd_parser_types::{
    misc::Namespace,
    quick_xml::{Error, WithDeserializer, WithSerializer},
    xml::AnyElement,
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub type Root = RootType;
#[derive(Debug)]
pub struct RootType {
    pub container: ContainerType,
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
            name: name.unwrap_or("Root"),
            is_root,
        })
    }
}
impl WithDeserializer for RootType {
    type Deserializer = quick_xml_deserialize::RootTypeDeserializer;
}
#[derive(Debug)]
pub struct ContainerType {
    pub content: Vec<ContainerTypeContent>,
}
#[derive(Debug)]
pub struct ContainerTypeContent {
    pub known: String,
    pub any: Option<AnyElement>,
}
impl WithSerializer for ContainerType {
    type Serializer<'x> = quick_xml_serialize::ContainerTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ContainerTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ContainerTypeSerializerState::Init__),
            name: name.unwrap_or("ContainerType"),
            is_root,
        })
    }
}
impl WithSerializer for ContainerTypeContent {
    type Serializer<'x> = quick_xml_serialize::ContainerTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::ContainerTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ContainerTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for ContainerType {
    type Deserializer = quick_xml_deserialize::ContainerTypeDeserializer;
}
impl WithDeserializer for ContainerTypeContent {
    type Deserializer = quick_xml_deserialize::ContainerTypeContentDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::{
        quick_xml::{
            BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
            RawByteStr, WithDeserializer,
        },
        xml::AnyElement,
    };
    #[derive(Debug)]
    pub struct RootTypeDeserializer {
        container: Option<super::ContainerType>,
        state__: Box<RootTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RootTypeDeserializerState {
        Init__,
        Container(Option<<super::ContainerType as WithDeserializer>::Deserializer>),
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
                container: None,
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
                S::Container(Some(deserializer)) => {
                    self.store_container(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_container(&mut self, value: super::ContainerType) -> Result<(), Error> {
            if self.container.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Container",
                )))?;
            }
            self.container = Some(value);
            Ok(())
        }
        fn handle_container<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ContainerType>,
            fallback: &mut Option<RootTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.container.is_some() {
                    fallback.get_or_insert(RootTypeDeserializerState::Container(None));
                    *self.state__ = RootTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = RootTypeDeserializerState::Container(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_container(data)?;
                    *self.state__ = RootTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(RootTypeDeserializerState::Container(Some(
                                deserializer,
                            )));
                            *self.state__ = RootTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                RootTypeDeserializerState::Container(Some(deserializer));
                        }
                    }
                    ret
                }
            })
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
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Container(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_container(helper, output, &mut fallback)? {
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
                        *self.state__ = RootTypeDeserializerState::Container(None);
                        event
                    }
                    (S::Container(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"Container", false)?;
                        match self.handle_container(helper, output, &mut fallback)? {
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
                        fallback.get_or_insert(S::Done__);
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::RootType, Error> {
            let state = replace(&mut *self.state__, RootTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::RootType {
                container: self
                    .container
                    .ok_or_else(|| ErrorKind::MissingElement("Container".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ContainerTypeDeserializer {
        content: Vec<super::ContainerTypeContent>,
        state__: Box<ContainerTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ContainerTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::ContainerTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ContainerTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: Vec::new(),
                state__: Box::new(ContainerTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ContainerTypeDeserializerState,
        ) -> Result<(), Error> {
            if let ContainerTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ContainerTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ContainerTypeContent>,
            fallback: &mut Option<ContainerTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback
                    .take()
                    .unwrap_or(ContainerTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = ContainerTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ = ContainerTypeDeserializerState::Content__(deserializer);
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ContainerTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state__ = ContainerTypeDeserializerState::Next__;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ContainerType> for ContainerTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ContainerType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ContainerType> {
            use ContainerTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::ContainerTypeContent as WithDeserializer>::Deserializer::init(
                                helper, event,
                            )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ContainerType, Error> {
            let state = replace(
                &mut *self.state__,
                ContainerTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ContainerType {
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct ContainerTypeContentDeserializer {
        known: Option<String>,
        any: Option<AnyElement>,
        state__: Box<ContainerTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    enum ContainerTypeContentDeserializerState {
        Init__,
        Known(Option<<String as WithDeserializer>::Deserializer>),
        Any(Option<<AnyElement as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ContainerTypeContentDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ContainerTypeContentDeserializerState,
        ) -> Result<(), Error> {
            use ContainerTypeContentDeserializerState as S;
            match state {
                S::Known(Some(deserializer)) => self.store_known(deserializer.finish(helper)?)?,
                S::Any(Some(deserializer)) => self.store_any(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_known(&mut self, value: String) -> Result<(), Error> {
            if self.known.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Known",
                )))?;
            }
            self.known = Some(value);
            Ok(())
        }
        fn store_any(&mut self, value: AnyElement) -> Result<(), Error> {
            if self.any.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"any3")))?;
            }
            self.any = Some(value);
            Ok(())
        }
        fn handle_known<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ContainerTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.known.is_some() {
                    fallback.get_or_insert(ContainerTypeContentDeserializerState::Known(None));
                    *self.state__ = ContainerTypeContentDeserializerState::Any(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = ContainerTypeContentDeserializerState::Known(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_known(data)?;
                    *self.state__ = ContainerTypeContentDeserializerState::Any(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ContainerTypeContentDeserializerState::Known(
                                Some(deserializer),
                            ));
                            *self.state__ = ContainerTypeContentDeserializerState::Any(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                ContainerTypeContentDeserializerState::Known(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_any<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, AnyElement>,
            fallback: &mut Option<ContainerTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(ContainerTypeContentDeserializerState::Any(None));
                *self.state__ = ContainerTypeContentDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_any(data)?;
                    *self.state__ = ContainerTypeContentDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ContainerTypeContentDeserializerState::Any(
                                Some(deserializer),
                            ));
                            *self.state__ = ContainerTypeContentDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                ContainerTypeContentDeserializerState::Any(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ContainerTypeContent> for ContainerTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ContainerTypeContent> {
            let deserializer = Self {
                known: None,
                any: None,
                state__: Box::new(ContainerTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, ContainerTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ContainerTypeContent> {
            use ContainerTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let mut is_any_retry = false;
            let mut any_fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Known(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_known(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Any(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_any(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, event @ Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = ContainerTypeContentDeserializerState::Known(None);
                        event
                    }
                    (S::Known(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"Known", false)?;
                        match self.handle_known(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Any(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if is_any_retry {
                            let output = <AnyElement as WithDeserializer>::Deserializer::init(
                                helper, event,
                            )?;
                            match self.handle_any(helper, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            any_fallback.get_or_insert(S::Any(None));
                            *self.state__ = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        if let Some(state) = any_fallback.take() {
                            is_any_retry = true;
                            *self.state__ = state;
                            event
                        } else {
                            fallback.get_or_insert(S::Done__);
                            break (DeserializerEvent::Continue(event), allow_any_element);
                        }
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
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::ContainerTypeContent, Error> {
            let state = replace(
                &mut *self.state__,
                ContainerTypeContentDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ContainerTypeContent {
                known: self
                    .known
                    .ok_or_else(|| ErrorKind::MissingElement("Known".into()))?,
                any: self.any,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::{
        quick_xml::{BytesEnd, BytesStart, Error, Event, IterSerializer, WithSerializer},
        xml::AnyElement,
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
        Container(<super::ContainerType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RootTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    RootTypeSerializerState::Init__ => {
                        *self.state =
                            RootTypeSerializerState::Container(WithSerializer::serializer(
                                &self.value.container,
                                Some("Container"),
                                false,
                            )?);
                        let bytes = BytesStart::new(self.name);
                        return Ok(Some(Event::Start(bytes)));
                    }
                    RootTypeSerializerState::Container(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = RootTypeSerializerState::End__,
                    },
                    RootTypeSerializerState::End__ => {
                        *self.state = RootTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    RootTypeSerializerState::Done__ => return Ok(None),
                    RootTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for RootTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = RootTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ContainerTypeSerializer<'ser> {
        pub(super) value: &'ser super::ContainerType,
        pub(super) state: Box<ContainerTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ContainerTypeSerializerState<'ser> {
        Init__,
        Content__(
            IterSerializer<'ser, &'ser [super::ContainerTypeContent], super::ContainerTypeContent>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ContainerTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ContainerTypeSerializerState::Init__ => {
                        *self.state = ContainerTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let bytes = BytesStart::new(self.name);
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ContainerTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ContainerTypeSerializerState::End__,
                    },
                    ContainerTypeSerializerState::End__ => {
                        *self.state = ContainerTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ContainerTypeSerializerState::Done__ => return Ok(None),
                    ContainerTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ContainerTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ContainerTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ContainerTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::ContainerTypeContent,
        pub(super) state: Box<ContainerTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum ContainerTypeContentSerializerState<'ser> {
        Init__,
        Known(<String as WithSerializer>::Serializer<'ser>),
        Any(IterSerializer<'ser, Option<&'ser AnyElement>, AnyElement>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ContainerTypeContentSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ContainerTypeContentSerializerState::Init__ => {
                        *self.state = ContainerTypeContentSerializerState::Known(
                            WithSerializer::serializer(&self.value.known, Some("Known"), false)?,
                        );
                    }
                    ContainerTypeContentSerializerState::Known(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = ContainerTypeContentSerializerState::Any(
                                IterSerializer::new(self.value.any.as_ref(), None, false),
                            )
                        }
                    },
                    ContainerTypeContentSerializerState::Any(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ContainerTypeContentSerializerState::Done__,
                    },
                    ContainerTypeContentSerializerState::Done__ => return Ok(None),
                    ContainerTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ContainerTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ContainerTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
