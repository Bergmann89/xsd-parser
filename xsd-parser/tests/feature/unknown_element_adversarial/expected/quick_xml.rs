use core::fmt::Debug;
use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{
        BoxedSerializer, ElementNamespace, Error, WithBoxedSerializer, WithDeserializer,
        WithSerializer,
    },
    AsAny,
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub type Root = RootType;
#[derive(Debug)]
pub struct RootType {
    pub a: String,
    pub pos: Option<String>,
}
impl WithSerializer for RootType {
    type Serializer<'x> = quick_xml_serialize::RootTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::RootTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::RootTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Root"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for RootType {
    type Deserializer = quick_xml_deserialize::RootTypeDeserializer;
}
pub type Max = MaxType;
#[derive(Debug)]
pub struct MaxType {
    pub a: Vec<String>,
}
impl WithSerializer for MaxType {
    type Serializer<'x> = quick_xml_serialize::MaxTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::MaxTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::MaxTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Max"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for MaxType {
    type Deserializer = quick_xml_deserialize::MaxTypeDeserializer;
}
pub type Outer = OuterType;
#[derive(Debug)]
pub struct OuterType {
    pub child: ChildType,
    pub b: Option<String>,
}
impl WithSerializer for OuterType {
    type Serializer<'x> = quick_xml_serialize::OuterTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::OuterTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::OuterTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Outer"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for OuterType {
    type Deserializer = quick_xml_deserialize::OuterTypeDeserializer;
}
#[derive(Debug)]
pub struct ChildType {
    pub inner: String,
    pub extra: Option<String>,
}
impl WithSerializer for ChildType {
    type Serializer<'x> = quick_xml_serialize::ChildTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::ChildTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ChildTypeSerializerState::Init__),
            name: name.unwrap_or("tns:ChildType"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for ChildType {
    type Deserializer = quick_xml_deserialize::ChildTypeDeserializer;
}
pub type Node = NodeType;
#[derive(Debug)]
pub struct NodeType {
    pub name: String,
    pub kid: Option<Box<NodeType>>,
}
impl WithSerializer for NodeType {
    type Serializer<'x> = quick_xml_serialize::NodeTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::NodeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::NodeTypeSerializerState::Init__),
            name: name.unwrap_or("tns:NodeType"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for NodeType {
    type Deserializer = quick_xml_deserialize::NodeTypeDeserializer;
}
pub type GRoot = GRootType;
#[derive(Debug)]
pub struct GRootType {
    pub rec_group: GRootRecGroupType,
}
impl WithSerializer for GRootType {
    type Serializer<'x> = quick_xml_serialize::GRootTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::GRootTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::GRootTypeSerializerState::Init__),
            name: name.unwrap_or("tns:GRoot"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for GRootType {
    type Deserializer = quick_xml_deserialize::GRootTypeDeserializer;
}
#[derive(Debug)]
pub struct GRootRecGroupType {
    pub v: String,
    pub via: Option<ViaType>,
}
impl WithSerializer for GRootRecGroupType {
    type Serializer<'x> = quick_xml_serialize::GRootRecGroupTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::GRootRecGroupTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::GRootRecGroupTypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for GRootRecGroupType {
    type Deserializer = quick_xml_deserialize::GRootRecGroupTypeDeserializer;
}
#[derive(Debug)]
pub struct ViaType {
    pub rec_group: ViaRecGroupType,
}
impl WithSerializer for ViaType {
    type Serializer<'x> = quick_xml_serialize::ViaTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::ViaTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ViaTypeSerializerState::Init__),
            name: name.unwrap_or("tns:ViaType"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for ViaType {
    type Deserializer = quick_xml_deserialize::ViaTypeDeserializer;
}
#[derive(Debug)]
pub struct ViaRecGroupType {
    pub v: String,
    pub via: Option<Box<ViaType>>,
}
impl WithSerializer for ViaRecGroupType {
    type Serializer<'x> = quick_xml_serialize::ViaRecGroupTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::ViaRecGroupTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ViaRecGroupTypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for ViaRecGroupType {
    type Deserializer = quick_xml_deserialize::ViaRecGroupTypeDeserializer;
}
pub type Scene = SceneType;
#[derive(Debug)]
pub struct SceneType {
    pub shape: Option<Shape>,
}
impl WithSerializer for SceneType {
    type Serializer<'x> = quick_xml_serialize::SceneTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::SceneTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SceneTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Scene"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for SceneType {
    type Deserializer = quick_xml_deserialize::SceneTypeDeserializer;
}
#[derive(Debug)]
pub struct Shape(pub Box<dyn ShapeTrait>);
pub trait ShapeTrait: Debug + AsAny + WithBoxedSerializer {}
impl Shape {
    pub fn new<T: ShapeTrait + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
impl WithSerializer for Shape {
    type Serializer<'x> = BoxedSerializer<'x>;
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
        if (*self.0).as_any().is::<ShapeType>() {
            return self.0.serializer_with_namespace(
                Some("tns:Circle"),
                ElementNamespace::Namespace(NS_TNS),
                is_root,
            );
        }
        if (*self.0).as_any().is::<ShapeType>() {
            return self.0.serializer_with_namespace(
                Some("tns:Square"),
                ElementNamespace::Namespace(NS_TNS),
                is_root,
            );
        }
        self.0.serializer_with_namespace(name, namespace, is_root)
    }
}
impl WithDeserializer for Shape {
    type Deserializer = quick_xml_deserialize::ShapeDeserializer;
}
#[derive(Debug)]
pub struct ShapeType {
    pub id: Option<i32>,
}
impl ShapeTrait for ShapeType {}
impl WithSerializer for ShapeType {
    type Serializer<'x> = quick_xml_serialize::ShapeTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::ShapeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ShapeTypeSerializerState::Init__),
            name: name.unwrap_or("tns:ShapeType"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for ShapeType {
    type Deserializer = quick_xml_deserialize::ShapeTypeDeserializer;
}
pub type Pick = PickType;
#[derive(Debug)]
pub enum PickType {
    X(String),
    Y(String),
}
impl WithSerializer for PickType {
    type Serializer<'x> = quick_xml_serialize::PickTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::PickTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::PickTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Pick"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for PickType {
    type Deserializer = quick_xml_deserialize::PickTypeDeserializer;
}
pub type Unordered = UnorderedType;
#[derive(Debug)]
pub struct UnorderedType {
    pub x: String,
    pub y: Option<String>,
}
impl WithSerializer for UnorderedType {
    type Serializer<'x> = quick_xml_serialize::UnorderedTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::UnorderedTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::UnorderedTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Unordered"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for UnorderedType {
    type Deserializer = quick_xml_deserialize::UnorderedTypeDeserializer;
}
pub type Nothing = NothingType;
#[derive(Debug)]
pub struct NothingType;
impl WithSerializer for NothingType {
    type Serializer<'x> = quick_xml_serialize::NothingTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::NothingTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::NothingTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Nothing"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for NothingType {
    type Deserializer = quick_xml_deserialize::NothingTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        QName, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct RootTypeDeserializer {
        a: Option<String>,
        pos: Option<String>,
        state__: Box<RootTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RootTypeDeserializerState {
        Init__,
        A(Option<<String as WithDeserializer>::Deserializer>),
        Pos(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl RootTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                a: None,
                pos: None,
                state__: Box::new(RootTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: RootTypeDeserializerState,
        ) -> Result<(), Error> {
            use RootTypeDeserializerState as S;
            match state {
                S::A(Some(deserializer)) => self.store_a(deserializer.finish(helper)?)?,
                S::Pos(Some(deserializer)) => self.store_pos(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_a(&mut self, value: String) -> Result<(), Error> {
            if self.a.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"A")))?;
            }
            self.a = Some(value);
            Ok(())
        }
        fn store_pos(&mut self, value: String) -> Result<(), Error> {
            if self.pos.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Pos")))?;
            }
            self.pos = Some(value);
            Ok(())
        }
        fn handle_a<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<RootTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RootTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::A(None));
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
                    self.store_a(data)?;
                    *self.state__ = S::Pos(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::A(Some(deserializer)));
                    *self.state__ = S::Pos(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_pos<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<RootTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RootTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Pos(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_pos(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Pos(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RootType> for RootTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RootType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RootType> {
            use RootTypeDeserializerState as S;
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
                    (S::Pos(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_pos(helper, output, &mut fallback)? {
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
                        *self.state__ = S::A(None);
                        event
                    }
                    (S::A(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"A",
                            false,
                        )?;
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
                    (S::Pos(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Pos",
                            false,
                        )?;
                        match self.handle_pos(helper, output, &mut fallback)? {
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
            let (event, allow_any) =
                helper.decide_unknown_element::<super::RootType>(event, allow_any)?;
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::RootType, Error> {
            let state = replace(&mut *self.state__, RootTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::RootType {
                a: helper.finish_element("A", self.a)?,
                pos: self.pos,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                Some(b"A" | b"Pos")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct MaxTypeDeserializer {
        a: Vec<String>,
        state__: Box<MaxTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum MaxTypeDeserializerState {
        Init__,
        A(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl MaxTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                a: Vec::new(),
                state__: Box::new(MaxTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: MaxTypeDeserializerState,
        ) -> Result<(), Error> {
            use MaxTypeDeserializerState as S;
            match state {
                S::A(Some(deserializer)) => self.store_a(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_a(&mut self, value: String) -> Result<(), Error> {
            self.a.push(value);
            Ok(())
        }
        fn handle_a<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<MaxTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MaxTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::A(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_a(data)?;
                    if self.a.len() < 2usize {
                        *self.state__ = S::A(None);
                    } else {
                        *self.state__ = S::Done__;
                    }
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::A(Some(deserializer)));
                    if self.a.len() < 1usize {
                        *self.state__ = S::A(None);
                    } else {
                        *self.state__ = S::Done__;
                    }
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::MaxType> for MaxTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MaxType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MaxType> {
            use MaxTypeDeserializerState as S;
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
                        *self.state__ = S::A(None);
                        event
                    }
                    (S::A(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"A",
                            false,
                        )?;
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
            let (event, allow_any) =
                helper.decide_unknown_element::<super::MaxType>(event, allow_any)?;
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::MaxType, Error> {
            let state = replace(&mut *self.state__, MaxTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::MaxType { a: self.a })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                Some(b"A")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct OuterTypeDeserializer {
        child: Option<super::ChildType>,
        b: Option<String>,
        state__: Box<OuterTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum OuterTypeDeserializerState {
        Init__,
        Child(Option<<super::ChildType as WithDeserializer>::Deserializer>),
        B(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl OuterTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                child: None,
                b: None,
                state__: Box::new(OuterTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: OuterTypeDeserializerState,
        ) -> Result<(), Error> {
            use OuterTypeDeserializerState as S;
            match state {
                S::Child(Some(deserializer)) => self.store_child(deserializer.finish(helper)?)?,
                S::B(Some(deserializer)) => self.store_b(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_child(&mut self, value: super::ChildType) -> Result<(), Error> {
            if self.child.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Child",
                )))?;
            }
            self.child = Some(value);
            Ok(())
        }
        fn store_b(&mut self, value: String) -> Result<(), Error> {
            if self.b.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"B")))?;
            }
            self.b = Some(value);
            Ok(())
        }
        fn handle_child<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ChildType>,
            fallback: &mut Option<OuterTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OuterTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Child(None));
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
                    self.store_child(data)?;
                    *self.state__ = S::B(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Child(Some(deserializer)));
                    *self.state__ = S::B(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_b<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<OuterTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OuterTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::B(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_b(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::B(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::OuterType> for OuterTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OuterType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OuterType> {
            use OuterTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Child(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_child(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Child(None);
                        event
                    }
                    (S::Child(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Child",
                            false,
                        )?;
                        match self.handle_child(helper, output, &mut fallback)? {
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
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"B",
                            false,
                        )?;
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
            let (event, allow_any) =
                helper.decide_unknown_element::<super::OuterType>(event, allow_any)?;
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::OuterType, Error> {
            let state = replace(&mut *self.state__, OuterTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::OuterType {
                child: helper.finish_element("Child", self.child)?,
                b: self.b,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                Some(b"Child" | b"B")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct ChildTypeDeserializer {
        inner: Option<String>,
        extra: Option<String>,
        state__: Box<ChildTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ChildTypeDeserializerState {
        Init__,
        Inner(Option<<String as WithDeserializer>::Deserializer>),
        Extra(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ChildTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                inner: None,
                extra: None,
                state__: Box::new(ChildTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ChildTypeDeserializerState,
        ) -> Result<(), Error> {
            use ChildTypeDeserializerState as S;
            match state {
                S::Inner(Some(deserializer)) => self.store_inner(deserializer.finish(helper)?)?,
                S::Extra(Some(deserializer)) => self.store_extra(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_inner(&mut self, value: String) -> Result<(), Error> {
            if self.inner.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Inner",
                )))?;
            }
            self.inner = Some(value);
            Ok(())
        }
        fn store_extra(&mut self, value: String) -> Result<(), Error> {
            if self.extra.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Extra",
                )))?;
            }
            self.extra = Some(value);
            Ok(())
        }
        fn handle_inner<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ChildTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ChildTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Inner(None));
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
                    self.store_inner(data)?;
                    *self.state__ = S::Extra(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Inner(Some(deserializer)));
                    *self.state__ = S::Extra(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_extra<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ChildTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ChildTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Extra(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_extra(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Extra(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ChildType> for ChildTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ChildType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ChildType> {
            use ChildTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Inner(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_inner(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Extra(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_extra(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Inner(None);
                        event
                    }
                    (S::Inner(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Inner",
                            false,
                        )?;
                        match self.handle_inner(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Extra(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Extra",
                            false,
                        )?;
                        match self.handle_extra(helper, output, &mut fallback)? {
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
            let (event, allow_any) =
                helper.decide_unknown_element::<super::ChildType>(event, allow_any)?;
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ChildType, Error> {
            let state = replace(&mut *self.state__, ChildTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::ChildType {
                inner: helper.finish_element("Inner", self.inner)?,
                extra: self.extra,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                Some(b"Inner" | b"Extra")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct NodeTypeDeserializer {
        name: Option<String>,
        kid: Option<super::NodeType>,
        state__: Box<NodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum NodeTypeDeserializerState {
        Init__,
        Name(Option<<String as WithDeserializer>::Deserializer>),
        Kid(Option<<super::NodeType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl NodeTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                name: None,
                kid: None,
                state__: Box::new(NodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: NodeTypeDeserializerState,
        ) -> Result<(), Error> {
            use NodeTypeDeserializerState as S;
            match state {
                S::Name(Some(deserializer)) => self.store_name(deserializer.finish(helper)?)?,
                S::Kid(Some(deserializer)) => self.store_kid(deserializer.finish(helper)?)?,
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
        fn store_kid(&mut self, value: super::NodeType) -> Result<(), Error> {
            if self.kid.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Kid")))?;
            }
            self.kid = Some(value);
            Ok(())
        }
        fn handle_name<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<NodeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use NodeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Name(None));
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
                    self.store_name(data)?;
                    *self.state__ = S::Kid(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Name(Some(deserializer)));
                    *self.state__ = S::Kid(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_kid<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::NodeType>,
            fallback: &mut Option<NodeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use NodeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Kid(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_kid(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Kid(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::NodeType> for NodeTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NodeType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NodeType> {
            use NodeTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Name(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_name(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Kid(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_kid(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Name(None);
                        event
                    }
                    (S::Name(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Name",
                            false,
                        )?;
                        match self.handle_name(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Kid(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Kid",
                            false,
                        )?;
                        match self.handle_kid(helper, output, &mut fallback)? {
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
            let (event, allow_any) =
                helper.decide_unknown_element::<super::NodeType>(event, allow_any)?;
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::NodeType, Error> {
            let state = replace(&mut *self.state__, NodeTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::NodeType {
                name: helper.finish_element("Name", self.name)?,
                kid: self.kid.map(Box::new),
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                Some(b"Name" | b"Kid")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct GRootTypeDeserializer {
        rec_group: Option<super::GRootRecGroupType>,
        state__: Box<GRootTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum GRootTypeDeserializerState {
        Init__,
        RecGroup(Option<<super::GRootRecGroupType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl GRootTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                rec_group: None,
                state__: Box::new(GRootTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: GRootTypeDeserializerState,
        ) -> Result<(), Error> {
            use GRootTypeDeserializerState as S;
            match state {
                S::RecGroup(Some(deserializer)) => {
                    self.store_rec_group(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_rec_group(&mut self, value: super::GRootRecGroupType) -> Result<(), Error> {
            if self.rec_group.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"RecGroup",
                )))?;
            }
            self.rec_group = Some(value);
            Ok(())
        }
        fn handle_rec_group<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::GRootRecGroupType>,
            fallback: &mut Option<GRootTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use GRootTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::RecGroup(None));
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
                    self.store_rec_group(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::RecGroup(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::GRootType> for GRootTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GRootType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GRootType> {
            use GRootTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::RecGroup(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_rec_group(helper, output, &mut fallback)? {
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
                        *self.state__ = S::RecGroup(None);
                        event
                    }
                    (S::RecGroup(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::GRootRecGroupType as WithDeserializer>::init(helper, event)?;
                        match self.handle_rec_group(helper, output, &mut fallback)? {
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
            let (event, allow_any) =
                helper.decide_unknown_element::<super::GRootType>(event, allow_any)?;
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::GRootType, Error> {
            let state = replace(&mut *self.state__, GRootTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::GRootType {
                rec_group: helper.finish_element("RecGroup", self.rec_group)?,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if <super::GRootRecGroupType as WithDeserializer>::Deserializer::is_known_start_tag(
                helper, x,
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct GRootRecGroupTypeDeserializer {
        v: Option<String>,
        via: Option<super::ViaType>,
        state__: Box<GRootRecGroupTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum GRootRecGroupTypeDeserializerState {
        Init__,
        V(Option<<String as WithDeserializer>::Deserializer>),
        Via(Option<<super::ViaType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl GRootRecGroupTypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: GRootRecGroupTypeDeserializerState,
        ) -> Result<(), Error> {
            use GRootRecGroupTypeDeserializerState as S;
            match state {
                S::V(Some(deserializer)) => self.store_v(deserializer.finish(helper)?)?,
                S::Via(Some(deserializer)) => self.store_via(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_v(&mut self, value: String) -> Result<(), Error> {
            if self.v.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"V")))?;
            }
            self.v = Some(value);
            Ok(())
        }
        fn store_via(&mut self, value: super::ViaType) -> Result<(), Error> {
            if self.via.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Via")))?;
            }
            self.via = Some(value);
            Ok(())
        }
        fn handle_v<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<GRootRecGroupTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use GRootRecGroupTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::V(None));
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
                    self.store_v(data)?;
                    *self.state__ = S::Via(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::V(Some(deserializer)));
                    *self.state__ = S::Via(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_via<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ViaType>,
            fallback: &mut Option<GRootRecGroupTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use GRootRecGroupTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Via(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_via(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Via(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::GRootRecGroupType> for GRootRecGroupTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GRootRecGroupType> {
            let deserializer = Self {
                v: None,
                via: None,
                state__: Box::new(GRootRecGroupTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, GRootRecGroupTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::GRootRecGroupType> {
            use GRootRecGroupTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::V(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_v(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Via(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_via(helper, output, &mut fallback)? {
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
                        *self.state__ = S::V(None);
                        event
                    }
                    (S::V(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"V",
                            false,
                        )?;
                        match self.handle_v(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Via(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Via",
                            false,
                        )?;
                        match self.handle_via(helper, output, &mut fallback)? {
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
        ) -> Result<super::GRootRecGroupType, Error> {
            let state = replace(
                &mut *self.state__,
                GRootRecGroupTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::GRootRecGroupType {
                v: helper.finish_element("V", self.v)?,
                via: self.via,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                Some(b"V" | b"Via")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct ViaTypeDeserializer {
        rec_group: Option<super::ViaRecGroupType>,
        state__: Box<ViaTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ViaTypeDeserializerState {
        Init__,
        RecGroup(Option<<super::ViaRecGroupType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ViaTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                rec_group: None,
                state__: Box::new(ViaTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ViaTypeDeserializerState,
        ) -> Result<(), Error> {
            use ViaTypeDeserializerState as S;
            match state {
                S::RecGroup(Some(deserializer)) => {
                    self.store_rec_group(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_rec_group(&mut self, value: super::ViaRecGroupType) -> Result<(), Error> {
            if self.rec_group.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"RecGroup",
                )))?;
            }
            self.rec_group = Some(value);
            Ok(())
        }
        fn handle_rec_group<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ViaRecGroupType>,
            fallback: &mut Option<ViaTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ViaTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::RecGroup(None));
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
                    self.store_rec_group(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::RecGroup(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ViaType> for ViaTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ViaType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ViaType> {
            use ViaTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::RecGroup(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_rec_group(helper, output, &mut fallback)? {
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
                        *self.state__ = S::RecGroup(None);
                        event
                    }
                    (S::RecGroup(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::ViaRecGroupType as WithDeserializer>::init(helper, event)?;
                        match self.handle_rec_group(helper, output, &mut fallback)? {
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
            let (event, allow_any) =
                helper.decide_unknown_element::<super::ViaType>(event, allow_any)?;
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ViaType, Error> {
            let state = replace(&mut *self.state__, ViaTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::ViaType {
                rec_group: helper.finish_element("RecGroup", self.rec_group)?,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if <super::ViaRecGroupType as WithDeserializer>::Deserializer::is_known_start_tag(
                helper, x,
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct ViaRecGroupTypeDeserializer {
        v: Option<String>,
        via: Option<super::ViaType>,
        state__: Box<ViaRecGroupTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ViaRecGroupTypeDeserializerState {
        Init__,
        V(Option<<String as WithDeserializer>::Deserializer>),
        Via(Option<<super::ViaType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ViaRecGroupTypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ViaRecGroupTypeDeserializerState,
        ) -> Result<(), Error> {
            use ViaRecGroupTypeDeserializerState as S;
            match state {
                S::V(Some(deserializer)) => self.store_v(deserializer.finish(helper)?)?,
                S::Via(Some(deserializer)) => self.store_via(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_v(&mut self, value: String) -> Result<(), Error> {
            if self.v.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"V")))?;
            }
            self.v = Some(value);
            Ok(())
        }
        fn store_via(&mut self, value: super::ViaType) -> Result<(), Error> {
            if self.via.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Via")))?;
            }
            self.via = Some(value);
            Ok(())
        }
        fn handle_v<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ViaRecGroupTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ViaRecGroupTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::V(None));
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
                    self.store_v(data)?;
                    *self.state__ = S::Via(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::V(Some(deserializer)));
                    *self.state__ = S::Via(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_via<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ViaType>,
            fallback: &mut Option<ViaRecGroupTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ViaRecGroupTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Via(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_via(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Via(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ViaRecGroupType> for ViaRecGroupTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ViaRecGroupType> {
            let deserializer = Self {
                v: None,
                via: None,
                state__: Box::new(ViaRecGroupTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, ViaRecGroupTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::ViaRecGroupType> {
            use ViaRecGroupTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::V(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_v(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Via(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_via(helper, output, &mut fallback)? {
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
                        *self.state__ = S::V(None);
                        event
                    }
                    (S::V(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"V",
                            false,
                        )?;
                        match self.handle_v(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Via(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Via",
                            false,
                        )?;
                        match self.handle_via(helper, output, &mut fallback)? {
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
        ) -> Result<super::ViaRecGroupType, Error> {
            let state = replace(
                &mut *self.state__,
                ViaRecGroupTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ViaRecGroupType {
                v: helper.finish_element("V", self.v)?,
                via: self.via.map(Box::new),
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                Some(b"V" | b"Via")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct SceneTypeDeserializer {
        shape: Option<super::Shape>,
        state__: Box<SceneTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SceneTypeDeserializerState {
        Init__,
        Shape(Option<<super::Shape as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SceneTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                shape: None,
                state__: Box::new(SceneTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SceneTypeDeserializerState,
        ) -> Result<(), Error> {
            use SceneTypeDeserializerState as S;
            match state {
                S::Shape(Some(deserializer)) => self.store_shape(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_shape(&mut self, value: super::Shape) -> Result<(), Error> {
            if self.shape.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Shape",
                )))?;
            }
            self.shape = Some(value);
            Ok(())
        }
        fn handle_shape<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::Shape>,
            fallback: &mut Option<SceneTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SceneTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Shape(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_shape(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Shape(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SceneType> for SceneTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SceneType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SceneType> {
            use SceneTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Shape(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_shape(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Shape(None);
                        event
                    }
                    (S::Shape(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = <super::Shape as WithDeserializer>::init(helper, event)?;
                        match self.handle_shape(helper, output, &mut fallback)? {
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
            let (event, allow_any) =
                helper.decide_unknown_element::<super::SceneType>(event, allow_any)?;
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::SceneType, Error> {
            let state = replace(&mut *self.state__, SceneTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::SceneType { shape: self.shape })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                Some(b"Shape")
            ) {
                return true;
            }
            if <super::Shape as WithDeserializer>::Deserializer::is_known_start_tag(helper, x) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub enum ShapeDeserializer {
        Circle(<super::ShapeType as WithDeserializer>::Deserializer),
        Square(<super::ShapeType as WithDeserializer>::Deserializer),
    }
    impl<'de> Deserializer<'de, super::Shape> for ShapeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Shape> {
            let Some(type_name) = helper.get_dynamic_type_from_tag(&event) else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::None,
                    allow_any: false,
                });
            };
            let type_name = type_name.into_owned();
            if matches!(
                helper.resolve_element_local_name(QName(&type_name), &super::NS_TNS),
                Some(b"Circle")
            ) {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = <super::ShapeType as WithDeserializer>::init(helper, event)?;
                return Ok(DeserializerOutput {
                    artifact: artifact.map(|x| super::Shape(Box::new(x)), |x| Self::Circle(x)),
                    event,
                    allow_any,
                });
            }
            if matches!(
                helper.resolve_element_local_name(QName(&type_name), &super::NS_TNS),
                Some(b"Square")
            ) {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = <super::ShapeType as WithDeserializer>::init(helper, event)?;
                return Ok(DeserializerOutput {
                    artifact: artifact.map(|x| super::Shape(Box::new(x)), |x| Self::Square(x)),
                    event,
                    allow_any,
                });
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::None,
                event: DeserializerEvent::Break(event),
                allow_any: false,
            })
        }
        fn next(
            self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Shape> {
            match self {
                Self::Circle(x) => {
                    let DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = x.next(helper, event)?;
                    Ok(DeserializerOutput {
                        artifact: artifact.map(|x| super::Shape(Box::new(x)), |x| Self::Circle(x)),
                        event,
                        allow_any,
                    })
                }
                Self::Square(x) => {
                    let DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = x.next(helper, event)?;
                    Ok(DeserializerOutput {
                        artifact: artifact.map(|x| super::Shape(Box::new(x)), |x| Self::Square(x)),
                        event,
                        allow_any,
                    })
                }
            }
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::Shape, Error> {
            match self {
                Self::Circle(x) => Ok(super::Shape(Box::new(x.finish(helper)?))),
                Self::Square(x) => Ok(super::Shape(Box::new(x.finish(helper)?))),
            }
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                Some(b"Circle" | b"Square")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct ShapeTypeDeserializer {
        id: Option<i32>,
        state__: Box<ShapeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ShapeTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl ShapeTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut id: Option<i32> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_attrib_local_name(attrib.key, &super::NS_TNS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id,
                state__: Box::new(ShapeTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ShapeTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::ShapeType> for ShapeTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ShapeType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ShapeType> {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(helper)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                let event = DeserializerEvent::Break(event);
                let allow_any = false;
                let (event, allow_any) =
                    helper.decide_unknown_element::<super::ShapeType>(event, allow_any)?;
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event,
                    allow_any,
                })
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ShapeType, Error> {
            let state = replace(&mut *self.state__, ShapeTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::ShapeType { id: self.id })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            let _ = (helper, x);
            false
        }
    }
    #[derive(Debug)]
    pub struct PickTypeDeserializer {
        state__: Box<PickTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum PickTypeDeserializerState {
        Init__,
        X(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Y(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::PickType),
        Unknown__,
    }
    impl PickTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                    Some(b"X")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_x(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                    Some(b"Y")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_y(helper, Default::default(), None, output);
                }
            }
            *self.state__ = PickTypeDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
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
                state__: Box::new(PickTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: PickTypeDeserializerState,
        ) -> Result<super::PickType, Error> {
            use PickTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::X(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_x(&mut values, value)?;
                    }
                    Ok(super::PickType::X(helper.finish_element("X", values)?))
                }
                S::Y(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_y(&mut values, value)?;
                    }
                    Ok(super::PickType::Y(helper.finish_element("Y", values)?))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_x(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"X")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_y(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Y")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_x<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PickTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let fresh = values.is_none() && fallback.is_none();
                *self.state__ = S::X(values, None, fallback);
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_x(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x(&mut values, data)?;
                    *self.state__ = S::X(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::X(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
        fn handle_y<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PickTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let fresh = values.is_none() && fallback.is_none();
                *self.state__ = S::Y(values, None, fallback);
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_y(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_y(&mut values, data)?;
                    *self.state__ = S::Y(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Y(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::PickType> for PickTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PickType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PickType> {
            use PickTypeDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::X(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_x(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Y(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_y(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::X(values, fallback, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"X",
                            false,
                        )?;
                        match self.handle_x(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Y(values, fallback, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Y",
                            false,
                        )?;
                        match self.handle_y(helper, values, fallback, output)? {
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
            let (event, allow_any) =
                helper.decide_unknown_element::<super::PickType>(event, allow_any)?;
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::PickType, Error> {
            Self::finish_state(helper, *self.state__)
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                Some(b"X" | b"Y")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct UnorderedTypeDeserializer {
        x: Option<String>,
        y: Option<String>,
        state__: Box<UnorderedTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum UnorderedTypeDeserializerState {
        Init__,
        Next__,
        X(<String as WithDeserializer>::Deserializer),
        Y(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl UnorderedTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
            fallback: &mut Option<UnorderedTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                    Some(b"X")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_x(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                    Some(b"Y")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_y(helper, output, &mut *fallback);
                }
            }
            *self.state__ = fallback
                .take()
                .unwrap_or(UnorderedTypeDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
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
                x: None,
                y: None,
                state__: Box::new(UnorderedTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: UnorderedTypeDeserializerState,
        ) -> Result<(), Error> {
            use UnorderedTypeDeserializerState as S;
            match state {
                S::X(deserializer) => self.store_x(deserializer.finish(helper)?)?,
                S::Y(deserializer) => self.store_y(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_x(&mut self, value: String) -> Result<(), Error> {
            if self.x.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"X")))?;
            }
            self.x = Some(value);
            Ok(())
        }
        fn store_y(&mut self, value: String) -> Result<(), Error> {
            if self.y.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Y")))?;
            }
            self.y = Some(value);
            Ok(())
        }
        fn handle_x<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<UnorderedTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use UnorderedTypeDeserializerState as S;
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
                    self.store_x(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::X(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_y<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<UnorderedTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use UnorderedTypeDeserializerState as S;
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
                    self.store_y(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Y(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::UnorderedType> for UnorderedTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UnorderedType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UnorderedType> {
            use UnorderedTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::X(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_x(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, .. } => event,
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Y(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_y(helper, output, &mut fallback)? {
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
            let (event, allow_any) =
                helper.decide_unknown_element::<super::UnorderedType>(event, allow_any)?;
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::UnorderedType, Error> {
            let state = replace(
                &mut *self.state__,
                UnorderedTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::UnorderedType {
                x: helper.finish_element("X", self.x)?,
                y: self.y,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                Some(b"X" | b"Y")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct NothingTypeDeserializer {
        state__: Box<NothingTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum NothingTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl NothingTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                state__: Box::new(NothingTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: NothingTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::NothingType> for NothingTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NothingType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NothingType> {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(helper)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                let event = DeserializerEvent::Break(event);
                let allow_any = false;
                let (event, allow_any) =
                    helper.decide_unknown_element::<super::NothingType>(event, allow_any)?;
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event,
                    allow_any,
                })
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::NothingType, Error> {
            let state = replace(&mut *self.state__, NothingTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::NothingType {})
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            let _ = (helper, x);
            false
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, ElementNamespace, Error, Event, IterSerializer, SerializeHelper,
        Serializer, WithSerializer,
    };
    #[derive(Debug)]
    pub struct RootTypeSerializer<'ser> {
        pub(super) value: &'ser super::RootType,
        pub(super) state: Box<RootTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum RootTypeSerializerState<'ser> {
        Init__,
        A(<String as WithSerializer>::Serializer<'ser>),
        Pos(IterSerializer<'ser, Option<&'ser String>, String>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RootTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    RootTypeSerializerState::Init__ => {
                        *self.state =
                            RootTypeSerializerState::A(WithSerializer::serializer_with_namespace(
                                &self.value.a,
                                Some("tns:A"),
                                ElementNamespace::Namespace(super::NS_TNS),
                                false,
                            )?);
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
                        return Ok(Some(Event::Start(bytes)));
                    }
                    RootTypeSerializerState::A(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = RootTypeSerializerState::Pos(
                                IterSerializer::new(
                                    self.value.pos.as_ref(),
                                    Some("tns:Pos"),
                                    false,
                                )
                                .with_namespace(ElementNamespace::Namespace(super::NS_TNS)),
                            )
                        }
                    },
                    RootTypeSerializerState::Pos(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = RootTypeSerializerState::End__,
                    },
                    RootTypeSerializerState::End__ => {
                        *self.state = RootTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    RootTypeSerializerState::Done__ => return Ok(None),
                    RootTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for RootTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = RootTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct MaxTypeSerializer<'ser> {
        pub(super) value: &'ser super::MaxType,
        pub(super) state: Box<MaxTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum MaxTypeSerializerState<'ser> {
        Init__,
        A(IterSerializer<'ser, &'ser [String], String>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> MaxTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MaxTypeSerializerState::Init__ => {
                        *self.state = MaxTypeSerializerState::A(
                            IterSerializer::new(&self.value.a[..], Some("tns:A"), false)
                                .with_namespace(ElementNamespace::Namespace(super::NS_TNS)),
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
                            helper.write_xmlns_unless_default(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    MaxTypeSerializerState::A(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = MaxTypeSerializerState::End__,
                    },
                    MaxTypeSerializerState::End__ => {
                        *self.state = MaxTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    MaxTypeSerializerState::Done__ => return Ok(None),
                    MaxTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for MaxTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = MaxTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct OuterTypeSerializer<'ser> {
        pub(super) value: &'ser super::OuterType,
        pub(super) state: Box<OuterTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum OuterTypeSerializerState<'ser> {
        Init__,
        Child(<super::ChildType as WithSerializer>::Serializer<'ser>),
        B(IterSerializer<'ser, Option<&'ser String>, String>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> OuterTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    OuterTypeSerializerState::Init__ => {
                        *self.state = OuterTypeSerializerState::Child(
                            WithSerializer::serializer_with_namespace(
                                &self.value.child,
                                Some("tns:Child"),
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
                            helper.write_xmlns_unless_default(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    OuterTypeSerializerState::Child(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = OuterTypeSerializerState::B(
                                IterSerializer::new(self.value.b.as_ref(), Some("tns:B"), false)
                                    .with_namespace(ElementNamespace::Namespace(super::NS_TNS)),
                            )
                        }
                    },
                    OuterTypeSerializerState::B(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = OuterTypeSerializerState::End__,
                    },
                    OuterTypeSerializerState::End__ => {
                        *self.state = OuterTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    OuterTypeSerializerState::Done__ => return Ok(None),
                    OuterTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for OuterTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = OuterTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ChildTypeSerializer<'ser> {
        pub(super) value: &'ser super::ChildType,
        pub(super) state: Box<ChildTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ChildTypeSerializerState<'ser> {
        Init__,
        Inner(<String as WithSerializer>::Serializer<'ser>),
        Extra(IterSerializer<'ser, Option<&'ser String>, String>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ChildTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ChildTypeSerializerState::Init__ => {
                        *self.state = ChildTypeSerializerState::Inner(
                            WithSerializer::serializer_with_namespace(
                                &self.value.inner,
                                Some("tns:Inner"),
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
                            helper.write_xmlns_unless_default(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ChildTypeSerializerState::Inner(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = ChildTypeSerializerState::Extra(
                                IterSerializer::new(
                                    self.value.extra.as_ref(),
                                    Some("tns:Extra"),
                                    false,
                                )
                                .with_namespace(ElementNamespace::Namespace(super::NS_TNS)),
                            )
                        }
                    },
                    ChildTypeSerializerState::Extra(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ChildTypeSerializerState::End__,
                    },
                    ChildTypeSerializerState::End__ => {
                        *self.state = ChildTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ChildTypeSerializerState::Done__ => return Ok(None),
                    ChildTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ChildTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ChildTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct NodeTypeSerializer<'ser> {
        pub(super) value: &'ser super::NodeType,
        pub(super) state: Box<NodeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum NodeTypeSerializerState<'ser> {
        Init__,
        Name(<String as WithSerializer>::Serializer<'ser>),
        Kid(IterSerializer<'ser, Option<&'ser super::NodeType>, super::NodeType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> NodeTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    NodeTypeSerializerState::Init__ => {
                        *self.state = NodeTypeSerializerState::Name(
                            WithSerializer::serializer_with_namespace(
                                &self.value.name,
                                Some("tns:Name"),
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
                            helper.write_xmlns_unless_default(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    NodeTypeSerializerState::Name(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = NodeTypeSerializerState::Kid(
                                IterSerializer::new(
                                    self.value.kid.as_deref(),
                                    Some("tns:Kid"),
                                    false,
                                )
                                .with_namespace(ElementNamespace::Namespace(super::NS_TNS)),
                            )
                        }
                    },
                    NodeTypeSerializerState::Kid(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = NodeTypeSerializerState::End__,
                    },
                    NodeTypeSerializerState::End__ => {
                        *self.state = NodeTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    NodeTypeSerializerState::Done__ => return Ok(None),
                    NodeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for NodeTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = NodeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct GRootTypeSerializer<'ser> {
        pub(super) value: &'ser super::GRootType,
        pub(super) state: Box<GRootTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum GRootTypeSerializerState<'ser> {
        Init__,
        RecGroup(<super::GRootRecGroupType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> GRootTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    GRootTypeSerializerState::Init__ => {
                        *self.state = GRootTypeSerializerState::RecGroup(
                            WithSerializer::serializer_with_namespace(
                                &self.value.rec_group,
                                Some("RecGroup"),
                                ElementNamespace::None,
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
                            helper.write_xmlns_unless_default(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    GRootTypeSerializerState::RecGroup(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = GRootTypeSerializerState::End__,
                    },
                    GRootTypeSerializerState::End__ => {
                        *self.state = GRootTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    GRootTypeSerializerState::Done__ => return Ok(None),
                    GRootTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for GRootTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = GRootTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct GRootRecGroupTypeSerializer<'ser> {
        pub(super) value: &'ser super::GRootRecGroupType,
        pub(super) state: Box<GRootRecGroupTypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum GRootRecGroupTypeSerializerState<'ser> {
        Init__,
        V(<String as WithSerializer>::Serializer<'ser>),
        Via(IterSerializer<'ser, Option<&'ser super::ViaType>, super::ViaType>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> GRootRecGroupTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    GRootRecGroupTypeSerializerState::Init__ => {
                        *self.state = GRootRecGroupTypeSerializerState::V(
                            WithSerializer::serializer_with_namespace(
                                &self.value.v,
                                Some("tns:V"),
                                ElementNamespace::Namespace(super::NS_TNS),
                                false,
                            )?,
                        );
                    }
                    GRootRecGroupTypeSerializerState::V(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = GRootRecGroupTypeSerializerState::Via(
                                IterSerializer::new(
                                    self.value.via.as_ref(),
                                    Some("tns:Via"),
                                    false,
                                )
                                .with_namespace(ElementNamespace::Namespace(super::NS_TNS)),
                            )
                        }
                    },
                    GRootRecGroupTypeSerializerState::Via(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = GRootRecGroupTypeSerializerState::Done__,
                    },
                    GRootRecGroupTypeSerializerState::Done__ => return Ok(None),
                    GRootRecGroupTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for GRootRecGroupTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = GRootRecGroupTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ViaTypeSerializer<'ser> {
        pub(super) value: &'ser super::ViaType,
        pub(super) state: Box<ViaTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ViaTypeSerializerState<'ser> {
        Init__,
        RecGroup(<super::ViaRecGroupType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ViaTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ViaTypeSerializerState::Init__ => {
                        *self.state = ViaTypeSerializerState::RecGroup(
                            WithSerializer::serializer_with_namespace(
                                &self.value.rec_group,
                                Some("RecGroup"),
                                ElementNamespace::None,
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
                            helper.write_xmlns_unless_default(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ViaTypeSerializerState::RecGroup(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ViaTypeSerializerState::End__,
                    },
                    ViaTypeSerializerState::End__ => {
                        *self.state = ViaTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ViaTypeSerializerState::Done__ => return Ok(None),
                    ViaTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ViaTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ViaTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ViaRecGroupTypeSerializer<'ser> {
        pub(super) value: &'ser super::ViaRecGroupType,
        pub(super) state: Box<ViaRecGroupTypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum ViaRecGroupTypeSerializerState<'ser> {
        Init__,
        V(<String as WithSerializer>::Serializer<'ser>),
        Via(IterSerializer<'ser, Option<&'ser super::ViaType>, super::ViaType>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ViaRecGroupTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ViaRecGroupTypeSerializerState::Init__ => {
                        *self.state = ViaRecGroupTypeSerializerState::V(
                            WithSerializer::serializer_with_namespace(
                                &self.value.v,
                                Some("tns:V"),
                                ElementNamespace::Namespace(super::NS_TNS),
                                false,
                            )?,
                        );
                    }
                    ViaRecGroupTypeSerializerState::V(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = ViaRecGroupTypeSerializerState::Via(
                                IterSerializer::new(
                                    self.value.via.as_deref(),
                                    Some("tns:Via"),
                                    false,
                                )
                                .with_namespace(ElementNamespace::Namespace(super::NS_TNS)),
                            )
                        }
                    },
                    ViaRecGroupTypeSerializerState::Via(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ViaRecGroupTypeSerializerState::Done__,
                    },
                    ViaRecGroupTypeSerializerState::Done__ => return Ok(None),
                    ViaRecGroupTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ViaRecGroupTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ViaRecGroupTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SceneTypeSerializer<'ser> {
        pub(super) value: &'ser super::SceneType,
        pub(super) state: Box<SceneTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SceneTypeSerializerState<'ser> {
        Init__,
        Shape(IterSerializer<'ser, Option<&'ser super::Shape>, super::Shape>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SceneTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SceneTypeSerializerState::Init__ => {
                        *self.state = SceneTypeSerializerState::Shape(
                            IterSerializer::new(
                                self.value.shape.as_ref(),
                                Some("tns:Shape"),
                                false,
                            )
                            .with_namespace(ElementNamespace::Namespace(super::NS_TNS)),
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
                            helper.write_xmlns_unless_default(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SceneTypeSerializerState::Shape(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = SceneTypeSerializerState::End__,
                    },
                    SceneTypeSerializerState::End__ => {
                        *self.state = SceneTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SceneTypeSerializerState::Done__ => return Ok(None),
                    SceneTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for SceneTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SceneTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ShapeTypeSerializer<'ser> {
        pub(super) value: &'ser super::ShapeType,
        pub(super) state: Box<ShapeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ShapeTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ShapeTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ShapeTypeSerializerState::Init__ => {
                        *self.state = ShapeTypeSerializerState::Done__;
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
                        helper.write_attrib_opt(&mut bytes, "id", &self.value.id)?;
                        helper.end_ns_scope();
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    ShapeTypeSerializerState::Done__ => return Ok(None),
                    ShapeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ShapeTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ShapeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PickTypeSerializer<'ser> {
        pub(super) value: &'ser super::PickType,
        pub(super) state: Box<PickTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum PickTypeSerializerState<'ser> {
        Init__,
        X(<String as WithSerializer>::Serializer<'ser>),
        Y(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PickTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PickTypeSerializerState::Init__ => {
                        match self.value {
                            super::PickType::X(x) => {
                                *self.state = PickTypeSerializerState::X(
                                    WithSerializer::serializer_with_namespace(
                                        x,
                                        Some("tns:X"),
                                        ElementNamespace::Namespace(super::NS_TNS),
                                        self.is_root,
                                    )?,
                                )
                            }
                            super::PickType::Y(x) => {
                                *self.state = PickTypeSerializerState::Y(
                                    WithSerializer::serializer_with_namespace(
                                        x,
                                        Some("tns:Y"),
                                        ElementNamespace::Namespace(super::NS_TNS),
                                        self.is_root,
                                    )?,
                                )
                            }
                        }
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
                        return Ok(Some(Event::Start(bytes)));
                    }
                    PickTypeSerializerState::X(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PickTypeSerializerState::End__,
                    },
                    PickTypeSerializerState::Y(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PickTypeSerializerState::End__,
                    },
                    PickTypeSerializerState::End__ => {
                        *self.state = PickTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    PickTypeSerializerState::Done__ => return Ok(None),
                    PickTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for PickTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PickTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct UnorderedTypeSerializer<'ser> {
        pub(super) value: &'ser super::UnorderedType,
        pub(super) state: Box<UnorderedTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum UnorderedTypeSerializerState<'ser> {
        Init__,
        X(<String as WithSerializer>::Serializer<'ser>),
        Y(IterSerializer<'ser, Option<&'ser String>, String>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> UnorderedTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    UnorderedTypeSerializerState::Init__ => {
                        *self.state = UnorderedTypeSerializerState::X(
                            WithSerializer::serializer_with_namespace(
                                &self.value.x,
                                Some("tns:X"),
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
                            helper.write_xmlns_unless_default(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    UnorderedTypeSerializerState::X(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = UnorderedTypeSerializerState::Y(
                                IterSerializer::new(self.value.y.as_ref(), Some("tns:Y"), false)
                                    .with_namespace(ElementNamespace::Namespace(super::NS_TNS)),
                            )
                        }
                    },
                    UnorderedTypeSerializerState::Y(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = UnorderedTypeSerializerState::End__,
                    },
                    UnorderedTypeSerializerState::End__ => {
                        *self.state = UnorderedTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    UnorderedTypeSerializerState::Done__ => return Ok(None),
                    UnorderedTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for UnorderedTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = UnorderedTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct NothingTypeSerializer<'ser> {
        pub(super) value: &'ser super::NothingType,
        pub(super) state: Box<NothingTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum NothingTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> NothingTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    NothingTypeSerializerState::Init__ => {
                        *self.state = NothingTypeSerializerState::Done__;
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
                        helper.end_ns_scope();
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    NothingTypeSerializerState::Done__ => return Ok(None),
                    NothingTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for NothingTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = NothingTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
