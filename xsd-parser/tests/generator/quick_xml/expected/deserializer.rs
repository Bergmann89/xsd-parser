use xsd_parser_types::{misc::Namespace, quick_xml::WithDeserializer};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
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
    type Deserializer = quick_xml_deserialize::MyChoiceTypeDeserializer;
}
impl WithDeserializer for MyChoiceTypeContent {
    type Deserializer = quick_xml_deserialize::MyChoiceTypeContentDeserializer;
}
#[derive(Debug)]
pub struct MySequenceType {
    pub once: i32,
    pub optional: Option<i32>,
    pub once_specify: i32,
    pub twice_or_more: Vec<i32>,
}
impl WithDeserializer for MySequenceType {
    type Deserializer = quick_xml_deserialize::MySequenceTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
        ErrorKind, Event, RawByteStr, WithDeserializer,
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
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                content: None,
                state__: Box::new(MyChoiceTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: MyChoiceTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let MyChoiceTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
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
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::MyChoiceTypeContent>,
            fallback: &mut Option<MyChoiceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback
                    .take()
                    .unwrap_or(MyChoiceTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = MyChoiceTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = MyChoiceTypeDeserializerState::Content__(deserializer);
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::MyChoiceType> for MyChoiceTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::MyChoiceType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MyChoiceType>
        where
            R: DeserializeReader,
        {
            use MyChoiceTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::MyChoiceTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
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
        fn finish<R>(mut self, reader: &R) -> Result<super::MyChoiceType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state__, MyChoiceTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::MyChoiceType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
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
        Once(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Optional(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        OnceSpecify(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        TwiceOrMore(Vec<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Done__(super::MyChoiceTypeContent),
        Unknown__,
    }
    impl MyChoiceTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<MyChoiceTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Once")
                ) {
                    let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_once(reader, Default::default(), output, &mut *fallback);
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Optional")
                ) {
                    let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_optional(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"OnceSpecify")
                ) {
                    let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_once_specify(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"TwiceOrMore")
                ) {
                    let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_twice_or_more(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
            }
            *self.state__ = fallback
                .take()
                .unwrap_or(MyChoiceTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: MyChoiceTypeContentDeserializerState,
        ) -> Result<super::MyChoiceTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use MyChoiceTypeContentDeserializerState as S;
            match state {
                S::Unknown__ => unreachable!(),
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Once(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_once(&mut values, value)?;
                    }
                    Ok(super::MyChoiceTypeContent::Once(
                        values.ok_or_else(|| ErrorKind::MissingElement("Once".into()))?,
                    ))
                }
                S::Optional(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_optional(&mut values, value)?;
                    }
                    Ok(super::MyChoiceTypeContent::Optional(values))
                }
                S::OnceSpecify(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_once_specify(&mut values, value)?;
                    }
                    Ok(super::MyChoiceTypeContent::OnceSpecify(values.ok_or_else(
                        || ErrorKind::MissingElement("OnceSpecify".into()),
                    )?))
                }
                S::TwiceOrMore(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_twice_or_more(&mut values, value)?;
                    }
                    Ok(super::MyChoiceTypeContent::TwiceOrMore(values))
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
        fn handle_once<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<MyChoiceTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = match fallback.take() {
                    None if values.is_none() => {
                        *self.state__ = MyChoiceTypeContentDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => MyChoiceTypeContentDeserializerState::Once(values, None),
                    Some(MyChoiceTypeContentDeserializerState::Once(_, Some(deserializer))) => {
                        MyChoiceTypeContentDeserializerState::Once(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(MyChoiceTypeContentDeserializerState::Once(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_once(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_once(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        MyChoiceTypeContentDeserializerState::Once(values, None),
                    )?;
                    *self.state__ = MyChoiceTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ =
                        MyChoiceTypeContentDeserializerState::Once(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_optional<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<MyChoiceTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = match fallback.take() {
                    None => MyChoiceTypeContentDeserializerState::Optional(values, None),
                    Some(MyChoiceTypeContentDeserializerState::Optional(_, Some(deserializer))) => {
                        MyChoiceTypeContentDeserializerState::Optional(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(MyChoiceTypeContentDeserializerState::Optional(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_optional(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_optional(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        MyChoiceTypeContentDeserializerState::Optional(values, None),
                    )?;
                    *self.state__ = MyChoiceTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ =
                        MyChoiceTypeContentDeserializerState::Optional(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_once_specify<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<MyChoiceTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = match fallback.take() {
                    None if values.is_none() => {
                        *self.state__ = MyChoiceTypeContentDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => MyChoiceTypeContentDeserializerState::OnceSpecify(values, None),
                    Some(MyChoiceTypeContentDeserializerState::OnceSpecify(
                        _,
                        Some(deserializer),
                    )) => MyChoiceTypeContentDeserializerState::OnceSpecify(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(MyChoiceTypeContentDeserializerState::OnceSpecify(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_once_specify(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_once_specify(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        MyChoiceTypeContentDeserializerState::OnceSpecify(values, None),
                    )?;
                    *self.state__ = MyChoiceTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = MyChoiceTypeContentDeserializerState::OnceSpecify(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_twice_or_more<'de, R>(
            &mut self,
            reader: &R,
            mut values: Vec<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<MyChoiceTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = match fallback.take() {
                    None if values.is_empty() => {
                        *self.state__ = MyChoiceTypeContentDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => MyChoiceTypeContentDeserializerState::TwiceOrMore(values, None),
                    Some(MyChoiceTypeContentDeserializerState::TwiceOrMore(
                        _,
                        Some(deserializer),
                    )) => MyChoiceTypeContentDeserializerState::TwiceOrMore(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(MyChoiceTypeContentDeserializerState::TwiceOrMore(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_twice_or_more(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_twice_or_more(&mut values, data)?;
                    *self.state__ = MyChoiceTypeContentDeserializerState::TwiceOrMore(values, None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ = MyChoiceTypeContentDeserializerState::TwiceOrMore(
                                values,
                                Some(deserializer),
                            );
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                MyChoiceTypeContentDeserializerState::TwiceOrMore(
                                    Default::default(),
                                    Some(deserializer),
                                ),
                            );
                            *self.state__ =
                                MyChoiceTypeContentDeserializerState::TwiceOrMore(values, None);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::MyChoiceTypeContent> for MyChoiceTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MyChoiceTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state__: Box::new(MyChoiceTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
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
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MyChoiceTypeContent>
        where
            R: DeserializeReader,
        {
            use MyChoiceTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Once(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_once(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Optional(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_optional(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::OnceSpecify(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_once_specify(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::TwiceOrMore(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_twice_or_more(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::Once(values, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Once",
                            false,
                        )?;
                        match self.handle_once(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Optional(values, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Optional",
                            false,
                        )?;
                        match self.handle_optional(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::OnceSpecify(values, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"OnceSpecify",
                            false,
                        )?;
                        match self.handle_once_specify(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::TwiceOrMore(values, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"TwiceOrMore",
                            false,
                        )?;
                        match self.handle_twice_or_more(reader, values, output, &mut fallback)? {
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
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::MyChoiceTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state__)
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
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                once: None,
                optional: None,
                once_specify: None,
                twice_or_more: Vec::new(),
                state__: Box::new(MySequenceTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: MySequenceTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use MySequenceTypeDeserializerState as S;
            match state {
                S::Once(Some(deserializer)) => self.store_once(deserializer.finish(reader)?)?,
                S::Optional(Some(deserializer)) => {
                    self.store_optional(deserializer.finish(reader)?)?
                }
                S::OnceSpecify(Some(deserializer)) => {
                    self.store_once_specify(deserializer.finish(reader)?)?
                }
                S::TwiceOrMore(Some(deserializer)) => {
                    self.store_twice_or_more(deserializer.finish(reader)?)?
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
        fn handle_once<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<MySequenceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.once.is_some() {
                    fallback.get_or_insert(MySequenceTypeDeserializerState::Once(None));
                    *self.state__ = MySequenceTypeDeserializerState::Optional(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = MySequenceTypeDeserializerState::Once(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_once(data)?;
                    *self.state__ = MySequenceTypeDeserializerState::Optional(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(MySequenceTypeDeserializerState::Once(Some(
                                deserializer,
                            )));
                            *self.state__ = MySequenceTypeDeserializerState::Optional(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                MySequenceTypeDeserializerState::Once(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_optional<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<MySequenceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(MySequenceTypeDeserializerState::Optional(None));
                *self.state__ = MySequenceTypeDeserializerState::OnceSpecify(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_optional(data)?;
                    *self.state__ = MySequenceTypeDeserializerState::OnceSpecify(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(MySequenceTypeDeserializerState::Optional(
                                Some(deserializer),
                            ));
                            *self.state__ = MySequenceTypeDeserializerState::OnceSpecify(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                MySequenceTypeDeserializerState::Optional(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_once_specify<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<MySequenceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.once_specify.is_some() {
                    fallback.get_or_insert(MySequenceTypeDeserializerState::OnceSpecify(None));
                    *self.state__ = MySequenceTypeDeserializerState::TwiceOrMore(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = MySequenceTypeDeserializerState::OnceSpecify(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_once_specify(data)?;
                    *self.state__ = MySequenceTypeDeserializerState::TwiceOrMore(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(MySequenceTypeDeserializerState::OnceSpecify(
                                Some(deserializer),
                            ));
                            *self.state__ = MySequenceTypeDeserializerState::TwiceOrMore(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                MySequenceTypeDeserializerState::OnceSpecify(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_twice_or_more<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<MySequenceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.twice_or_more.len() < 2usize {
                    *self.state__ = MySequenceTypeDeserializerState::TwiceOrMore(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(MySequenceTypeDeserializerState::TwiceOrMore(None));
                    *self.state__ = MySequenceTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_twice_or_more(data)?;
                    *self.state__ = MySequenceTypeDeserializerState::TwiceOrMore(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(MySequenceTypeDeserializerState::TwiceOrMore(
                                Some(deserializer),
                            ));
                            *self.state__ = MySequenceTypeDeserializerState::TwiceOrMore(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                MySequenceTypeDeserializerState::TwiceOrMore(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::MySequenceType> for MySequenceTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::MySequenceType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MySequenceType>
        where
            R: DeserializeReader,
        {
            use MySequenceTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Once(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_once(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_optional(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_once_specify(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_twice_or_more(reader, output, &mut fallback)? {
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
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = MySequenceTypeDeserializerState::Once(None);
                        event
                    }
                    (S::Once(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Once",
                            false,
                        )?;
                        match self.handle_once(reader, output, &mut fallback)? {
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
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Optional",
                            false,
                        )?;
                        match self.handle_optional(reader, output, &mut fallback)? {
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
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"OnceSpecify",
                            false,
                        )?;
                        match self.handle_once_specify(reader, output, &mut fallback)? {
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
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"TwiceOrMore",
                            false,
                        )?;
                        match self.handle_twice_or_more(reader, output, &mut fallback)? {
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
        fn finish<R>(mut self, reader: &R) -> Result<super::MySequenceType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state__,
                MySequenceTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::MySequenceType {
                once: self
                    .once
                    .ok_or_else(|| ErrorKind::MissingElement("Once".into()))?,
                optional: self.optional,
                once_specify: self
                    .once_specify
                    .ok_or_else(|| ErrorKind::MissingElement("OnceSpecify".into()))?,
                twice_or_more: self.twice_or_more,
            })
        }
    }
}
