use xsd_parser::{
    models::schema::Namespace,
    quick_xml::{Error, WithDeserializer, WithSerializer},
    xml::{Mixed, Text},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub type MixedAll = MixedAllType;
#[derive(Debug)]
pub struct MixedAllType {
    pub text_before: Option<Text>,
    pub fuu: Mixed<i32>,
    pub bar: Mixed<String>,
}
impl WithSerializer for MixedAllType {
    type Serializer<'x> = quick_xml_serialize::MixedAllTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::MixedAllTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::MixedAllTypeSerializerState::Init__),
            name: name.unwrap_or("tns:MixedAllType"),
            is_root,
        })
    }
}
impl WithDeserializer for MixedAllType {
    type Deserializer = quick_xml_deserialize::MixedAllTypeDeserializer;
}
pub type MixedChoice = MixedChoiceType;
#[derive(Debug)]
pub struct MixedChoiceType {
    pub text_before: Option<Text>,
    pub content: MixedChoiceTypeContent,
}
#[derive(Debug)]
pub enum MixedChoiceTypeContent {
    Fuu(Mixed<i32>),
    Bar(Mixed<String>),
}
impl WithSerializer for MixedChoiceType {
    type Serializer<'x> = quick_xml_serialize::MixedChoiceTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::MixedChoiceTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::MixedChoiceTypeSerializerState::Init__),
            name: name.unwrap_or("tns:MixedChoiceType"),
            is_root,
        })
    }
}
impl WithSerializer for MixedChoiceTypeContent {
    type Serializer<'x> = quick_xml_serialize::MixedChoiceTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::MixedChoiceTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::MixedChoiceTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for MixedChoiceType {
    type Deserializer = quick_xml_deserialize::MixedChoiceTypeDeserializer;
}
impl WithDeserializer for MixedChoiceTypeContent {
    type Deserializer = quick_xml_deserialize::MixedChoiceTypeContentDeserializer;
}
pub type MixedChoiceList = MixedChoiceListType;
#[derive(Debug)]
pub struct MixedChoiceListType {
    pub content: Vec<MixedChoiceListTypeContent>,
}
#[derive(Debug)]
pub enum MixedChoiceListTypeContent {
    Fuu(i32),
    Bar(String),
    Text(Text),
}
impl WithSerializer for MixedChoiceListType {
    type Serializer<'x> = quick_xml_serialize::MixedChoiceListTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::MixedChoiceListTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::MixedChoiceListTypeSerializerState::Init__),
            name: name.unwrap_or("tns:MixedChoiceListType"),
            is_root,
        })
    }
}
impl WithSerializer for MixedChoiceListTypeContent {
    type Serializer<'x> = quick_xml_serialize::MixedChoiceListTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::MixedChoiceListTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::MixedChoiceListTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for MixedChoiceListType {
    type Deserializer = quick_xml_deserialize::MixedChoiceListTypeDeserializer;
}
impl WithDeserializer for MixedChoiceListTypeContent {
    type Deserializer = quick_xml_deserialize::MixedChoiceListTypeContentDeserializer;
}
pub type MixedSequence = MixedSequenceType;
#[derive(Debug)]
pub struct MixedSequenceType {
    pub text_before: Option<Text>,
    pub fuu: i32,
    pub text_after_fuu: Option<Text>,
    pub bar: String,
    pub text_after_bar: Option<Text>,
}
impl WithSerializer for MixedSequenceType {
    type Serializer<'x> = quick_xml_serialize::MixedSequenceTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::MixedSequenceTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::MixedSequenceTypeSerializerState::Init__),
            name: name.unwrap_or("tns:MixedSequenceType"),
            is_root,
        })
    }
}
impl WithDeserializer for MixedSequenceType {
    type Deserializer = quick_xml_deserialize::MixedSequenceTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::{
        quick_xml::{
            filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer,
            DeserializerArtifact, DeserializerEvent, DeserializerOutput, DeserializerResult,
            ElementHandlerOutput, Error, ErrorKind, Event, RawByteStr, WithDeserializer,
        },
        xml::{Mixed, Text},
    };
    #[derive(Debug)]
    pub struct MixedAllTypeDeserializer {
        text_before: Option<Text>,
        fuu: Option<Mixed<i32>>,
        bar: Option<Mixed<String>>,
        state: Box<MixedAllTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum MixedAllTypeDeserializerState {
        Init__,
        Next__,
        TextBefore(<Text as WithDeserializer>::Deserializer),
        Fuu(<Mixed<i32> as WithDeserializer>::Deserializer),
        Bar(<Mixed<String> as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl MixedAllTypeDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<MixedAllTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Fuu")
                ) {
                    let output =
                        <Mixed<i32> as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_fuu(reader, output, &mut *fallback);
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Bar")
                ) {
                    let output =
                        <Mixed<String> as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_bar(reader, output, &mut *fallback);
                }
            }
            let output = <Text as WithDeserializer>::Deserializer::init(reader, event)?;
            self.handle_text_before(reader, output, &mut *fallback)
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
                text_before: None,
                fuu: None,
                bar: None,
                state: Box::new(MixedAllTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: MixedAllTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use MixedAllTypeDeserializerState as S;
            match state {
                S::TextBefore(deserializer) => {
                    self.store_text_before(deserializer.finish(reader)?)?
                }
                S::Fuu(deserializer) => self.store_fuu(deserializer.finish(reader)?)?,
                S::Bar(deserializer) => self.store_bar(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_text_before(&mut self, value: Text) -> Result<(), Error> {
            if self.text_before.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"text_before",
                )))?;
            }
            self.text_before = Some(value);
            Ok(())
        }
        fn store_fuu(&mut self, value: Mixed<i32>) -> Result<(), Error> {
            if self.fuu.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Fuu")))?;
            }
            self.fuu = Some(value);
            Ok(())
        }
        fn store_bar(&mut self, value: Mixed<String>) -> Result<(), Error> {
            if self.bar.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Bar")))?;
            }
            self.bar = Some(value);
            Ok(())
        }
        fn handle_text_before<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, Text>,
            fallback: &mut Option<MixedAllTypeDeserializerState>,
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
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                *self.state = match ret {
                    ElementHandlerOutput::Continue { .. } => MixedAllTypeDeserializerState::Next__,
                    ElementHandlerOutput::Break { .. } => fallback
                        .take()
                        .unwrap_or(MixedAllTypeDeserializerState::Next__),
                };
                return Ok(ret);
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_before(data)?;
                    *self.state = MixedAllTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(MixedAllTypeDeserializerState::TextBefore(
                                deserializer,
                            ));
                            *self.state = MixedAllTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = MixedAllTypeDeserializerState::TextBefore(deserializer);
                        }
                    }
                    ret
                }
            })
        }
        fn handle_fuu<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, Mixed<i32>>,
            fallback: &mut Option<MixedAllTypeDeserializerState>,
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
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                *self.state = match ret {
                    ElementHandlerOutput::Continue { .. } => MixedAllTypeDeserializerState::Next__,
                    ElementHandlerOutput::Break { .. } => fallback
                        .take()
                        .unwrap_or(MixedAllTypeDeserializerState::Next__),
                };
                return Ok(ret);
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_fuu(data)?;
                    *self.state = MixedAllTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(MixedAllTypeDeserializerState::Fuu(deserializer));
                            *self.state = MixedAllTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = MixedAllTypeDeserializerState::Fuu(deserializer);
                        }
                    }
                    ret
                }
            })
        }
        fn handle_bar<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, Mixed<String>>,
            fallback: &mut Option<MixedAllTypeDeserializerState>,
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
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                *self.state = match ret {
                    ElementHandlerOutput::Continue { .. } => MixedAllTypeDeserializerState::Next__,
                    ElementHandlerOutput::Break { .. } => fallback
                        .take()
                        .unwrap_or(MixedAllTypeDeserializerState::Next__),
                };
                return Ok(ret);
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_bar(data)?;
                    *self.state = MixedAllTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(MixedAllTypeDeserializerState::Bar(deserializer));
                            *self.state = MixedAllTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = MixedAllTypeDeserializerState::Bar(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::MixedAllType> for MixedAllTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::MixedAllType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedAllType>
        where
            R: DeserializeReader,
        {
            use MixedAllTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::TextBefore(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_before(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, .. } => event,
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Fuu(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fuu(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, .. } => event,
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Bar(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_bar(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, .. } => event,
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
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
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, .. } => event,
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::MixedAllType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, MixedAllTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::MixedAllType {
                text_before: self.text_before,
                fuu: self
                    .fuu
                    .ok_or_else(|| ErrorKind::MissingElement("Fuu".into()))?,
                bar: self
                    .bar
                    .ok_or_else(|| ErrorKind::MissingElement("Bar".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct MixedChoiceTypeDeserializer {
        text_before: Option<Text>,
        content: Option<super::MixedChoiceTypeContent>,
        state: Box<MixedChoiceTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum MixedChoiceTypeDeserializerState {
        Init__,
        TextBefore(Option<<Text as WithDeserializer>::Deserializer>),
        Content(Option<<super::MixedChoiceTypeContent as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl MixedChoiceTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                text_before: None,
                content: None,
                state: Box::new(MixedChoiceTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: MixedChoiceTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use MixedChoiceTypeDeserializerState as S;
            match state {
                S::TextBefore(Some(deserializer)) => {
                    self.store_text_before(deserializer.finish(reader)?)?
                }
                S::Content(Some(deserializer)) => {
                    self.store_content(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_text_before(&mut self, value: Text) -> Result<(), Error> {
            if self.text_before.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"text_before",
                )))?;
            }
            self.text_before = Some(value);
            Ok(())
        }
        fn store_content(&mut self, value: super::MixedChoiceTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"content",
                )))?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_text_before<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, Text>,
            fallback: &mut Option<MixedChoiceTypeDeserializerState>,
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
                fallback.get_or_insert(MixedChoiceTypeDeserializerState::TextBefore(None));
                *self.state = MixedChoiceTypeDeserializerState::Content(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_before(data)?;
                    *self.state = MixedChoiceTypeDeserializerState::Content(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(MixedChoiceTypeDeserializerState::TextBefore(
                                Some(deserializer),
                            ));
                            *self.state = MixedChoiceTypeDeserializerState::Content(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                MixedChoiceTypeDeserializerState::TextBefore(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::MixedChoiceTypeContent>,
            fallback: &mut Option<MixedChoiceTypeDeserializerState>,
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
                if self.content.is_some() {
                    fallback.get_or_insert(MixedChoiceTypeDeserializerState::Content(None));
                    *self.state = MixedChoiceTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = MixedChoiceTypeDeserializerState::Content(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = MixedChoiceTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(MixedChoiceTypeDeserializerState::Content(
                                Some(deserializer),
                            ));
                            *self.state = MixedChoiceTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                MixedChoiceTypeDeserializerState::Content(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::MixedChoiceType> for MixedChoiceTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::MixedChoiceType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedChoiceType>
        where
            R: DeserializeReader,
        {
            use MixedChoiceTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::TextBefore(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_before(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Content(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
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
                        *self.state = MixedChoiceTypeDeserializerState::TextBefore(None);
                        event
                    }
                    (S::TextBefore(None), event) => {
                        let output = <Text as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_text_before(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Content(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = < super :: MixedChoiceTypeContent as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content(reader, output, &mut fallback)? {
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
        fn finish<R>(mut self, reader: &R) -> Result<super::MixedChoiceType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                MixedChoiceTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::MixedChoiceType {
                text_before: self.text_before,
                content: self
                    .content
                    .ok_or_else(|| ErrorKind::MissingElement("content".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct MixedChoiceTypeContentDeserializer {
        state: Box<MixedChoiceTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum MixedChoiceTypeContentDeserializerState {
        Init__,
        Fuu(
            Option<Mixed<i32>>,
            Option<<Mixed<i32> as WithDeserializer>::Deserializer>,
        ),
        Bar(
            Option<Mixed<String>>,
            Option<<Mixed<String> as WithDeserializer>::Deserializer>,
        ),
        Done__(super::MixedChoiceTypeContent),
        Unknown__,
    }
    impl MixedChoiceTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<MixedChoiceTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Fuu")
                ) {
                    let output =
                        <Mixed<i32> as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_fuu(reader, Default::default(), output, &mut *fallback);
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Bar")
                ) {
                    let output =
                        <Mixed<String> as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_bar(reader, Default::default(), output, &mut *fallback);
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(MixedChoiceTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: MixedChoiceTypeContentDeserializerState,
        ) -> Result<super::MixedChoiceTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use MixedChoiceTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Fuu(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_fuu(&mut values, value)?;
                    }
                    Ok(super::MixedChoiceTypeContent::Fuu(
                        values.ok_or_else(|| ErrorKind::MissingElement("Fuu".into()))?,
                    ))
                }
                S::Bar(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_bar(&mut values, value)?;
                    }
                    Ok(super::MixedChoiceTypeContent::Bar(
                        values.ok_or_else(|| ErrorKind::MissingElement("Bar".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
            }
        }
        fn store_fuu(values: &mut Option<Mixed<i32>>, value: Mixed<i32>) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Fuu")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_bar(
            values: &mut Option<Mixed<String>>,
            value: Mixed<String>,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Bar")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_fuu<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<Mixed<i32>>,
            output: DeserializerOutput<'de, Mixed<i32>>,
            fallback: &mut Option<MixedChoiceTypeContentDeserializerState>,
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
                    None if values.is_none() => {
                        *self.state = MixedChoiceTypeContentDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => MixedChoiceTypeContentDeserializerState::Fuu(values, None),
                    Some(MixedChoiceTypeContentDeserializerState::Fuu(_, Some(deserializer))) => {
                        MixedChoiceTypeContentDeserializerState::Fuu(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(MixedChoiceTypeContentDeserializerState::Fuu(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_fuu(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fuu(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        MixedChoiceTypeContentDeserializerState::Fuu(values, None),
                    )?;
                    *self.state = MixedChoiceTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        MixedChoiceTypeContentDeserializerState::Fuu(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_bar<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<Mixed<String>>,
            output: DeserializerOutput<'de, Mixed<String>>,
            fallback: &mut Option<MixedChoiceTypeContentDeserializerState>,
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
                    None if values.is_none() => {
                        *self.state = MixedChoiceTypeContentDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => MixedChoiceTypeContentDeserializerState::Bar(values, None),
                    Some(MixedChoiceTypeContentDeserializerState::Bar(_, Some(deserializer))) => {
                        MixedChoiceTypeContentDeserializerState::Bar(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(MixedChoiceTypeContentDeserializerState::Bar(_, Some(deserializer))) => {
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
                        MixedChoiceTypeContentDeserializerState::Bar(values, None),
                    )?;
                    *self.state = MixedChoiceTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        MixedChoiceTypeContentDeserializerState::Bar(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::MixedChoiceTypeContent> for MixedChoiceTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedChoiceTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(MixedChoiceTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, MixedChoiceTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::MixedChoiceTypeContent>
        where
            R: DeserializeReader,
        {
            use MixedChoiceTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Fuu(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fuu(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bar(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_bar(reader, values, output, &mut fallback)? {
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
                    (S::Fuu(values, None), event) => {
                        let output =
                            <Mixed<i32> as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_fuu(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bar(values, None), event) => {
                        let output =
                            <Mixed<String> as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_bar(reader, values, output, &mut fallback)? {
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
        fn finish<R>(self, reader: &R) -> Result<super::MixedChoiceTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct MixedChoiceListTypeDeserializer {
        content: Vec<super::MixedChoiceListTypeContent>,
        state: Box<MixedChoiceListTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum MixedChoiceListTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::MixedChoiceListTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl MixedChoiceListTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                content: Vec::new(),
                state: Box::new(MixedChoiceListTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: MixedChoiceListTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let MixedChoiceListTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::MixedChoiceListTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::MixedChoiceListTypeContent>,
            fallback: &mut Option<MixedChoiceListTypeDeserializerState>,
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
                *self.state = fallback
                    .take()
                    .unwrap_or(MixedChoiceListTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = MixedChoiceListTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                MixedChoiceListTypeDeserializerState::Content__(deserializer);
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                MixedChoiceListTypeDeserializerState::Content__(deserializer),
                            );
                            *self.state = MixedChoiceListTypeDeserializerState::Next__;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::MixedChoiceListType> for MixedChoiceListTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedChoiceListType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedChoiceListType>
        where
            R: DeserializeReader,
        {
            use MixedChoiceListTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
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
                        let output = < super :: MixedChoiceListTypeContent as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::MixedChoiceListType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                MixedChoiceListTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::MixedChoiceListType {
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct MixedChoiceListTypeContentDeserializer {
        state: Box<MixedChoiceListTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum MixedChoiceListTypeContentDeserializerState {
        Init__,
        Fuu(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Bar(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Text(
            Option<Text>,
            Option<<Text as WithDeserializer>::Deserializer>,
        ),
        Done__(super::MixedChoiceListTypeContent),
        Unknown__,
    }
    impl MixedChoiceListTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<MixedChoiceListTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Fuu")
                ) {
                    let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_fuu(reader, Default::default(), output, &mut *fallback);
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Bar")
                ) {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_bar(reader, Default::default(), output, &mut *fallback);
                }
            }
            let output = <Text as WithDeserializer>::Deserializer::init(reader, event)?;
            self.handle_text(reader, Default::default(), output, &mut *fallback)
        }
        fn finish_state<R>(
            reader: &R,
            state: MixedChoiceListTypeContentDeserializerState,
        ) -> Result<super::MixedChoiceListTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use MixedChoiceListTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Fuu(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_fuu(&mut values, value)?;
                    }
                    Ok(super::MixedChoiceListTypeContent::Fuu(
                        values.ok_or_else(|| ErrorKind::MissingElement("Fuu".into()))?,
                    ))
                }
                S::Bar(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_bar(&mut values, value)?;
                    }
                    Ok(super::MixedChoiceListTypeContent::Bar(
                        values.ok_or_else(|| ErrorKind::MissingElement("Bar".into()))?,
                    ))
                }
                S::Text(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_text(&mut values, value)?;
                    }
                    Ok(super::MixedChoiceListTypeContent::Text(
                        values.ok_or_else(|| ErrorKind::MissingElement("text".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
            }
        }
        fn store_fuu(values: &mut Option<i32>, value: i32) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Fuu")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_bar(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Bar")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_text(values: &mut Option<Text>, value: Text) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"text")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_fuu<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<MixedChoiceListTypeContentDeserializerState>,
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
                    None if values.is_none() => {
                        *self.state = MixedChoiceListTypeContentDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => MixedChoiceListTypeContentDeserializerState::Fuu(values, None),
                    Some(MixedChoiceListTypeContentDeserializerState::Fuu(
                        _,
                        Some(deserializer),
                    )) => {
                        MixedChoiceListTypeContentDeserializerState::Fuu(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(MixedChoiceListTypeContentDeserializerState::Fuu(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_fuu(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fuu(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        MixedChoiceListTypeContentDeserializerState::Fuu(values, None),
                    )?;
                    *self.state = MixedChoiceListTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = MixedChoiceListTypeContentDeserializerState::Fuu(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_bar<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<MixedChoiceListTypeContentDeserializerState>,
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
                    None if values.is_none() => {
                        *self.state = MixedChoiceListTypeContentDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => MixedChoiceListTypeContentDeserializerState::Bar(values, None),
                    Some(MixedChoiceListTypeContentDeserializerState::Bar(
                        _,
                        Some(deserializer),
                    )) => {
                        MixedChoiceListTypeContentDeserializerState::Bar(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(MixedChoiceListTypeContentDeserializerState::Bar(_, Some(deserializer))) => {
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
                        MixedChoiceListTypeContentDeserializerState::Bar(values, None),
                    )?;
                    *self.state = MixedChoiceListTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = MixedChoiceListTypeContentDeserializerState::Bar(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_text<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<Text>,
            output: DeserializerOutput<'de, Text>,
            fallback: &mut Option<MixedChoiceListTypeContentDeserializerState>,
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
                    None if values.is_none() => {
                        *self.state = MixedChoiceListTypeContentDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => MixedChoiceListTypeContentDeserializerState::Text(values, None),
                    Some(MixedChoiceListTypeContentDeserializerState::Text(
                        _,
                        Some(deserializer),
                    )) => MixedChoiceListTypeContentDeserializerState::Text(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(MixedChoiceListTypeContentDeserializerState::Text(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_text(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_text(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        MixedChoiceListTypeContentDeserializerState::Text(values, None),
                    )?;
                    *self.state = MixedChoiceListTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = MixedChoiceListTypeContentDeserializerState::Text(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::MixedChoiceListTypeContent>
        for MixedChoiceListTypeContentDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedChoiceListTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(MixedChoiceListTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state,
                        MixedChoiceListTypeContentDeserializerState::Init__
                    ) =>
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
        ) -> DeserializerResult<'de, super::MixedChoiceListTypeContent>
        where
            R: DeserializeReader,
        {
            use MixedChoiceListTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Fuu(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fuu(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bar(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_bar(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Text(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text(reader, values, output, &mut fallback)? {
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
                    (S::Fuu(values, None), event) => {
                        let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_fuu(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
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
                    (S::Text(values, None), event) => {
                        let output = <Text as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_text(reader, values, output, &mut fallback)? {
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
        fn finish<R>(self, reader: &R) -> Result<super::MixedChoiceListTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct MixedSequenceTypeDeserializer {
        text_before: Option<Text>,
        fuu: Option<i32>,
        text_after_fuu: Option<Text>,
        bar: Option<String>,
        text_after_bar: Option<Text>,
        state: Box<MixedSequenceTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum MixedSequenceTypeDeserializerState {
        Init__,
        TextBefore(Option<<Text as WithDeserializer>::Deserializer>),
        Fuu(Option<<i32 as WithDeserializer>::Deserializer>),
        TextAfterFuu(Option<<Text as WithDeserializer>::Deserializer>),
        Bar(Option<<String as WithDeserializer>::Deserializer>),
        TextAfterBar(Option<<Text as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl MixedSequenceTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                text_before: None,
                fuu: None,
                text_after_fuu: None,
                bar: None,
                text_after_bar: None,
                state: Box::new(MixedSequenceTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: MixedSequenceTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use MixedSequenceTypeDeserializerState as S;
            match state {
                S::TextBefore(Some(deserializer)) => {
                    self.store_text_before(deserializer.finish(reader)?)?
                }
                S::Fuu(Some(deserializer)) => self.store_fuu(deserializer.finish(reader)?)?,
                S::TextAfterFuu(Some(deserializer)) => {
                    self.store_text_after_fuu(deserializer.finish(reader)?)?
                }
                S::Bar(Some(deserializer)) => self.store_bar(deserializer.finish(reader)?)?,
                S::TextAfterBar(Some(deserializer)) => {
                    self.store_text_after_bar(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_text_before(&mut self, value: Text) -> Result<(), Error> {
            if self.text_before.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"text_before",
                )))?;
            }
            self.text_before = Some(value);
            Ok(())
        }
        fn store_fuu(&mut self, value: i32) -> Result<(), Error> {
            if self.fuu.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Fuu")))?;
            }
            self.fuu = Some(value);
            Ok(())
        }
        fn store_text_after_fuu(&mut self, value: Text) -> Result<(), Error> {
            if self.text_after_fuu.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"text_after_Fuu",
                )))?;
            }
            self.text_after_fuu = Some(value);
            Ok(())
        }
        fn store_bar(&mut self, value: String) -> Result<(), Error> {
            if self.bar.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Bar")))?;
            }
            self.bar = Some(value);
            Ok(())
        }
        fn store_text_after_bar(&mut self, value: Text) -> Result<(), Error> {
            if self.text_after_bar.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"text_after_Bar",
                )))?;
            }
            self.text_after_bar = Some(value);
            Ok(())
        }
        fn handle_text_before<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, Text>,
            fallback: &mut Option<MixedSequenceTypeDeserializerState>,
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
                fallback.get_or_insert(MixedSequenceTypeDeserializerState::TextBefore(None));
                *self.state = MixedSequenceTypeDeserializerState::Fuu(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_before(data)?;
                    *self.state = MixedSequenceTypeDeserializerState::Fuu(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(MixedSequenceTypeDeserializerState::TextBefore(
                                Some(deserializer),
                            ));
                            *self.state = MixedSequenceTypeDeserializerState::Fuu(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                MixedSequenceTypeDeserializerState::TextBefore(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_fuu<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<MixedSequenceTypeDeserializerState>,
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
                if self.fuu.is_some() {
                    fallback.get_or_insert(MixedSequenceTypeDeserializerState::Fuu(None));
                    *self.state = MixedSequenceTypeDeserializerState::TextAfterFuu(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = MixedSequenceTypeDeserializerState::Fuu(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_fuu(data)?;
                    *self.state = MixedSequenceTypeDeserializerState::TextAfterFuu(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(MixedSequenceTypeDeserializerState::Fuu(Some(
                                deserializer,
                            )));
                            *self.state = MixedSequenceTypeDeserializerState::TextAfterFuu(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                MixedSequenceTypeDeserializerState::Fuu(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_text_after_fuu<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, Text>,
            fallback: &mut Option<MixedSequenceTypeDeserializerState>,
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
                fallback.get_or_insert(MixedSequenceTypeDeserializerState::TextAfterFuu(None));
                *self.state = MixedSequenceTypeDeserializerState::Bar(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_after_fuu(data)?;
                    *self.state = MixedSequenceTypeDeserializerState::Bar(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                MixedSequenceTypeDeserializerState::TextAfterFuu(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = MixedSequenceTypeDeserializerState::Bar(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = MixedSequenceTypeDeserializerState::TextAfterFuu(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_bar<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<MixedSequenceTypeDeserializerState>,
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
                if self.bar.is_some() {
                    fallback.get_or_insert(MixedSequenceTypeDeserializerState::Bar(None));
                    *self.state = MixedSequenceTypeDeserializerState::TextAfterBar(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = MixedSequenceTypeDeserializerState::Bar(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_bar(data)?;
                    *self.state = MixedSequenceTypeDeserializerState::TextAfterBar(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(MixedSequenceTypeDeserializerState::Bar(Some(
                                deserializer,
                            )));
                            *self.state = MixedSequenceTypeDeserializerState::TextAfterBar(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                MixedSequenceTypeDeserializerState::Bar(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_text_after_bar<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, Text>,
            fallback: &mut Option<MixedSequenceTypeDeserializerState>,
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
                fallback.get_or_insert(MixedSequenceTypeDeserializerState::TextAfterBar(None));
                *self.state = MixedSequenceTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_after_bar(data)?;
                    *self.state = MixedSequenceTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                MixedSequenceTypeDeserializerState::TextAfterBar(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = MixedSequenceTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = MixedSequenceTypeDeserializerState::TextAfterBar(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::MixedSequenceType> for MixedSequenceTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedSequenceType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedSequenceType>
        where
            R: DeserializeReader,
        {
            use MixedSequenceTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::TextBefore(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_before(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Fuu(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fuu(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TextAfterFuu(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_after_fuu(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Bar(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_bar(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TextAfterBar(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_after_bar(reader, output, &mut fallback)? {
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
                        *self.state = MixedSequenceTypeDeserializerState::TextBefore(None);
                        event
                    }
                    (S::TextBefore(None), event) => {
                        let output = <Text as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_text_before(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Fuu(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Fuu",
                            false,
                        )?;
                        match self.handle_fuu(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TextAfterFuu(None), event) => {
                        let output = <Text as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_text_after_fuu(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Bar(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Bar",
                            false,
                        )?;
                        match self.handle_bar(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TextAfterBar(None), event) => {
                        let output = <Text as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_text_after_bar(reader, output, &mut fallback)? {
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
        fn finish<R>(mut self, reader: &R) -> Result<super::MixedSequenceType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                MixedSequenceTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::MixedSequenceType {
                text_before: self.text_before,
                fuu: self
                    .fuu
                    .ok_or_else(|| ErrorKind::MissingElement("Fuu".into()))?,
                text_after_fuu: self.text_after_fuu,
                bar: self
                    .bar
                    .ok_or_else(|| ErrorKind::MissingElement("Bar".into()))?,
                text_after_bar: self.text_after_bar,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use core::iter::Iterator;
    use xsd_parser::{
        quick_xml::{BytesEnd, BytesStart, Error, Event, IterSerializer, WithSerializer},
        xml::{Mixed, Text},
    };
    #[derive(Debug)]
    pub struct MixedAllTypeSerializer<'ser> {
        pub(super) value: &'ser super::MixedAllType,
        pub(super) state: Box<MixedAllTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum MixedAllTypeSerializerState<'ser> {
        Init__,
        TextBefore(IterSerializer<'ser, Option<&'ser Text>, Text>),
        Fuu(<Mixed<i32> as WithSerializer>::Serializer<'ser>),
        Bar(<Mixed<String> as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> MixedAllTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MixedAllTypeSerializerState::Init__ => {
                        *self.state = MixedAllTypeSerializerState::TextBefore(IterSerializer::new(
                            self.value.text_before.as_ref(),
                            Some(""),
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    MixedAllTypeSerializerState::TextBefore(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                MixedAllTypeSerializerState::Fuu(WithSerializer::serializer(
                                    &self.value.fuu,
                                    Some("tns:Fuu"),
                                    false,
                                )?)
                        }
                    },
                    MixedAllTypeSerializerState::Fuu(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                MixedAllTypeSerializerState::Bar(WithSerializer::serializer(
                                    &self.value.bar,
                                    Some("tns:Bar"),
                                    false,
                                )?)
                        }
                    },
                    MixedAllTypeSerializerState::Bar(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = MixedAllTypeSerializerState::End__,
                    },
                    MixedAllTypeSerializerState::End__ => {
                        *self.state = MixedAllTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    MixedAllTypeSerializerState::Done__ => return Ok(None),
                    MixedAllTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for MixedAllTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = MixedAllTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct MixedChoiceTypeSerializer<'ser> {
        pub(super) value: &'ser super::MixedChoiceType,
        pub(super) state: Box<MixedChoiceTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum MixedChoiceTypeSerializerState<'ser> {
        Init__,
        TextBefore(IterSerializer<'ser, Option<&'ser Text>, Text>),
        Content(<super::MixedChoiceTypeContent as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> MixedChoiceTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MixedChoiceTypeSerializerState::Init__ => {
                        *self.state = MixedChoiceTypeSerializerState::TextBefore(
                            IterSerializer::new(self.value.text_before.as_ref(), Some(""), false),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    MixedChoiceTypeSerializerState::TextBefore(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = MixedChoiceTypeSerializerState::Content(
                                WithSerializer::serializer(&self.value.content, Some(""), false)?,
                            )
                        }
                    },
                    MixedChoiceTypeSerializerState::Content(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = MixedChoiceTypeSerializerState::End__,
                    },
                    MixedChoiceTypeSerializerState::End__ => {
                        *self.state = MixedChoiceTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    MixedChoiceTypeSerializerState::Done__ => return Ok(None),
                    MixedChoiceTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for MixedChoiceTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = MixedChoiceTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct MixedChoiceTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::MixedChoiceTypeContent,
        pub(super) state: Box<MixedChoiceTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum MixedChoiceTypeContentSerializerState<'ser> {
        Init__,
        Fuu(<Mixed<i32> as WithSerializer>::Serializer<'ser>),
        Bar(<Mixed<String> as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> MixedChoiceTypeContentSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MixedChoiceTypeContentSerializerState::Init__ => match self.value {
                        super::MixedChoiceTypeContent::Fuu(x) => {
                            *self.state = MixedChoiceTypeContentSerializerState::Fuu(
                                WithSerializer::serializer(x, Some("tns:Fuu"), false)?,
                            )
                        }
                        super::MixedChoiceTypeContent::Bar(x) => {
                            *self.state = MixedChoiceTypeContentSerializerState::Bar(
                                WithSerializer::serializer(x, Some("tns:Bar"), false)?,
                            )
                        }
                    },
                    MixedChoiceTypeContentSerializerState::Fuu(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = MixedChoiceTypeContentSerializerState::Done__,
                    },
                    MixedChoiceTypeContentSerializerState::Bar(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = MixedChoiceTypeContentSerializerState::Done__,
                    },
                    MixedChoiceTypeContentSerializerState::Done__ => return Ok(None),
                    MixedChoiceTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for MixedChoiceTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = MixedChoiceTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct MixedChoiceListTypeSerializer<'ser> {
        pub(super) value: &'ser super::MixedChoiceListType,
        pub(super) state: Box<MixedChoiceListTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum MixedChoiceListTypeSerializerState<'ser> {
        Init__,
        Content__(
            IterSerializer<
                'ser,
                &'ser [super::MixedChoiceListTypeContent],
                super::MixedChoiceListTypeContent,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> MixedChoiceListTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MixedChoiceListTypeSerializerState::Init__ => {
                        *self.state = MixedChoiceListTypeSerializerState::Content__(
                            IterSerializer::new(&self.value.content[..], None, false),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    MixedChoiceListTypeSerializerState::Content__(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = MixedChoiceListTypeSerializerState::End__,
                        }
                    }
                    MixedChoiceListTypeSerializerState::End__ => {
                        *self.state = MixedChoiceListTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    MixedChoiceListTypeSerializerState::Done__ => return Ok(None),
                    MixedChoiceListTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for MixedChoiceListTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = MixedChoiceListTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct MixedChoiceListTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::MixedChoiceListTypeContent,
        pub(super) state: Box<MixedChoiceListTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum MixedChoiceListTypeContentSerializerState<'ser> {
        Init__,
        Fuu(<i32 as WithSerializer>::Serializer<'ser>),
        Bar(<String as WithSerializer>::Serializer<'ser>),
        Text(<Text as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> MixedChoiceListTypeContentSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MixedChoiceListTypeContentSerializerState::Init__ => match self.value {
                        super::MixedChoiceListTypeContent::Fuu(x) => {
                            *self.state = MixedChoiceListTypeContentSerializerState::Fuu(
                                WithSerializer::serializer(x, Some("tns:Fuu"), false)?,
                            )
                        }
                        super::MixedChoiceListTypeContent::Bar(x) => {
                            *self.state = MixedChoiceListTypeContentSerializerState::Bar(
                                WithSerializer::serializer(x, Some("tns:Bar"), false)?,
                            )
                        }
                        super::MixedChoiceListTypeContent::Text(x) => {
                            *self.state = MixedChoiceListTypeContentSerializerState::Text(
                                WithSerializer::serializer(x, Some("text"), false)?,
                            )
                        }
                    },
                    MixedChoiceListTypeContentSerializerState::Fuu(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = MixedChoiceListTypeContentSerializerState::Done__,
                        }
                    }
                    MixedChoiceListTypeContentSerializerState::Bar(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = MixedChoiceListTypeContentSerializerState::Done__,
                        }
                    }
                    MixedChoiceListTypeContentSerializerState::Text(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = MixedChoiceListTypeContentSerializerState::Done__,
                        }
                    }
                    MixedChoiceListTypeContentSerializerState::Done__ => return Ok(None),
                    MixedChoiceListTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for MixedChoiceListTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = MixedChoiceListTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct MixedSequenceTypeSerializer<'ser> {
        pub(super) value: &'ser super::MixedSequenceType,
        pub(super) state: Box<MixedSequenceTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum MixedSequenceTypeSerializerState<'ser> {
        Init__,
        TextBefore(IterSerializer<'ser, Option<&'ser Text>, Text>),
        Fuu(<i32 as WithSerializer>::Serializer<'ser>),
        TextAfterFuu(IterSerializer<'ser, Option<&'ser Text>, Text>),
        Bar(<String as WithSerializer>::Serializer<'ser>),
        TextAfterBar(IterSerializer<'ser, Option<&'ser Text>, Text>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> MixedSequenceTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MixedSequenceTypeSerializerState::Init__ => {
                        *self.state =
                            MixedSequenceTypeSerializerState::TextBefore(IterSerializer::new(
                                self.value.text_before.as_ref(),
                                Some("text_before"),
                                false,
                            ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    MixedSequenceTypeSerializerState::TextBefore(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = MixedSequenceTypeSerializerState::Fuu(
                                    WithSerializer::serializer(
                                        &self.value.fuu,
                                        Some("tns:Fuu"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    MixedSequenceTypeSerializerState::Fuu(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                MixedSequenceTypeSerializerState::TextAfterFuu(IterSerializer::new(
                                    self.value.text_after_fuu.as_ref(),
                                    Some("text_after_Fuu"),
                                    false,
                                ))
                        }
                    },
                    MixedSequenceTypeSerializerState::TextAfterFuu(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = MixedSequenceTypeSerializerState::Bar(
                                    WithSerializer::serializer(
                                        &self.value.bar,
                                        Some("tns:Bar"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    MixedSequenceTypeSerializerState::Bar(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                MixedSequenceTypeSerializerState::TextAfterBar(IterSerializer::new(
                                    self.value.text_after_bar.as_ref(),
                                    Some("text_after_Bar"),
                                    false,
                                ))
                        }
                    },
                    MixedSequenceTypeSerializerState::TextAfterBar(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = MixedSequenceTypeSerializerState::End__,
                        }
                    }
                    MixedSequenceTypeSerializerState::End__ => {
                        *self.state = MixedSequenceTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    MixedSequenceTypeSerializerState::Done__ => return Ok(None),
                    MixedSequenceTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for MixedSequenceTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = MixedSequenceTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
