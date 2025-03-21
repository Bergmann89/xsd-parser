pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_DEFAULT: Namespace = Namespace::new_const(b"http://example.com");
use xsd_parser::{
    quick_xml::{deserialize_new::WithDeserializer, Error, WithSerializer},
    schema::Namespace,
};
pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub once: i32,
    pub optional: Option<i32>,
    pub once_specify: i32,
    pub twice_or_more: Vec<i32>,
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
            state: quick_xml_serialize::FooTypeSerializerState::Init__,
            name: name.unwrap_or("FooType"),
            is_root,
        })
    }
}
impl WithDeserializer for FooType {
    type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
}
pub mod quick_xml_serialize {
    use core::iter::Iterator;
    use xsd_parser::quick_xml::{
        BytesEnd, BytesStart, Error, Event, IterSerializer, WithSerializer,
    };
    #[derive(Debug)]
    pub struct FooTypeSerializer<'ser> {
        pub(super) value: &'ser super::FooType,
        pub(super) state: FooTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum FooTypeSerializerState<'ser> {
        Init__,
        Once(<i32 as WithSerializer>::Serializer<'ser>),
        Optional(IterSerializer<'ser, Option<i32>, i32>),
        OnceSpecify(<i32 as WithSerializer>::Serializer<'ser>),
        TwiceOrMore(IterSerializer<'ser, Vec<i32>, i32>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    FooTypeSerializerState::Init__ => {
                        self.state = FooTypeSerializerState::Once(WithSerializer::serializer(
                            &self.value.once,
                            Some("Once"),
                            false,
                        )?);
                        let bytes = BytesStart::new(self.name);
                        return Ok(Some(Event::Start(bytes)));
                    }
                    FooTypeSerializerState::Once(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = FooTypeSerializerState::Optional(IterSerializer::new(
                                &self.value.optional,
                                Some("Optional"),
                                false,
                            ))
                        }
                    },
                    FooTypeSerializerState::Optional(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state =
                                FooTypeSerializerState::OnceSpecify(WithSerializer::serializer(
                                    &self.value.once_specify,
                                    Some("OnceSpecify"),
                                    false,
                                )?)
                        }
                    },
                    FooTypeSerializerState::OnceSpecify(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = FooTypeSerializerState::TwiceOrMore(IterSerializer::new(
                                &self.value.twice_or_more,
                                Some("TwiceOrMore"),
                                false,
                            ))
                        }
                    },
                    FooTypeSerializerState::TwiceOrMore(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = FooTypeSerializerState::End__,
                    },
                    FooTypeSerializerState::End__ => {
                        self.state = FooTypeSerializerState::Done__;
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
                    self.state = FooTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        deserialize_new::{
            DeserializeReader, Deserializer, DeserializerArtifact, DeserializerOutput,
            DeserializerResult, ElementHandlerOutput, WithDeserializer,
        },
        filter_xmlns_attributes, BytesStart, Error, ErrorKind, Event, RawByteStr,
    };
    #[derive(Debug)]
    pub struct FooTypeDeserializer {
        once: Option<i32>,
        optional: Option<i32>,
        once_specify: Option<i32>,
        twice_or_more: Vec<i32>,
        state: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Init__,
        Once(Option<<i32 as WithDeserializer>::Deserializer>),
        Optional(Option<<i32 as WithDeserializer>::Deserializer>),
        OnceSpecify(Option<<i32 as WithDeserializer>::Deserializer>),
        TwiceOrMore(Option<<i32 as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl FooTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                once: None,
                optional: None,
                once_specify: None,
                twice_or_more: Vec::new(),
                state: Box::new(FooTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: FooTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use FooTypeDeserializerState as S;
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
                fallback.get_or_insert(FooTypeDeserializerState::Once(None));
                *self.state = FooTypeDeserializerState::Optional(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_once(data)?;
                    *self.state = FooTypeDeserializerState::Once(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    if let Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) = event
                    {
                        fallback.get_or_insert(FooTypeDeserializerState::Once(Some(deserializer)));
                        *self.state = FooTypeDeserializerState::Optional(None);
                        ElementHandlerOutput::continue_(event, allow_any)
                    } else {
                        *self.state = FooTypeDeserializerState::Once(Some(deserializer));
                        ElementHandlerOutput::break_(event, allow_any)
                    }
                }
            })
        }
        fn handle_optional<'de, R>(
            &mut self,
            reader: &R,
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
                fallback.get_or_insert(FooTypeDeserializerState::Optional(None));
                *self.state = FooTypeDeserializerState::OnceSpecify(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_optional(data)?;
                    *self.state = FooTypeDeserializerState::Optional(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    if let Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) = event
                    {
                        fallback
                            .get_or_insert(FooTypeDeserializerState::Optional(Some(deserializer)));
                        *self.state = FooTypeDeserializerState::OnceSpecify(None);
                        ElementHandlerOutput::continue_(event, allow_any)
                    } else {
                        *self.state = FooTypeDeserializerState::Optional(Some(deserializer));
                        ElementHandlerOutput::break_(event, allow_any)
                    }
                }
            })
        }
        fn handle_once_specify<'de, R>(
            &mut self,
            reader: &R,
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
                fallback.get_or_insert(FooTypeDeserializerState::OnceSpecify(None));
                *self.state = FooTypeDeserializerState::TwiceOrMore(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_once_specify(data)?;
                    *self.state = FooTypeDeserializerState::OnceSpecify(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    if let Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) = event
                    {
                        fallback.get_or_insert(FooTypeDeserializerState::OnceSpecify(Some(
                            deserializer,
                        )));
                        *self.state = FooTypeDeserializerState::TwiceOrMore(None);
                        ElementHandlerOutput::continue_(event, allow_any)
                    } else {
                        *self.state = FooTypeDeserializerState::OnceSpecify(Some(deserializer));
                        ElementHandlerOutput::break_(event, allow_any)
                    }
                }
            })
        }
        fn handle_twice_or_more<'de, R>(
            &mut self,
            reader: &R,
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
                fallback.get_or_insert(FooTypeDeserializerState::TwiceOrMore(None));
                *self.state = FooTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_twice_or_more(data)?;
                    *self.state = FooTypeDeserializerState::TwiceOrMore(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    if let Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) = event
                    {
                        fallback.get_or_insert(FooTypeDeserializerState::TwiceOrMore(Some(
                            deserializer,
                        )));
                        *self.state = FooTypeDeserializerState::Done__;
                        ElementHandlerOutput::continue_(event, allow_any)
                    } else {
                        *self.state = FooTypeDeserializerState::TwiceOrMore(Some(deserializer));
                        ElementHandlerOutput::break_(event, allow_any)
                    }
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::FooType> for FooTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::FooType>
        where
            R: DeserializeReader,
        {
            dbg!("INIT", &event);
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
            dbg!("NEXT", &event, &self);
            use FooTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Once(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_once(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break {
                                event, allow_any, ..
                            } => break (event, allow_any),
                        }
                    }
                    (S::Optional(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_optional(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break {
                                event, allow_any, ..
                            } => break (event, allow_any),
                        }
                    }
                    (S::OnceSpecify(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_once_specify(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break {
                                event, allow_any, ..
                            } => break (event, allow_any),
                        }
                    }
                    (S::TwiceOrMore(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_twice_or_more(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break {
                                event, allow_any, ..
                            } => break (event, allow_any),
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = FooTypeDeserializerState::Once(None);
                        event
                    }
                    (S::Once(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DEFAULT), b"Once") {
                            let output =
                                <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_once(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break {
                                    event, allow_any, ..
                                } => break (event, allow_any),
                            }
                        } else {
                            *self.state = S::Optional(None);
                            event
                        }
                    }
                    (S::Optional(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_DEFAULT),
                            b"Optional",
                        ) {
                            let output =
                                <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_optional(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break {
                                    event, allow_any, ..
                                } => break (event, allow_any),
                            }
                        } else {
                            *self.state = S::OnceSpecify(None);
                            event
                        }
                    }
                    (S::OnceSpecify(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_DEFAULT),
                            b"OnceSpecify",
                        ) {
                            let output =
                                <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_once_specify(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break {
                                    event, allow_any, ..
                                } => break (event, allow_any),
                            }
                        } else {
                            *self.state = S::TwiceOrMore(None);
                            event
                        }
                    }
                    (S::TwiceOrMore(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_DEFAULT),
                            b"TwiceOrMore",
                        ) {
                            let output =
                                <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_twice_or_more(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break {
                                    event, allow_any, ..
                                } => break (event, allow_any),
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => break (Some(event), allow_any_element),
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (Some(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::FooType, Error>
        where
            R: DeserializeReader,
        {
            dbg!("FINISH", &self);
            let state = replace(&mut *self.state, FooTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::FooType {
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
