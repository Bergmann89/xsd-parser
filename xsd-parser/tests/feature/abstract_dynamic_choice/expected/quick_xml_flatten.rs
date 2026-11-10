use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{ElementNamespace, Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_TNS: Namespace = Namespace::new_const(b"urn:example");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
#[derive(Debug)]
pub enum PayloadType {
    Leaf(LeafType),
}
impl WithSerializer for PayloadType {
    type Serializer<'x> = quick_xml_serialize::PayloadTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        WithSerializer::serializer_with_namespace(self, name, ElementNamespace::Unknown, is_root)
    }
    fn serializer_with_namespace<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        namespace: ElementNamespace,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::PayloadTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::PayloadTypeSerializerState::Init__),
            name,
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for PayloadType {
    type Deserializer = quick_xml_deserialize::PayloadTypeDeserializer;
}
#[derive(Debug)]
pub struct LeafType {
    pub id: u32,
    pub label: String,
}
impl WithSerializer for LeafType {
    type Serializer<'x> = quick_xml_serialize::LeafTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        WithSerializer::serializer_with_namespace(self, name, ElementNamespace::Unknown, is_root)
    }
    fn serializer_with_namespace<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        namespace: ElementNamespace,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::LeafTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::LeafTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Leaf"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for LeafType {
    type Deserializer = quick_xml_deserialize::LeafTypeDeserializer;
}
#[derive(Debug)]
pub enum MiddleType {
    Leaf(LeafType),
}
impl WithSerializer for MiddleType {
    type Serializer<'x> = quick_xml_serialize::MiddleTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        WithSerializer::serializer_with_namespace(self, name, ElementNamespace::Unknown, is_root)
    }
    fn serializer_with_namespace<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        namespace: ElementNamespace,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::MiddleTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::MiddleTypeSerializerState::Init__),
            name,
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for MiddleType {
    type Deserializer = quick_xml_deserialize::MiddleTypeDeserializer;
}
#[derive(Debug)]
pub enum EmptyType {}
impl WithSerializer for EmptyType {
    type Serializer<'x> = quick_xml_serialize::EmptyTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        WithSerializer::serializer_with_namespace(self, name, ElementNamespace::Unknown, is_root)
    }
    fn serializer_with_namespace<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        namespace: ElementNamespace,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _namespace = namespace;
        let _is_root = is_root;
        Ok(quick_xml_serialize::EmptyTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::EmptyTypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for EmptyType {
    type Deserializer = quick_xml_deserialize::EmptyTypeDeserializer;
}
#[derive(Debug)]
pub enum Head {
    Member(String),
}
impl WithSerializer for Head {
    type Serializer<'x> = quick_xml_serialize::HeadSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        WithSerializer::serializer_with_namespace(self, name, ElementNamespace::Unknown, is_root)
    }
    fn serializer_with_namespace<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        namespace: ElementNamespace,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _namespace = namespace;
        Ok(quick_xml_serialize::HeadSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::HeadSerializerState::Init__),
            is_root,
        })
    }
}
impl WithDeserializer for Head {
    type Deserializer = quick_xml_deserialize::HeadDeserializer;
}
#[derive(Debug)]
pub enum AbstractMember {
    Member(String),
}
impl WithSerializer for AbstractMember {
    type Serializer<'x> = quick_xml_serialize::AbstractMemberSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        WithSerializer::serializer_with_namespace(self, name, ElementNamespace::Unknown, is_root)
    }
    fn serializer_with_namespace<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        namespace: ElementNamespace,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _namespace = namespace;
        Ok(quick_xml_serialize::AbstractMemberSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::AbstractMemberSerializerState::Init__),
            is_root,
        })
    }
}
impl WithDeserializer for AbstractMember {
    type Deserializer = quick_xml_deserialize::AbstractMemberDeserializer;
}
#[derive(Debug)]
pub enum EmptyHead {}
impl WithSerializer for EmptyHead {
    type Serializer<'x> = quick_xml_serialize::EmptyHeadSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        WithSerializer::serializer_with_namespace(self, name, ElementNamespace::Unknown, is_root)
    }
    fn serializer_with_namespace<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        namespace: ElementNamespace,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _namespace = namespace;
        let _is_root = is_root;
        Ok(quick_xml_serialize::EmptyHeadSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::EmptyHeadSerializerState::Init__),
        })
    }
}
impl WithDeserializer for EmptyHead {
    type Deserializer = quick_xml_deserialize::EmptyHeadDeserializer;
}
#[derive(Debug)]
pub enum EmptyAbstractMember {}
impl WithSerializer for EmptyAbstractMember {
    type Serializer<'x> = quick_xml_serialize::EmptyAbstractMemberSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        WithSerializer::serializer_with_namespace(self, name, ElementNamespace::Unknown, is_root)
    }
    fn serializer_with_namespace<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        namespace: ElementNamespace,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _namespace = namespace;
        let _is_root = is_root;
        Ok(quick_xml_serialize::EmptyAbstractMemberSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::EmptyAbstractMemberSerializerState::Init__),
        })
    }
}
impl WithDeserializer for EmptyAbstractMember {
    type Deserializer = quick_xml_deserialize::EmptyAbstractMemberDeserializer;
}
pub type Message = MessageType;
#[derive(Debug)]
pub struct MessageType {
    pub payload: PayloadType,
    pub head: Head,
}
impl WithSerializer for MessageType {
    type Serializer<'x> = quick_xml_serialize::MessageTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        WithSerializer::serializer_with_namespace(self, name, ElementNamespace::Unknown, is_root)
    }
    fn serializer_with_namespace<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        namespace: ElementNamespace,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::MessageTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::MessageTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Message"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for MessageType {
    type Deserializer = quick_xml_deserialize::MessageTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        QName, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct PayloadTypeDeserializer {
        state__: Box<PayloadTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum PayloadTypeDeserializerState {
        Init__,
        Leaf(
            Option<super::LeafType>,
            Option<<super::LeafType as WithDeserializer>::Deserializer>,
            Option<<super::LeafType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::PayloadType),
        Unknown__,
    }
    impl PayloadTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
            ns: Option<&[u8]>,
            name: &[u8],
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if helper.check_start_tag_name(&event, ns, name) {
                    if let Some(type_name) = helper.get_dynamic_type_from_attrib_bytes(x)? {
                        let type_name = type_name.into_owned();
                        if matches!(
                            helper.resolve_element_local_name(QName(&type_name), &super::NS_TNS),
                            Some(b"Leaf")
                        ) {
                            let output =
                                <super::LeafType as WithDeserializer>::init(helper, event)?;
                            return self.handle_leaf(helper, Default::default(), None, output);
                        }
                    }
                }
            }
            *self.state__ = PayloadTypeDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn next_with_name<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
            ns: Option<&[u8]>,
            name: &[u8],
        ) -> DeserializerResult<'de, super::PayloadType> {
            use PayloadTypeDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Leaf(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_leaf(helper, values, fallback, output)? {
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
                    (S::Init__, event) => match self.find_suitable(helper, event, ns, name)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Leaf(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Leaf",
                            false,
                        )?;
                        match self.handle_leaf(helper, values, fallback, output)? {
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
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: PayloadTypeDeserializerState,
        ) -> Result<super::PayloadType, Error> {
            use PayloadTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Leaf(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_leaf(&mut values, value)?;
                    }
                    Ok(super::PayloadType::Leaf(
                        helper.finish_element("Leaf", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_leaf(
            values: &mut Option<super::LeafType>,
            value: super::LeafType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Leaf")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_leaf<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::LeafType>,
            fallback: Option<<super::LeafType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::LeafType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PayloadTypeDeserializerState as S;
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
                Self::store_leaf(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_leaf(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Leaf(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Leaf(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::PayloadType> for PayloadTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PayloadType> {
            Self::init_with_name(helper, event, Some(&super::NS_TNS), b"Payload")
        }
        fn init_with_name(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
            ns: Option<&[u8]>,
            name: &[u8],
        ) -> DeserializerResult<'de, super::PayloadType> {
            let deserializer = Self {
                state__: Box::new(PayloadTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next_with_name(helper, event, ns, name)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, PayloadTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::PayloadType> {
            self.next_with_name(helper, event, Some(&super::NS_TNS), b"Payload")
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::PayloadType, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct LeafTypeDeserializer {
        id: u32,
        label: String,
        state__: Box<LeafTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum LeafTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl LeafTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut id: Option<u32> = None;
            let mut label: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_attrib_local_name(attrib.key, &super::NS_TNS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_attrib_local_name(attrib.key, &super::NS_TNS),
                    Some(b"label")
                ) {
                    helper.read_attrib(&mut label, b"label", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id.ok_or_else(|| ErrorKind::MissingAttribute("id".into()))?,
                label: label.ok_or_else(|| ErrorKind::MissingAttribute("label".into()))?,
                state__: Box::new(LeafTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: LeafTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::LeafType> for LeafTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::LeafType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::LeafType> {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(helper)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                })
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::LeafType, Error> {
            let state = replace(&mut *self.state__, LeafTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::LeafType {
                id: self.id,
                label: self.label,
            })
        }
    }
    #[derive(Debug)]
    pub struct MiddleTypeDeserializer {
        state__: Box<MiddleTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum MiddleTypeDeserializerState {
        Init__,
        Leaf(
            Option<super::LeafType>,
            Option<<super::LeafType as WithDeserializer>::Deserializer>,
            Option<<super::LeafType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::MiddleType),
        Unknown__,
    }
    impl MiddleTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
            ns: Option<&[u8]>,
            name: &[u8],
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if helper.check_start_tag_name(&event, ns, name) {
                    if let Some(type_name) = helper.get_dynamic_type_from_attrib_bytes(x)? {
                        let type_name = type_name.into_owned();
                        if matches!(
                            helper.resolve_element_local_name(QName(&type_name), &super::NS_TNS),
                            Some(b"Leaf")
                        ) {
                            let output =
                                <super::LeafType as WithDeserializer>::init(helper, event)?;
                            return self.handle_leaf(helper, Default::default(), None, output);
                        }
                    }
                }
            }
            *self.state__ = MiddleTypeDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn next_with_name<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
            ns: Option<&[u8]>,
            name: &[u8],
        ) -> DeserializerResult<'de, super::MiddleType> {
            use MiddleTypeDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Leaf(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_leaf(helper, values, fallback, output)? {
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
                    (S::Init__, event) => match self.find_suitable(helper, event, ns, name)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Leaf(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Leaf",
                            false,
                        )?;
                        match self.handle_leaf(helper, values, fallback, output)? {
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
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: MiddleTypeDeserializerState,
        ) -> Result<super::MiddleType, Error> {
            use MiddleTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Leaf(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_leaf(&mut values, value)?;
                    }
                    Ok(super::MiddleType::Leaf(
                        helper.finish_element("Leaf", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_leaf(
            values: &mut Option<super::LeafType>,
            value: super::LeafType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Leaf")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_leaf<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::LeafType>,
            fallback: Option<<super::LeafType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::LeafType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MiddleTypeDeserializerState as S;
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
                Self::store_leaf(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_leaf(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Leaf(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Leaf(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::MiddleType> for MiddleTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MiddleType> {
            Self::init_with_name(helper, event, Some(&super::NS_TNS), b"Middle")
        }
        fn init_with_name(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
            ns: Option<&[u8]>,
            name: &[u8],
        ) -> DeserializerResult<'de, super::MiddleType> {
            let deserializer = Self {
                state__: Box::new(MiddleTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next_with_name(helper, event, ns, name)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, MiddleTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::MiddleType> {
            self.next_with_name(helper, event, Some(&super::NS_TNS), b"Middle")
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::MiddleType, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct EmptyTypeDeserializer {
        state__: Box<EmptyTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum EmptyTypeDeserializerState {
        Init__,
        Done__(super::EmptyType),
        Unknown__,
    }
    impl EmptyTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            _: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(_) | Event::Empty(_) = &event {}
            *self.state__ = EmptyTypeDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            _: &mut DeserializeHelper,
            state: EmptyTypeDeserializerState,
        ) -> Result<super::EmptyType, Error> {
            use EmptyTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
    }
    impl<'de> Deserializer<'de, super::EmptyType> for EmptyTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EmptyType> {
            let deserializer = Self {
                state__: Box::new(EmptyTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, EmptyTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::EmptyType> {
            use EmptyTypeDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
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
                    (state @ S::Done__(_), event) => {
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::EmptyType, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct HeadDeserializer {
        state__: Box<HeadDeserializerState>,
    }
    #[derive(Debug)]
    pub enum HeadDeserializerState {
        Init__,
        Member(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::Head),
        Unknown__,
    }
    impl HeadDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                    Some(b"Member")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_member(helper, Default::default(), None, output);
                }
            }
            *self.state__ = HeadDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: HeadDeserializerState,
        ) -> Result<super::Head, Error> {
            use HeadDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Member(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_member(&mut values, value)?;
                    }
                    Ok(super::Head::Member(
                        helper.finish_element("Member", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_member(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Member",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_member<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeadDeserializerState as S;
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
                Self::store_member(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_member(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Member(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Member(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::Head> for HeadDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Head> {
            let deserializer = Self {
                state__: Box::new(HeadDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, HeadDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::Head> {
            use HeadDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Member(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_member(helper, values, fallback, output)? {
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
                        S::Member(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Member",
                            false,
                        )?;
                        match self.handle_member(helper, values, fallback, output)? {
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::Head, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct AbstractMemberDeserializer {
        state__: Box<AbstractMemberDeserializerState>,
    }
    #[derive(Debug)]
    pub enum AbstractMemberDeserializerState {
        Init__,
        Member(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::AbstractMember),
        Unknown__,
    }
    impl AbstractMemberDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                    Some(b"Member")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_member(helper, Default::default(), None, output);
                }
            }
            *self.state__ = AbstractMemberDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: AbstractMemberDeserializerState,
        ) -> Result<super::AbstractMember, Error> {
            use AbstractMemberDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Member(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_member(&mut values, value)?;
                    }
                    Ok(super::AbstractMember::Member(
                        helper.finish_element("Member", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_member(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Member",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_member<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AbstractMemberDeserializerState as S;
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
                Self::store_member(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_member(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Member(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Member(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::AbstractMember> for AbstractMemberDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AbstractMember> {
            let deserializer = Self {
                state__: Box::new(AbstractMemberDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, AbstractMemberDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::AbstractMember> {
            use AbstractMemberDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Member(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_member(helper, values, fallback, output)? {
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
                        S::Member(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Member",
                            false,
                        )?;
                        match self.handle_member(helper, values, fallback, output)? {
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::AbstractMember, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct EmptyHeadDeserializer {
        state__: Box<EmptyHeadDeserializerState>,
    }
    #[derive(Debug)]
    pub enum EmptyHeadDeserializerState {
        Init__,
        Done__(super::EmptyHead),
        Unknown__,
    }
    impl EmptyHeadDeserializer {
        fn find_suitable<'de>(
            &mut self,
            _: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(_) | Event::Empty(_) = &event {}
            *self.state__ = EmptyHeadDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            _: &mut DeserializeHelper,
            state: EmptyHeadDeserializerState,
        ) -> Result<super::EmptyHead, Error> {
            use EmptyHeadDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
    }
    impl<'de> Deserializer<'de, super::EmptyHead> for EmptyHeadDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EmptyHead> {
            let deserializer = Self {
                state__: Box::new(EmptyHeadDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, EmptyHeadDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::EmptyHead> {
            use EmptyHeadDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
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
                    (state @ S::Done__(_), event) => {
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::EmptyHead, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct EmptyAbstractMemberDeserializer {
        state__: Box<EmptyAbstractMemberDeserializerState>,
    }
    #[derive(Debug)]
    pub enum EmptyAbstractMemberDeserializerState {
        Init__,
        Done__(super::EmptyAbstractMember),
        Unknown__,
    }
    impl EmptyAbstractMemberDeserializer {
        fn find_suitable<'de>(
            &mut self,
            _: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(_) | Event::Empty(_) = &event {}
            *self.state__ = EmptyAbstractMemberDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            _: &mut DeserializeHelper,
            state: EmptyAbstractMemberDeserializerState,
        ) -> Result<super::EmptyAbstractMember, Error> {
            use EmptyAbstractMemberDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
    }
    impl<'de> Deserializer<'de, super::EmptyAbstractMember> for EmptyAbstractMemberDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EmptyAbstractMember> {
            let deserializer = Self {
                state__: Box::new(EmptyAbstractMemberDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, EmptyAbstractMemberDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::EmptyAbstractMember> {
            use EmptyAbstractMemberDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
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
                    (state @ S::Done__(_), event) => {
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
        ) -> Result<super::EmptyAbstractMember, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct MessageTypeDeserializer {
        payload: Option<super::PayloadType>,
        head: Option<super::Head>,
        state__: Box<MessageTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum MessageTypeDeserializerState {
        Init__,
        Payload(Option<<super::PayloadType as WithDeserializer>::Deserializer>),
        Head(Option<<super::Head as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl MessageTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                payload: None,
                head: None,
                state__: Box::new(MessageTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: MessageTypeDeserializerState,
        ) -> Result<(), Error> {
            use MessageTypeDeserializerState as S;
            match state {
                S::Payload(Some(deserializer)) => {
                    self.store_payload(deserializer.finish(helper)?)?
                }
                S::Head(Some(deserializer)) => self.store_head(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_payload(&mut self, value: super::PayloadType) -> Result<(), Error> {
            if self.payload.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Payload",
                )))?;
            }
            self.payload = Some(value);
            Ok(())
        }
        fn store_head(&mut self, value: super::Head) -> Result<(), Error> {
            if self.head.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Head")))?;
            }
            self.head = Some(value);
            Ok(())
        }
        fn handle_payload<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::PayloadType>,
            fallback: &mut Option<MessageTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MessageTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Payload(None));
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
                    self.store_payload(data)?;
                    *self.state__ = S::Head(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Payload(Some(deserializer)));
                    *self.state__ = S::Head(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_head<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::Head>,
            fallback: &mut Option<MessageTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MessageTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Head(None));
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
                    self.store_head(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Head(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::MessageType> for MessageTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MessageType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MessageType> {
            use MessageTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Payload(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_payload(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Head(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_head(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Payload(None);
                        event
                    }
                    (S::Payload(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = <super::PayloadType as WithDeserializer>::init_with_name(
                            helper,
                            event,
                            Some(&super::NS_TNS),
                            b"Payload",
                        )?;
                        match self.handle_payload(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Head(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = <super::Head as WithDeserializer>::init_with_name(
                            helper,
                            event,
                            Some(&super::NS_TNS),
                            b"Head",
                        )?;
                        match self.handle_head(helper, output, &mut fallback)? {
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
            let event = event.into_break();
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::MessageType, Error> {
            let state = replace(&mut *self.state__, MessageTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::MessageType {
                payload: helper.finish_element("Payload", self.payload)?,
                head: helper.finish_element("Head", self.head)?,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, ElementNamespace, Error, Event, SerializeHelper, Serializer,
        WithSerializer, XsiTypeSerializer,
    };
    #[derive(Debug)]
    pub struct PayloadTypeSerializer<'ser> {
        pub(super) value: &'ser super::PayloadType,
        pub(super) state: Box<PayloadTypeSerializerState<'ser>>,
        pub(super) name: Option<&'ser str>,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum PayloadTypeSerializerState<'ser> {
        Init__,
        Leaf(XsiTypeSerializer<'ser, <super::LeafType as WithSerializer>::Serializer<'ser>>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PayloadTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PayloadTypeSerializerState::Init__ => match self.value {
                        super::PayloadType::Leaf(x) => {
                            *self.state = PayloadTypeSerializerState::Leaf(
                                XsiTypeSerializer::new(
                                    WithSerializer::serializer_with_namespace(
                                        x,
                                        Some("tns:Leaf"),
                                        ElementNamespace::Unknown,
                                        self.is_root,
                                    )?,
                                    self.name.unwrap_or("tns:Payload"),
                                    self.is_root,
                                )
                                .with_namespace(self.namespace.clone())
                                .with_type_namespace(super::NS_TNS),
                            )
                        }
                    },
                    PayloadTypeSerializerState::Leaf(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PayloadTypeSerializerState::Done__,
                    },
                    PayloadTypeSerializerState::Done__ => return Ok(None),
                    PayloadTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for PayloadTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PayloadTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct LeafTypeSerializer<'ser> {
        pub(super) value: &'ser super::LeafType,
        pub(super) state: Box<LeafTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum LeafTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> LeafTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    LeafTypeSerializerState::Init__ => {
                        *self.state = LeafTypeSerializerState::Done__;
                        let namespace = if self.is_root {
                            ElementNamespace::Namespace(super::NS_TNS)
                        } else {
                            self.namespace.clone()
                        };
                        self.name = helper.element_name(self.name, &namespace);
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns_for_element(&mut bytes, self.name, &namespace);
                        if self.is_root {
                            helper.write_default_namespace(&mut bytes, self.name, &namespace);
                        }
                        if self.is_root {
                            helper.write_xmlns_unless_default(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        helper.write_attrib(&mut bytes, "id", &self.value.id)?;
                        helper.write_attrib(&mut bytes, "label", &self.value.label)?;
                        helper.end_ns_scope();
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    LeafTypeSerializerState::Done__ => return Ok(None),
                    LeafTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for LeafTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = LeafTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct MiddleTypeSerializer<'ser> {
        pub(super) value: &'ser super::MiddleType,
        pub(super) state: Box<MiddleTypeSerializerState<'ser>>,
        pub(super) name: Option<&'ser str>,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum MiddleTypeSerializerState<'ser> {
        Init__,
        Leaf(XsiTypeSerializer<'ser, <super::LeafType as WithSerializer>::Serializer<'ser>>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> MiddleTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MiddleTypeSerializerState::Init__ => match self.value {
                        super::MiddleType::Leaf(x) => {
                            *self.state = MiddleTypeSerializerState::Leaf(
                                XsiTypeSerializer::new(
                                    WithSerializer::serializer_with_namespace(
                                        x,
                                        Some("tns:Leaf"),
                                        ElementNamespace::Unknown,
                                        self.is_root,
                                    )?,
                                    self.name.unwrap_or("tns:Middle"),
                                    self.is_root,
                                )
                                .with_namespace(self.namespace.clone())
                                .with_type_namespace(super::NS_TNS),
                            )
                        }
                    },
                    MiddleTypeSerializerState::Leaf(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = MiddleTypeSerializerState::Done__,
                    },
                    MiddleTypeSerializerState::Done__ => return Ok(None),
                    MiddleTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for MiddleTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = MiddleTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct EmptyTypeSerializer<'ser> {
        pub(super) value: &'ser super::EmptyType,
        pub(super) state: Box<EmptyTypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum EmptyTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> EmptyTypeSerializer<'ser> {
        fn next_event(&mut self, _: &mut SerializeHelper) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EmptyTypeSerializerState::Init__ => match *self.value {},
                    EmptyTypeSerializerState::Done__ => return Ok(None),
                    EmptyTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for EmptyTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = EmptyTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct HeadSerializer<'ser> {
        pub(super) value: &'ser super::Head,
        pub(super) state: Box<HeadSerializerState<'ser>>,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum HeadSerializerState<'ser> {
        Init__,
        Member(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> HeadSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    HeadSerializerState::Init__ => match self.value {
                        super::Head::Member(x) => {
                            *self.state = HeadSerializerState::Member(
                                WithSerializer::serializer_with_namespace(
                                    x,
                                    Some("tns:Member"),
                                    ElementNamespace::Namespace(super::NS_TNS),
                                    self.is_root,
                                )?,
                            )
                        }
                    },
                    HeadSerializerState::Member(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = HeadSerializerState::Done__,
                    },
                    HeadSerializerState::Done__ => return Ok(None),
                    HeadSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for HeadSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = HeadSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct AbstractMemberSerializer<'ser> {
        pub(super) value: &'ser super::AbstractMember,
        pub(super) state: Box<AbstractMemberSerializerState<'ser>>,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum AbstractMemberSerializerState<'ser> {
        Init__,
        Member(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> AbstractMemberSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    AbstractMemberSerializerState::Init__ => match self.value {
                        super::AbstractMember::Member(x) => {
                            *self.state = AbstractMemberSerializerState::Member(
                                WithSerializer::serializer_with_namespace(
                                    x,
                                    Some("tns:Member"),
                                    ElementNamespace::Namespace(super::NS_TNS),
                                    self.is_root,
                                )?,
                            )
                        }
                    },
                    AbstractMemberSerializerState::Member(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = AbstractMemberSerializerState::Done__,
                        }
                    }
                    AbstractMemberSerializerState::Done__ => return Ok(None),
                    AbstractMemberSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for AbstractMemberSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = AbstractMemberSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct EmptyHeadSerializer<'ser> {
        pub(super) value: &'ser super::EmptyHead,
        pub(super) state: Box<EmptyHeadSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum EmptyHeadSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> EmptyHeadSerializer<'ser> {
        fn next_event(&mut self, _: &mut SerializeHelper) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EmptyHeadSerializerState::Init__ => match *self.value {},
                    EmptyHeadSerializerState::Done__ => return Ok(None),
                    EmptyHeadSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for EmptyHeadSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = EmptyHeadSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct EmptyAbstractMemberSerializer<'ser> {
        pub(super) value: &'ser super::EmptyAbstractMember,
        pub(super) state: Box<EmptyAbstractMemberSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum EmptyAbstractMemberSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> EmptyAbstractMemberSerializer<'ser> {
        fn next_event(&mut self, _: &mut SerializeHelper) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EmptyAbstractMemberSerializerState::Init__ => match *self.value {},
                    EmptyAbstractMemberSerializerState::Done__ => return Ok(None),
                    EmptyAbstractMemberSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for EmptyAbstractMemberSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = EmptyAbstractMemberSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct MessageTypeSerializer<'ser> {
        pub(super) value: &'ser super::MessageType,
        pub(super) state: Box<MessageTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum MessageTypeSerializerState<'ser> {
        Init__,
        Payload(<super::PayloadType as WithSerializer>::Serializer<'ser>),
        Head(<super::Head as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> MessageTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MessageTypeSerializerState::Init__ => {
                        *self.state = MessageTypeSerializerState::Payload(
                            WithSerializer::serializer_with_namespace(
                                &self.value.payload,
                                Some("tns:Payload"),
                                ElementNamespace::Namespace(super::NS_TNS),
                                false,
                            )?,
                        );
                        let namespace = if self.is_root {
                            ElementNamespace::Namespace(super::NS_TNS)
                        } else {
                            self.namespace.clone()
                        };
                        self.name = helper.element_name(self.name, &namespace);
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns_for_element(&mut bytes, self.name, &namespace);
                        if self.is_root {
                            helper.write_default_namespace(&mut bytes, self.name, &namespace);
                        }
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_XSI),
                                &super::NS_XSI,
                            );
                            helper.write_xmlns_unless_default(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    MessageTypeSerializerState::Payload(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = MessageTypeSerializerState::Head(
                                WithSerializer::serializer_with_namespace(
                                    &self.value.head,
                                    Some("tns:Head"),
                                    ElementNamespace::Namespace(super::NS_TNS),
                                    false,
                                )?,
                            )
                        }
                    },
                    MessageTypeSerializerState::Head(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = MessageTypeSerializerState::End__,
                    },
                    MessageTypeSerializerState::End__ => {
                        *self.state = MessageTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    MessageTypeSerializerState::Done__ => return Ok(None),
                    MessageTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for MessageTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = MessageTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
