pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
use xsd_parser::{
    quick_xml::{deserialize_new::WithDeserializer, Error, WithSerializer},
    schema::Namespace,
};
pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub min: i32,
    pub max: i32,
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
            name: name.unwrap_or("tns:FooType"),
            is_root,
        })
    }
}
impl WithDeserializer for FooType {
    type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
}
pub mod quick_xml_serialize {
    use core::iter::Iterator;
    use xsd_parser::quick_xml::{BytesEnd, BytesStart, Error, Event, WithSerializer};
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
        Min(<i32 as WithSerializer>::Serializer<'ser>),
        Max(<i32 as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    FooTypeSerializerState::Init__ => {
                        self.state = FooTypeSerializerState::Min(WithSerializer::serializer(
                            &self.value.min,
                            Some("tns:Min"),
                            false,
                        )?);
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    FooTypeSerializerState::Min(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = FooTypeSerializerState::Max(WithSerializer::serializer(
                                &self.value.max,
                                Some("tns:Max"),
                                false,
                            )?)
                        }
                    },
                    FooTypeSerializerState::Max(x) => match x.next().transpose()? {
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
            DeserializeReader, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult, ElementHandlerOutput, WithDeserializer,
        },
        filter_xmlns_attributes, BytesStart, Error, ErrorKind, Event, RawByteStr,
    };
    #[derive(Debug)]
    pub struct FooTypeDeserializer {
        min: Option<i32>,
        max: Option<i32>,
        state: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Init__,
        Min(Option<<i32 as WithDeserializer>::Deserializer>),
        Max(Option<<i32 as WithDeserializer>::Deserializer>),
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
                min: None,
                max: None,
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
                S::Min(Some(deserializer)) => self.store_min(deserializer.finish(reader)?)?,
                S::Max(Some(deserializer)) => self.store_max(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_min(&mut self, value: i32) -> Result<(), Error> {
            if self.min.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Min")))?;
            }
            self.min = Some(value);
            Ok(())
        }
        fn store_max(&mut self, value: i32) -> Result<(), Error> {
            if self.max.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Max")))?;
            }
            self.max = Some(value);
            Ok(())
        }
        fn handle_min<'de, R>(
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
                if self.min.is_some() {
                    fallback.get_or_insert(FooTypeDeserializerState::Min(None));
                    *self.state = FooTypeDeserializerState::Max(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = FooTypeDeserializerState::Min(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_min(data)?;
                    *self.state = FooTypeDeserializerState::Max(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(FooTypeDeserializerState::Min(Some(deserializer)));
                            *self.state = FooTypeDeserializerState::Max(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = FooTypeDeserializerState::Min(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_max<'de, R>(
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
                if self.max.is_some() {
                    fallback.get_or_insert(FooTypeDeserializerState::Max(None));
                    *self.state = FooTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = FooTypeDeserializerState::Max(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_max(data)?;
                    *self.state = FooTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(FooTypeDeserializerState::Max(Some(deserializer)));
                            *self.state = FooTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = FooTypeDeserializerState::Max(Some(deserializer));
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
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Min(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_min(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break {
                                event, allow_any, ..
                            } => break (event, allow_any),
                        }
                    }
                    (S::Max(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_max(reader, output, &mut fallback)? {
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
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = FooTypeDeserializerState::Min(None);
                        event
                    }
                    (S::Min(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_TNS), b"Min") {
                            let output =
                                <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_min(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break {
                                    event, allow_any, ..
                                } => break (event, allow_any),
                            }
                        } else {
                            *self.state = S::Max(None);
                            event
                        }
                    }
                    (S::Max(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_TNS), b"Max") {
                            let output =
                                <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_max(reader, output, &mut fallback)? {
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
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
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
            let state = replace(&mut *self.state, FooTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::FooType {
                min: self
                    .min
                    .ok_or_else(|| ErrorKind::MissingElement("Min".into()))?,
                max: self
                    .max
                    .ok_or_else(|| ErrorKind::MissingElement("Max".into()))?,
            })
        }
    }
}
