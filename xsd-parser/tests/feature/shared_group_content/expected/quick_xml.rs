use std::borrow::Cow;
use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{
        DeserializeBytes, DeserializeHelper, Error, SerializeBytes, SerializeHelper,
        WithDeserializer, WithDeserializerFromBytes, WithSerializeToBytes, WithSerializer,
    },
    xml::{Base64String, HexString},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
#[derive(Debug, Default)]
pub struct EntitiesType(pub Vec<String>);
impl SerializeBytes for EntitiesType {
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        if self.0.is_empty() {
            return Ok(None);
        }
        let mut data = String::new();
        for item in &self.0 {
            if let Some(bytes) = item.serialize_bytes(helper)? {
                if !data.is_empty() {
                    data.push(' ');
                }
                data.push_str(&bytes);
            }
        }
        Ok(Some(Cow::Owned(data)))
    }
}
impl WithSerializeToBytes for EntitiesType {}
impl DeserializeBytes for EntitiesType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
impl WithDeserializerFromBytes for EntitiesType {}
pub type EntityType = String;
pub type IdType = String;
pub type IdrefType = String;
pub type IdrefsType = EntitiesType;
pub type NcNameType = String;
pub type NmtokenType = String;
pub type NmtokensType = EntitiesType;
pub type NotationType = String;
pub type NameType = String;
pub type QNameType = String;
pub type AnySimpleType = String;
pub type AnyUriType = String;
pub type Base64BinaryType = Base64String;
pub type BooleanType = bool;
pub type ByteType = i8;
pub type DateType = String;
pub type DateTimeType = String;
pub type DecimalType = f64;
pub type DoubleType = f64;
pub type DurationType = String;
pub type FloatType = f32;
pub type GDayType = String;
pub type GMonthType = String;
pub type GMonthDayType = String;
pub type GYearType = String;
pub type GYearMonthType = String;
pub type HexBinaryType = HexString;
pub type IntType = i32;
pub type IntegerType = i32;
pub type LanguageType = String;
pub type LongType = i64;
pub type NegativeIntegerType = isize;
pub type NonNegativeIntegerType = usize;
pub type NonPositiveIntegerType = isize;
pub type NormalizedStringType = String;
pub type PositiveIntegerType = usize;
pub type ShortType = i16;
pub type StringType = String;
pub type TimeType = String;
pub type TokenType = String;
pub type UnsignedByteType = u8;
pub type UnsignedIntType = u32;
pub type UnsignedLongType = u64;
pub type UnsignedShortType = u16;
#[derive(Debug)]
pub struct ElementAType {
    pub attr_1: Option<String>,
    pub content: Vec<ElementAChoiceGroupType>,
}
impl WithSerializer for ElementAType {
    type Serializer<'x> = quick_xml_serialize::ElementATypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ElementATypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ElementATypeSerializerState::Init__),
            name: name.unwrap_or("ElementAType"),
            is_root,
        })
    }
}
impl WithDeserializer for ElementAType {
    type Deserializer = quick_xml_deserialize::ElementATypeDeserializer;
}
#[derive(Debug)]
pub struct ElementBType {
    pub attr_2: Option<String>,
    pub content: Vec<ElementAChoiceGroupType>,
}
impl WithSerializer for ElementBType {
    type Serializer<'x> = quick_xml_serialize::ElementBTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ElementBTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ElementBTypeSerializerState::Init__),
            name: name.unwrap_or("ElementBType"),
            is_root,
        })
    }
}
impl WithDeserializer for ElementBType {
    type Deserializer = quick_xml_deserialize::ElementBTypeDeserializer;
}
#[derive(Debug)]
pub enum ElementAChoiceGroupType {
    ElementA(ElementAType),
    ElementB(ElementBType),
}
impl WithSerializer for ElementAChoiceGroupType {
    type Serializer<'x> = quick_xml_serialize::ElementAChoiceGroupTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        Ok(quick_xml_serialize::ElementAChoiceGroupTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ElementAChoiceGroupTypeSerializerState::Init__),
            is_root,
        })
    }
}
impl WithDeserializer for ElementAChoiceGroupType {
    type Deserializer = quick_xml_deserialize::ElementAChoiceGroupTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct ElementATypeDeserializer {
        attr_1: Option<String>,
        content: Vec<super::ElementAChoiceGroupType>,
        state__: Box<ElementATypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ElementATypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::ElementAChoiceGroupType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ElementATypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut attr_1: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"attr1")
                ) {
                    helper.read_attrib(&mut attr_1, b"attr1", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                attr_1: attr_1,
                content: Vec::new(),
                state__: Box::new(ElementATypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ElementATypeDeserializerState,
        ) -> Result<(), Error> {
            if let ElementATypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ElementAChoiceGroupType) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ElementAChoiceGroupType>,
            fallback: &mut Option<ElementATypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ElementATypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::ElementAType> for ElementATypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ElementAType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ElementAType> {
            use ElementATypeDeserializerState as S;
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
                        let output = <super::ElementAChoiceGroupType as WithDeserializer>::init(
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ElementAType, Error> {
            let state = replace(&mut *self.state__, ElementATypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::ElementAType {
                attr_1: self.attr_1,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ElementBTypeDeserializer {
        attr_2: Option<String>,
        content: Vec<super::ElementAChoiceGroupType>,
        state__: Box<ElementBTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ElementBTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::ElementAChoiceGroupType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ElementBTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut attr_2: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"attr2")
                ) {
                    helper.read_attrib(&mut attr_2, b"attr2", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                attr_2: attr_2,
                content: Vec::new(),
                state__: Box::new(ElementBTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ElementBTypeDeserializerState,
        ) -> Result<(), Error> {
            if let ElementBTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ElementAChoiceGroupType) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ElementAChoiceGroupType>,
            fallback: &mut Option<ElementBTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ElementBTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::ElementBType> for ElementBTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ElementBType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ElementBType> {
            use ElementBTypeDeserializerState as S;
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
                        let output = <super::ElementAChoiceGroupType as WithDeserializer>::init(
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ElementBType, Error> {
            let state = replace(&mut *self.state__, ElementBTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::ElementBType {
                attr_2: self.attr_2,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ElementAChoiceGroupTypeDeserializer {
        state__: Box<ElementAChoiceGroupTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ElementAChoiceGroupTypeDeserializerState {
        Init__,
        ElementA(
            Option<super::ElementAType>,
            Option<<super::ElementAType as WithDeserializer>::Deserializer>,
            Option<<super::ElementAType as WithDeserializer>::Deserializer>,
        ),
        ElementB(
            Option<super::ElementBType>,
            Option<<super::ElementBType as WithDeserializer>::Deserializer>,
            Option<<super::ElementBType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ElementAChoiceGroupType),
        Unknown__,
    }
    impl ElementAChoiceGroupTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"ElementA")
                ) {
                    let output = <super::ElementAType as WithDeserializer>::init(helper, event)?;
                    return self.handle_element_a(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"ElementB")
                ) {
                    let output = <super::ElementBType as WithDeserializer>::init(helper, event)?;
                    return self.handle_element_b(helper, Default::default(), None, output);
                }
            }
            *self.state__ = ElementAChoiceGroupTypeDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: ElementAChoiceGroupTypeDeserializerState,
        ) -> Result<super::ElementAChoiceGroupType, Error> {
            use ElementAChoiceGroupTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::ElementA(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_element_a(&mut values, value)?;
                    }
                    Ok(super::ElementAChoiceGroupType::ElementA(
                        helper.finish_element("ElementA", values)?,
                    ))
                }
                S::ElementB(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_element_b(&mut values, value)?;
                    }
                    Ok(super::ElementAChoiceGroupType::ElementB(
                        helper.finish_element("ElementB", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_element_a(
            values: &mut Option<super::ElementAType>,
            value: super::ElementAType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ElementA",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_element_b(
            values: &mut Option<super::ElementBType>,
            value: super::ElementBType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ElementB",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_element_a<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ElementAType>,
            fallback: Option<<super::ElementAType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ElementAType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ElementAChoiceGroupTypeDeserializerState as S;
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
                Self::store_element_a(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_element_a(&mut values, data)?;
                    let data = Self::finish_state(helper, S::ElementA(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::ElementA(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_element_b<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ElementBType>,
            fallback: Option<<super::ElementBType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ElementBType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ElementAChoiceGroupTypeDeserializerState as S;
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
                Self::store_element_b(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_element_b(&mut values, data)?;
                    let data = Self::finish_state(helper, S::ElementB(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::ElementB(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ElementAChoiceGroupType>
        for ElementAChoiceGroupTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ElementAChoiceGroupType> {
            let deserializer = Self {
                state__: Box::new(ElementAChoiceGroupTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        ElementAChoiceGroupTypeDeserializerState::Init__
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
        ) -> DeserializerResult<'de, super::ElementAChoiceGroupType> {
            use ElementAChoiceGroupTypeDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::ElementA(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_element_a(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::ElementB(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_element_b(helper, values, fallback, output)? {
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
                        S::ElementA(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"ElementA",
                            false,
                        )?;
                        match self.handle_element_a(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::ElementB(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"ElementB",
                            false,
                        )?;
                        match self.handle_element_b(helper, values, fallback, output)? {
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
        ) -> Result<super::ElementAChoiceGroupType, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, Error, Event, IterSerializer, SerializeHelper, Serializer,
        WithSerializer,
    };
    #[derive(Debug)]
    pub struct ElementATypeSerializer<'ser> {
        pub(super) value: &'ser super::ElementAType,
        pub(super) state: Box<ElementATypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ElementATypeSerializerState<'ser> {
        Init__,
        Content__(
            IterSerializer<
                'ser,
                &'ser [super::ElementAChoiceGroupType],
                super::ElementAChoiceGroupType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ElementATypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ElementATypeSerializerState::Init__ => {
                        *self.state = ElementATypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns_for_tag(&mut bytes, self.name, &super::NS_TNS);
                        }
                        helper.write_attrib_opt(&mut bytes, "attr1", &self.value.attr_1)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ElementATypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ElementATypeSerializerState::End__,
                        }
                    }
                    ElementATypeSerializerState::End__ => {
                        *self.state = ElementATypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ElementATypeSerializerState::Done__ => return Ok(None),
                    ElementATypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ElementATypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ElementATypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ElementBTypeSerializer<'ser> {
        pub(super) value: &'ser super::ElementBType,
        pub(super) state: Box<ElementBTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ElementBTypeSerializerState<'ser> {
        Init__,
        Content__(
            IterSerializer<
                'ser,
                &'ser [super::ElementAChoiceGroupType],
                super::ElementAChoiceGroupType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ElementBTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ElementBTypeSerializerState::Init__ => {
                        *self.state = ElementBTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns_for_tag(&mut bytes, self.name, &super::NS_TNS);
                        }
                        helper.write_attrib_opt(&mut bytes, "attr2", &self.value.attr_2)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ElementBTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ElementBTypeSerializerState::End__,
                        }
                    }
                    ElementBTypeSerializerState::End__ => {
                        *self.state = ElementBTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ElementBTypeSerializerState::Done__ => return Ok(None),
                    ElementBTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ElementBTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ElementBTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ElementAChoiceGroupTypeSerializer<'ser> {
        pub(super) value: &'ser super::ElementAChoiceGroupType,
        pub(super) state: Box<ElementAChoiceGroupTypeSerializerState<'ser>>,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ElementAChoiceGroupTypeSerializerState<'ser> {
        Init__,
        ElementA(<super::ElementAType as WithSerializer>::Serializer<'ser>),
        ElementB(<super::ElementBType as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ElementAChoiceGroupTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ElementAChoiceGroupTypeSerializerState::Init__ => match self.value {
                        super::ElementAChoiceGroupType::ElementA(x) => {
                            *self.state = ElementAChoiceGroupTypeSerializerState::ElementA(
                                WithSerializer::serializer(x, Some("ElementA"), self.is_root)?,
                            )
                        }
                        super::ElementAChoiceGroupType::ElementB(x) => {
                            *self.state = ElementAChoiceGroupTypeSerializerState::ElementB(
                                WithSerializer::serializer(x, Some("ElementB"), self.is_root)?,
                            )
                        }
                    },
                    ElementAChoiceGroupTypeSerializerState::ElementA(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ElementAChoiceGroupTypeSerializerState::Done__,
                        }
                    }
                    ElementAChoiceGroupTypeSerializerState::ElementB(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ElementAChoiceGroupTypeSerializerState::Done__,
                        }
                    }
                    ElementAChoiceGroupTypeSerializerState::Done__ => return Ok(None),
                    ElementAChoiceGroupTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ElementAChoiceGroupTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ElementAChoiceGroupTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
