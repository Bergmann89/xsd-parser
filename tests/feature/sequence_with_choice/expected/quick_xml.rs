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
    pub content_3: FooContent3Type,
    pub content_6: FooContent6Type,
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
#[derive(Debug, Clone)]
pub enum FooContent3Type {
    Element1(i32),
    Element2(String),
}
impl WithSerializer for FooContent3Type {
    type Serializer<'x> = quick_xml_serialize::FooContent3TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::FooContent3TypeSerializer {
            value: self,
            state: quick_xml_serialize::FooContent3TypeSerializerState::Init__,
        })
    }
}
impl WithDeserializer for FooContent3Type {
    type Deserializer = quick_xml_deserialize::FooContent3TypeDeserializer;
}
#[derive(Debug, Clone)]
pub enum FooContent6Type {
    Element3(i32),
    Element4(String),
}
impl WithSerializer for FooContent6Type {
    type Serializer<'x> = quick_xml_serialize::FooContent6TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::FooContent6TypeSerializer {
            value: self,
            state: quick_xml_serialize::FooContent6TypeSerializerState::Init__,
        })
    }
}
impl WithDeserializer for FooContent6Type {
    type Deserializer = quick_xml_deserialize::FooContent6TypeDeserializer;
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
        Content3(<super::FooContent3Type as WithSerializer>::Serializer<'ser>),
        Content6(<super::FooContent6Type as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    FooTypeSerializerState::Init__ => {
                        self.state = FooTypeSerializerState::Content3(WithSerializer::serializer(
                            &self.value.content_3,
                            Some("tns:Content3"),
                            false,
                        )?);
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    FooTypeSerializerState::Content3(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state =
                                FooTypeSerializerState::Content6(WithSerializer::serializer(
                                    &self.value.content_6,
                                    Some("tns:Content6"),
                                    false,
                                )?)
                        }
                    },
                    FooTypeSerializerState::Content6(x) => match x.next().transpose()? {
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
    #[derive(Debug)]
    pub struct FooContent3TypeSerializer<'ser> {
        pub(super) value: &'ser super::FooContent3Type,
        pub(super) state: FooContent3TypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    pub(super) enum FooContent3TypeSerializerState<'ser> {
        Init__,
        Element1(<i32 as WithSerializer>::Serializer<'ser>),
        Element2(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooContent3TypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    FooContent3TypeSerializerState::Init__ => match self.value {
                        super::FooContent3Type::Element1(x) => {
                            self.state = FooContent3TypeSerializerState::Element1(
                                WithSerializer::serializer(x, Some("tns:Element1"), false)?,
                            )
                        }
                        super::FooContent3Type::Element2(x) => {
                            self.state = FooContent3TypeSerializerState::Element2(
                                WithSerializer::serializer(x, Some("tns:Element2"), false)?,
                            )
                        }
                    },
                    FooContent3TypeSerializerState::Element1(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = FooContent3TypeSerializerState::Done__,
                    },
                    FooContent3TypeSerializerState::Element2(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = FooContent3TypeSerializerState::Done__,
                    },
                    FooContent3TypeSerializerState::Done__ => return Ok(None),
                    FooContent3TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for FooContent3TypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = FooContent3TypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct FooContent6TypeSerializer<'ser> {
        pub(super) value: &'ser super::FooContent6Type,
        pub(super) state: FooContent6TypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    pub(super) enum FooContent6TypeSerializerState<'ser> {
        Init__,
        Element3(<i32 as WithSerializer>::Serializer<'ser>),
        Element4(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooContent6TypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    FooContent6TypeSerializerState::Init__ => match self.value {
                        super::FooContent6Type::Element3(x) => {
                            self.state = FooContent6TypeSerializerState::Element3(
                                WithSerializer::serializer(x, Some("tns:Element3"), false)?,
                            )
                        }
                        super::FooContent6Type::Element4(x) => {
                            self.state = FooContent6TypeSerializerState::Element4(
                                WithSerializer::serializer(x, Some("tns:Element4"), false)?,
                            )
                        }
                    },
                    FooContent6TypeSerializerState::Element3(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = FooContent6TypeSerializerState::Done__,
                    },
                    FooContent6TypeSerializerState::Element4(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = FooContent6TypeSerializerState::Done__,
                    },
                    FooContent6TypeSerializerState::Done__ => return Ok(None),
                    FooContent6TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for FooContent6TypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = FooContent6TypeSerializerState::Done__;
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
        content_3: Option<super::FooContent3Type>,
        content_6: Option<super::FooContent6Type>,
        state: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Init__,
        Content3(Option<<super::FooContent3Type as WithDeserializer>::Deserializer>),
        Content6(Option<<super::FooContent6Type as WithDeserializer>::Deserializer>),
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
                content_3: None,
                content_6: None,
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
                S::Content3(Some(deserializer)) => {
                    self.store_content_3(deserializer.finish(reader)?)?
                }
                S::Content6(Some(deserializer)) => {
                    self.store_content_6(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_content_3(&mut self, value: super::FooContent3Type) -> Result<(), Error> {
            if self.content_3.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Content3",
                )))?;
            }
            self.content_3 = Some(value);
            Ok(())
        }
        fn store_content_6(&mut self, value: super::FooContent6Type) -> Result<(), Error> {
            if self.content_6.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Content6",
                )))?;
            }
            self.content_6 = Some(value);
            Ok(())
        }
        fn handle_content_3<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::FooContent3Type>,
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
                fallback.get_or_insert(FooTypeDeserializerState::Content3(None));
                *self.state = FooTypeDeserializerState::Content6(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content_3(data)?;
                    *self.state = FooTypeDeserializerState::Content3(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    if let Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) = event
                    {
                        fallback
                            .get_or_insert(FooTypeDeserializerState::Content3(Some(deserializer)));
                        *self.state = FooTypeDeserializerState::Content6(None);
                        ElementHandlerOutput::continue_(event, allow_any)
                    } else {
                        *self.state = FooTypeDeserializerState::Content3(Some(deserializer));
                        ElementHandlerOutput::break_(event, allow_any)
                    }
                }
            })
        }
        fn handle_content_6<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::FooContent6Type>,
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
                fallback.get_or_insert(FooTypeDeserializerState::Content6(None));
                *self.state = FooTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content_6(data)?;
                    *self.state = FooTypeDeserializerState::Content6(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    if let Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) = event
                    {
                        fallback
                            .get_or_insert(FooTypeDeserializerState::Content6(Some(deserializer)));
                        *self.state = FooTypeDeserializerState::Done__;
                        ElementHandlerOutput::continue_(event, allow_any)
                    } else {
                        *self.state = FooTypeDeserializerState::Content6(Some(deserializer));
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
                    (S::Content3(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content_3(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break {
                                event, allow_any, ..
                            } => break (event, allow_any),
                        }
                    }
                    (S::Content6(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content_6(reader, output, &mut fallback)? {
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
                        *self.state = FooTypeDeserializerState::Content3(None);
                        event
                    }
                    (S::Content3(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::FooContent3Type as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content_3(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break {
                                event, allow_any, ..
                            } => break (event, allow_any),
                        }
                    }
                    (S::Content6(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::FooContent6Type as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content_6(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break {
                                event, allow_any, ..
                            } => break (event, allow_any),
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
            let state = replace(&mut *self.state, FooTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::FooType {
                content_3: self
                    .content_3
                    .ok_or_else(|| ErrorKind::MissingElement("Content3".into()))?,
                content_6: self
                    .content_6
                    .ok_or_else(|| ErrorKind::MissingElement("Content6".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub enum FooContent3TypeDeserializer {
        Init__,
        Element1(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Element2(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Unknown__,
    }
    impl FooContent3TypeDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<FooContent3TypeDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::break_(Some(event), false));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Element1")
            ) {
                let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_element_1(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Element2")
            ) {
                let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_element_2(reader, Default::default(), output, &mut *fallback);
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::break_(Some(event), false))
        }
        fn store_element_1(values: &mut Option<i32>, value: i32) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Element1",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_element_2(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Element2",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_element_1<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Element1(_, Some(deserializer))) => {
                        Self::Element1(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Element1(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_element_1(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_element_1(&mut values, data)?;
                    *self = Self::Element1(values, None);
                    Ok(ElementHandlerOutput::Break {
                        event,
                        allow_any,
                        finish: true,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => match event {
                    Some(event @ (Event::Start(_) | Event::Empty(_))) => {
                        fallback
                            .get_or_insert(Self::Element1(Default::default(), Some(deserializer)));
                        *self = Self::Element1(values, None);
                        Ok(ElementHandlerOutput::continue_(event, allow_any))
                    }
                    Some(event @ Event::End(_)) => {
                        *self = Self::Element1(values, Some(deserializer));
                        Ok(ElementHandlerOutput::continue_(event, allow_any))
                    }
                    _ => {
                        *self = Self::Element1(values, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                },
            }
        }
        fn handle_element_2<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Element2(_, Some(deserializer))) => {
                        Self::Element2(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Element2(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_element_2(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_element_2(&mut values, data)?;
                    *self = Self::Element2(values, None);
                    Ok(ElementHandlerOutput::Break {
                        event,
                        allow_any,
                        finish: true,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => match event {
                    Some(event @ (Event::Start(_) | Event::Empty(_))) => {
                        fallback
                            .get_or_insert(Self::Element2(Default::default(), Some(deserializer)));
                        *self = Self::Element2(values, None);
                        Ok(ElementHandlerOutput::continue_(event, allow_any))
                    }
                    Some(event @ Event::End(_)) => {
                        *self = Self::Element2(values, Some(deserializer));
                        Ok(ElementHandlerOutput::continue_(event, allow_any))
                    }
                    _ => {
                        *self = Self::Element2(values, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                },
            }
        }
    }
    impl<'de> Deserializer<'de, super::FooContent3Type> for FooContent3TypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::FooContent3Type>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooContent3Type>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any, finish) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::Element1(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_element_1(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break {
                                event,
                                allow_any,
                                finish,
                            } => break (event, allow_any, finish),
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Element2(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_element_2(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break {
                                event,
                                allow_any,
                                finish,
                            } => break (event, allow_any, finish),
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: Some(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break {
                                event,
                                allow_any,
                                finish,
                            } => break (event, allow_any, finish),
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Element1(values, None), event) => {
                        let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_element_1(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break {
                                event,
                                allow_any,
                                finish,
                            } => break (event, allow_any, finish),
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Element2(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_element_2(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break {
                                event,
                                allow_any,
                                finish,
                            } => break (event, allow_any, finish),
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if finish {
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
        fn finish<R>(self, reader: &R) -> Result<super::FooContent3Type, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::Element1(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_element_1(&mut values, value)?;
                    }
                    Ok(super::FooContent3Type::Element1(values.ok_or_else(
                        || ErrorKind::MissingElement("Element1".into()),
                    )?))
                }
                Self::Element2(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_element_2(&mut values, value)?;
                    }
                    Ok(super::FooContent3Type::Element2(values.ok_or_else(
                        || ErrorKind::MissingElement("Element2".into()),
                    )?))
                }
                Self::Unknown__ => unreachable!(),
            }
        }
    }
    #[derive(Debug)]
    pub enum FooContent6TypeDeserializer {
        Init__,
        Element3(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Element4(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Unknown__,
    }
    impl FooContent6TypeDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<FooContent6TypeDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::break_(Some(event), false));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Element3")
            ) {
                let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_element_3(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Element4")
            ) {
                let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_element_4(reader, Default::default(), output, &mut *fallback);
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::break_(Some(event), false))
        }
        fn store_element_3(values: &mut Option<i32>, value: i32) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Element3",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_element_4(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Element4",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_element_3<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Element3(_, Some(deserializer))) => {
                        Self::Element3(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Element3(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_element_3(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_element_3(&mut values, data)?;
                    *self = Self::Element3(values, None);
                    Ok(ElementHandlerOutput::Break {
                        event,
                        allow_any,
                        finish: true,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => match event {
                    Some(event @ (Event::Start(_) | Event::Empty(_))) => {
                        fallback
                            .get_or_insert(Self::Element3(Default::default(), Some(deserializer)));
                        *self = Self::Element3(values, None);
                        Ok(ElementHandlerOutput::continue_(event, allow_any))
                    }
                    Some(event @ Event::End(_)) => {
                        *self = Self::Element3(values, Some(deserializer));
                        Ok(ElementHandlerOutput::continue_(event, allow_any))
                    }
                    _ => {
                        *self = Self::Element3(values, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                },
            }
        }
        fn handle_element_4<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Element4(_, Some(deserializer))) => {
                        Self::Element4(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Element4(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_element_4(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_element_4(&mut values, data)?;
                    *self = Self::Element4(values, None);
                    Ok(ElementHandlerOutput::Break {
                        event,
                        allow_any,
                        finish: true,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => match event {
                    Some(event @ (Event::Start(_) | Event::Empty(_))) => {
                        fallback
                            .get_or_insert(Self::Element4(Default::default(), Some(deserializer)));
                        *self = Self::Element4(values, None);
                        Ok(ElementHandlerOutput::continue_(event, allow_any))
                    }
                    Some(event @ Event::End(_)) => {
                        *self = Self::Element4(values, Some(deserializer));
                        Ok(ElementHandlerOutput::continue_(event, allow_any))
                    }
                    _ => {
                        *self = Self::Element4(values, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                },
            }
        }
    }
    impl<'de> Deserializer<'de, super::FooContent6Type> for FooContent6TypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::FooContent6Type>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooContent6Type>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any, finish) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::Element3(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_element_3(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break {
                                event,
                                allow_any,
                                finish,
                            } => break (event, allow_any, finish),
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Element4(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_element_4(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break {
                                event,
                                allow_any,
                                finish,
                            } => break (event, allow_any, finish),
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: Some(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break {
                                event,
                                allow_any,
                                finish,
                            } => break (event, allow_any, finish),
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Element3(values, None), event) => {
                        let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_element_3(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break {
                                event,
                                allow_any,
                                finish,
                            } => break (event, allow_any, finish),
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Element4(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_element_4(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break {
                                event,
                                allow_any,
                                finish,
                            } => break (event, allow_any, finish),
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if finish {
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
        fn finish<R>(self, reader: &R) -> Result<super::FooContent6Type, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::Element3(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_element_3(&mut values, value)?;
                    }
                    Ok(super::FooContent6Type::Element3(values.ok_or_else(
                        || ErrorKind::MissingElement("Element3".into()),
                    )?))
                }
                Self::Element4(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_element_4(&mut values, value)?;
                    }
                    Ok(super::FooContent6Type::Element4(values.ok_or_else(
                        || ErrorKind::MissingElement("Element4".into()),
                    )?))
                }
                Self::Unknown__ => unreachable!(),
            }
        }
    }
}
