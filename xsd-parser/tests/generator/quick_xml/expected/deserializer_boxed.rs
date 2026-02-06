use xsd_parser_types::{misc::Namespace, quick_xml::WithDeserializer};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
#[derive(Debug)]
pub struct MyChoiceType {
    pub content: MyChoiceTypeContent,
}
#[derive(Debug)]
pub enum MyChoiceTypeContent {
    Once(i32),
    Optional(Option<i32>),
    OnceSpecify(i32),
    TwiceOrMore(Vec<i32>),
}
impl WithDeserializer for MyChoiceType {
    type Deserializer = Box<quick_xml_deserialize::MyChoiceTypeDeserializer>;
}
impl WithDeserializer for MyChoiceTypeContent {
    type Deserializer = Box<quick_xml_deserialize::MyChoiceTypeContentDeserializer>;
}
#[derive(Debug)]
pub struct MySequenceType {
    pub once: i32,
    pub optional: Option<i32>,
    pub once_specify: i32,
    pub twice_or_more: Vec<i32>,
}
impl WithDeserializer for MySequenceType {
    type Deserializer = Box<quick_xml_deserialize::MySequenceTypeDeserializer>;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct MyChoiceTypeDeserializer {
        content: Option<super::MyChoiceTypeContent>,
        state__: Box<MyChoiceTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum MyChoiceTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::MyChoiceTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl MyChoiceTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Box::new(Self {
                content: None,
                state__: Box::new(MyChoiceTypeDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: MyChoiceTypeDeserializerState,
        ) -> Result<(), Error> {
            if let MyChoiceTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::MyChoiceTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::MyChoiceTypeContent>,
            fallback: &mut Option<MyChoiceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MyChoiceTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::MyChoiceType> for Box<MyChoiceTypeDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MyChoiceType> {
            helper.init_deserializer_from_start_event(
                event,
                MyChoiceTypeDeserializer::from_bytes_start,
            )
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MyChoiceType> {
            use MyChoiceTypeDeserializerState as S;
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
                            <super::MyChoiceTypeContent as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::MyChoiceType, Error> {
            let state = replace(&mut *self.state__, MyChoiceTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::MyChoiceType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct MyChoiceTypeContentDeserializer {
        state__: Box<MyChoiceTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum MyChoiceTypeContentDeserializerState {
        Init__,
        Once(
            Option<i32>,
            Option<<i32 as WithDeserializer>::Deserializer>,
            Option<<i32 as WithDeserializer>::Deserializer>,
        ),
        Optional(
            Option<i32>,
            Option<<i32 as WithDeserializer>::Deserializer>,
            Option<<i32 as WithDeserializer>::Deserializer>,
        ),
        OnceSpecify(
            Option<i32>,
            Option<<i32 as WithDeserializer>::Deserializer>,
            Option<<i32 as WithDeserializer>::Deserializer>,
        ),
        TwiceOrMore(
            Vec<i32>,
            Option<<i32 as WithDeserializer>::Deserializer>,
            Option<<i32 as WithDeserializer>::Deserializer>,
        ),
        Done__(super::MyChoiceTypeContent),
        Unknown__,
    }
    impl MyChoiceTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Once")
                ) {
                    let output = <i32 as WithDeserializer>::init(helper, event)?;
                    return self.handle_once(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Optional")
                ) {
                    let output = <i32 as WithDeserializer>::init(helper, event)?;
                    return self.handle_optional(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"OnceSpecify")
                ) {
                    let output = <i32 as WithDeserializer>::init(helper, event)?;
                    return self.handle_once_specify(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"TwiceOrMore")
                ) {
                    let output = <i32 as WithDeserializer>::init(helper, event)?;
                    return self.handle_twice_or_more(helper, Default::default(), None, output);
                }
            }
            *self.state__ = MyChoiceTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: MyChoiceTypeContentDeserializerState,
        ) -> Result<super::MyChoiceTypeContent, Error> {
            use MyChoiceTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Once(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        MyChoiceTypeContentDeserializer::store_once(&mut values, value)?;
                    }
                    Ok(super::MyChoiceTypeContent::Once(
                        helper.finish_element("Once", values)?,
                    ))
                }
                S::Optional(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        MyChoiceTypeContentDeserializer::store_optional(&mut values, value)?;
                    }
                    Ok(super::MyChoiceTypeContent::Optional(values))
                }
                S::OnceSpecify(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        MyChoiceTypeContentDeserializer::store_once_specify(&mut values, value)?;
                    }
                    Ok(super::MyChoiceTypeContent::OnceSpecify(
                        helper.finish_element("OnceSpecify", values)?,
                    ))
                }
                S::TwiceOrMore(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        MyChoiceTypeContentDeserializer::store_twice_or_more(&mut values, value)?;
                    }
                    Ok(super::MyChoiceTypeContent::TwiceOrMore(
                        helper.finish_vec(2usize, None, values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_once(values: &mut Option<i32>, value: i32) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Once")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_optional(values: &mut Option<i32>, value: i32) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Optional",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_once_specify(values: &mut Option<i32>, value: i32) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"OnceSpecify",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_twice_or_more(values: &mut Vec<i32>, value: i32) -> Result<(), Error> {
            values.push(value);
            Ok(())
        }
        fn handle_once<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<i32>,
            fallback: Option<<i32 as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, i32>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MyChoiceTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                MyChoiceTypeContentDeserializer::store_once(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    MyChoiceTypeContentDeserializer::store_once(&mut values, data)?;
                    let data = MyChoiceTypeContentDeserializer::finish_state(
                        helper,
                        S::Once(values, None, None),
                    )?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Once(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_optional<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<i32>,
            fallback: Option<<i32 as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, i32>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MyChoiceTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                MyChoiceTypeContentDeserializer::store_optional(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    MyChoiceTypeContentDeserializer::store_optional(&mut values, data)?;
                    let data = MyChoiceTypeContentDeserializer::finish_state(
                        helper,
                        S::Optional(values, None, None),
                    )?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Optional(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_once_specify<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<i32>,
            fallback: Option<<i32 as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, i32>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MyChoiceTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                MyChoiceTypeContentDeserializer::store_once_specify(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    MyChoiceTypeContentDeserializer::store_once_specify(&mut values, data)?;
                    let data = MyChoiceTypeContentDeserializer::finish_state(
                        helper,
                        S::OnceSpecify(values, None, None),
                    )?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::OnceSpecify(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_twice_or_more<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Vec<i32>,
            fallback: Option<<i32 as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, i32>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MyChoiceTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if fallback.is_none() && values.is_empty() {
                    *self.state__ = S::Init__;
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else if values.len() + usize::from(fallback.is_some()) < 2usize {
                    *self.state__ = S::TwiceOrMore(values, None, fallback);
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else {
                    *self.state__ = S::TwiceOrMore(values, None, fallback);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                MyChoiceTypeContentDeserializer::store_twice_or_more(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    MyChoiceTypeContentDeserializer::store_twice_or_more(&mut values, data)?;
                    *self.state__ = S::TwiceOrMore(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    if ret.is_continue_start_or_empty() {
                        *self.state__ = S::TwiceOrMore(values, Some(deserializer), None);
                    } else {
                        *self.state__ = S::TwiceOrMore(values, None, Some(deserializer));
                    }
                    Ok(ret)
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::MyChoiceTypeContent> for Box<MyChoiceTypeContentDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MyChoiceTypeContent> {
            let deserializer = Box::new(MyChoiceTypeContentDeserializer {
                state__: Box::new(MyChoiceTypeContentDeserializerState::Init__),
            });
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, MyChoiceTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::MyChoiceTypeContent> {
            use MyChoiceTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Once(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_once(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Optional(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_optional(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::OnceSpecify(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_once_specify(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::TwiceOrMore(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_twice_or_more(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(
                                MyChoiceTypeContentDeserializer::finish_state(helper, state)?,
                            ),
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
                        S::Once(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Once",
                            false,
                        )?;
                        match self.handle_once(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Optional(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Optional",
                            false,
                        )?;
                        match self.handle_optional(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::OnceSpecify(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"OnceSpecify",
                            false,
                        )?;
                        match self.handle_once_specify(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::TwiceOrMore(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"TwiceOrMore",
                            false,
                        )?;
                        match self.handle_twice_or_more(helper, values, fallback, output)? {
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
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::MyChoiceTypeContent, Error> {
            MyChoiceTypeContentDeserializer::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct MySequenceTypeDeserializer {
        once: Option<i32>,
        optional: Option<i32>,
        once_specify: Option<i32>,
        twice_or_more: Vec<i32>,
        state__: Box<MySequenceTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum MySequenceTypeDeserializerState {
        Init__,
        Once(Option<<i32 as WithDeserializer>::Deserializer>),
        Optional(Option<<i32 as WithDeserializer>::Deserializer>),
        OnceSpecify(Option<<i32 as WithDeserializer>::Deserializer>),
        TwiceOrMore(Option<<i32 as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl MySequenceTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Box::new(Self {
                once: None,
                optional: None,
                once_specify: None,
                twice_or_more: Vec::new(),
                state__: Box::new(MySequenceTypeDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: MySequenceTypeDeserializerState,
        ) -> Result<(), Error> {
            use MySequenceTypeDeserializerState as S;
            match state {
                S::Once(Some(deserializer)) => self.store_once(deserializer.finish(helper)?)?,
                S::Optional(Some(deserializer)) => {
                    self.store_optional(deserializer.finish(helper)?)?
                }
                S::OnceSpecify(Some(deserializer)) => {
                    self.store_once_specify(deserializer.finish(helper)?)?
                }
                S::TwiceOrMore(Some(deserializer)) => {
                    self.store_twice_or_more(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_once(&mut self, value: i32) -> Result<(), Error> {
            if self.once.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Once")))?;
            }
            self.once = Some(value);
            Ok(())
        }
        fn store_optional(&mut self, value: i32) -> Result<(), Error> {
            if self.optional.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Optional",
                )))?;
            }
            self.optional = Some(value);
            Ok(())
        }
        fn store_once_specify(&mut self, value: i32) -> Result<(), Error> {
            if self.once_specify.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"OnceSpecify",
                )))?;
            }
            self.once_specify = Some(value);
            Ok(())
        }
        fn store_twice_or_more(&mut self, value: i32) -> Result<(), Error> {
            self.twice_or_more.push(value);
            Ok(())
        }
        fn handle_once<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<MySequenceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MySequenceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Once(None));
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
                    self.store_once(data)?;
                    *self.state__ = S::Optional(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Once(Some(deserializer)));
                    *self.state__ = S::Optional(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_optional<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<MySequenceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MySequenceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Optional(None));
                *self.state__ = S::OnceSpecify(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_optional(data)?;
                    *self.state__ = S::OnceSpecify(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Optional(Some(deserializer)));
                    *self.state__ = S::OnceSpecify(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_once_specify<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<MySequenceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MySequenceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::OnceSpecify(None));
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
                    self.store_once_specify(data)?;
                    *self.state__ = S::TwiceOrMore(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::OnceSpecify(Some(deserializer)));
                    *self.state__ = S::TwiceOrMore(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_twice_or_more<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<MySequenceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MySequenceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else if self.twice_or_more.len() < 2usize {
                    fallback.get_or_insert(S::TwiceOrMore(None));
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else {
                    fallback.get_or_insert(S::TwiceOrMore(None));
                    *self.state__ = S::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_twice_or_more(data)?;
                    *self.state__ = S::TwiceOrMore(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::TwiceOrMore(Some(deserializer)));
                    *self.state__ = S::TwiceOrMore(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::MySequenceType> for Box<MySequenceTypeDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MySequenceType> {
            helper.init_deserializer_from_start_event(
                event,
                MySequenceTypeDeserializer::from_bytes_start,
            )
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MySequenceType> {
            use MySequenceTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Once(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_once(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Optional(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_optional(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::OnceSpecify(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_once_specify(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TwiceOrMore(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_twice_or_more(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Once(None);
                        event
                    }
                    (S::Once(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Once",
                            false,
                        )?;
                        match self.handle_once(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Optional(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Optional",
                            false,
                        )?;
                        match self.handle_optional(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::OnceSpecify(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"OnceSpecify",
                            false,
                        )?;
                        match self.handle_once_specify(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TwiceOrMore(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"TwiceOrMore",
                            false,
                        )?;
                        match self.handle_twice_or_more(helper, output, &mut fallback)? {
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
        ) -> Result<super::MySequenceType, Error> {
            let state = replace(
                &mut *self.state__,
                MySequenceTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::MySequenceType {
                once: helper.finish_element("Once", self.once)?,
                optional: self.optional,
                once_specify: helper.finish_element("OnceSpecify", self.once_specify)?,
                twice_or_more: helper.finish_vec(2usize, None, self.twice_or_more)?,
            })
        }
    }
}
