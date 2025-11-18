use xsd_parser_types::{
    misc::Namespace,
    quick_xml::{Error, WithDeserializer, WithSerializer},
    xml::{AnyAttributes, AnyElement, Mixed, Text},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub type AttributeValue = AttributeValueType;
#[derive(Debug)]
pub struct AttributeValueType {
    pub base_attrib: String,
    pub data_type: String,
    pub any_attribute: AnyAttributes,
    pub text_before: Option<Text>,
    pub base_element: Mixed<String>,
    pub any: Vec<Mixed<AnyElement>>,
}
impl WithSerializer for AttributeValueType {
    type Serializer<'x> = quick_xml_serialize::AttributeValueTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::AttributeValueTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::AttributeValueTypeSerializerState::Init__),
            name: name.unwrap_or("tns:AttributeValueType"),
            is_root,
        })
    }
}
impl WithDeserializer for AttributeValueType {
    type Deserializer = quick_xml_deserialize::AttributeValueTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::{
        quick_xml::{
            filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer,
            DeserializerArtifact, DeserializerEvent, DeserializerOutput, DeserializerResult,
            ElementHandlerOutput, Error, ErrorKind, Event, RawByteStr, WithDeserializer,
        },
        xml::{AnyAttributes, AnyElement, Mixed, Text},
    };
    #[derive(Debug)]
    pub struct AttributeValueTypeDeserializer {
        base_attrib: String,
        data_type: String,
        any_attribute: AnyAttributes,
        text_before: Option<Text>,
        base_element: Option<Mixed<String>>,
        any: Vec<Mixed<AnyElement>>,
        state__: Box<AttributeValueTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AttributeValueTypeDeserializerState {
        Init__,
        TextBefore(Option<<Text as WithDeserializer>::Deserializer>),
        BaseElement(Option<<Mixed<String> as WithDeserializer>::Deserializer>),
        Any(Option<<Mixed<AnyElement> as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl AttributeValueTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut base_attrib: Option<String> = None;
            let mut data_type: Option<String> = None;
            let mut any_attribute = AnyAttributes::default();
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"BaseAttrib")
                ) {
                    reader.read_attrib(&mut base_attrib, b"BaseAttrib", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"DataType")
                ) {
                    reader.read_attrib(&mut data_type, b"DataType", &attrib.value)?;
                } else {
                    any_attribute.push(attrib)?;
                }
            }
            Ok(Self {
                base_attrib: base_attrib.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("BaseAttrib".into()))
                })?,
                data_type: data_type.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("DataType".into()))
                })?,
                any_attribute: any_attribute,
                text_before: None,
                base_element: None,
                any: Vec::new(),
                state__: Box::new(AttributeValueTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: AttributeValueTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use AttributeValueTypeDeserializerState as S;
            match state {
                S::TextBefore(Some(deserializer)) => {
                    self.store_text_before(deserializer.finish(reader)?)?
                }
                S::BaseElement(Some(deserializer)) => {
                    self.store_base_element(deserializer.finish(reader)?)?
                }
                S::Any(Some(deserializer)) => self.store_any(deserializer.finish(reader)?)?,
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
        fn store_base_element(&mut self, value: Mixed<String>) -> Result<(), Error> {
            if self.base_element.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"BaseElement",
                )))?;
            }
            self.base_element = Some(value);
            Ok(())
        }
        fn store_any(&mut self, value: Mixed<AnyElement>) -> Result<(), Error> {
            self.any.push(value);
            Ok(())
        }
        fn handle_text_before<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, Text>,
            fallback: &mut Option<AttributeValueTypeDeserializerState>,
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
                fallback.get_or_insert(AttributeValueTypeDeserializerState::TextBefore(None));
                *self.state__ = AttributeValueTypeDeserializerState::BaseElement(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_before(data)?;
                    *self.state__ = AttributeValueTypeDeserializerState::BaseElement(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                AttributeValueTypeDeserializerState::TextBefore(Some(deserializer)),
                            );
                            *self.state__ = AttributeValueTypeDeserializerState::BaseElement(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                AttributeValueTypeDeserializerState::TextBefore(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_base_element<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, Mixed<String>>,
            fallback: &mut Option<AttributeValueTypeDeserializerState>,
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
                if self.base_element.is_some() {
                    fallback.get_or_insert(AttributeValueTypeDeserializerState::BaseElement(None));
                    *self.state__ = AttributeValueTypeDeserializerState::Any(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = AttributeValueTypeDeserializerState::BaseElement(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_base_element(data)?;
                    *self.state__ = AttributeValueTypeDeserializerState::Any(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                AttributeValueTypeDeserializerState::BaseElement(Some(
                                    deserializer,
                                )),
                            );
                            *self.state__ = AttributeValueTypeDeserializerState::Any(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ = AttributeValueTypeDeserializerState::BaseElement(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_any<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, Mixed<AnyElement>>,
            fallback: &mut Option<AttributeValueTypeDeserializerState>,
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
                fallback.get_or_insert(AttributeValueTypeDeserializerState::Any(None));
                *self.state__ = AttributeValueTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_any(data)?;
                    *self.state__ = AttributeValueTypeDeserializerState::Any(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(AttributeValueTypeDeserializerState::Any(Some(
                                deserializer,
                            )));
                            *self.state__ = AttributeValueTypeDeserializerState::Any(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                AttributeValueTypeDeserializerState::Any(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::AttributeValueType> for AttributeValueTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AttributeValueType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AttributeValueType>
        where
            R: DeserializeReader,
        {
            use AttributeValueTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let mut is_any_retry = false;
            let mut any_fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
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
                    (S::BaseElement(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_base_element(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Any(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_any(reader, output, &mut fallback)? {
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
                        *self.state__ = AttributeValueTypeDeserializerState::TextBefore(None);
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
                    (S::BaseElement(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"BaseElement",
                            false,
                        )?;
                        match self.handle_base_element(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Any(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if is_any_retry {
                            let output =
                                <Mixed<AnyElement> as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_any(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            any_fallback.get_or_insert(S::Any(None));
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
        fn finish<R>(mut self, reader: &R) -> Result<super::AttributeValueType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state__,
                AttributeValueTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::AttributeValueType {
                base_attrib: self.base_attrib,
                data_type: self.data_type,
                any_attribute: self.any_attribute,
                text_before: self.text_before,
                base_element: self
                    .base_element
                    .ok_or_else(|| ErrorKind::MissingElement("BaseElement".into()))?,
                any: self.any,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::{
        quick_xml::{
            write_attrib, BytesEnd, BytesStart, Error, Event, IterSerializer, WithSerializer,
        },
        xml::{AnyElement, Mixed, Text},
    };
    #[derive(Debug)]
    pub struct AttributeValueTypeSerializer<'ser> {
        pub(super) value: &'ser super::AttributeValueType,
        pub(super) state: Box<AttributeValueTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum AttributeValueTypeSerializerState<'ser> {
        Init__,
        TextBefore(IterSerializer<'ser, Option<&'ser Text>, Text>),
        BaseElement(<Mixed<String> as WithSerializer>::Serializer<'ser>),
        Any(IterSerializer<'ser, &'ser [Mixed<AnyElement>], Mixed<AnyElement>>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> AttributeValueTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    AttributeValueTypeSerializerState::Init__ => {
                        *self.state = AttributeValueTypeSerializerState::TextBefore(
                            IterSerializer::new(self.value.text_before.as_ref(), Some(""), false),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        write_attrib(&mut bytes, "BaseAttrib", &self.value.base_attrib)?;
                        write_attrib(&mut bytes, "DataType", &self.value.data_type)?;
                        bytes.extend_attributes(self.value.any_attribute.attributes());
                        return Ok(Some(Event::Start(bytes)));
                    }
                    AttributeValueTypeSerializerState::TextBefore(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = AttributeValueTypeSerializerState::BaseElement(
                                    WithSerializer::serializer(
                                        &self.value.base_element,
                                        Some("tns:BaseElement"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    AttributeValueTypeSerializerState::BaseElement(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = AttributeValueTypeSerializerState::Any(
                                    IterSerializer::new(&self.value.any[..], None, false),
                                )
                            }
                        }
                    }
                    AttributeValueTypeSerializerState::Any(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = AttributeValueTypeSerializerState::End__,
                    },
                    AttributeValueTypeSerializerState::End__ => {
                        *self.state = AttributeValueTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    AttributeValueTypeSerializerState::Done__ => return Ok(None),
                    AttributeValueTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for AttributeValueTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = AttributeValueTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
