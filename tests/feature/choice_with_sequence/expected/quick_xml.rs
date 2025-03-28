use xsd_parser::{
    quick_xml::{Error, WithDeserializer, WithSerializer},
    schema::Namespace,
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub type Foo = FooType;
#[derive(Debug)]
pub enum FooType {
    Content2(FooContent2Type),
    Content3(FooContent3Type),
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
#[derive(Debug)]
pub struct FooContent2Type {
    pub element_1: i32,
    pub element_2: String,
}
impl WithSerializer for FooContent2Type {
    type Serializer<'x> = quick_xml_serialize::FooContent2TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::FooContent2TypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::FooContent2TypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for FooContent2Type {
    type Deserializer = quick_xml_deserialize::FooContent2TypeDeserializer;
}
#[derive(Debug)]
pub struct FooContent3Type {
    pub element_3: i32,
    pub element_4: String,
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
            state: Box::new(quick_xml_serialize::FooContent3TypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for FooContent3Type {
    type Deserializer = quick_xml_deserialize::FooContent3TypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
        ErrorKind, Event, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct FooTypeDeserializer {
        state: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum FooTypeDeserializerState {
        Init__,
        Content2(
            Option<super::FooContent2Type>,
            Option<<super::FooContent2Type as WithDeserializer>::Deserializer>,
        ),
        Content3(
            Option<super::FooContent3Type>,
            Option<<super::FooContent3Type as WithDeserializer>::Deserializer>,
        ),
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
            let (Event::Start(_) | Event::Empty(_)) = &event else {
                *self.state = fallback.take().unwrap_or(FooTypeDeserializerState::Init__);
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            let mut allow_any_element = false;
            let event = {
                let output = <super::FooContent2Type as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                match self.handle_content_2(reader, Default::default(), output, &mut *fallback)? {
                    ElementHandlerOutput::Continue { event, allow_any } => {
                        allow_any_element = allow_any_element || allow_any;
                        event
                    }
                    output => {
                        return Ok(output);
                    }
                }
            };
            let event = {
                let output = <super::FooContent3Type as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                match self.handle_content_3(reader, Default::default(), output, &mut *fallback)? {
                    ElementHandlerOutput::Continue { event, allow_any } => {
                        allow_any_element = allow_any_element || allow_any;
                        event
                    }
                    output => {
                        return Ok(output);
                    }
                }
            };
            *self.state = fallback.take().unwrap_or(FooTypeDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
        }
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                state: Box::new(FooTypeDeserializerState::Init__),
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
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Content2(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_content_2(&mut values, value)?;
                    }
                    Ok(super::FooType::Content2(values.ok_or_else(|| {
                        ErrorKind::MissingElement("Content2".into())
                    })?))
                }
                S::Content3(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_content_3(&mut values, value)?;
                    }
                    Ok(super::FooType::Content3(values.ok_or_else(|| {
                        ErrorKind::MissingElement("Content3".into())
                    })?))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
            }
        }
        fn store_content_2(
            values: &mut Option<super::FooContent2Type>,
            value: super::FooContent2Type,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Content2",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_content_3(
            values: &mut Option<super::FooContent3Type>,
            value: super::FooContent3Type,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Content3",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_content_2<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::FooContent2Type>,
            output: DeserializerOutput<'de, super::FooContent2Type>,
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
                *self.state = match fallback.take() {
                    None => FooTypeDeserializerState::Init__,
                    Some(FooTypeDeserializerState::Content2(_, Some(deserializer))) => {
                        FooTypeDeserializerState::Content2(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(FooTypeDeserializerState::Content2(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_content_2(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_content_2(&mut values, data)?;
                    *self.state = FooTypeDeserializerState::Content2(values, None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = FooTypeDeserializerState::Content2(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_content_3<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::FooContent3Type>,
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
                *self.state = match fallback.take() {
                    None => FooTypeDeserializerState::Init__,
                    Some(FooTypeDeserializerState::Content3(_, Some(deserializer))) => {
                        FooTypeDeserializerState::Content3(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(FooTypeDeserializerState::Content3(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_content_3(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_content_3(&mut values, data)?;
                    *self.state = FooTypeDeserializerState::Content3(values, None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = FooTypeDeserializerState::Content3(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
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
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content2(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content_2(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Content3(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content_3(reader, values, output, &mut fallback)? {
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
                    (S::Content2(values, None), event) => {
                        let output =
                            <super::FooContent2Type as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content_2(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Content3(values, None), event) => {
                        let output =
                            <super::FooContent3Type as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content_3(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
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
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct FooContent2TypeDeserializer {
        element_1: Option<i32>,
        element_2: Option<String>,
        state: Box<FooContent2TypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooContent2TypeDeserializerState {
        Init__,
        Element1(Option<<i32 as WithDeserializer>::Deserializer>),
        Element2(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl FooContent2TypeDeserializer {
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: FooContent2TypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use FooContent2TypeDeserializerState as S;
            match state {
                S::Element1(Some(deserializer)) => {
                    self.store_element_1(deserializer.finish(reader)?)?
                }
                S::Element2(Some(deserializer)) => {
                    self.store_element_2(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_element_1(&mut self, value: i32) -> Result<(), Error> {
            if self.element_1.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Element1",
                )))?;
            }
            self.element_1 = Some(value);
            Ok(())
        }
        fn store_element_2(&mut self, value: String) -> Result<(), Error> {
            if self.element_2.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Element2",
                )))?;
            }
            self.element_2 = Some(value);
            Ok(())
        }
        fn handle_element_1<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<FooContent2TypeDeserializerState>,
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
                if self.element_1.is_some() {
                    fallback.get_or_insert(FooContent2TypeDeserializerState::Element1(None));
                    *self.state = FooContent2TypeDeserializerState::Element2(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = FooContent2TypeDeserializerState::Element1(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_element_1(data)?;
                    *self.state = FooContent2TypeDeserializerState::Element2(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(FooContent2TypeDeserializerState::Element1(
                                Some(deserializer),
                            ));
                            *self.state = FooContent2TypeDeserializerState::Element2(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                FooContent2TypeDeserializerState::Element1(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_element_2<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<FooContent2TypeDeserializerState>,
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
                if self.element_2.is_some() {
                    fallback.get_or_insert(FooContent2TypeDeserializerState::Element2(None));
                    *self.state = FooContent2TypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = FooContent2TypeDeserializerState::Element2(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_element_2(data)?;
                    *self.state = FooContent2TypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(FooContent2TypeDeserializerState::Element2(
                                Some(deserializer),
                            ));
                            *self.state = FooContent2TypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                FooContent2TypeDeserializerState::Element2(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::FooContent2Type> for FooContent2TypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::FooContent2Type>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                element_1: None,
                element_2: None,
                state: Box::new(FooContent2TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, FooContent2TypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::FooContent2Type>
        where
            R: DeserializeReader,
        {
            use FooContent2TypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Element1(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_element_1(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Element2(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_element_2(reader, output, &mut fallback)? {
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
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = FooContent2TypeDeserializerState::Element1(None);
                        event
                    }
                    (S::Element1(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_TNS), b"Element1") {
                            let output =
                                <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_element_1(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Element2(None);
                            event
                        }
                    }
                    (S::Element2(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_TNS), b"Element2") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_element_2(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
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
        fn finish<R>(mut self, reader: &R) -> Result<super::FooContent2Type, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                FooContent2TypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::FooContent2Type {
                element_1: self
                    .element_1
                    .ok_or_else(|| ErrorKind::MissingElement("Element1".into()))?,
                element_2: self
                    .element_2
                    .ok_or_else(|| ErrorKind::MissingElement("Element2".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct FooContent3TypeDeserializer {
        element_3: Option<i32>,
        element_4: Option<String>,
        state: Box<FooContent3TypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooContent3TypeDeserializerState {
        Init__,
        Element3(Option<<i32 as WithDeserializer>::Deserializer>),
        Element4(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl FooContent3TypeDeserializer {
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: FooContent3TypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use FooContent3TypeDeserializerState as S;
            match state {
                S::Element3(Some(deserializer)) => {
                    self.store_element_3(deserializer.finish(reader)?)?
                }
                S::Element4(Some(deserializer)) => {
                    self.store_element_4(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_element_3(&mut self, value: i32) -> Result<(), Error> {
            if self.element_3.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Element3",
                )))?;
            }
            self.element_3 = Some(value);
            Ok(())
        }
        fn store_element_4(&mut self, value: String) -> Result<(), Error> {
            if self.element_4.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Element4",
                )))?;
            }
            self.element_4 = Some(value);
            Ok(())
        }
        fn handle_element_3<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<FooContent3TypeDeserializerState>,
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
                if self.element_3.is_some() {
                    fallback.get_or_insert(FooContent3TypeDeserializerState::Element3(None));
                    *self.state = FooContent3TypeDeserializerState::Element4(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = FooContent3TypeDeserializerState::Element3(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_element_3(data)?;
                    *self.state = FooContent3TypeDeserializerState::Element4(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(FooContent3TypeDeserializerState::Element3(
                                Some(deserializer),
                            ));
                            *self.state = FooContent3TypeDeserializerState::Element4(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                FooContent3TypeDeserializerState::Element3(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_element_4<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<FooContent3TypeDeserializerState>,
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
                if self.element_4.is_some() {
                    fallback.get_or_insert(FooContent3TypeDeserializerState::Element4(None));
                    *self.state = FooContent3TypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = FooContent3TypeDeserializerState::Element4(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_element_4(data)?;
                    *self.state = FooContent3TypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(FooContent3TypeDeserializerState::Element4(
                                Some(deserializer),
                            ));
                            *self.state = FooContent3TypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                FooContent3TypeDeserializerState::Element4(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::FooContent3Type> for FooContent3TypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::FooContent3Type>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                element_3: None,
                element_4: None,
                state: Box::new(FooContent3TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, FooContent3TypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::FooContent3Type>
        where
            R: DeserializeReader,
        {
            use FooContent3TypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Element3(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_element_3(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Element4(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_element_4(reader, output, &mut fallback)? {
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
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = FooContent3TypeDeserializerState::Element3(None);
                        event
                    }
                    (S::Element3(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_TNS), b"Element3") {
                            let output =
                                <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_element_3(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Element4(None);
                            event
                        }
                    }
                    (S::Element4(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_TNS), b"Element4") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_element_4(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
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
        fn finish<R>(mut self, reader: &R) -> Result<super::FooContent3Type, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                FooContent3TypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::FooContent3Type {
                element_3: self
                    .element_3
                    .ok_or_else(|| ErrorKind::MissingElement("Element3".into()))?,
                element_4: self
                    .element_4
                    .ok_or_else(|| ErrorKind::MissingElement("Element4".into()))?,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use core::iter::Iterator;
    use xsd_parser::quick_xml::{BytesEnd, BytesStart, Error, Event, WithSerializer};
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
        Content2(<super::FooContent2Type as WithSerializer>::Serializer<'ser>),
        Content3(<super::FooContent3Type as WithSerializer>::Serializer<'ser>),
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
                            super::FooType::Content2(x) => {
                                *self.state = FooTypeSerializerState::Content2(
                                    WithSerializer::serializer(x, Some("tns:Content2"), false)?,
                                )
                            }
                            super::FooType::Content3(x) => {
                                *self.state = FooTypeSerializerState::Content3(
                                    WithSerializer::serializer(x, Some("tns:Content3"), false)?,
                                )
                            }
                        }
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    FooTypeSerializerState::Content2(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooTypeSerializerState::End__,
                    },
                    FooTypeSerializerState::Content3(x) => match x.next().transpose()? {
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
    #[derive(Debug)]
    pub struct FooContent2TypeSerializer<'ser> {
        pub(super) value: &'ser super::FooContent2Type,
        pub(super) state: Box<FooContent2TypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum FooContent2TypeSerializerState<'ser> {
        Init__,
        Element1(<i32 as WithSerializer>::Serializer<'ser>),
        Element2(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooContent2TypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FooContent2TypeSerializerState::Init__ => {
                        *self.state =
                            FooContent2TypeSerializerState::Element1(WithSerializer::serializer(
                                &self.value.element_1,
                                Some("tns:Element1"),
                                false,
                            )?);
                    }
                    FooContent2TypeSerializerState::Element1(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = FooContent2TypeSerializerState::Element2(
                                WithSerializer::serializer(
                                    &self.value.element_2,
                                    Some("tns:Element2"),
                                    false,
                                )?,
                            )
                        }
                    },
                    FooContent2TypeSerializerState::Element2(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooContent2TypeSerializerState::Done__,
                    },
                    FooContent2TypeSerializerState::Done__ => return Ok(None),
                    FooContent2TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for FooContent2TypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = FooContent2TypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct FooContent3TypeSerializer<'ser> {
        pub(super) value: &'ser super::FooContent3Type,
        pub(super) state: Box<FooContent3TypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum FooContent3TypeSerializerState<'ser> {
        Init__,
        Element3(<i32 as WithSerializer>::Serializer<'ser>),
        Element4(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooContent3TypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FooContent3TypeSerializerState::Init__ => {
                        *self.state =
                            FooContent3TypeSerializerState::Element3(WithSerializer::serializer(
                                &self.value.element_3,
                                Some("tns:Element3"),
                                false,
                            )?);
                    }
                    FooContent3TypeSerializerState::Element3(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = FooContent3TypeSerializerState::Element4(
                                WithSerializer::serializer(
                                    &self.value.element_4,
                                    Some("tns:Element4"),
                                    false,
                                )?,
                            )
                        }
                    },
                    FooContent3TypeSerializerState::Element4(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooContent3TypeSerializerState::Done__,
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
                    *self.state = FooContent3TypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
