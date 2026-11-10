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
pub struct PayloadType {
    pub content: PayloadTypeContent,
}
#[derive(Debug)]
pub enum PayloadTypeContent {
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
impl WithSerializer for PayloadTypeContent {
    type Serializer<'x> = quick_xml_serialize::PayloadTypeContentSerializer<'x>;
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
        Ok(quick_xml_serialize::PayloadTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::PayloadTypeContentSerializerState::Init__),
            name,
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for PayloadType {
    type Deserializer = quick_xml_deserialize::PayloadTypeDeserializer;
}
impl WithDeserializer for PayloadTypeContent {
    type Deserializer = quick_xml_deserialize::PayloadTypeContentDeserializer;
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
pub struct MiddleType {
    pub content: MiddleTypeContent,
}
#[derive(Debug)]
pub enum MiddleTypeContent {
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
impl WithSerializer for MiddleTypeContent {
    type Serializer<'x> = quick_xml_serialize::MiddleTypeContentSerializer<'x>;
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
        Ok(quick_xml_serialize::MiddleTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::MiddleTypeContentSerializerState::Init__),
            name,
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for MiddleType {
    type Deserializer = quick_xml_deserialize::MiddleTypeDeserializer;
}
impl WithDeserializer for MiddleTypeContent {
    type Deserializer = quick_xml_deserialize::MiddleTypeContentDeserializer;
}
#[derive(Debug)]
pub struct EmptyType {
    pub content: EmptyTypeContent,
}
#[derive(Debug)]
pub enum EmptyTypeContent {}
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
        Ok(quick_xml_serialize::EmptyTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::EmptyTypeSerializerState::Init__),
            name,
            namespace,
            is_root,
        })
    }
}
impl WithSerializer for EmptyTypeContent {
    type Serializer<'x> = quick_xml_serialize::EmptyTypeContentSerializer<'x>;
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
        Ok(quick_xml_serialize::EmptyTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::EmptyTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for EmptyType {
    type Deserializer = quick_xml_deserialize::EmptyTypeDeserializer;
}
impl WithDeserializer for EmptyTypeContent {
    type Deserializer = quick_xml_deserialize::EmptyTypeContentDeserializer;
}
#[derive(Debug)]
pub struct Head {
    pub content: HeadContent,
}
#[derive(Debug)]
pub enum HeadContent {
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
        Ok(quick_xml_serialize::HeadSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::HeadSerializerState::Init__),
            name,
            namespace,
            is_root,
        })
    }
}
impl WithSerializer for HeadContent {
    type Serializer<'x> = quick_xml_serialize::HeadContentSerializer<'x>;
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
        Ok(quick_xml_serialize::HeadContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::HeadContentSerializerState::Init__),
            is_root,
        })
    }
}
impl WithDeserializer for Head {
    type Deserializer = quick_xml_deserialize::HeadDeserializer;
}
impl WithDeserializer for HeadContent {
    type Deserializer = quick_xml_deserialize::HeadContentDeserializer;
}
#[derive(Debug)]
pub struct AbstractMember {
    pub content: AbstractMemberContent,
}
#[derive(Debug)]
pub enum AbstractMemberContent {
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
        Ok(quick_xml_serialize::AbstractMemberSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::AbstractMemberSerializerState::Init__),
            name,
            namespace,
            is_root,
        })
    }
}
impl WithSerializer for AbstractMemberContent {
    type Serializer<'x> = quick_xml_serialize::AbstractMemberContentSerializer<'x>;
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
        Ok(quick_xml_serialize::AbstractMemberContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::AbstractMemberContentSerializerState::Init__),
            is_root,
        })
    }
}
impl WithDeserializer for AbstractMember {
    type Deserializer = quick_xml_deserialize::AbstractMemberDeserializer;
}
impl WithDeserializer for AbstractMemberContent {
    type Deserializer = quick_xml_deserialize::AbstractMemberContentDeserializer;
}
#[derive(Debug)]
pub struct EmptyHead {
    pub content: EmptyHeadContent,
}
#[derive(Debug)]
pub enum EmptyHeadContent {}
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
        Ok(quick_xml_serialize::EmptyHeadSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::EmptyHeadSerializerState::Init__),
            name,
            namespace,
            is_root,
        })
    }
}
impl WithSerializer for EmptyHeadContent {
    type Serializer<'x> = quick_xml_serialize::EmptyHeadContentSerializer<'x>;
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
        Ok(quick_xml_serialize::EmptyHeadContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::EmptyHeadContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for EmptyHead {
    type Deserializer = quick_xml_deserialize::EmptyHeadDeserializer;
}
impl WithDeserializer for EmptyHeadContent {
    type Deserializer = quick_xml_deserialize::EmptyHeadContentDeserializer;
}
#[derive(Debug)]
pub struct EmptyAbstractMember {
    pub content: EmptyAbstractMemberContent,
}
#[derive(Debug)]
pub enum EmptyAbstractMemberContent {}
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
        Ok(quick_xml_serialize::EmptyAbstractMemberSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::EmptyAbstractMemberSerializerState::Init__),
            name,
            namespace,
            is_root,
        })
    }
}
impl WithSerializer for EmptyAbstractMemberContent {
    type Serializer<'x> = quick_xml_serialize::EmptyAbstractMemberContentSerializer<'x>;
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
        Ok(quick_xml_serialize::EmptyAbstractMemberContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::EmptyAbstractMemberContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for EmptyAbstractMember {
    type Deserializer = quick_xml_deserialize::EmptyAbstractMemberDeserializer;
}
impl WithDeserializer for EmptyAbstractMemberContent {
    type Deserializer = quick_xml_deserialize::EmptyAbstractMemberContentDeserializer;
}
pub type Message = MessageType;
#[derive(Debug)]
pub struct MessageType {
    pub content: MessageTypeContent,
}
#[derive(Debug)]
pub struct MessageTypeContent {
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
impl WithSerializer for MessageTypeContent {
    type Serializer<'x> = quick_xml_serialize::MessageTypeContentSerializer<'x>;
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
        Ok(quick_xml_serialize::MessageTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::MessageTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for MessageType {
    type Deserializer = quick_xml_deserialize::MessageTypeDeserializer;
}
impl WithDeserializer for MessageTypeContent {
    type Deserializer = quick_xml_deserialize::MessageTypeContentDeserializer;
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
        content: Option<super::PayloadTypeContent>,
        state__: Box<PayloadTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PayloadTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::PayloadTypeContent as WithDeserializer>::Deserializer),
        Done__,
        Unknown__,
    }
    impl PayloadTypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: PayloadTypeDeserializerState,
        ) -> Result<(), Error> {
            if let PayloadTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::PayloadTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::PayloadTypeContent>,
            fallback: &mut Option<PayloadTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PayloadTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Init__;
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
    impl<'de> Deserializer<'de, super::PayloadType> for PayloadTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PayloadType> {
            let deserializer = Self {
                content: None,
                state__: Box::new(PayloadTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
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
            use PayloadTypeDeserializerState as S;
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
                            <super::PayloadTypeContent as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::PayloadType, Error> {
            let state = replace(&mut *self.state__, PayloadTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::PayloadType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PayloadTypeContentDeserializer {
        state__: Box<PayloadTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum PayloadTypeContentDeserializerState {
        Init__,
        Leaf(
            Option<super::LeafType>,
            Option<<super::LeafType as WithDeserializer>::Deserializer>,
            Option<<super::LeafType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::PayloadTypeContent),
        Unknown__,
    }
    impl PayloadTypeContentDeserializer {
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
            *self.state__ = PayloadTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn next_with_name<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
            ns: Option<&[u8]>,
            name: &[u8],
        ) -> DeserializerResult<'de, super::PayloadTypeContent> {
            use PayloadTypeContentDeserializerState as S;
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
            state: PayloadTypeContentDeserializerState,
        ) -> Result<super::PayloadTypeContent, Error> {
            use PayloadTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Leaf(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_leaf(&mut values, value)?;
                    }
                    Ok(super::PayloadTypeContent::Leaf(
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
            use PayloadTypeContentDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::PayloadTypeContent> for PayloadTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PayloadTypeContent> {
            Self::init_with_name(helper, event, Some(&super::NS_TNS), b"Payload")
        }
        fn init_with_name(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
            ns: Option<&[u8]>,
            name: &[u8],
        ) -> DeserializerResult<'de, super::PayloadTypeContent> {
            let deserializer = Self {
                state__: Box::new(PayloadTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next_with_name(helper, event, ns, name)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, PayloadTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::PayloadTypeContent> {
            self.next_with_name(helper, event, Some(&super::NS_TNS), b"Payload")
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::PayloadTypeContent, Error> {
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
        content: Option<super::MiddleTypeContent>,
        state__: Box<MiddleTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum MiddleTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::MiddleTypeContent as WithDeserializer>::Deserializer),
        Done__,
        Unknown__,
    }
    impl MiddleTypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: MiddleTypeDeserializerState,
        ) -> Result<(), Error> {
            if let MiddleTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::MiddleTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::MiddleTypeContent>,
            fallback: &mut Option<MiddleTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MiddleTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Init__;
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
    impl<'de> Deserializer<'de, super::MiddleType> for MiddleTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MiddleType> {
            let deserializer = Self {
                content: None,
                state__: Box::new(MiddleTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
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
            use MiddleTypeDeserializerState as S;
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
                            <super::MiddleTypeContent as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::MiddleType, Error> {
            let state = replace(&mut *self.state__, MiddleTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::MiddleType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct MiddleTypeContentDeserializer {
        state__: Box<MiddleTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum MiddleTypeContentDeserializerState {
        Init__,
        Leaf(
            Option<super::LeafType>,
            Option<<super::LeafType as WithDeserializer>::Deserializer>,
            Option<<super::LeafType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::MiddleTypeContent),
        Unknown__,
    }
    impl MiddleTypeContentDeserializer {
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
            *self.state__ = MiddleTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn next_with_name<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
            ns: Option<&[u8]>,
            name: &[u8],
        ) -> DeserializerResult<'de, super::MiddleTypeContent> {
            use MiddleTypeContentDeserializerState as S;
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
            state: MiddleTypeContentDeserializerState,
        ) -> Result<super::MiddleTypeContent, Error> {
            use MiddleTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Leaf(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_leaf(&mut values, value)?;
                    }
                    Ok(super::MiddleTypeContent::Leaf(
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
            use MiddleTypeContentDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::MiddleTypeContent> for MiddleTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MiddleTypeContent> {
            Self::init_with_name(helper, event, Some(&super::NS_TNS), b"Middle")
        }
        fn init_with_name(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
            ns: Option<&[u8]>,
            name: &[u8],
        ) -> DeserializerResult<'de, super::MiddleTypeContent> {
            let deserializer = Self {
                state__: Box::new(MiddleTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next_with_name(helper, event, ns, name)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, MiddleTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::MiddleTypeContent> {
            self.next_with_name(helper, event, Some(&super::NS_TNS), b"Middle")
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::MiddleTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct EmptyTypeDeserializer {
        content: Option<super::EmptyTypeContent>,
        state__: Box<EmptyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum EmptyTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::EmptyTypeContent as WithDeserializer>::Deserializer),
        Done__,
        Unknown__,
    }
    impl EmptyTypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: EmptyTypeDeserializerState,
        ) -> Result<(), Error> {
            if let EmptyTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::EmptyTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::EmptyTypeContent>,
            fallback: &mut Option<EmptyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use EmptyTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Init__;
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
    impl<'de> Deserializer<'de, super::EmptyType> for EmptyTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EmptyType> {
            let deserializer = Self {
                content: None,
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
                            <super::EmptyTypeContent as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::EmptyType, Error> {
            let state = replace(&mut *self.state__, EmptyTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::EmptyType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct EmptyTypeContentDeserializer {
        state__: Box<EmptyTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum EmptyTypeContentDeserializerState {
        Init__,
        Done__(super::EmptyTypeContent),
        Unknown__,
    }
    impl EmptyTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            _: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(_) | Event::Empty(_) = &event {}
            *self.state__ = EmptyTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            _: &mut DeserializeHelper,
            state: EmptyTypeContentDeserializerState,
        ) -> Result<super::EmptyTypeContent, Error> {
            use EmptyTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
    }
    impl<'de> Deserializer<'de, super::EmptyTypeContent> for EmptyTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EmptyTypeContent> {
            let deserializer = Self {
                state__: Box::new(EmptyTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, EmptyTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::EmptyTypeContent> {
            use EmptyTypeContentDeserializerState as S;
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::EmptyTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct HeadDeserializer {
        content: Option<super::HeadContent>,
        state__: Box<HeadDeserializerState>,
    }
    #[derive(Debug)]
    enum HeadDeserializerState {
        Init__,
        Next__,
        Content__(<super::HeadContent as WithDeserializer>::Deserializer),
        Done__,
        Unknown__,
    }
    impl HeadDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: HeadDeserializerState,
        ) -> Result<(), Error> {
            if let HeadDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::HeadContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::HeadContent>,
            fallback: &mut Option<HeadDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HeadDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Init__;
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
    impl<'de> Deserializer<'de, super::Head> for HeadDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Head> {
            let deserializer = Self {
                content: None,
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
                        let output = <super::HeadContent as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::Head, Error> {
            let state = replace(&mut *self.state__, HeadDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::Head {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct HeadContentDeserializer {
        state__: Box<HeadContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum HeadContentDeserializerState {
        Init__,
        Member(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::HeadContent),
        Unknown__,
    }
    impl HeadContentDeserializer {
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
            *self.state__ = HeadContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: HeadContentDeserializerState,
        ) -> Result<super::HeadContent, Error> {
            use HeadContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Member(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_member(&mut values, value)?;
                    }
                    Ok(super::HeadContent::Member(
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
            use HeadContentDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::HeadContent> for HeadContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HeadContent> {
            let deserializer = Self {
                state__: Box::new(HeadContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, HeadContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::HeadContent> {
            use HeadContentDeserializerState as S;
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::HeadContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct AbstractMemberDeserializer {
        content: Option<super::AbstractMemberContent>,
        state__: Box<AbstractMemberDeserializerState>,
    }
    #[derive(Debug)]
    enum AbstractMemberDeserializerState {
        Init__,
        Next__,
        Content__(<super::AbstractMemberContent as WithDeserializer>::Deserializer),
        Done__,
        Unknown__,
    }
    impl AbstractMemberDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: AbstractMemberDeserializerState,
        ) -> Result<(), Error> {
            if let AbstractMemberDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::AbstractMemberContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AbstractMemberContent>,
            fallback: &mut Option<AbstractMemberDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AbstractMemberDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Init__;
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
    impl<'de> Deserializer<'de, super::AbstractMember> for AbstractMemberDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AbstractMember> {
            let deserializer = Self {
                content: None,
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
                        let output = <super::AbstractMemberContent as WithDeserializer>::init(
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
        ) -> Result<super::AbstractMember, Error> {
            let state = replace(
                &mut *self.state__,
                AbstractMemberDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::AbstractMember {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct AbstractMemberContentDeserializer {
        state__: Box<AbstractMemberContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum AbstractMemberContentDeserializerState {
        Init__,
        Member(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::AbstractMemberContent),
        Unknown__,
    }
    impl AbstractMemberContentDeserializer {
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
            *self.state__ = AbstractMemberContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: AbstractMemberContentDeserializerState,
        ) -> Result<super::AbstractMemberContent, Error> {
            use AbstractMemberContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Member(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_member(&mut values, value)?;
                    }
                    Ok(super::AbstractMemberContent::Member(
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
            use AbstractMemberContentDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::AbstractMemberContent> for AbstractMemberContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AbstractMemberContent> {
            let deserializer = Self {
                state__: Box::new(AbstractMemberContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, AbstractMemberContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::AbstractMemberContent> {
            use AbstractMemberContentDeserializerState as S;
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
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::AbstractMemberContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct EmptyHeadDeserializer {
        content: Option<super::EmptyHeadContent>,
        state__: Box<EmptyHeadDeserializerState>,
    }
    #[derive(Debug)]
    enum EmptyHeadDeserializerState {
        Init__,
        Next__,
        Content__(<super::EmptyHeadContent as WithDeserializer>::Deserializer),
        Done__,
        Unknown__,
    }
    impl EmptyHeadDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: EmptyHeadDeserializerState,
        ) -> Result<(), Error> {
            if let EmptyHeadDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::EmptyHeadContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::EmptyHeadContent>,
            fallback: &mut Option<EmptyHeadDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use EmptyHeadDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Init__;
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
    impl<'de> Deserializer<'de, super::EmptyHead> for EmptyHeadDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EmptyHead> {
            let deserializer = Self {
                content: None,
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
                            <super::EmptyHeadContent as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::EmptyHead, Error> {
            let state = replace(&mut *self.state__, EmptyHeadDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::EmptyHead {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct EmptyHeadContentDeserializer {
        state__: Box<EmptyHeadContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum EmptyHeadContentDeserializerState {
        Init__,
        Done__(super::EmptyHeadContent),
        Unknown__,
    }
    impl EmptyHeadContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            _: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(_) | Event::Empty(_) = &event {}
            *self.state__ = EmptyHeadContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            _: &mut DeserializeHelper,
            state: EmptyHeadContentDeserializerState,
        ) -> Result<super::EmptyHeadContent, Error> {
            use EmptyHeadContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
    }
    impl<'de> Deserializer<'de, super::EmptyHeadContent> for EmptyHeadContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EmptyHeadContent> {
            let deserializer = Self {
                state__: Box::new(EmptyHeadContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, EmptyHeadContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::EmptyHeadContent> {
            use EmptyHeadContentDeserializerState as S;
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::EmptyHeadContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct EmptyAbstractMemberDeserializer {
        content: Option<super::EmptyAbstractMemberContent>,
        state__: Box<EmptyAbstractMemberDeserializerState>,
    }
    #[derive(Debug)]
    enum EmptyAbstractMemberDeserializerState {
        Init__,
        Next__,
        Content__(<super::EmptyAbstractMemberContent as WithDeserializer>::Deserializer),
        Done__,
        Unknown__,
    }
    impl EmptyAbstractMemberDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: EmptyAbstractMemberDeserializerState,
        ) -> Result<(), Error> {
            if let EmptyAbstractMemberDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::EmptyAbstractMemberContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::EmptyAbstractMemberContent>,
            fallback: &mut Option<EmptyAbstractMemberDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use EmptyAbstractMemberDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Init__;
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
    impl<'de> Deserializer<'de, super::EmptyAbstractMember> for EmptyAbstractMemberDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EmptyAbstractMember> {
            let deserializer = Self {
                content: None,
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
                        let output = <super::EmptyAbstractMemberContent as WithDeserializer>::init(
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
        ) -> Result<super::EmptyAbstractMember, Error> {
            let state = replace(
                &mut *self.state__,
                EmptyAbstractMemberDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::EmptyAbstractMember {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct EmptyAbstractMemberContentDeserializer {
        state__: Box<EmptyAbstractMemberContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum EmptyAbstractMemberContentDeserializerState {
        Init__,
        Done__(super::EmptyAbstractMemberContent),
        Unknown__,
    }
    impl EmptyAbstractMemberContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            _: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(_) | Event::Empty(_) = &event {}
            *self.state__ = EmptyAbstractMemberContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            _: &mut DeserializeHelper,
            state: EmptyAbstractMemberContentDeserializerState,
        ) -> Result<super::EmptyAbstractMemberContent, Error> {
            use EmptyAbstractMemberContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
    }
    impl<'de> Deserializer<'de, super::EmptyAbstractMemberContent>
        for EmptyAbstractMemberContentDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EmptyAbstractMemberContent> {
            let deserializer = Self {
                state__: Box::new(EmptyAbstractMemberContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        EmptyAbstractMemberContentDeserializerState::Init__
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
        ) -> DeserializerResult<'de, super::EmptyAbstractMemberContent> {
            use EmptyAbstractMemberContentDeserializerState as S;
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
        ) -> Result<super::EmptyAbstractMemberContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct MessageTypeDeserializer {
        content: Option<super::MessageTypeContent>,
        state__: Box<MessageTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum MessageTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::MessageTypeContent as WithDeserializer>::Deserializer),
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
                content: None,
                state__: Box::new(MessageTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: MessageTypeDeserializerState,
        ) -> Result<(), Error> {
            if let MessageTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::MessageTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::MessageTypeContent>,
            fallback: &mut Option<MessageTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MessageTypeDeserializerState as S;
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
                            <super::MessageTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            let event = event.into_break();
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::MessageType, Error> {
            let state = replace(&mut *self.state__, MessageTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::MessageType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct MessageTypeContentDeserializer {
        payload: Option<super::PayloadType>,
        head: Option<super::Head>,
        state__: Box<MessageTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    enum MessageTypeContentDeserializerState {
        Init__,
        Payload(Option<<super::PayloadType as WithDeserializer>::Deserializer>),
        Head(Option<<super::Head as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl MessageTypeContentDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: MessageTypeContentDeserializerState,
        ) -> Result<(), Error> {
            use MessageTypeContentDeserializerState as S;
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
            fallback: &mut Option<MessageTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MessageTypeContentDeserializerState as S;
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
            fallback: &mut Option<MessageTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MessageTypeContentDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::MessageTypeContent> for MessageTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MessageTypeContent> {
            let deserializer = Self {
                payload: None,
                head: None,
                state__: Box::new(MessageTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, MessageTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::MessageTypeContent> {
            use MessageTypeContentDeserializerState as S;
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
        ) -> Result<super::MessageTypeContent, Error> {
            let state = replace(
                &mut *self.state__,
                MessageTypeContentDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::MessageTypeContent {
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
        Content__(<super::PayloadTypeContent as WithSerializer>::Serializer<'ser>),
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
                    PayloadTypeSerializerState::Init__ => {
                        *self.state = PayloadTypeSerializerState::Content__(
                            WithSerializer::serializer_with_namespace(
                                &self.value.content,
                                self.name,
                                self.namespace.clone(),
                                self.is_root,
                            )?,
                        );
                    }
                    PayloadTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
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
    pub struct PayloadTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::PayloadTypeContent,
        pub(super) state: Box<PayloadTypeContentSerializerState<'ser>>,
        pub(super) name: Option<&'ser str>,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum PayloadTypeContentSerializerState<'ser> {
        Init__,
        Leaf(XsiTypeSerializer<'ser, <super::LeafType as WithSerializer>::Serializer<'ser>>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PayloadTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PayloadTypeContentSerializerState::Init__ => match self.value {
                        super::PayloadTypeContent::Leaf(x) => {
                            *self.state = PayloadTypeContentSerializerState::Leaf(
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
                    PayloadTypeContentSerializerState::Leaf(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PayloadTypeContentSerializerState::Done__,
                        }
                    }
                    PayloadTypeContentSerializerState::Done__ => return Ok(None),
                    PayloadTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for PayloadTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PayloadTypeContentSerializerState::Done__;
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
        Content__(<super::MiddleTypeContent as WithSerializer>::Serializer<'ser>),
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
                    MiddleTypeSerializerState::Init__ => {
                        *self.state = MiddleTypeSerializerState::Content__(
                            WithSerializer::serializer_with_namespace(
                                &self.value.content,
                                self.name,
                                self.namespace.clone(),
                                self.is_root,
                            )?,
                        );
                    }
                    MiddleTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
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
    pub struct MiddleTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::MiddleTypeContent,
        pub(super) state: Box<MiddleTypeContentSerializerState<'ser>>,
        pub(super) name: Option<&'ser str>,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum MiddleTypeContentSerializerState<'ser> {
        Init__,
        Leaf(XsiTypeSerializer<'ser, <super::LeafType as WithSerializer>::Serializer<'ser>>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> MiddleTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MiddleTypeContentSerializerState::Init__ => match self.value {
                        super::MiddleTypeContent::Leaf(x) => {
                            *self.state = MiddleTypeContentSerializerState::Leaf(
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
                    MiddleTypeContentSerializerState::Leaf(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = MiddleTypeContentSerializerState::Done__,
                        }
                    }
                    MiddleTypeContentSerializerState::Done__ => return Ok(None),
                    MiddleTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for MiddleTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = MiddleTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct EmptyTypeSerializer<'ser> {
        pub(super) value: &'ser super::EmptyType,
        pub(super) state: Box<EmptyTypeSerializerState<'ser>>,
        pub(super) name: Option<&'ser str>,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum EmptyTypeSerializerState<'ser> {
        Init__,
        Content__(<super::EmptyTypeContent as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> EmptyTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EmptyTypeSerializerState::Init__ => {
                        *self.state = EmptyTypeSerializerState::Content__(
                            WithSerializer::serializer_with_namespace(
                                &self.value.content,
                                self.name,
                                self.namespace.clone(),
                                self.is_root,
                            )?,
                        );
                    }
                    EmptyTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = EmptyTypeSerializerState::Done__,
                    },
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
    pub struct EmptyTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::EmptyTypeContent,
        pub(super) state: Box<EmptyTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum EmptyTypeContentSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> EmptyTypeContentSerializer<'ser> {
        fn next_event(&mut self, _: &mut SerializeHelper) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EmptyTypeContentSerializerState::Init__ => match *self.value {},
                    EmptyTypeContentSerializerState::Done__ => return Ok(None),
                    EmptyTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for EmptyTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = EmptyTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct HeadSerializer<'ser> {
        pub(super) value: &'ser super::Head,
        pub(super) state: Box<HeadSerializerState<'ser>>,
        pub(super) name: Option<&'ser str>,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum HeadSerializerState<'ser> {
        Init__,
        Content__(<super::HeadContent as WithSerializer>::Serializer<'ser>),
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
                    HeadSerializerState::Init__ => {
                        *self.state = HeadSerializerState::Content__(
                            WithSerializer::serializer_with_namespace(
                                &self.value.content,
                                self.name,
                                self.namespace.clone(),
                                self.is_root,
                            )?,
                        );
                    }
                    HeadSerializerState::Content__(x) => match x.next(helper).transpose()? {
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
    pub struct HeadContentSerializer<'ser> {
        pub(super) value: &'ser super::HeadContent,
        pub(super) state: Box<HeadContentSerializerState<'ser>>,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum HeadContentSerializerState<'ser> {
        Init__,
        Member(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> HeadContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    HeadContentSerializerState::Init__ => match self.value {
                        super::HeadContent::Member(x) => {
                            *self.state = HeadContentSerializerState::Member(
                                WithSerializer::serializer_with_namespace(
                                    x,
                                    Some("tns:Member"),
                                    ElementNamespace::Namespace(super::NS_TNS),
                                    self.is_root,
                                )?,
                            )
                        }
                    },
                    HeadContentSerializerState::Member(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = HeadContentSerializerState::Done__,
                    },
                    HeadContentSerializerState::Done__ => return Ok(None),
                    HeadContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for HeadContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = HeadContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct AbstractMemberSerializer<'ser> {
        pub(super) value: &'ser super::AbstractMember,
        pub(super) state: Box<AbstractMemberSerializerState<'ser>>,
        pub(super) name: Option<&'ser str>,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum AbstractMemberSerializerState<'ser> {
        Init__,
        Content__(<super::AbstractMemberContent as WithSerializer>::Serializer<'ser>),
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
                    AbstractMemberSerializerState::Init__ => {
                        *self.state = AbstractMemberSerializerState::Content__(
                            WithSerializer::serializer_with_namespace(
                                &self.value.content,
                                self.name,
                                self.namespace.clone(),
                                self.is_root,
                            )?,
                        );
                    }
                    AbstractMemberSerializerState::Content__(x) => {
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
    pub struct AbstractMemberContentSerializer<'ser> {
        pub(super) value: &'ser super::AbstractMemberContent,
        pub(super) state: Box<AbstractMemberContentSerializerState<'ser>>,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum AbstractMemberContentSerializerState<'ser> {
        Init__,
        Member(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> AbstractMemberContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    AbstractMemberContentSerializerState::Init__ => match self.value {
                        super::AbstractMemberContent::Member(x) => {
                            *self.state = AbstractMemberContentSerializerState::Member(
                                WithSerializer::serializer_with_namespace(
                                    x,
                                    Some("tns:Member"),
                                    ElementNamespace::Namespace(super::NS_TNS),
                                    self.is_root,
                                )?,
                            )
                        }
                    },
                    AbstractMemberContentSerializerState::Member(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = AbstractMemberContentSerializerState::Done__,
                        }
                    }
                    AbstractMemberContentSerializerState::Done__ => return Ok(None),
                    AbstractMemberContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for AbstractMemberContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = AbstractMemberContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct EmptyHeadSerializer<'ser> {
        pub(super) value: &'ser super::EmptyHead,
        pub(super) state: Box<EmptyHeadSerializerState<'ser>>,
        pub(super) name: Option<&'ser str>,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum EmptyHeadSerializerState<'ser> {
        Init__,
        Content__(<super::EmptyHeadContent as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> EmptyHeadSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EmptyHeadSerializerState::Init__ => {
                        *self.state = EmptyHeadSerializerState::Content__(
                            WithSerializer::serializer_with_namespace(
                                &self.value.content,
                                self.name,
                                self.namespace.clone(),
                                self.is_root,
                            )?,
                        );
                    }
                    EmptyHeadSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = EmptyHeadSerializerState::Done__,
                    },
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
    pub struct EmptyHeadContentSerializer<'ser> {
        pub(super) value: &'ser super::EmptyHeadContent,
        pub(super) state: Box<EmptyHeadContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum EmptyHeadContentSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> EmptyHeadContentSerializer<'ser> {
        fn next_event(&mut self, _: &mut SerializeHelper) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EmptyHeadContentSerializerState::Init__ => match *self.value {},
                    EmptyHeadContentSerializerState::Done__ => return Ok(None),
                    EmptyHeadContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for EmptyHeadContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = EmptyHeadContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct EmptyAbstractMemberSerializer<'ser> {
        pub(super) value: &'ser super::EmptyAbstractMember,
        pub(super) state: Box<EmptyAbstractMemberSerializerState<'ser>>,
        pub(super) name: Option<&'ser str>,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum EmptyAbstractMemberSerializerState<'ser> {
        Init__,
        Content__(<super::EmptyAbstractMemberContent as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> EmptyAbstractMemberSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EmptyAbstractMemberSerializerState::Init__ => {
                        *self.state = EmptyAbstractMemberSerializerState::Content__(
                            WithSerializer::serializer_with_namespace(
                                &self.value.content,
                                self.name,
                                self.namespace.clone(),
                                self.is_root,
                            )?,
                        );
                    }
                    EmptyAbstractMemberSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = EmptyAbstractMemberSerializerState::Done__,
                        }
                    }
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
    pub struct EmptyAbstractMemberContentSerializer<'ser> {
        pub(super) value: &'ser super::EmptyAbstractMemberContent,
        pub(super) state: Box<EmptyAbstractMemberContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum EmptyAbstractMemberContentSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> EmptyAbstractMemberContentSerializer<'ser> {
        fn next_event(&mut self, _: &mut SerializeHelper) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EmptyAbstractMemberContentSerializerState::Init__ => match *self.value {},
                    EmptyAbstractMemberContentSerializerState::Done__ => return Ok(None),
                    EmptyAbstractMemberContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for EmptyAbstractMemberContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = EmptyAbstractMemberContentSerializerState::Done__;
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
        Content__(<super::MessageTypeContent as WithSerializer>::Serializer<'ser>),
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
                        *self.state = MessageTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
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
                    MessageTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
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
    #[derive(Debug)]
    pub struct MessageTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::MessageTypeContent,
        pub(super) state: Box<MessageTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum MessageTypeContentSerializerState<'ser> {
        Init__,
        Payload(<super::PayloadType as WithSerializer>::Serializer<'ser>),
        Head(<super::Head as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> MessageTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MessageTypeContentSerializerState::Init__ => {
                        *self.state = MessageTypeContentSerializerState::Payload(
                            WithSerializer::serializer_with_namespace(
                                &self.value.payload,
                                Some("tns:Payload"),
                                ElementNamespace::Namespace(super::NS_TNS),
                                false,
                            )?,
                        );
                    }
                    MessageTypeContentSerializerState::Payload(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = MessageTypeContentSerializerState::Head(
                                    WithSerializer::serializer_with_namespace(
                                        &self.value.head,
                                        Some("tns:Head"),
                                        ElementNamespace::Namespace(super::NS_TNS),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    MessageTypeContentSerializerState::Head(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = MessageTypeContentSerializerState::Done__,
                        }
                    }
                    MessageTypeContentSerializerState::Done__ => return Ok(None),
                    MessageTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for MessageTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = MessageTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
