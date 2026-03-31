use xsd_parser_types::{misc::Namespace, quick_xml::WithDeserializer};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub content: FooTypeContent,
}
#[derive(Debug)]
pub enum FooTypeContent {
    Content2(Vec<FooContent2Type>),
}
impl WithDeserializer for FooType {
    type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
}
impl WithDeserializer for FooTypeContent {
    type Deserializer = quick_xml_deserialize::FooTypeContentDeserializer;
}
#[derive(Debug)]
pub struct FooContent2Type {
    pub info: String,
}
impl WithDeserializer for FooContent2Type {
    type Deserializer = quick_xml_deserialize::FooContent2TypeDeserializer;
}
pub type Bar = BarType;
#[derive(Debug)]
pub struct BarType {
    pub content: BarTypeContent,
}
#[derive(Debug)]
pub enum BarTypeContent {
    Content4(Vec<BarContent4Type>),
}
impl WithDeserializer for BarType {
    type Deserializer = quick_xml_deserialize::BarTypeDeserializer;
}
impl WithDeserializer for BarTypeContent {
    type Deserializer = quick_xml_deserialize::BarTypeContentDeserializer;
}
#[derive(Debug)]
pub struct BarContent4Type {
    pub info: String,
}
impl WithDeserializer for BarContent4Type {
    type Deserializer = quick_xml_deserialize::BarContent4TypeDeserializer;
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
        content: Option<super::FooTypeContent>,
        state__: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::FooTypeContent as WithDeserializer>::Deserializer),
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
                content: None,
                state__: Box::new(FooTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: FooTypeDeserializerState,
        ) -> Result<(), Error> {
            if let FooTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::FooTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::FooTypeContent>,
            fallback: &mut Option<FooTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FooTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
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
                            <super::FooTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::FooType, Error> {
            let state = replace(&mut *self.state__, FooTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::FooType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct FooTypeContentDeserializer {
        state__: Box<FooTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum FooTypeContentDeserializerState {
        Init__,
        Content2(
            Vec<super::FooContent2Type>,
            Option<<super::FooContent2Type as WithDeserializer>::Deserializer>,
            Option<<super::FooContent2Type as WithDeserializer>::Deserializer>,
        ),
        Done__(super::FooTypeContent),
        Unknown__,
    }
    impl FooTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let mut event = event;
            let mut allow_any_element = false;
            if let Event::Start(_) | Event::Empty(_) = &event {
                event = {
                    let output = <super::FooContent2Type as WithDeserializer>::init(helper, event)?;
                    match self.handle_content_2(helper, Default::default(), None, output)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        output => {
                            return Ok(output);
                        }
                    }
                };
            }
            *self.state__ = FooTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: FooTypeContentDeserializerState,
        ) -> Result<super::FooTypeContent, Error> {
            use FooTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Content2(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_content_2(&mut values, value)?;
                    }
                    Ok(super::FooTypeContent::Content2(
                        helper.finish_vec(1usize, None, values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_content_2(
            values: &mut Vec<super::FooContent2Type>,
            value: super::FooContent2Type,
        ) -> Result<(), Error> {
            values.push(value);
            Ok(())
        }
        fn handle_content_2<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Vec<super::FooContent2Type>,
            fallback: Option<<super::FooContent2Type as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::FooContent2Type>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FooTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if fallback.is_none() && values.is_empty() {
                    *self.state__ = S::Init__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else if values.len() + usize::from(fallback.is_some()) < 1usize {
                    *self.state__ = S::Content2(values, None, fallback);
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else {
                    *self.state__ = S::Content2(values, None, fallback);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_content_2(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_content_2(&mut values, data)?;
                    *self.state__ = S::Content2(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    if ret.is_continue_start_or_empty() {
                        *self.state__ = S::Content2(values, Some(deserializer), None);
                    } else {
                        *self.state__ = S::Content2(values, None, Some(deserializer));
                    }
                    Ok(ret)
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::FooTypeContent> for FooTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooTypeContent> {
            let deserializer = Self {
                state__: Box::new(FooTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, FooTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::FooTypeContent> {
            use FooTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content2(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content_2(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Content2(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            <super::FooContent2Type as WithDeserializer>::init(helper, event)?;
                        match self.handle_content_2(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::FooTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct FooContent2TypeDeserializer {
        info: Option<String>,
        state__: Box<FooContent2TypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooContent2TypeDeserializerState {
        Init__,
        Info(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl FooContent2TypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: FooContent2TypeDeserializerState,
        ) -> Result<(), Error> {
            use FooContent2TypeDeserializerState as S;
            match state {
                S::Info(Some(deserializer)) => self.store_info(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_info(&mut self, value: String) -> Result<(), Error> {
            if self.info.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"info")))?;
            }
            self.info = Some(value);
            Ok(())
        }
        fn handle_info<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<FooContent2TypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FooContent2TypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Info(None));
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
                    self.store_info(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Info(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::FooContent2Type> for FooContent2TypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooContent2Type> {
            let deserializer = Self {
                info: None,
                state__: Box::new(FooContent2TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, FooContent2TypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::FooContent2Type> {
            use FooContent2TypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Info(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_info(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Info(None);
                        event
                    }
                    (S::Info(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"info",
                            false,
                        )?;
                        match self.handle_info(helper, output, &mut fallback)? {
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
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::FooContent2Type, Error> {
            let state = replace(
                &mut *self.state__,
                FooContent2TypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::FooContent2Type {
                info: helper.finish_element("info", self.info)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct BarTypeDeserializer {
        content: Option<super::BarTypeContent>,
        state__: Box<BarTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum BarTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::BarTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl BarTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: None,
                state__: Box::new(BarTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: BarTypeDeserializerState,
        ) -> Result<(), Error> {
            if let BarTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::BarTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::BarTypeContent>,
            fallback: &mut Option<BarTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use BarTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::BarType> for BarTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::BarType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::BarType> {
            use BarTypeDeserializerState as S;
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
                            <super::BarTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::BarType, Error> {
            let state = replace(&mut *self.state__, BarTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::BarType {
                content: helper.finish_default(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct BarTypeContentDeserializer {
        state__: Box<BarTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum BarTypeContentDeserializerState {
        Init__,
        Content4(
            Vec<super::BarContent4Type>,
            Option<<super::BarContent4Type as WithDeserializer>::Deserializer>,
            Option<<super::BarContent4Type as WithDeserializer>::Deserializer>,
        ),
        Done__(super::BarTypeContent),
        Unknown__,
    }
    impl BarTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let mut event = event;
            let mut allow_any_element = false;
            if let Event::Start(_) | Event::Empty(_) = &event {
                event = {
                    let output = <super::BarContent4Type as WithDeserializer>::init(helper, event)?;
                    match self.handle_content_4(helper, Default::default(), None, output)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        output => {
                            return Ok(output);
                        }
                    }
                };
            }
            *self.state__ = BarTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: BarTypeContentDeserializerState,
        ) -> Result<super::BarTypeContent, Error> {
            use BarTypeContentDeserializerState as S;
            match state {
                S::Init__ => Ok(super::BarTypeContent::Content4(Vec::new())),
                S::Content4(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_content_4(&mut values, value)?;
                    }
                    Ok(super::BarTypeContent::Content4(values))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_content_4(
            values: &mut Vec<super::BarContent4Type>,
            value: super::BarContent4Type,
        ) -> Result<(), Error> {
            values.push(value);
            Ok(())
        }
        fn handle_content_4<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Vec<super::BarContent4Type>,
            fallback: Option<<super::BarContent4Type as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::BarContent4Type>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use BarTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if fallback.is_none() && values.is_empty() {
                    *self.state__ = S::Init__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = S::Content4(values, None, fallback);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_content_4(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_content_4(&mut values, data)?;
                    *self.state__ = S::Content4(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    if ret.is_continue_start_or_empty() {
                        *self.state__ = S::Content4(values, Some(deserializer), None);
                    } else {
                        *self.state__ = S::Content4(values, None, Some(deserializer));
                    }
                    Ok(ret)
                }
            }
        }
    }
    impl Default for BarTypeContentDeserializer {
        fn default() -> Self {
            Self {
                state__: Box::new(BarTypeContentDeserializerState::Init__),
            }
        }
    }
    impl<'de> Deserializer<'de, super::BarTypeContent> for BarTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::BarTypeContent> {
            let deserializer = Self::default();
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, BarTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::BarTypeContent> {
            use BarTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content4(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content_4(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Content4(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            <super::BarContent4Type as WithDeserializer>::init(helper, event)?;
                        match self.handle_content_4(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::BarTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct BarContent4TypeDeserializer {
        info: Option<String>,
        state__: Box<BarContent4TypeDeserializerState>,
    }
    #[derive(Debug)]
    enum BarContent4TypeDeserializerState {
        Init__,
        Info(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl BarContent4TypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: BarContent4TypeDeserializerState,
        ) -> Result<(), Error> {
            use BarContent4TypeDeserializerState as S;
            match state {
                S::Info(Some(deserializer)) => self.store_info(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_info(&mut self, value: String) -> Result<(), Error> {
            if self.info.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"info")))?;
            }
            self.info = Some(value);
            Ok(())
        }
        fn handle_info<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<BarContent4TypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use BarContent4TypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Info(None));
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
                    self.store_info(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Info(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::BarContent4Type> for BarContent4TypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::BarContent4Type> {
            let deserializer = Self {
                info: None,
                state__: Box::new(BarContent4TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, BarContent4TypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::BarContent4Type> {
            use BarContent4TypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Info(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_info(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Info(None);
                        event
                    }
                    (S::Info(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"info",
                            false,
                        )?;
                        match self.handle_info(helper, output, &mut fallback)? {
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
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::BarContent4Type, Error> {
            let state = replace(
                &mut *self.state__,
                BarContent4TypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::BarContent4Type {
                info: helper.finish_element("info", self.info)?,
            })
        }
    }
}
