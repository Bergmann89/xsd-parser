use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{Error, WithDeserializer, WithSerializer},
    xml::{Mixed, Text},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
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
    use xsd_parser_types::{
        quick_xml::{
            BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
            RawByteStr, WithDeserializer,
        },
        xml::{Mixed, Text},
    };
    #[derive(Debug)]
    pub struct MixedAllTypeDeserializer {
        text_before: Option<Text>,
        fuu: Option<Mixed<i32>>,
        bar: Option<Mixed<String>>,
        state__: Box<MixedAllTypeDeserializerState>,
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
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
            fallback: &mut Option<MixedAllTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Fuu")
                ) {
                    let output = <Mixed<i32> as WithDeserializer>::init(helper, event)?;
                    return self.handle_fuu(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Bar")
                ) {
                    let output = <Mixed<String> as WithDeserializer>::init(helper, event)?;
                    return self.handle_bar(helper, output, &mut *fallback);
                }
            }
            let output = <Text as WithDeserializer>::init(helper, event)?;
            self.handle_text_before(helper, output, &mut *fallback)
        }
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                text_before: None,
                fuu: None,
                bar: None,
                state__: Box::new(MixedAllTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: MixedAllTypeDeserializerState,
        ) -> Result<(), Error> {
            use MixedAllTypeDeserializerState as S;
            match state {
                S::TextBefore(deserializer) => {
                    self.store_text_before(deserializer.finish(helper)?)?
                }
                S::Fuu(deserializer) => self.store_fuu(deserializer.finish(helper)?)?,
                S::Bar(deserializer) => self.store_bar(deserializer.finish(helper)?)?,
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
        fn handle_text_before<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, Text>,
            fallback: &mut Option<MixedAllTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedAllTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_before(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::TextBefore(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_fuu<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, Mixed<i32>>,
            fallback: &mut Option<MixedAllTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedAllTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_fuu(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Fuu(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_bar<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, Mixed<String>>,
            fallback: &mut Option<MixedAllTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedAllTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_bar(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Bar(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::MixedAllType> for MixedAllTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedAllType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedAllType> {
            use MixedAllTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::TextBefore(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_text_before(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, .. } => event,
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Fuu(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fuu(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, .. } => event,
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Bar(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_bar(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, .. } => event,
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
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
                        match self.find_suitable(helper, event, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, .. } => event,
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::MixedAllType, Error> {
            let state = replace(&mut *self.state__, MixedAllTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::MixedAllType {
                text_before: self.text_before,
                fuu: helper.finish_element("Fuu", self.fuu)?,
                bar: helper.finish_element("Bar", self.bar)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct MixedChoiceTypeDeserializer {
        text_before: Option<Text>,
        content: Option<super::MixedChoiceTypeContent>,
        state__: Box<MixedChoiceTypeDeserializerState>,
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
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                text_before: None,
                content: None,
                state__: Box::new(MixedChoiceTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: MixedChoiceTypeDeserializerState,
        ) -> Result<(), Error> {
            use MixedChoiceTypeDeserializerState as S;
            match state {
                S::TextBefore(Some(deserializer)) => {
                    self.store_text_before(deserializer.finish(helper)?)?
                }
                S::Content(Some(deserializer)) => {
                    self.store_content(deserializer.finish(helper)?)?
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
        fn handle_text_before<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, Text>,
            fallback: &mut Option<MixedChoiceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedChoiceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::TextBefore(None));
                *self.state__ = S::Content(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_before(data)?;
                    *self.state__ = S::Content(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::TextBefore(Some(deserializer)));
                    *self.state__ = S::Content(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::MixedChoiceTypeContent>,
            fallback: &mut Option<MixedChoiceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedChoiceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Content(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Content(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::MixedChoiceType> for MixedChoiceTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedChoiceType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedChoiceType> {
            use MixedChoiceTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::TextBefore(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_text_before(helper, output, &mut fallback)? {
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
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::TextBefore(None);
                        event
                    }
                    (S::TextBefore(None), event) => {
                        let output = <Text as WithDeserializer>::init(helper, event)?;
                        match self.handle_text_before(helper, output, &mut fallback)? {
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
                        let output = <super::MixedChoiceTypeContent as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_content(helper, output, &mut fallback)? {
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
        ) -> Result<super::MixedChoiceType, Error> {
            let state = replace(
                &mut *self.state__,
                MixedChoiceTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::MixedChoiceType {
                text_before: self.text_before,
                content: helper.finish_element("content", self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct MixedChoiceTypeContentDeserializer {
        state__: Box<MixedChoiceTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum MixedChoiceTypeContentDeserializerState {
        Init__,
        Fuu(
            Option<Mixed<i32>>,
            Option<<Mixed<i32> as WithDeserializer>::Deserializer>,
            Option<<Mixed<i32> as WithDeserializer>::Deserializer>,
        ),
        Bar(
            Option<Mixed<String>>,
            Option<<Mixed<String> as WithDeserializer>::Deserializer>,
            Option<<Mixed<String> as WithDeserializer>::Deserializer>,
        ),
        Done__(super::MixedChoiceTypeContent),
        Unknown__,
    }
    impl MixedChoiceTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Fuu")
                ) {
                    let output = <Mixed<i32> as WithDeserializer>::init(helper, event)?;
                    return self.handle_fuu(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Bar")
                ) {
                    let output = <Mixed<String> as WithDeserializer>::init(helper, event)?;
                    return self.handle_bar(helper, Default::default(), None, output);
                }
            }
            *self.state__ = MixedChoiceTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: MixedChoiceTypeContentDeserializerState,
        ) -> Result<super::MixedChoiceTypeContent, Error> {
            use MixedChoiceTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Fuu(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_fuu(&mut values, value)?;
                    }
                    Ok(super::MixedChoiceTypeContent::Fuu(
                        helper.finish_element("Fuu", values)?,
                    ))
                }
                S::Bar(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_bar(&mut values, value)?;
                    }
                    Ok(super::MixedChoiceTypeContent::Bar(
                        helper.finish_element("Bar", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
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
        fn handle_fuu<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<Mixed<i32>>,
            fallback: Option<<Mixed<i32> as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, Mixed<i32>>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedChoiceTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_fuu(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fuu(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Fuu(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Fuu(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_bar<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<Mixed<String>>,
            fallback: Option<<Mixed<String> as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, Mixed<String>>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedChoiceTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_bar(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bar(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Bar(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Bar(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::MixedChoiceTypeContent> for MixedChoiceTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedChoiceTypeContent> {
            let deserializer = Self {
                state__: Box::new(MixedChoiceTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, MixedChoiceTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::MixedChoiceTypeContent> {
            use MixedChoiceTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Fuu(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fuu(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bar(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_bar(helper, values, fallback, output)? {
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
                        S::Fuu(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Fuu",
                            false,
                        )?;
                        match self.handle_fuu(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Bar(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Bar",
                            false,
                        )?;
                        match self.handle_bar(helper, values, fallback, output)? {
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
                    (state, Event::Text(_) | Event::CData(_)) => {
                        *self.state__ = state;
                        break (DeserializerEvent::None, false);
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
        ) -> Result<super::MixedChoiceTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct MixedChoiceListTypeDeserializer {
        content: Vec<super::MixedChoiceListTypeContent>,
        state__: Box<MixedChoiceListTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum MixedChoiceListTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::MixedChoiceListTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl MixedChoiceListTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: Vec::new(),
                state__: Box::new(MixedChoiceListTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: MixedChoiceListTypeDeserializerState,
        ) -> Result<(), Error> {
            if let MixedChoiceListTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::MixedChoiceListTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::MixedChoiceListTypeContent>,
            fallback: &mut Option<MixedChoiceListTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedChoiceListTypeDeserializerState as S;
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
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::MixedChoiceListType> for MixedChoiceListTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedChoiceListType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedChoiceListType> {
            use MixedChoiceListTypeDeserializerState as S;
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
                        let output = <super::MixedChoiceListTypeContent as WithDeserializer>::init(
                            helper, event,
                        )?;
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
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::MixedChoiceListType, Error> {
            let state = replace(
                &mut *self.state__,
                MixedChoiceListTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::MixedChoiceListType {
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct MixedChoiceListTypeContentDeserializer {
        state__: Box<MixedChoiceListTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum MixedChoiceListTypeContentDeserializerState {
        Init__,
        Fuu(
            Option<i32>,
            Option<<i32 as WithDeserializer>::Deserializer>,
            Option<<i32 as WithDeserializer>::Deserializer>,
        ),
        Bar(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Text(
            Option<Text>,
            Option<<Text as WithDeserializer>::Deserializer>,
            Option<<Text as WithDeserializer>::Deserializer>,
        ),
        Done__(super::MixedChoiceListTypeContent),
        Unknown__,
    }
    impl MixedChoiceListTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let mut event = event;
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Fuu")
                ) {
                    let output = <i32 as WithDeserializer>::init(helper, event)?;
                    return self.handle_fuu(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Bar")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_bar(helper, Default::default(), None, output);
                }
            }
            event = {
                let output = <Text as WithDeserializer>::init(helper, event)?;
                match self.handle_text(helper, Default::default(), None, output)? {
                    ElementHandlerOutput::Continue { event, .. } => event,
                    output => {
                        return Ok(output);
                    }
                }
            };
            *self.state__ = MixedChoiceListTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: MixedChoiceListTypeContentDeserializerState,
        ) -> Result<super::MixedChoiceListTypeContent, Error> {
            use MixedChoiceListTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Fuu(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_fuu(&mut values, value)?;
                    }
                    Ok(super::MixedChoiceListTypeContent::Fuu(
                        helper.finish_element("Fuu", values)?,
                    ))
                }
                S::Bar(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_bar(&mut values, value)?;
                    }
                    Ok(super::MixedChoiceListTypeContent::Bar(
                        helper.finish_element("Bar", values)?,
                    ))
                }
                S::Text(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_text(&mut values, value)?;
                    }
                    Ok(super::MixedChoiceListTypeContent::Text(
                        helper.finish_element("text", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
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
        fn handle_fuu<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<i32>,
            fallback: Option<<i32 as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, i32>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedChoiceListTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_fuu(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fuu(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Fuu(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Fuu(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_bar<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedChoiceListTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_bar(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bar(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Bar(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Bar(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_text<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<Text>,
            fallback: Option<<Text as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, Text>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedChoiceListTypeContentDeserializerState as S;
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
                Self::store_text(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_text(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Text(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Text(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::MixedChoiceListTypeContent>
        for MixedChoiceListTypeContentDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedChoiceListTypeContent> {
            let deserializer = Self {
                state__: Box::new(MixedChoiceListTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        MixedChoiceListTypeContentDeserializerState::Init__
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
        ) -> DeserializerResult<'de, super::MixedChoiceListTypeContent> {
            use MixedChoiceListTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Fuu(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fuu(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bar(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_bar(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Text(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_text(helper, values, fallback, output)? {
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
                        S::Fuu(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Fuu",
                            false,
                        )?;
                        match self.handle_fuu(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Bar(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Bar",
                            false,
                        )?;
                        match self.handle_bar(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Text(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = <Text as WithDeserializer>::init(helper, event)?;
                        match self.handle_text(helper, values, fallback, output)? {
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
        ) -> Result<super::MixedChoiceListTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct MixedSequenceTypeDeserializer {
        text_before: Option<Text>,
        fuu: Option<i32>,
        text_after_fuu: Option<Text>,
        bar: Option<String>,
        text_after_bar: Option<Text>,
        state__: Box<MixedSequenceTypeDeserializerState>,
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
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                text_before: None,
                fuu: None,
                text_after_fuu: None,
                bar: None,
                text_after_bar: None,
                state__: Box::new(MixedSequenceTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: MixedSequenceTypeDeserializerState,
        ) -> Result<(), Error> {
            use MixedSequenceTypeDeserializerState as S;
            match state {
                S::TextBefore(Some(deserializer)) => {
                    self.store_text_before(deserializer.finish(helper)?)?
                }
                S::Fuu(Some(deserializer)) => self.store_fuu(deserializer.finish(helper)?)?,
                S::TextAfterFuu(Some(deserializer)) => {
                    self.store_text_after_fuu(deserializer.finish(helper)?)?
                }
                S::Bar(Some(deserializer)) => self.store_bar(deserializer.finish(helper)?)?,
                S::TextAfterBar(Some(deserializer)) => {
                    self.store_text_after_bar(deserializer.finish(helper)?)?
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
        fn handle_text_before<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, Text>,
            fallback: &mut Option<MixedSequenceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedSequenceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::TextBefore(None));
                *self.state__ = S::Fuu(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_before(data)?;
                    *self.state__ = S::Fuu(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::TextBefore(Some(deserializer)));
                    *self.state__ = S::Fuu(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_fuu<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<MixedSequenceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedSequenceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Fuu(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_fuu(data)?;
                    *self.state__ = S::TextAfterFuu(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Fuu(Some(deserializer)));
                    *self.state__ = S::TextAfterFuu(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_text_after_fuu<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, Text>,
            fallback: &mut Option<MixedSequenceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedSequenceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::TextAfterFuu(None));
                *self.state__ = S::Bar(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_after_fuu(data)?;
                    *self.state__ = S::Bar(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::TextAfterFuu(Some(deserializer)));
                    *self.state__ = S::Bar(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_bar<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<MixedSequenceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedSequenceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Bar(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_bar(data)?;
                    *self.state__ = S::TextAfterBar(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Bar(Some(deserializer)));
                    *self.state__ = S::TextAfterBar(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_text_after_bar<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, Text>,
            fallback: &mut Option<MixedSequenceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedSequenceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::TextAfterBar(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_after_bar(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::TextAfterBar(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::MixedSequenceType> for MixedSequenceTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedSequenceType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedSequenceType> {
            use MixedSequenceTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::TextBefore(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_text_before(helper, output, &mut fallback)? {
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
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fuu(helper, output, &mut fallback)? {
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
                        let output = deserializer.next(helper, event)?;
                        match self.handle_text_after_fuu(helper, output, &mut fallback)? {
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
                        let output = deserializer.next(helper, event)?;
                        match self.handle_bar(helper, output, &mut fallback)? {
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
                        let output = deserializer.next(helper, event)?;
                        match self.handle_text_after_bar(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::TextBefore(None);
                        event
                    }
                    (S::TextBefore(None), event) => {
                        let output = <Text as WithDeserializer>::init(helper, event)?;
                        match self.handle_text_before(helper, output, &mut fallback)? {
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
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Fuu",
                            false,
                        )?;
                        match self.handle_fuu(helper, output, &mut fallback)? {
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
                        let output = <Text as WithDeserializer>::init(helper, event)?;
                        match self.handle_text_after_fuu(helper, output, &mut fallback)? {
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
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Bar",
                            false,
                        )?;
                        match self.handle_bar(helper, output, &mut fallback)? {
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
                        let output = <Text as WithDeserializer>::init(helper, event)?;
                        match self.handle_text_after_bar(helper, output, &mut fallback)? {
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
        ) -> Result<super::MixedSequenceType, Error> {
            let state = replace(
                &mut *self.state__,
                MixedSequenceTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::MixedSequenceType {
                text_before: self.text_before,
                fuu: helper.finish_element("Fuu", self.fuu)?,
                text_after_fuu: self.text_after_fuu,
                bar: helper.finish_element("Bar", self.bar)?,
                text_after_bar: self.text_after_bar,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::{
        quick_xml::{
            BytesEnd, BytesStart, Error, Event, IterSerializer, SerializeHelper, Serializer,
            WithSerializer,
        },
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
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MixedAllTypeSerializerState::Init__ => {
                        *self.state = MixedAllTypeSerializerState::TextBefore(IterSerializer::new(
                            self.value.text_before.as_ref(),
                            Some(""),
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    MixedAllTypeSerializerState::TextBefore(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    MixedAllTypeSerializerState::Fuu(WithSerializer::serializer(
                                        &self.value.fuu,
                                        Some("tns:Fuu"),
                                        false,
                                    )?)
                            }
                        }
                    }
                    MixedAllTypeSerializerState::Fuu(x) => match x.next(helper).transpose()? {
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
                    MixedAllTypeSerializerState::Bar(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = MixedAllTypeSerializerState::End__,
                    },
                    MixedAllTypeSerializerState::End__ => {
                        *self.state = MixedAllTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    MixedAllTypeSerializerState::Done__ => return Ok(None),
                    MixedAllTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for MixedAllTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
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
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MixedChoiceTypeSerializerState::Init__ => {
                        *self.state = MixedChoiceTypeSerializerState::TextBefore(
                            IterSerializer::new(self.value.text_before.as_ref(), Some(""), false),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    MixedChoiceTypeSerializerState::TextBefore(x) => match x
                        .next(helper)
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = MixedChoiceTypeSerializerState::Content(
                                WithSerializer::serializer(&self.value.content, Some(""), false)?,
                            )
                        }
                    },
                    MixedChoiceTypeSerializerState::Content(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = MixedChoiceTypeSerializerState::End__,
                        }
                    }
                    MixedChoiceTypeSerializerState::End__ => {
                        *self.state = MixedChoiceTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    MixedChoiceTypeSerializerState::Done__ => return Ok(None),
                    MixedChoiceTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for MixedChoiceTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
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
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
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
                    MixedChoiceTypeContentSerializerState::Fuu(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = MixedChoiceTypeContentSerializerState::Done__,
                        }
                    }
                    MixedChoiceTypeContentSerializerState::Bar(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = MixedChoiceTypeContentSerializerState::Done__,
                        }
                    }
                    MixedChoiceTypeContentSerializerState::Done__ => return Ok(None),
                    MixedChoiceTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for MixedChoiceTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
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
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MixedChoiceListTypeSerializerState::Init__ => {
                        *self.state = MixedChoiceListTypeSerializerState::Content__(
                            IterSerializer::new(&self.value.content[..], None, false),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    MixedChoiceListTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = MixedChoiceListTypeSerializerState::End__,
                        }
                    }
                    MixedChoiceListTypeSerializerState::End__ => {
                        *self.state = MixedChoiceListTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    MixedChoiceListTypeSerializerState::Done__ => return Ok(None),
                    MixedChoiceListTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for MixedChoiceListTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
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
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
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
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = MixedChoiceListTypeContentSerializerState::Done__,
                        }
                    }
                    MixedChoiceListTypeContentSerializerState::Bar(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = MixedChoiceListTypeContentSerializerState::Done__,
                        }
                    }
                    MixedChoiceListTypeContentSerializerState::Text(x) => {
                        match x.next(helper).transpose()? {
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
    impl<'ser> Serializer<'ser> for MixedChoiceListTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
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
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
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
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    MixedSequenceTypeSerializerState::TextBefore(x) => {
                        match x.next(helper).transpose()? {
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
                    MixedSequenceTypeSerializerState::Fuu(x) => match x.next(helper).transpose()? {
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
                        match x.next(helper).transpose()? {
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
                    MixedSequenceTypeSerializerState::Bar(x) => match x.next(helper).transpose()? {
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
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = MixedSequenceTypeSerializerState::End__,
                        }
                    }
                    MixedSequenceTypeSerializerState::End__ => {
                        *self.state = MixedSequenceTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    MixedSequenceTypeSerializerState::Done__ => return Ok(None),
                    MixedSequenceTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for MixedSequenceTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
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
