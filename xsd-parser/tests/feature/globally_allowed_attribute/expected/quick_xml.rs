use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub content: FooTypeContent,
}
#[derive(Debug)]
pub enum FooTypeContent {
    Once(i32),
    Optional(Option<i32>),
    OnceSpecify(i32),
    TwiceOrMore(Vec<i32>),
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
impl WithSerializer for FooTypeContent {
    type Serializer<'x> = quick_xml_serialize::FooTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::FooTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::FooTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for FooType {
    type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
}
impl WithDeserializer for FooTypeContent {
    type Deserializer = quick_xml_deserialize::FooTypeContentDeserializer;
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
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(FooTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = FooTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = FooTypeDeserializerState::Content__(deserializer);
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
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
                            <super::FooTypeContent as WithDeserializer>::Deserializer::init(
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::FooType, Error> {
            let state = replace(&mut *self.state__, FooTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::FooType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
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
        Once(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Optional(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        OnceSpecify(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        TwiceOrMore(Vec<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Done__(super::FooTypeContent),
        Unknown__,
    }
    impl FooTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
            fallback: &mut Option<FooTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Once")
                ) {
                    let output = <i32 as WithDeserializer>::Deserializer::init(helper, event)?;
                    return self.handle_once(helper, Default::default(), output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Optional")
                ) {
                    let output = <i32 as WithDeserializer>::Deserializer::init(helper, event)?;
                    return self.handle_optional(
                        helper,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"OnceSpecify")
                ) {
                    let output = <i32 as WithDeserializer>::Deserializer::init(helper, event)?;
                    return self.handle_once_specify(
                        helper,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"TwiceOrMore")
                ) {
                    let output = <i32 as WithDeserializer>::Deserializer::init(helper, event)?;
                    return self.handle_twice_or_more(
                        helper,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
            }
            *self.state__ = fallback
                .take()
                .unwrap_or(FooTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: FooTypeContentDeserializerState,
        ) -> Result<super::FooTypeContent, Error> {
            use FooTypeContentDeserializerState as S;
            match state {
                S::Unknown__ => unreachable!(),
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Once(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_once(&mut values, value)?;
                    }
                    Ok(super::FooTypeContent::Once(
                        values.ok_or_else(|| ErrorKind::MissingElement("Once".into()))?,
                    ))
                }
                S::Optional(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_optional(&mut values, value)?;
                    }
                    Ok(super::FooTypeContent::Optional(values))
                }
                S::OnceSpecify(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_once_specify(&mut values, value)?;
                    }
                    Ok(super::FooTypeContent::OnceSpecify(values.ok_or_else(
                        || ErrorKind::MissingElement("OnceSpecify".into()),
                    )?))
                }
                S::TwiceOrMore(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_twice_or_more(&mut values, value)?;
                    }
                    Ok(super::FooTypeContent::TwiceOrMore(values))
                }
                S::Done__(data) => Ok(data),
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
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<FooTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = match fallback.take() {
                    None if values.is_none() => {
                        *self.state__ = FooTypeContentDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => FooTypeContentDeserializerState::Once(values, None),
                    Some(FooTypeContentDeserializerState::Once(_, Some(deserializer))) => {
                        FooTypeContentDeserializerState::Once(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(FooTypeContentDeserializerState::Once(_, Some(deserializer))) => {
                    let data = deserializer.finish(helper)?;
                    Self::store_once(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_once(&mut values, data)?;
                    let data = Self::finish_state(
                        helper,
                        FooTypeContentDeserializerState::Once(values, None),
                    )?;
                    *self.state__ = FooTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ =
                        FooTypeContentDeserializerState::Once(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_optional<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<FooTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = match fallback.take() {
                    None => FooTypeContentDeserializerState::Optional(values, None),
                    Some(FooTypeContentDeserializerState::Optional(_, Some(deserializer))) => {
                        FooTypeContentDeserializerState::Optional(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(FooTypeContentDeserializerState::Optional(_, Some(deserializer))) => {
                    let data = deserializer.finish(helper)?;
                    Self::store_optional(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_optional(&mut values, data)?;
                    let data = Self::finish_state(
                        helper,
                        FooTypeContentDeserializerState::Optional(values, None),
                    )?;
                    *self.state__ = FooTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ =
                        FooTypeContentDeserializerState::Optional(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_once_specify<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<FooTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = match fallback.take() {
                    None if values.is_none() => {
                        *self.state__ = FooTypeContentDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => FooTypeContentDeserializerState::OnceSpecify(values, None),
                    Some(FooTypeContentDeserializerState::OnceSpecify(_, Some(deserializer))) => {
                        FooTypeContentDeserializerState::OnceSpecify(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(FooTypeContentDeserializerState::OnceSpecify(_, Some(deserializer))) => {
                    let data = deserializer.finish(helper)?;
                    Self::store_once_specify(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_once_specify(&mut values, data)?;
                    let data = Self::finish_state(
                        helper,
                        FooTypeContentDeserializerState::OnceSpecify(values, None),
                    )?;
                    *self.state__ = FooTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ =
                        FooTypeContentDeserializerState::OnceSpecify(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_twice_or_more<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Vec<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<FooTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = match fallback.take() {
                    None if values.is_empty() => {
                        *self.state__ = FooTypeContentDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => FooTypeContentDeserializerState::TwiceOrMore(values, None),
                    Some(FooTypeContentDeserializerState::TwiceOrMore(_, Some(deserializer))) => {
                        FooTypeContentDeserializerState::TwiceOrMore(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(FooTypeContentDeserializerState::TwiceOrMore(_, Some(deserializer))) => {
                    let data = deserializer.finish(helper)?;
                    Self::store_twice_or_more(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_twice_or_more(&mut values, data)?;
                    *self.state__ = FooTypeContentDeserializerState::TwiceOrMore(values, None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ = FooTypeContentDeserializerState::TwiceOrMore(
                                values,
                                Some(deserializer),
                            );
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(FooTypeContentDeserializerState::TwiceOrMore(
                                Default::default(),
                                Some(deserializer),
                            ));
                            *self.state__ =
                                FooTypeContentDeserializerState::TwiceOrMore(values, None);
                        }
                    }
                    ret
                }
            })
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
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Once(values, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_once(helper, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Optional(values, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_optional(helper, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::OnceSpecify(values, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_once_specify(helper, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::TwiceOrMore(values, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_twice_or_more(helper, values, output, &mut fallback)? {
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
                    (S::Init__, event) => match self.find_suitable(helper, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::Once(values, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Once",
                            false,
                        )?;
                        match self.handle_once(helper, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Optional(values, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Optional",
                            false,
                        )?;
                        match self.handle_optional(helper, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::OnceSpecify(values, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"OnceSpecify",
                            false,
                        )?;
                        match self.handle_once_specify(helper, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::TwiceOrMore(values, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"TwiceOrMore",
                            false,
                        )?;
                        match self.handle_twice_or_more(helper, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state__ = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
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
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, Error, Event, IterSerializer, SerializeHelper, Serializer,
        WithSerializer,
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
        Content__(<super::FooTypeContent as WithSerializer>::Serializer<'ser>),
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
                        *self.state = FooTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
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
                    FooTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
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
    #[derive(Debug)]
    pub struct FooTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::FooTypeContent,
        pub(super) state: Box<FooTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum FooTypeContentSerializerState<'ser> {
        Init__,
        Once(<i32 as WithSerializer>::Serializer<'ser>),
        Optional(IterSerializer<'ser, Option<&'ser i32>, i32>),
        OnceSpecify(<i32 as WithSerializer>::Serializer<'ser>),
        TwiceOrMore(IterSerializer<'ser, &'ser [i32], i32>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FooTypeContentSerializerState::Init__ => match self.value {
                        super::FooTypeContent::Once(x) => {
                            *self.state = FooTypeContentSerializerState::Once(
                                WithSerializer::serializer(x, Some("tns:Once"), false)?,
                            )
                        }
                        super::FooTypeContent::Optional(x) => {
                            *self.state = FooTypeContentSerializerState::Optional(
                                IterSerializer::new(x.as_ref(), Some("tns:Optional"), false),
                            )
                        }
                        super::FooTypeContent::OnceSpecify(x) => {
                            *self.state = FooTypeContentSerializerState::OnceSpecify(
                                WithSerializer::serializer(x, Some("tns:OnceSpecify"), false)?,
                            )
                        }
                        super::FooTypeContent::TwiceOrMore(x) => {
                            *self.state = FooTypeContentSerializerState::TwiceOrMore(
                                IterSerializer::new(&x[..], Some("tns:TwiceOrMore"), false),
                            )
                        }
                    },
                    FooTypeContentSerializerState::Once(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooTypeContentSerializerState::Done__,
                    },
                    FooTypeContentSerializerState::Optional(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = FooTypeContentSerializerState::Done__,
                        }
                    }
                    FooTypeContentSerializerState::OnceSpecify(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = FooTypeContentSerializerState::Done__,
                        }
                    }
                    FooTypeContentSerializerState::TwiceOrMore(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = FooTypeContentSerializerState::Done__,
                        }
                    }
                    FooTypeContentSerializerState::Done__ => return Ok(None),
                    FooTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for FooTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = FooTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
