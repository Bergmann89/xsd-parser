use xsd_parser::{
    models::schema::Namespace,
    quick_xml::{Error, WithDeserializer, WithSerializer},
    xml::{AnyAttributes, AnyElement},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub any_attribute: AnyAttributes,
    pub name: String,
    pub any_0: Vec<AnyElement>,
    pub choice: Vec<ChoiceType>,
    pub any_1: Vec<AnyElement>,
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
pub struct ChoiceType {
    pub any_attribute: AnyAttributes,
    pub content: ChoiceTypeContent,
}
#[derive(Debug)]
pub enum ChoiceTypeContent {
    Name(String),
    Any(AnyElement),
}
impl WithSerializer for ChoiceType {
    type Serializer<'x> = quick_xml_serialize::ChoiceTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ChoiceTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ChoiceTypeSerializerState::Init__),
            name: name.unwrap_or("tns:ChoiceType"),
            is_root,
        })
    }
}
impl WithSerializer for ChoiceTypeContent {
    type Serializer<'x> = quick_xml_serialize::ChoiceTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::ChoiceTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ChoiceTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for ChoiceType {
    type Deserializer = quick_xml_deserialize::ChoiceTypeDeserializer;
}
impl WithDeserializer for ChoiceTypeContent {
    type Deserializer = quick_xml_deserialize::ChoiceTypeContentDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::{
        quick_xml::{
            filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer,
            DeserializerArtifact, DeserializerEvent, DeserializerOutput, DeserializerResult,
            ElementHandlerOutput, Error, ErrorKind, Event, RawByteStr, WithDeserializer,
        },
        xml::{AnyAttributes, AnyElement},
    };
    #[derive(Debug)]
    pub struct FooTypeDeserializer {
        any_attribute: AnyAttributes,
        name: Option<String>,
        any_0: Vec<AnyElement>,
        choice: Vec<super::ChoiceType>,
        any_1: Vec<AnyElement>,
        state__: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Init__,
        Name(Option<<String as WithDeserializer>::Deserializer>),
        Any0(Option<<AnyElement as WithDeserializer>::Deserializer>),
        Choice(Option<<super::ChoiceType as WithDeserializer>::Deserializer>),
        Any1(Option<<AnyElement as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl FooTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut any_attribute = AnyAttributes::default();
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                any_attribute.push(attrib)?;
            }
            Ok(Self {
                any_attribute: any_attribute,
                name: None,
                any_0: Vec::new(),
                choice: Vec::new(),
                any_1: Vec::new(),
                state__: Box::new(FooTypeDeserializerState::Init__),
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
                S::Name(Some(deserializer)) => self.store_name(deserializer.finish(reader)?)?,
                S::Any0(Some(deserializer)) => self.store_any_0(deserializer.finish(reader)?)?,
                S::Choice(Some(deserializer)) => self.store_choice(deserializer.finish(reader)?)?,
                S::Any1(Some(deserializer)) => self.store_any_1(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_name(&mut self, value: String) -> Result<(), Error> {
            if self.name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Name")))?;
            }
            self.name = Some(value);
            Ok(())
        }
        fn store_any_0(&mut self, value: AnyElement) -> Result<(), Error> {
            self.any_0.push(value);
            Ok(())
        }
        fn store_choice(&mut self, value: super::ChoiceType) -> Result<(), Error> {
            self.choice.push(value);
            Ok(())
        }
        fn store_any_1(&mut self, value: AnyElement) -> Result<(), Error> {
            self.any_1.push(value);
            Ok(())
        }
        fn handle_name<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
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
                if self.name.is_some() {
                    fallback.get_or_insert(FooTypeDeserializerState::Name(None));
                    *self.state__ = FooTypeDeserializerState::Any0(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = FooTypeDeserializerState::Name(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_name(data)?;
                    *self.state__ = FooTypeDeserializerState::Any0(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(FooTypeDeserializerState::Name(Some(deserializer)));
                            *self.state__ = FooTypeDeserializerState::Any0(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ = FooTypeDeserializerState::Name(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_any_0<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, AnyElement>,
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
                fallback.get_or_insert(FooTypeDeserializerState::Any0(None));
                *self.state__ = FooTypeDeserializerState::Choice(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_any_0(data)?;
                    *self.state__ = FooTypeDeserializerState::Any0(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(FooTypeDeserializerState::Any0(Some(deserializer)));
                            *self.state__ = FooTypeDeserializerState::Any0(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ = FooTypeDeserializerState::Any0(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_choice<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ChoiceType>,
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
                fallback.get_or_insert(FooTypeDeserializerState::Choice(None));
                *self.state__ = FooTypeDeserializerState::Any1(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_choice(data)?;
                    *self.state__ = FooTypeDeserializerState::Choice(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(FooTypeDeserializerState::Choice(Some(
                                deserializer,
                            )));
                            *self.state__ = FooTypeDeserializerState::Choice(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ = FooTypeDeserializerState::Choice(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_any_1<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, AnyElement>,
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
                fallback.get_or_insert(FooTypeDeserializerState::Any1(None));
                *self.state__ = FooTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_any_1(data)?;
                    *self.state__ = FooTypeDeserializerState::Any1(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(FooTypeDeserializerState::Any1(Some(deserializer)));
                            *self.state__ = FooTypeDeserializerState::Any1(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ = FooTypeDeserializerState::Any1(Some(deserializer));
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
            let mut is_any_retry = false;
            let mut any_fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Name(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_name(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Any0(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_any_0(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Choice(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_choice(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Any1(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_any_1(reader, output, &mut fallback)? {
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
                        *self.state__ = FooTypeDeserializerState::Name(None);
                        event
                    }
                    (S::Name(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Name",
                            false,
                        )?;
                        match self.handle_name(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Any0(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if is_any_retry {
                            let output = <AnyElement as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_any_0(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            any_fallback.get_or_insert(S::Any0(None));
                            *self.state__ = S::Choice(None);
                            event
                        }
                    }
                    (S::Choice(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Choice",
                            true,
                        )?;
                        match self.handle_choice(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Any1(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if is_any_retry {
                            let output = <AnyElement as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_any_1(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            any_fallback.get_or_insert(S::Any1(None));
                            *self.state__ = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        if let Some(state) = any_fallback.take() {
                            is_any_retry = true;
                            *self.state__ = state;
                            event
                        } else {
                            fallback.get_or_insert(S::Done__);
                            break (DeserializerEvent::Continue(event), allow_any_element);
                        }
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
        fn finish<R>(mut self, reader: &R) -> Result<super::FooType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state__, FooTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::FooType {
                any_attribute: self.any_attribute,
                name: self
                    .name
                    .ok_or_else(|| ErrorKind::MissingElement("Name".into()))?,
                any_0: self.any_0,
                choice: self.choice,
                any_1: self.any_1,
            })
        }
    }
    #[derive(Debug)]
    pub struct ChoiceTypeDeserializer {
        any_attribute: AnyAttributes,
        content: Option<super::ChoiceTypeContent>,
        state__: Box<ChoiceTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ChoiceTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::ChoiceTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ChoiceTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut any_attribute = AnyAttributes::default();
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                any_attribute.push(attrib)?;
            }
            Ok(Self {
                any_attribute: any_attribute,
                content: None,
                state__: Box::new(ChoiceTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ChoiceTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let ChoiceTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ChoiceTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ChoiceTypeContent>,
            fallback: &mut Option<ChoiceTypeDeserializerState>,
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
                    .unwrap_or(ChoiceTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = ChoiceTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = ChoiceTypeDeserializerState::Content__(deserializer);
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ChoiceType> for ChoiceTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ChoiceType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ChoiceType>
        where
            R: DeserializeReader,
        {
            use ChoiceTypeDeserializerState as S;
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
                            <super::ChoiceTypeContent as WithDeserializer>::Deserializer::init(
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
        fn finish<R>(mut self, reader: &R) -> Result<super::ChoiceType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state__, ChoiceTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::ChoiceType {
                any_attribute: self.any_attribute,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ChoiceTypeContentDeserializer {
        state__: Box<ChoiceTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ChoiceTypeContentDeserializerState {
        Init__,
        Name(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Any(
            Option<AnyElement>,
            Option<<AnyElement as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ChoiceTypeContent),
        Unknown__,
    }
    impl ChoiceTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<ChoiceTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Name")
                ) {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_name(reader, Default::default(), output, &mut *fallback);
                }
                event = {
                    let output =
                        <AnyElement as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_any(reader, Default::default(), output, &mut *fallback)? {
                        ElementHandlerOutput::Continue { event, .. } => event,
                        output => {
                            return Ok(output);
                        }
                    }
                };
            }
            *self.state__ = fallback
                .take()
                .unwrap_or(ChoiceTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: ChoiceTypeContentDeserializerState,
        ) -> Result<super::ChoiceTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use ChoiceTypeContentDeserializerState as S;
            match state {
                S::Unknown__ => unreachable!(),
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Name(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_name(&mut values, value)?;
                    }
                    Ok(super::ChoiceTypeContent::Name(
                        values.ok_or_else(|| ErrorKind::MissingElement("Name".into()))?,
                    ))
                }
                S::Any(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_any(&mut values, value)?;
                    }
                    Ok(super::ChoiceTypeContent::Any(
                        values.ok_or_else(|| ErrorKind::MissingElement("any2".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
            }
        }
        fn store_name(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Name")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_any(values: &mut Option<AnyElement>, value: AnyElement) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"any2")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_name<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ChoiceTypeContentDeserializerState>,
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
                        *self.state__ = ChoiceTypeContentDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => ChoiceTypeContentDeserializerState::Name(values, None),
                    Some(ChoiceTypeContentDeserializerState::Name(_, Some(deserializer))) => {
                        ChoiceTypeContentDeserializerState::Name(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(ChoiceTypeContentDeserializerState::Name(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_name(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_name(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        ChoiceTypeContentDeserializerState::Name(values, None),
                    )?;
                    *self.state__ = ChoiceTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ =
                        ChoiceTypeContentDeserializerState::Name(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_any<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<AnyElement>,
            output: DeserializerOutput<'de, AnyElement>,
            fallback: &mut Option<ChoiceTypeContentDeserializerState>,
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
                        *self.state__ = ChoiceTypeContentDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => ChoiceTypeContentDeserializerState::Any(values, None),
                    Some(ChoiceTypeContentDeserializerState::Any(_, Some(deserializer))) => {
                        ChoiceTypeContentDeserializerState::Any(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(ChoiceTypeContentDeserializerState::Any(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_any(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_any(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        ChoiceTypeContentDeserializerState::Any(values, None),
                    )?;
                    *self.state__ = ChoiceTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ =
                        ChoiceTypeContentDeserializerState::Any(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ChoiceTypeContent> for ChoiceTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ChoiceTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state__: Box::new(ChoiceTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, ChoiceTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::ChoiceTypeContent>
        where
            R: DeserializeReader,
        {
            use ChoiceTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Name(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_name(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Any(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_any(reader, values, output, &mut fallback)? {
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
                    (S::Name(values, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Name",
                            false,
                        )?;
                        match self.handle_name(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Any(values, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"any2", true)?;
                        match self.handle_any(reader, values, output, &mut fallback)? {
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
                    (state, Event::Text(_) | Event::CData(_)) => {
                        *self.state__ = state;
                        break (DeserializerEvent::None, false);
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
        fn finish<R>(self, reader: &R) -> Result<super::ChoiceTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state__)
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser::{
        quick_xml::{BytesEnd, BytesStart, Error, Event, IterSerializer, WithSerializer},
        xml::AnyElement,
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
        Name(<String as WithSerializer>::Serializer<'ser>),
        Any0(IterSerializer<'ser, &'ser [AnyElement], AnyElement>),
        Choice(IterSerializer<'ser, &'ser [super::ChoiceType], super::ChoiceType>),
        Any1(IterSerializer<'ser, &'ser [AnyElement], AnyElement>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FooTypeSerializerState::Init__ => {
                        *self.state = FooTypeSerializerState::Name(WithSerializer::serializer(
                            &self.value.name,
                            Some("tns:Name"),
                            false,
                        )?);
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        bytes.extend_attributes(self.value.any_attribute.attributes());
                        return Ok(Some(Event::Start(bytes)));
                    }
                    FooTypeSerializerState::Name(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = FooTypeSerializerState::Any0(IterSerializer::new(
                                &self.value.any_0[..],
                                None,
                                false,
                            ))
                        }
                    },
                    FooTypeSerializerState::Any0(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = FooTypeSerializerState::Choice(IterSerializer::new(
                                &self.value.choice[..],
                                Some("tns:Choice"),
                                false,
                            ))
                        }
                    },
                    FooTypeSerializerState::Choice(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = FooTypeSerializerState::Any1(IterSerializer::new(
                                &self.value.any_1[..],
                                None,
                                false,
                            ))
                        }
                    },
                    FooTypeSerializerState::Any1(x) => match x.next().transpose()? {
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
    pub struct ChoiceTypeSerializer<'ser> {
        pub(super) value: &'ser super::ChoiceType,
        pub(super) state: Box<ChoiceTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ChoiceTypeSerializerState<'ser> {
        Init__,
        Content__(<super::ChoiceTypeContent as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ChoiceTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ChoiceTypeSerializerState::Init__ => {
                        *self.state = ChoiceTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        bytes.extend_attributes(self.value.any_attribute.attributes());
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ChoiceTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ChoiceTypeSerializerState::End__,
                    },
                    ChoiceTypeSerializerState::End__ => {
                        *self.state = ChoiceTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ChoiceTypeSerializerState::Done__ => return Ok(None),
                    ChoiceTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ChoiceTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ChoiceTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ChoiceTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::ChoiceTypeContent,
        pub(super) state: Box<ChoiceTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum ChoiceTypeContentSerializerState<'ser> {
        Init__,
        Name(<String as WithSerializer>::Serializer<'ser>),
        Any(<AnyElement as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ChoiceTypeContentSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ChoiceTypeContentSerializerState::Init__ => match self.value {
                        super::ChoiceTypeContent::Name(x) => {
                            *self.state = ChoiceTypeContentSerializerState::Name(
                                WithSerializer::serializer(x, Some("tns:Name"), false)?,
                            )
                        }
                        super::ChoiceTypeContent::Any(x) => {
                            *self.state = ChoiceTypeContentSerializerState::Any(
                                WithSerializer::serializer(x, None, false)?,
                            )
                        }
                    },
                    ChoiceTypeContentSerializerState::Name(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ChoiceTypeContentSerializerState::Done__,
                    },
                    ChoiceTypeContentSerializerState::Any(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ChoiceTypeContentSerializerState::Done__,
                    },
                    ChoiceTypeContentSerializerState::Done__ => return Ok(None),
                    ChoiceTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ChoiceTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ChoiceTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
