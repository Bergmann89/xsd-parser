use xsd_parser::{
    models::schema::Namespace,
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub outer_1: FooOuterType,
    pub outer_2: FooOuterType,
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
            name: name.unwrap_or("tns:Foo"),
            is_root,
        })
    }
}
impl WithDeserializer for FooType {
    type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
}
#[derive(Debug)]
pub enum FooOuterType {
    Bar(String),
    Baz(i32),
    Inner(FooInnerType),
}
impl WithSerializer for FooOuterType {
    type Serializer<'x> = quick_xml_serialize::FooOuterTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::FooOuterTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::FooOuterTypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for FooOuterType {
    type Deserializer = quick_xml_deserialize::FooOuterTypeDeserializer;
}
#[derive(Debug)]
pub struct FooInnerType {
    pub fizz: String,
    pub buzz: i32,
}
impl WithSerializer for FooInnerType {
    type Serializer<'x> = quick_xml_serialize::FooInnerTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::FooInnerTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::FooInnerTypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for FooInnerType {
    type Deserializer = quick_xml_deserialize::FooInnerTypeDeserializer;
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
        outer_1: Option<super::FooOuterType>,
        outer_2: Option<super::FooOuterType>,
        state: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Init__,
        Outer1(Option<<super::FooOuterType as WithDeserializer>::Deserializer>),
        Outer2(Option<<super::FooOuterType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl FooTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                outer_1: None,
                outer_2: None,
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
                S::Outer1(Some(deserializer)) => {
                    self.store_outer_1(deserializer.finish(reader)?)?
                }
                S::Outer2(Some(deserializer)) => {
                    self.store_outer_2(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_outer_1(&mut self, value: super::FooOuterType) -> Result<(), Error> {
            if self.outer_1.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Outer",
                )))?;
            }
            self.outer_1 = Some(value);
            Ok(())
        }
        fn store_outer_2(&mut self, value: super::FooOuterType) -> Result<(), Error> {
            if self.outer_2.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Outer_2",
                )))?;
            }
            self.outer_2 = Some(value);
            Ok(())
        }
        fn handle_outer_1<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::FooOuterType>,
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
                if self.outer_1.is_some() {
                    fallback.get_or_insert(FooTypeDeserializerState::Outer1(None));
                    *self.state = FooTypeDeserializerState::Outer2(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = FooTypeDeserializerState::Outer1(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_outer_1(data)?;
                    *self.state = FooTypeDeserializerState::Outer2(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(FooTypeDeserializerState::Outer1(Some(
                                deserializer,
                            )));
                            *self.state = FooTypeDeserializerState::Outer2(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = FooTypeDeserializerState::Outer1(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_outer_2<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::FooOuterType>,
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
                if self.outer_2.is_some() {
                    fallback.get_or_insert(FooTypeDeserializerState::Outer2(None));
                    *self.state = FooTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = FooTypeDeserializerState::Outer2(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_outer_2(data)?;
                    *self.state = FooTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(FooTypeDeserializerState::Outer2(Some(
                                deserializer,
                            )));
                            *self.state = FooTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = FooTypeDeserializerState::Outer2(Some(deserializer));
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
                    (S::Outer1(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_outer_1(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Outer2(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_outer_2(reader, output, &mut fallback)? {
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
                        *self.state = FooTypeDeserializerState::Outer1(None);
                        event
                    }
                    (S::Outer1(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = <super::FooOuterType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_outer_1(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Outer2(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = <super::FooOuterType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_outer_2(reader, output, &mut fallback)? {
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
                outer_1: self
                    .outer_1
                    .ok_or_else(|| ErrorKind::MissingElement("Outer".into()))?,
                outer_2: self
                    .outer_2
                    .ok_or_else(|| ErrorKind::MissingElement("Outer_2".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct FooOuterTypeDeserializer {
        state: Box<FooOuterTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum FooOuterTypeDeserializerState {
        Init__,
        Bar(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Baz(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Inner(
            Option<super::FooInnerType>,
            Option<<super::FooInnerType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::FooOuterType),
        Unknown__,
    }
    impl FooOuterTypeDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<FooOuterTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut allow_any_element = false;
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Bar")
                ) {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_bar(reader, Default::default(), output, &mut *fallback);
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Baz")
                ) {
                    let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_baz(reader, Default::default(), output, &mut *fallback);
                }
                event = {
                    let output = <super::FooInnerType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_inner(reader, Default::default(), output, &mut *fallback)? {
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
            *self.state = fallback
                .take()
                .unwrap_or(FooOuterTypeDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
        }
        fn finish_state<R>(
            reader: &R,
            state: FooOuterTypeDeserializerState,
        ) -> Result<super::FooOuterType, Error>
        where
            R: DeserializeReader,
        {
            use FooOuterTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Bar(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_bar(&mut values, value)?;
                    }
                    Ok(super::FooOuterType::Bar(
                        values.ok_or_else(|| ErrorKind::MissingElement("Bar".into()))?,
                    ))
                }
                S::Baz(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_baz(&mut values, value)?;
                    }
                    Ok(super::FooOuterType::Baz(
                        values.ok_or_else(|| ErrorKind::MissingElement("Baz".into()))?,
                    ))
                }
                S::Inner(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_inner(&mut values, value)?;
                    }
                    Ok(super::FooOuterType::Inner(values.ok_or_else(|| {
                        ErrorKind::MissingElement("Inner".into())
                    })?))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
            }
        }
        fn store_bar(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Bar")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_baz(values: &mut Option<i32>, value: i32) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Baz")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_inner(
            values: &mut Option<super::FooInnerType>,
            value: super::FooInnerType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Inner",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_bar<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<FooOuterTypeDeserializerState>,
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
                    None => FooOuterTypeDeserializerState::Bar(values, None),
                    Some(FooOuterTypeDeserializerState::Bar(_, Some(deserializer))) => {
                        FooOuterTypeDeserializerState::Bar(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(FooOuterTypeDeserializerState::Bar(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_bar(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bar(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        FooOuterTypeDeserializerState::Bar(values, None),
                    )?;
                    *self.state = FooOuterTypeDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = FooOuterTypeDeserializerState::Bar(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_baz<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<FooOuterTypeDeserializerState>,
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
                    None => FooOuterTypeDeserializerState::Baz(values, None),
                    Some(FooOuterTypeDeserializerState::Baz(_, Some(deserializer))) => {
                        FooOuterTypeDeserializerState::Baz(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(FooOuterTypeDeserializerState::Baz(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_baz(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_baz(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        FooOuterTypeDeserializerState::Baz(values, None),
                    )?;
                    *self.state = FooOuterTypeDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = FooOuterTypeDeserializerState::Baz(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_inner<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::FooInnerType>,
            output: DeserializerOutput<'de, super::FooInnerType>,
            fallback: &mut Option<FooOuterTypeDeserializerState>,
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
                    None => FooOuterTypeDeserializerState::Inner(values, None),
                    Some(FooOuterTypeDeserializerState::Inner(_, Some(deserializer))) => {
                        FooOuterTypeDeserializerState::Inner(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(FooOuterTypeDeserializerState::Inner(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_inner(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_inner(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        FooOuterTypeDeserializerState::Inner(values, None),
                    )?;
                    *self.state = FooOuterTypeDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = FooOuterTypeDeserializerState::Inner(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::FooOuterType> for FooOuterTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::FooOuterType>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(FooOuterTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, FooOuterTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::FooOuterType>
        where
            R: DeserializeReader,
        {
            use FooOuterTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Bar(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_bar(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Baz(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_baz(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Inner(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_inner(reader, values, output, &mut fallback)? {
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
                    (S::Bar(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_bar(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Baz(values, None), event) => {
                        let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_baz(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Inner(values, None), event) => {
                        let output = <super::FooInnerType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_inner(reader, values, output, &mut fallback)? {
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
        fn finish<R>(self, reader: &R) -> Result<super::FooOuterType, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct FooInnerTypeDeserializer {
        fizz: Option<String>,
        buzz: Option<i32>,
        state: Box<FooInnerTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooInnerTypeDeserializerState {
        Init__,
        Fizz(Option<<String as WithDeserializer>::Deserializer>),
        Buzz(Option<<i32 as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl FooInnerTypeDeserializer {
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: FooInnerTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use FooInnerTypeDeserializerState as S;
            match state {
                S::Fizz(Some(deserializer)) => self.store_fizz(deserializer.finish(reader)?)?,
                S::Buzz(Some(deserializer)) => self.store_buzz(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_fizz(&mut self, value: String) -> Result<(), Error> {
            if self.fizz.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Fizz")))?;
            }
            self.fizz = Some(value);
            Ok(())
        }
        fn store_buzz(&mut self, value: i32) -> Result<(), Error> {
            if self.buzz.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Buzz")))?;
            }
            self.buzz = Some(value);
            Ok(())
        }
        fn handle_fizz<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<FooInnerTypeDeserializerState>,
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
                if self.fizz.is_some() {
                    fallback.get_or_insert(FooInnerTypeDeserializerState::Fizz(None));
                    *self.state = FooInnerTypeDeserializerState::Buzz(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = FooInnerTypeDeserializerState::Fizz(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_fizz(data)?;
                    *self.state = FooInnerTypeDeserializerState::Buzz(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(FooInnerTypeDeserializerState::Fizz(Some(
                                deserializer,
                            )));
                            *self.state = FooInnerTypeDeserializerState::Buzz(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = FooInnerTypeDeserializerState::Fizz(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_buzz<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<FooInnerTypeDeserializerState>,
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
                if self.buzz.is_some() {
                    fallback.get_or_insert(FooInnerTypeDeserializerState::Buzz(None));
                    *self.state = FooInnerTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = FooInnerTypeDeserializerState::Buzz(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_buzz(data)?;
                    *self.state = FooInnerTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(FooInnerTypeDeserializerState::Buzz(Some(
                                deserializer,
                            )));
                            *self.state = FooInnerTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = FooInnerTypeDeserializerState::Buzz(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::FooInnerType> for FooInnerTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::FooInnerType>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                fizz: None,
                buzz: None,
                state: Box::new(FooInnerTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, FooInnerTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::FooInnerType>
        where
            R: DeserializeReader,
        {
            use FooInnerTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Fizz(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fizz(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Buzz(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_buzz(reader, output, &mut fallback)? {
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
                        *self.state = FooInnerTypeDeserializerState::Fizz(None);
                        event
                    }
                    (S::Fizz(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_TNS), b"Fizz") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_fizz(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Buzz(None);
                            event
                        }
                    }
                    (S::Buzz(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_TNS), b"Buzz") {
                            let output =
                                <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_buzz(reader, output, &mut fallback)? {
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
        fn finish<R>(mut self, reader: &R) -> Result<super::FooInnerType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, FooInnerTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::FooInnerType {
                fizz: self
                    .fizz
                    .ok_or_else(|| ErrorKind::MissingElement("Fizz".into()))?,
                buzz: self
                    .buzz
                    .ok_or_else(|| ErrorKind::MissingElement("Buzz".into()))?,
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
        Outer1(<super::FooOuterType as WithSerializer>::Serializer<'ser>),
        Outer2(<super::FooOuterType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FooTypeSerializerState::Init__ => {
                        *self.state = FooTypeSerializerState::Outer1(WithSerializer::serializer(
                            &self.value.outer_1,
                            Some("tns:Outer"),
                            false,
                        )?);
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    FooTypeSerializerState::Outer1(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                FooTypeSerializerState::Outer2(WithSerializer::serializer(
                                    &self.value.outer_2,
                                    Some("tns:Outer_2"),
                                    false,
                                )?)
                        }
                    },
                    FooTypeSerializerState::Outer2(x) => match x.next().transpose()? {
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
    pub struct FooOuterTypeSerializer<'ser> {
        pub(super) value: &'ser super::FooOuterType,
        pub(super) state: Box<FooOuterTypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum FooOuterTypeSerializerState<'ser> {
        Init__,
        Bar(<String as WithSerializer>::Serializer<'ser>),
        Baz(<i32 as WithSerializer>::Serializer<'ser>),
        Inner(<super::FooInnerType as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooOuterTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FooOuterTypeSerializerState::Init__ => match self.value {
                        super::FooOuterType::Bar(x) => {
                            *self.state = FooOuterTypeSerializerState::Bar(
                                WithSerializer::serializer(x, Some("tns:Bar"), false)?,
                            )
                        }
                        super::FooOuterType::Baz(x) => {
                            *self.state = FooOuterTypeSerializerState::Baz(
                                WithSerializer::serializer(x, Some("tns:Baz"), false)?,
                            )
                        }
                        super::FooOuterType::Inner(x) => {
                            *self.state = FooOuterTypeSerializerState::Inner(
                                WithSerializer::serializer(x, Some("tns:Inner"), false)?,
                            )
                        }
                    },
                    FooOuterTypeSerializerState::Bar(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooOuterTypeSerializerState::Done__,
                    },
                    FooOuterTypeSerializerState::Baz(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooOuterTypeSerializerState::Done__,
                    },
                    FooOuterTypeSerializerState::Inner(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooOuterTypeSerializerState::Done__,
                    },
                    FooOuterTypeSerializerState::Done__ => return Ok(None),
                    FooOuterTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for FooOuterTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = FooOuterTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct FooInnerTypeSerializer<'ser> {
        pub(super) value: &'ser super::FooInnerType,
        pub(super) state: Box<FooInnerTypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum FooInnerTypeSerializerState<'ser> {
        Init__,
        Fizz(<String as WithSerializer>::Serializer<'ser>),
        Buzz(<i32 as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooInnerTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FooInnerTypeSerializerState::Init__ => {
                        *self.state = FooInnerTypeSerializerState::Fizz(
                            WithSerializer::serializer(&self.value.fizz, Some("tns:Fizz"), false)?,
                        );
                    }
                    FooInnerTypeSerializerState::Fizz(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                FooInnerTypeSerializerState::Buzz(WithSerializer::serializer(
                                    &self.value.buzz,
                                    Some("tns:Buzz"),
                                    false,
                                )?)
                        }
                    },
                    FooInnerTypeSerializerState::Buzz(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooInnerTypeSerializerState::Done__,
                    },
                    FooInnerTypeSerializerState::Done__ => return Ok(None),
                    FooInnerTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for FooInnerTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = FooInnerTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
