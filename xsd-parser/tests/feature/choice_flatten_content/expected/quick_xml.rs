use xsd_parser_types::{
    misc::Namespace,
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub type Foo = FooType;
#[derive(Debug)]
pub enum FooType {
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
impl WithDeserializer for FooType {
    type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
        ErrorKind, Event, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct FooTypeDeserializer {
        state__: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum FooTypeDeserializerState {
        Init__,
        Once(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Optional(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        OnceSpecify(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        TwiceOrMore(Vec<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Done__(super::FooType),
        Unknown__,
    }
    impl FooTypeDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<FooTypeDeserializerState>,
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
            *self.state__ = fallback.take().unwrap_or(FooTypeDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                state__: Box::new(FooTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            reader: &R,
            state: FooTypeDeserializerState,
        ) -> Result<super::FooType, Error>
        where
            R: DeserializeReader,
        {
            use FooTypeDeserializerState as S;
            match state {
                S::Unknown__ => unreachable!(),
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Once(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_once(&mut values, value)?;
                    }
                    Ok(super::FooType::Once(
                        values.ok_or_else(|| ErrorKind::MissingElement("Once".into()))?,
                    ))
                }
                S::Optional(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_optional(&mut values, value)?;
                    }
                    Ok(super::FooType::Optional(values))
                }
                S::OnceSpecify(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_once_specify(&mut values, value)?;
                    }
                    Ok(super::FooType::OnceSpecify(values.ok_or_else(|| {
                        ErrorKind::MissingElement("OnceSpecify".into())
                    })?))
                }
                S::TwiceOrMore(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_twice_or_more(&mut values, value)?;
                    }
                    Ok(super::FooType::TwiceOrMore(values))
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
            fallback: &mut Option<FooTypeDeserializerState>,
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
                        *self.state__ = FooTypeDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => FooTypeDeserializerState::Once(values, None),
                    Some(FooTypeDeserializerState::Once(_, Some(deserializer))) => {
                        FooTypeDeserializerState::Once(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(FooTypeDeserializerState::Once(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_once(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_once(&mut values, data)?;
                    *self.state__ = FooTypeDeserializerState::Once(values, None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = FooTypeDeserializerState::Once(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_optional<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<FooTypeDeserializerState>,
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
                    None => FooTypeDeserializerState::Optional(values, None),
                    Some(FooTypeDeserializerState::Optional(_, Some(deserializer))) => {
                        FooTypeDeserializerState::Optional(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(FooTypeDeserializerState::Optional(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_optional(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_optional(&mut values, data)?;
                    *self.state__ = FooTypeDeserializerState::Optional(values, None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = FooTypeDeserializerState::Optional(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_once_specify<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<FooTypeDeserializerState>,
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
                        *self.state__ = FooTypeDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => FooTypeDeserializerState::OnceSpecify(values, None),
                    Some(FooTypeDeserializerState::OnceSpecify(_, Some(deserializer))) => {
                        FooTypeDeserializerState::OnceSpecify(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(FooTypeDeserializerState::OnceSpecify(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_once_specify(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_once_specify(&mut values, data)?;
                    *self.state__ = FooTypeDeserializerState::OnceSpecify(values, None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ =
                        FooTypeDeserializerState::OnceSpecify(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_twice_or_more<'de, R>(
            &mut self,
            reader: &R,
            mut values: Vec<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<FooTypeDeserializerState>,
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
                        *self.state__ = FooTypeDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => FooTypeDeserializerState::TwiceOrMore(values, None),
                    Some(FooTypeDeserializerState::TwiceOrMore(_, Some(deserializer))) => {
                        FooTypeDeserializerState::TwiceOrMore(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(FooTypeDeserializerState::TwiceOrMore(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_twice_or_more(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_twice_or_more(&mut values, data)?;
                    *self.state__ = FooTypeDeserializerState::TwiceOrMore(values, None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let can_have_more = values.len().saturating_add(1) < 100usize;
                    let ret = if can_have_more {
                        ElementHandlerOutput::from_event(event, allow_any)
                    } else {
                        ElementHandlerOutput::from_event_end(event, allow_any)
                    };
                    match (can_have_more, &ret) {
                        (true, ElementHandlerOutput::Continue { .. }) => {
                            fallback.get_or_insert(FooTypeDeserializerState::TwiceOrMore(
                                Default::default(),
                                Some(deserializer),
                            ));
                            *self.state__ = FooTypeDeserializerState::TwiceOrMore(values, None);
                        }
                        (false, _) | (_, ElementHandlerOutput::Break { .. }) => {
                            *self.state__ =
                                FooTypeDeserializerState::TwiceOrMore(values, Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::FooType> for FooTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::FooType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooType>
        where
            R: DeserializeReader,
        {
            use FooTypeDeserializerState as S;
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
                    (state, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::None,
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
        fn finish<R>(self, reader: &R) -> Result<super::FooType, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state__)
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, Error, Event, IterSerializer, WithSerializer,
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
        Once(<i32 as WithSerializer>::Serializer<'ser>),
        Optional(IterSerializer<'ser, Option<&'ser i32>, i32>),
        OnceSpecify(<i32 as WithSerializer>::Serializer<'ser>),
        TwiceOrMore(IterSerializer<'ser, &'ser [i32], i32>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FooTypeSerializerState::Init__ => {
                        match self.value {
                            super::FooType::Once(x) => {
                                *self.state = FooTypeSerializerState::Once(
                                    WithSerializer::serializer(x, Some("tns:Once"), self.is_root)?,
                                )
                            }
                            super::FooType::Optional(x) => {
                                *self.state = FooTypeSerializerState::Optional(IterSerializer::new(
                                    x.as_ref(),
                                    Some("tns:Optional"),
                                    self.is_root,
                                ))
                            }
                            super::FooType::OnceSpecify(x) => {
                                *self.state =
                                    FooTypeSerializerState::OnceSpecify(WithSerializer::serializer(
                                        x,
                                        Some("tns:OnceSpecify"),
                                        self.is_root,
                                    )?)
                            }
                            super::FooType::TwiceOrMore(x) => {
                                *self.state =
                                    FooTypeSerializerState::TwiceOrMore(IterSerializer::new(
                                        &x[..],
                                        Some("tns:TwiceOrMore"),
                                        self.is_root,
                                    ))
                            }
                        }
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    FooTypeSerializerState::Once(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooTypeSerializerState::End__,
                    },
                    FooTypeSerializerState::Optional(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooTypeSerializerState::End__,
                    },
                    FooTypeSerializerState::OnceSpecify(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooTypeSerializerState::End__,
                    },
                    FooTypeSerializerState::TwiceOrMore(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooTypeSerializerState::End__,
                    },
                    FooTypeSerializerState::End__ => {
                        *self.state = FooTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    FooTypeSerializerState::Done__ => return Ok(None),
                    FooTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for FooTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
