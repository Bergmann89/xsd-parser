use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub type Simple = SimpleType;
#[derive(Debug)]
pub struct SimpleType {
    pub attrib_a: Option<xs::StringType>,
    pub attrib_b: Option<xs::StringType>,
    pub content: xs::StringType,
}
impl WithSerializer for SimpleType {
    type Serializer<'x> = quick_xml_serialize::SimpleTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::SimpleTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SimpleTypeSerializerState::Init__),
            name: name.unwrap_or("SimpleType"),
            is_root,
        })
    }
}
impl WithDeserializer for SimpleType {
    type Deserializer = quick_xml_deserialize::SimpleTypeDeserializer;
}
pub type Sequence = SequenceType;
#[derive(Debug)]
pub struct SequenceType {
    pub attrib_a: Option<xs::StringType>,
    pub attrib_b: Option<xs::StringType>,
    pub content: SequenceTypeContent,
}
#[derive(Debug)]
pub struct SequenceTypeContent {
    pub a: Option<xs::StringType>,
    pub b: Option<xs::StringType>,
    pub c: Option<xs::StringType>,
}
impl WithSerializer for SequenceType {
    type Serializer<'x> = quick_xml_serialize::SequenceTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::SequenceTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SequenceTypeSerializerState::Init__),
            name: name.unwrap_or("SequenceType"),
            is_root,
        })
    }
}
impl WithSerializer for SequenceTypeContent {
    type Serializer<'x> = quick_xml_serialize::SequenceTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::SequenceTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SequenceTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for SequenceType {
    type Deserializer = quick_xml_deserialize::SequenceTypeDeserializer;
}
impl WithDeserializer for SequenceTypeContent {
    type Deserializer = quick_xml_deserialize::SequenceTypeContentDeserializer;
}
pub type NestedSeq = NestedSeqType;
#[derive(Debug)]
pub struct NestedSeqType {
    pub attrib_a: Option<xs::StringType>,
    pub attrib_b: Option<xs::StringType>,
    pub content: NestedSeqTypeContent,
}
#[derive(Debug)]
pub struct NestedSeqTypeContent {
    pub inner_choice: Option<NestedSeqInnerChoiceType>,
    pub d: Option<xs::StringType>,
}
impl WithSerializer for NestedSeqType {
    type Serializer<'x> = quick_xml_serialize::NestedSeqTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::NestedSeqTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::NestedSeqTypeSerializerState::Init__),
            name: name.unwrap_or("NestedSeqType"),
            is_root,
        })
    }
}
impl WithSerializer for NestedSeqTypeContent {
    type Serializer<'x> = quick_xml_serialize::NestedSeqTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::NestedSeqTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::NestedSeqTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for NestedSeqType {
    type Deserializer = quick_xml_deserialize::NestedSeqTypeDeserializer;
}
impl WithDeserializer for NestedSeqTypeContent {
    type Deserializer = quick_xml_deserialize::NestedSeqTypeContentDeserializer;
}
#[derive(Debug)]
pub struct NestedSeqInnerChoiceType {
    pub content: NestedSeqInnerChoiceTypeContent,
}
#[derive(Debug)]
pub enum NestedSeqInnerChoiceTypeContent {
    FinalSeq(NestedSeqFinalSeqType),
}
impl WithSerializer for NestedSeqInnerChoiceType {
    type Serializer<'x> = quick_xml_serialize::NestedSeqInnerChoiceTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::NestedSeqInnerChoiceTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::NestedSeqInnerChoiceTypeSerializerState::Init__),
        })
    }
}
impl WithSerializer for NestedSeqInnerChoiceTypeContent {
    type Serializer<'x> = quick_xml_serialize::NestedSeqInnerChoiceTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(
            quick_xml_serialize::NestedSeqInnerChoiceTypeContentSerializer {
                value: self,
                state: Box::new(
                    quick_xml_serialize::NestedSeqInnerChoiceTypeContentSerializerState::Init__,
                ),
            },
        )
    }
}
impl WithDeserializer for NestedSeqInnerChoiceType {
    type Deserializer = quick_xml_deserialize::NestedSeqInnerChoiceTypeDeserializer;
}
impl WithDeserializer for NestedSeqInnerChoiceTypeContent {
    type Deserializer = quick_xml_deserialize::NestedSeqInnerChoiceTypeContentDeserializer;
}
#[derive(Debug)]
pub struct NestedSeqFinalSeqType {
    pub content: NestedSeqFinalSeqTypeContent,
}
#[derive(Debug)]
pub struct NestedSeqFinalSeqTypeContent {
    pub a: Option<xs::StringType>,
    pub b: Option<xs::StringType>,
    pub c: Option<xs::StringType>,
}
impl WithSerializer for NestedSeqFinalSeqType {
    type Serializer<'x> = quick_xml_serialize::NestedSeqFinalSeqTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::NestedSeqFinalSeqTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::NestedSeqFinalSeqTypeSerializerState::Init__),
        })
    }
}
impl WithSerializer for NestedSeqFinalSeqTypeContent {
    type Serializer<'x> = quick_xml_serialize::NestedSeqFinalSeqTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(
            quick_xml_serialize::NestedSeqFinalSeqTypeContentSerializer {
                value: self,
                state: Box::new(
                    quick_xml_serialize::NestedSeqFinalSeqTypeContentSerializerState::Init__,
                ),
            },
        )
    }
}
impl WithDeserializer for NestedSeqFinalSeqType {
    type Deserializer = quick_xml_deserialize::NestedSeqFinalSeqTypeDeserializer;
}
impl WithDeserializer for NestedSeqFinalSeqTypeContent {
    type Deserializer = quick_xml_deserialize::NestedSeqFinalSeqTypeContentDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, ContentDeserializer, DeserializeHelper, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
        ErrorKind, Event, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct SimpleTypeDeserializer {
        attrib_a: Option<super::xs::StringType>,
        attrib_b: Option<super::xs::StringType>,
        content: Option<super::xs::StringType>,
        state__: Box<SimpleTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SimpleTypeDeserializerState {
        Init__,
        Content__(<super::xs::StringType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl SimpleTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut attrib_a: Option<super::xs::StringType> = None;
            let mut attrib_b: Option<super::xs::StringType> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"attrib-a" {
                    helper.read_attrib(&mut attrib_a, b"attrib-a", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"attrib-b" {
                    helper.read_attrib(&mut attrib_b, b"attrib-b", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                attrib_a: attrib_a,
                attrib_b: attrib_b,
                content: None,
                state__: Box::new(SimpleTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SimpleTypeDeserializerState,
        ) -> Result<(), Error> {
            if let SimpleTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::xs::StringType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::xs::StringType>,
        ) -> DeserializerResult<'de, super::SimpleType> {
            use SimpleTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SimpleType> for SimpleTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SimpleType> {
            use SimpleTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::SimpleType, Error> {
            let state = replace(&mut *self.state__, SimpleTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::SimpleType {
                attrib_a: self.attrib_a,
                attrib_b: self.attrib_b,
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct SequenceTypeDeserializer {
        attrib_a: Option<super::xs::StringType>,
        attrib_b: Option<super::xs::StringType>,
        content: Option<super::SequenceTypeContent>,
        state__: Box<SequenceTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SequenceTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::SequenceTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl SequenceTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut attrib_a: Option<super::xs::StringType> = None;
            let mut attrib_b: Option<super::xs::StringType> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"attrib-a" {
                    helper.read_attrib(&mut attrib_a, b"attrib-a", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"attrib-b" {
                    helper.read_attrib(&mut attrib_b, b"attrib-b", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                attrib_a: attrib_a,
                attrib_b: attrib_b,
                content: None,
                state__: Box::new(SequenceTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SequenceTypeDeserializerState,
        ) -> Result<(), Error> {
            if let SequenceTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::SequenceTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::SequenceTypeContent>,
            fallback: &mut Option<SequenceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SequenceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SequenceType> for SequenceTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SequenceType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SequenceType> {
            use SequenceTypeDeserializerState as S;
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
                            <super::SequenceTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::SequenceType, Error> {
            let state = replace(&mut *self.state__, SequenceTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::SequenceType {
                attrib_a: self.attrib_a,
                attrib_b: self.attrib_b,
                content: helper.finish_default(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct SequenceTypeContentDeserializer {
        a: Option<super::xs::StringType>,
        b: Option<super::xs::StringType>,
        c: Option<super::xs::StringType>,
        state__: Box<SequenceTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    enum SequenceTypeContentDeserializerState {
        Init__,
        A(Option<<super::xs::StringType as WithDeserializer>::Deserializer>),
        B(Option<<super::xs::StringType as WithDeserializer>::Deserializer>),
        C(Option<<super::xs::StringType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SequenceTypeContentDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SequenceTypeContentDeserializerState,
        ) -> Result<(), Error> {
            use SequenceTypeContentDeserializerState as S;
            match state {
                S::A(Some(deserializer)) => self.store_a(deserializer.finish(helper)?)?,
                S::B(Some(deserializer)) => self.store_b(deserializer.finish(helper)?)?,
                S::C(Some(deserializer)) => self.store_c(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_a(&mut self, value: super::xs::StringType) -> Result<(), Error> {
            if self.a.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"A")))?;
            }
            self.a = Some(value);
            Ok(())
        }
        fn store_b(&mut self, value: super::xs::StringType) -> Result<(), Error> {
            if self.b.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"B")))?;
            }
            self.b = Some(value);
            Ok(())
        }
        fn store_c(&mut self, value: super::xs::StringType) -> Result<(), Error> {
            if self.c.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"C")))?;
            }
            self.c = Some(value);
            Ok(())
        }
        fn handle_a<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::xs::StringType>,
            fallback: &mut Option<SequenceTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SequenceTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::A(None));
                *self.state__ = S::B(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_a(data)?;
                    *self.state__ = S::B(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::A(Some(deserializer)));
                    *self.state__ = S::B(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_b<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::xs::StringType>,
            fallback: &mut Option<SequenceTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SequenceTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::B(None));
                *self.state__ = S::C(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_b(data)?;
                    *self.state__ = S::C(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::B(Some(deserializer)));
                    *self.state__ = S::C(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_c<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::xs::StringType>,
            fallback: &mut Option<SequenceTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SequenceTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::C(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_c(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::C(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl Default for SequenceTypeContentDeserializer {
        fn default() -> Self {
            Self {
                a: None,
                b: None,
                c: None,
                state__: Box::new(SequenceTypeContentDeserializerState::Init__),
            }
        }
    }
    impl<'de> Deserializer<'de, super::SequenceTypeContent> for SequenceTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SequenceTypeContent> {
            let deserializer = Self {
                a: None,
                b: None,
                c: None,
                state__: Box::new(SequenceTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, SequenceTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::SequenceTypeContent> {
            use SequenceTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::A(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_a(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::B(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_b(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::C(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_c(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::A(None);
                        event
                    }
                    (S::A(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"A", false)?;
                        match self.handle_a(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::B(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"B", false)?;
                        match self.handle_b(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::C(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"C", false)?;
                        match self.handle_c(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Done__;
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
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::SequenceTypeContent, Error> {
            let state = replace(
                &mut *self.state__,
                SequenceTypeContentDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::SequenceTypeContent {
                a: self.a,
                b: self.b,
                c: self.c,
            })
        }
    }
    #[derive(Debug)]
    pub struct NestedSeqTypeDeserializer {
        attrib_a: Option<super::xs::StringType>,
        attrib_b: Option<super::xs::StringType>,
        content: Option<super::NestedSeqTypeContent>,
        state__: Box<NestedSeqTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum NestedSeqTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::NestedSeqTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl NestedSeqTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut attrib_a: Option<super::xs::StringType> = None;
            let mut attrib_b: Option<super::xs::StringType> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"attrib-a" {
                    helper.read_attrib(&mut attrib_a, b"attrib-a", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"attrib-b" {
                    helper.read_attrib(&mut attrib_b, b"attrib-b", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                attrib_a: attrib_a,
                attrib_b: attrib_b,
                content: None,
                state__: Box::new(NestedSeqTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: NestedSeqTypeDeserializerState,
        ) -> Result<(), Error> {
            if let NestedSeqTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::NestedSeqTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::NestedSeqTypeContent>,
            fallback: &mut Option<NestedSeqTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use NestedSeqTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::NestedSeqType> for NestedSeqTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NestedSeqType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NestedSeqType> {
            use NestedSeqTypeDeserializerState as S;
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
                            <super::NestedSeqTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::NestedSeqType, Error> {
            let state = replace(
                &mut *self.state__,
                NestedSeqTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::NestedSeqType {
                attrib_a: self.attrib_a,
                attrib_b: self.attrib_b,
                content: helper.finish_default(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct NestedSeqTypeContentDeserializer {
        inner_choice: Option<super::NestedSeqInnerChoiceType>,
        d: Option<super::xs::StringType>,
        state__: Box<NestedSeqTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    enum NestedSeqTypeContentDeserializerState {
        Init__,
        InnerChoice(Option<<super::NestedSeqInnerChoiceType as WithDeserializer>::Deserializer>),
        D(Option<<super::xs::StringType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl NestedSeqTypeContentDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: NestedSeqTypeContentDeserializerState,
        ) -> Result<(), Error> {
            use NestedSeqTypeContentDeserializerState as S;
            match state {
                S::InnerChoice(Some(deserializer)) => {
                    self.store_inner_choice(deserializer.finish(helper)?)?
                }
                S::D(Some(deserializer)) => self.store_d(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_inner_choice(
            &mut self,
            value: super::NestedSeqInnerChoiceType,
        ) -> Result<(), Error> {
            if self.inner_choice.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"InnerChoice",
                )))?;
            }
            self.inner_choice = Some(value);
            Ok(())
        }
        fn store_d(&mut self, value: super::xs::StringType) -> Result<(), Error> {
            if self.d.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"D")))?;
            }
            self.d = Some(value);
            Ok(())
        }
        fn handle_inner_choice<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::NestedSeqInnerChoiceType>,
            fallback: &mut Option<NestedSeqTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use NestedSeqTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::InnerChoice(None));
                *self.state__ = S::D(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_inner_choice(data)?;
                    *self.state__ = S::D(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::InnerChoice(Some(deserializer)));
                    *self.state__ = S::D(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_d<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::xs::StringType>,
            fallback: &mut Option<NestedSeqTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use NestedSeqTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::D(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_d(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::D(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl Default for NestedSeqTypeContentDeserializer {
        fn default() -> Self {
            Self {
                inner_choice: None,
                d: None,
                state__: Box::new(NestedSeqTypeContentDeserializerState::Init__),
            }
        }
    }
    impl<'de> Deserializer<'de, super::NestedSeqTypeContent> for NestedSeqTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NestedSeqTypeContent> {
            let deserializer = Self {
                inner_choice: None,
                d: None,
                state__: Box::new(NestedSeqTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, NestedSeqTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::NestedSeqTypeContent> {
            use NestedSeqTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::InnerChoice(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_inner_choice(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::D(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_d(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::InnerChoice(None);
                        event
                    }
                    (S::InnerChoice(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = <super::NestedSeqInnerChoiceType as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_inner_choice(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::D(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"D", false)?;
                        match self.handle_d(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Done__;
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
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::NestedSeqTypeContent, Error> {
            let state = replace(
                &mut *self.state__,
                NestedSeqTypeContentDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::NestedSeqTypeContent {
                inner_choice: self.inner_choice,
                d: self.d,
            })
        }
    }
    #[derive(Debug)]
    pub struct NestedSeqInnerChoiceTypeDeserializer {
        content: Option<super::NestedSeqInnerChoiceTypeContent>,
        state__: Box<NestedSeqInnerChoiceTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum NestedSeqInnerChoiceTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::NestedSeqInnerChoiceTypeContent as WithDeserializer>::Deserializer),
        Done__,
        Unknown__,
    }
    impl NestedSeqInnerChoiceTypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: NestedSeqInnerChoiceTypeDeserializerState,
        ) -> Result<(), Error> {
            if let NestedSeqInnerChoiceTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(
            &mut self,
            value: super::NestedSeqInnerChoiceTypeContent,
        ) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::NestedSeqInnerChoiceTypeContent>,
            fallback: &mut Option<NestedSeqInnerChoiceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use NestedSeqInnerChoiceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl Default for NestedSeqInnerChoiceTypeDeserializer {
        fn default() -> Self {
            Self {
                content: None,
                state__: Box::new(NestedSeqInnerChoiceTypeDeserializerState::Init__),
            }
        }
    }
    impl<'de> Deserializer<'de, super::NestedSeqInnerChoiceType>
        for NestedSeqInnerChoiceTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NestedSeqInnerChoiceType> {
            let deserializer = Self {
                content: None,
                state__: Box::new(NestedSeqInnerChoiceTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        NestedSeqInnerChoiceTypeDeserializerState::Init__
                    ) =>
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
        ) -> DeserializerResult<'de, super::NestedSeqInnerChoiceType> {
            use NestedSeqInnerChoiceTypeDeserializerState as S;
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
                    (_, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::NestedSeqInnerChoiceTypeContent as WithDeserializer>::init(
                                helper, event,
                            )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = match &*self.state__ {
                S::Done__ => DeserializerArtifact::Data(self.finish(helper)?),
                _ => DeserializerArtifact::Deserializer(self),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::NestedSeqInnerChoiceType, Error> {
            let state = replace(
                &mut *self.state__,
                NestedSeqInnerChoiceTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::NestedSeqInnerChoiceType {
                content: helper.finish_default(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct NestedSeqInnerChoiceTypeContentDeserializer {
        state__: Box<NestedSeqInnerChoiceTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum NestedSeqInnerChoiceTypeContentDeserializerState {
        Init__,
        FinalSeq(
            Option<super::NestedSeqFinalSeqType>,
            Option<<super::NestedSeqFinalSeqType as WithDeserializer>::Deserializer>,
            Option<<super::NestedSeqFinalSeqType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::NestedSeqInnerChoiceTypeContent),
        Unknown__,
    }
    impl NestedSeqInnerChoiceTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let mut event = event;
            let mut allow_any_element = false;
            if let Event::Start(_) | Event::Empty(_) = &event {
                event = {
                    let output =
                        <super::NestedSeqFinalSeqType as WithDeserializer>::init(helper, event)?;
                    match self.handle_final_seq(helper, Default::default(), None, output)? {
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
            *self.state__ = NestedSeqInnerChoiceTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: NestedSeqInnerChoiceTypeContentDeserializerState,
        ) -> Result<super::NestedSeqInnerChoiceTypeContent, Error> {
            use NestedSeqInnerChoiceTypeContentDeserializerState as S;
            match state {
                S::Init__ => Ok(super::NestedSeqInnerChoiceTypeContent::FinalSeq(
                    helper.finish_default(None)?,
                )),
                S::FinalSeq(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_final_seq(&mut values, value)?;
                    }
                    Ok(super::NestedSeqInnerChoiceTypeContent::FinalSeq(
                        helper.finish_default(values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_final_seq(
            values: &mut Option<super::NestedSeqFinalSeqType>,
            value: super::NestedSeqFinalSeqType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"FinalSeq",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_final_seq<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::NestedSeqFinalSeqType>,
            fallback: Option<<super::NestedSeqFinalSeqType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::NestedSeqFinalSeqType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use NestedSeqInnerChoiceTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_final_seq(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_final_seq(&mut values, data)?;
                    let data = Self::finish_state(helper, S::FinalSeq(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::FinalSeq(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl Default for NestedSeqInnerChoiceTypeContentDeserializer {
        fn default() -> Self {
            Self {
                state__: Box::new(NestedSeqInnerChoiceTypeContentDeserializerState::Init__),
            }
        }
    }
    impl<'de> Deserializer<'de, super::NestedSeqInnerChoiceTypeContent>
        for NestedSeqInnerChoiceTypeContentDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NestedSeqInnerChoiceTypeContent> {
            let deserializer = Self::default();
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        NestedSeqInnerChoiceTypeContentDeserializerState::Init__
                    ) =>
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
        ) -> DeserializerResult<'de, super::NestedSeqInnerChoiceTypeContent> {
            use NestedSeqInnerChoiceTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::FinalSeq(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_final_seq(helper, values, fallback, output)? {
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
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::FinalSeq(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = <super::NestedSeqFinalSeqType as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_final_seq(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
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
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::NestedSeqInnerChoiceTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct NestedSeqFinalSeqTypeDeserializer {
        content: Option<super::NestedSeqFinalSeqTypeContent>,
        state__: Box<NestedSeqFinalSeqTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum NestedSeqFinalSeqTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::NestedSeqFinalSeqTypeContent as WithDeserializer>::Deserializer),
        Done__,
        Unknown__,
    }
    impl NestedSeqFinalSeqTypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: NestedSeqFinalSeqTypeDeserializerState,
        ) -> Result<(), Error> {
            if let NestedSeqFinalSeqTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(
            &mut self,
            value: super::NestedSeqFinalSeqTypeContent,
        ) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::NestedSeqFinalSeqTypeContent>,
            fallback: &mut Option<NestedSeqFinalSeqTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use NestedSeqFinalSeqTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl Default for NestedSeqFinalSeqTypeDeserializer {
        fn default() -> Self {
            Self {
                content: None,
                state__: Box::new(NestedSeqFinalSeqTypeDeserializerState::Init__),
            }
        }
    }
    impl<'de> Deserializer<'de, super::NestedSeqFinalSeqType> for NestedSeqFinalSeqTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NestedSeqFinalSeqType> {
            let deserializer = Self {
                content: None,
                state__: Box::new(NestedSeqFinalSeqTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, NestedSeqFinalSeqTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::NestedSeqFinalSeqType> {
            use NestedSeqFinalSeqTypeDeserializerState as S;
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
                    (_, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::NestedSeqFinalSeqTypeContent as WithDeserializer>::init(
                                helper, event,
                            )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = match &*self.state__ {
                S::Done__ => DeserializerArtifact::Data(self.finish(helper)?),
                _ => DeserializerArtifact::Deserializer(self),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::NestedSeqFinalSeqType, Error> {
            let state = replace(
                &mut *self.state__,
                NestedSeqFinalSeqTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::NestedSeqFinalSeqType {
                content: helper.finish_default(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct NestedSeqFinalSeqTypeContentDeserializer {
        a: Option<super::xs::StringType>,
        b: Option<super::xs::StringType>,
        c: Option<super::xs::StringType>,
        state__: Box<NestedSeqFinalSeqTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    enum NestedSeqFinalSeqTypeContentDeserializerState {
        Init__,
        A(Option<<super::xs::StringType as WithDeserializer>::Deserializer>),
        B(Option<<super::xs::StringType as WithDeserializer>::Deserializer>),
        C(Option<<super::xs::StringType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl NestedSeqFinalSeqTypeContentDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: NestedSeqFinalSeqTypeContentDeserializerState,
        ) -> Result<(), Error> {
            use NestedSeqFinalSeqTypeContentDeserializerState as S;
            match state {
                S::A(Some(deserializer)) => self.store_a(deserializer.finish(helper)?)?,
                S::B(Some(deserializer)) => self.store_b(deserializer.finish(helper)?)?,
                S::C(Some(deserializer)) => self.store_c(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_a(&mut self, value: super::xs::StringType) -> Result<(), Error> {
            if self.a.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"A")))?;
            }
            self.a = Some(value);
            Ok(())
        }
        fn store_b(&mut self, value: super::xs::StringType) -> Result<(), Error> {
            if self.b.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"B")))?;
            }
            self.b = Some(value);
            Ok(())
        }
        fn store_c(&mut self, value: super::xs::StringType) -> Result<(), Error> {
            if self.c.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"C")))?;
            }
            self.c = Some(value);
            Ok(())
        }
        fn handle_a<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::xs::StringType>,
            fallback: &mut Option<NestedSeqFinalSeqTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use NestedSeqFinalSeqTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::A(None));
                *self.state__ = S::B(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_a(data)?;
                    *self.state__ = S::B(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::A(Some(deserializer)));
                    *self.state__ = S::B(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_b<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::xs::StringType>,
            fallback: &mut Option<NestedSeqFinalSeqTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use NestedSeqFinalSeqTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::B(None));
                *self.state__ = S::C(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_b(data)?;
                    *self.state__ = S::C(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::B(Some(deserializer)));
                    *self.state__ = S::C(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_c<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::xs::StringType>,
            fallback: &mut Option<NestedSeqFinalSeqTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use NestedSeqFinalSeqTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::C(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_c(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::C(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl Default for NestedSeqFinalSeqTypeContentDeserializer {
        fn default() -> Self {
            Self {
                a: None,
                b: None,
                c: None,
                state__: Box::new(NestedSeqFinalSeqTypeContentDeserializerState::Init__),
            }
        }
    }
    impl<'de> Deserializer<'de, super::NestedSeqFinalSeqTypeContent>
        for NestedSeqFinalSeqTypeContentDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NestedSeqFinalSeqTypeContent> {
            let deserializer = Self {
                a: None,
                b: None,
                c: None,
                state__: Box::new(NestedSeqFinalSeqTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        NestedSeqFinalSeqTypeContentDeserializerState::Init__
                    ) =>
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
        ) -> DeserializerResult<'de, super::NestedSeqFinalSeqTypeContent> {
            use NestedSeqFinalSeqTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::A(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_a(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::B(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_b(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::C(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_c(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::A(None);
                        event
                    }
                    (S::A(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"A", false)?;
                        match self.handle_a(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::B(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"B", false)?;
                        match self.handle_b(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::C(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"C", false)?;
                        match self.handle_c(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Done__;
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
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::NestedSeqFinalSeqTypeContent, Error> {
            let state = replace(
                &mut *self.state__,
                NestedSeqFinalSeqTypeContentDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::NestedSeqFinalSeqTypeContent {
                a: self.a,
                b: self.b,
                c: self.c,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::{
        misc::{Namespace, NamespacePrefix},
        quick_xml::{
            BytesEnd, BytesStart, Error, Event, IterSerializer, SerializeHelper, Serializer,
            WithSerializer,
        },
    };
    #[derive(Debug)]
    pub struct SimpleTypeSerializer<'ser> {
        pub(super) value: &'ser super::SimpleType,
        pub(super) state: Box<SimpleTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SimpleTypeSerializerState<'ser> {
        Init__,
        Content__(<super::xs::StringType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SimpleTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SimpleTypeSerializerState::Init__ => {
                        *self.state = SimpleTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&NamespacePrefix::XSI),
                                &Namespace::XSI,
                            );
                        }
                        helper.write_attrib_opt(&mut bytes, "attrib-a", &self.value.attrib_a)?;
                        helper.write_attrib_opt(&mut bytes, "attrib-b", &self.value.attrib_b)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SimpleTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = SimpleTypeSerializerState::End__,
                    },
                    SimpleTypeSerializerState::End__ => {
                        *self.state = SimpleTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SimpleTypeSerializerState::Done__ => return Ok(None),
                    SimpleTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for SimpleTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SimpleTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SequenceTypeSerializer<'ser> {
        pub(super) value: &'ser super::SequenceType,
        pub(super) state: Box<SequenceTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SequenceTypeSerializerState<'ser> {
        Init__,
        Content__(<super::SequenceTypeContent as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SequenceTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SequenceTypeSerializerState::Init__ => {
                        *self.state = SequenceTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&NamespacePrefix::XSI),
                                &Namespace::XSI,
                            );
                        }
                        helper.write_attrib_opt(&mut bytes, "attrib-a", &self.value.attrib_a)?;
                        helper.write_attrib_opt(&mut bytes, "attrib-b", &self.value.attrib_b)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SequenceTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SequenceTypeSerializerState::End__,
                        }
                    }
                    SequenceTypeSerializerState::End__ => {
                        *self.state = SequenceTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SequenceTypeSerializerState::Done__ => return Ok(None),
                    SequenceTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for SequenceTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SequenceTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SequenceTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::SequenceTypeContent,
        pub(super) state: Box<SequenceTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum SequenceTypeContentSerializerState<'ser> {
        Init__,
        A(IterSerializer<'ser, Option<&'ser super::xs::StringType>, super::xs::StringType>),
        B(IterSerializer<'ser, Option<&'ser super::xs::StringType>, super::xs::StringType>),
        C(IterSerializer<'ser, Option<&'ser super::xs::StringType>, super::xs::StringType>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SequenceTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SequenceTypeContentSerializerState::Init__ => {
                        *self.state = SequenceTypeContentSerializerState::A(IterSerializer::new(
                            self.value.a.as_ref(),
                            Some("A"),
                            false,
                        ));
                    }
                    SequenceTypeContentSerializerState::A(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = SequenceTypeContentSerializerState::B(
                                IterSerializer::new(self.value.b.as_ref(), Some("B"), false),
                            )
                        }
                    },
                    SequenceTypeContentSerializerState::B(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = SequenceTypeContentSerializerState::C(
                                IterSerializer::new(self.value.c.as_ref(), Some("C"), false),
                            )
                        }
                    },
                    SequenceTypeContentSerializerState::C(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = SequenceTypeContentSerializerState::Done__,
                    },
                    SequenceTypeContentSerializerState::Done__ => return Ok(None),
                    SequenceTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for SequenceTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SequenceTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct NestedSeqTypeSerializer<'ser> {
        pub(super) value: &'ser super::NestedSeqType,
        pub(super) state: Box<NestedSeqTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum NestedSeqTypeSerializerState<'ser> {
        Init__,
        Content__(<super::NestedSeqTypeContent as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> NestedSeqTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    NestedSeqTypeSerializerState::Init__ => {
                        *self.state = NestedSeqTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&NamespacePrefix::XSI),
                                &Namespace::XSI,
                            );
                        }
                        helper.write_attrib_opt(&mut bytes, "attrib-a", &self.value.attrib_a)?;
                        helper.write_attrib_opt(&mut bytes, "attrib-b", &self.value.attrib_b)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    NestedSeqTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = NestedSeqTypeSerializerState::End__,
                        }
                    }
                    NestedSeqTypeSerializerState::End__ => {
                        *self.state = NestedSeqTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    NestedSeqTypeSerializerState::Done__ => return Ok(None),
                    NestedSeqTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for NestedSeqTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = NestedSeqTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct NestedSeqTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::NestedSeqTypeContent,
        pub(super) state: Box<NestedSeqTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum NestedSeqTypeContentSerializerState<'ser> {
        Init__,
        InnerChoice(
            IterSerializer<
                'ser,
                Option<&'ser super::NestedSeqInnerChoiceType>,
                super::NestedSeqInnerChoiceType,
            >,
        ),
        D(IterSerializer<'ser, Option<&'ser super::xs::StringType>, super::xs::StringType>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> NestedSeqTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    NestedSeqTypeContentSerializerState::Init__ => {
                        *self.state =
                            NestedSeqTypeContentSerializerState::InnerChoice(IterSerializer::new(
                                self.value.inner_choice.as_ref(),
                                Some("InnerChoice"),
                                false,
                            ));
                    }
                    NestedSeqTypeContentSerializerState::InnerChoice(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = NestedSeqTypeContentSerializerState::D(
                                    IterSerializer::new(self.value.d.as_ref(), Some("D"), false),
                                )
                            }
                        }
                    }
                    NestedSeqTypeContentSerializerState::D(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = NestedSeqTypeContentSerializerState::Done__,
                        }
                    }
                    NestedSeqTypeContentSerializerState::Done__ => return Ok(None),
                    NestedSeqTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for NestedSeqTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = NestedSeqTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct NestedSeqInnerChoiceTypeSerializer<'ser> {
        pub(super) value: &'ser super::NestedSeqInnerChoiceType,
        pub(super) state: Box<NestedSeqInnerChoiceTypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum NestedSeqInnerChoiceTypeSerializerState<'ser> {
        Init__,
        Content__(<super::NestedSeqInnerChoiceTypeContent as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> NestedSeqInnerChoiceTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    NestedSeqInnerChoiceTypeSerializerState::Init__ => {
                        *self.state = NestedSeqInnerChoiceTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                    }
                    NestedSeqInnerChoiceTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = NestedSeqInnerChoiceTypeSerializerState::Done__,
                        }
                    }
                    NestedSeqInnerChoiceTypeSerializerState::Done__ => return Ok(None),
                    NestedSeqInnerChoiceTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for NestedSeqInnerChoiceTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = NestedSeqInnerChoiceTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct NestedSeqInnerChoiceTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::NestedSeqInnerChoiceTypeContent,
        pub(super) state: Box<NestedSeqInnerChoiceTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum NestedSeqInnerChoiceTypeContentSerializerState<'ser> {
        Init__,
        FinalSeq(<super::NestedSeqFinalSeqType as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> NestedSeqInnerChoiceTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    NestedSeqInnerChoiceTypeContentSerializerState::Init__ => match self.value {
                        super::NestedSeqInnerChoiceTypeContent::FinalSeq(x) => {
                            *self.state = NestedSeqInnerChoiceTypeContentSerializerState::FinalSeq(
                                WithSerializer::serializer(x, Some("FinalSeq"), false)?,
                            )
                        }
                    },
                    NestedSeqInnerChoiceTypeContentSerializerState::FinalSeq(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = NestedSeqInnerChoiceTypeContentSerializerState::Done__
                            }
                        }
                    }
                    NestedSeqInnerChoiceTypeContentSerializerState::Done__ => return Ok(None),
                    NestedSeqInnerChoiceTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for NestedSeqInnerChoiceTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = NestedSeqInnerChoiceTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct NestedSeqFinalSeqTypeSerializer<'ser> {
        pub(super) value: &'ser super::NestedSeqFinalSeqType,
        pub(super) state: Box<NestedSeqFinalSeqTypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum NestedSeqFinalSeqTypeSerializerState<'ser> {
        Init__,
        Content__(<super::NestedSeqFinalSeqTypeContent as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> NestedSeqFinalSeqTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    NestedSeqFinalSeqTypeSerializerState::Init__ => {
                        *self.state = NestedSeqFinalSeqTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                    }
                    NestedSeqFinalSeqTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = NestedSeqFinalSeqTypeSerializerState::Done__,
                        }
                    }
                    NestedSeqFinalSeqTypeSerializerState::Done__ => return Ok(None),
                    NestedSeqFinalSeqTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for NestedSeqFinalSeqTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = NestedSeqFinalSeqTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct NestedSeqFinalSeqTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::NestedSeqFinalSeqTypeContent,
        pub(super) state: Box<NestedSeqFinalSeqTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum NestedSeqFinalSeqTypeContentSerializerState<'ser> {
        Init__,
        A(IterSerializer<'ser, Option<&'ser super::xs::StringType>, super::xs::StringType>),
        B(IterSerializer<'ser, Option<&'ser super::xs::StringType>, super::xs::StringType>),
        C(IterSerializer<'ser, Option<&'ser super::xs::StringType>, super::xs::StringType>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> NestedSeqFinalSeqTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    NestedSeqFinalSeqTypeContentSerializerState::Init__ => {
                        *self.state = NestedSeqFinalSeqTypeContentSerializerState::A(
                            IterSerializer::new(self.value.a.as_ref(), Some("A"), false),
                        );
                    }
                    NestedSeqFinalSeqTypeContentSerializerState::A(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = NestedSeqFinalSeqTypeContentSerializerState::B(
                                    IterSerializer::new(self.value.b.as_ref(), Some("B"), false),
                                )
                            }
                        }
                    }
                    NestedSeqFinalSeqTypeContentSerializerState::B(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = NestedSeqFinalSeqTypeContentSerializerState::C(
                                    IterSerializer::new(self.value.c.as_ref(), Some("C"), false),
                                )
                            }
                        }
                    }
                    NestedSeqFinalSeqTypeContentSerializerState::C(x) => match x
                        .next(helper)
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = NestedSeqFinalSeqTypeContentSerializerState::Done__,
                    },
                    NestedSeqFinalSeqTypeContentSerializerState::Done__ => return Ok(None),
                    NestedSeqFinalSeqTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for NestedSeqFinalSeqTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = NestedSeqFinalSeqTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
pub mod xs {
    pub type StringType = String;
}
