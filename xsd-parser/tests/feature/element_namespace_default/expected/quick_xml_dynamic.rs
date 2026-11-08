use core::fmt::Debug;
use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{
        BoxedSerializer, BytesStart, CollectNamespaces, ElementNamespace, Error, SerializeHelper,
        WithBoxedSerializer, WithDeserializer, WithSerializer,
    },
    xml::{AnyElement, AnySimpleType, Nillable},
    AsAny,
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_UNNAMED_5: Namespace = Namespace::new_const(b"T");
pub const NS_O: Namespace = Namespace::new_const(b"O");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const PREFIX_O: NamespacePrefix = NamespacePrefix::new_const(b"o");
pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub child: Option<ChildType>,
    pub item: Vec<String>,
    pub ref_: String,
    pub shape: Shape,
    pub n: Nillable<String>,
    pub any_value: AnySimpleType,
    pub node: NodeType,
    pub any: AnyElement,
}
impl CollectNamespaces for FooType {
    fn collect_namespaces(&self, helper: &mut SerializeHelper, bytes: &mut BytesStart<'_>) {
        if let Some(inner) = &self.child {
            inner.collect_namespaces(helper, bytes);
        }
        for item in &self.item {
            item.collect_namespaces(helper, bytes);
        }
        self.ref_.collect_namespaces(helper, bytes);
        self.shape.collect_namespaces(helper, bytes);
        self.n.collect_namespaces(helper, bytes);
        self.any_value.collect_namespaces(helper, bytes);
        self.node.collect_namespaces(helper, bytes);
        self.any.collect_namespaces(helper, bytes);
    }
}
impl WithSerializer for FooType {
    type Serializer<'x> = quick_xml_serialize::FooTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::FooTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::FooTypeSerializerState::Init__),
            name: name.unwrap_or("Foo"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for FooType {
    type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
}
#[derive(Debug)]
pub struct ChildType {
    pub value: String,
}
impl CollectNamespaces for ChildType {
    fn collect_namespaces(&self, helper: &mut SerializeHelper, bytes: &mut BytesStart<'_>) {
        self.value.collect_namespaces(helper, bytes);
    }
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
            name: name.unwrap_or("ChildType"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for ChildType {
    type Deserializer = quick_xml_deserialize::ChildTypeDeserializer;
}
#[derive(Debug)]
pub struct Shape(pub Box<dyn ShapeTrait>);
pub trait ShapeTrait: Debug + AsAny + CollectNamespaces + WithBoxedSerializer {}
impl Shape {
    pub fn new<T: ShapeTrait + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
impl CollectNamespaces for Shape {
    fn collect_namespaces(&self, helper: &mut SerializeHelper, bytes: &mut BytesStart<'_>) {
        self.0.collect_namespaces(helper, bytes);
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
        if (*self.0).as_any().is::<CircleType>() {
            return self.0.serializer_with_namespace(
                Some("o:Circle"),
                ElementNamespace::Namespace(NS_O),
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
pub struct NodeType {
    pub name: String,
    pub node: Option<Box<NodeType>>,
}
impl CollectNamespaces for NodeType {
    fn collect_namespaces(&self, helper: &mut SerializeHelper, bytes: &mut BytesStart<'_>) {
        self.name.collect_namespaces(helper, bytes);
        if let Some(inner) = &self.node {
            inner.collect_namespaces(helper, bytes);
        }
    }
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
            name: name.unwrap_or("NodeType"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for NodeType {
    type Deserializer = quick_xml_deserialize::NodeTypeDeserializer;
}
#[derive(Debug)]
pub struct CircleType {
    pub radius: i32,
}
impl ShapeTrait for CircleType {}
impl CollectNamespaces for CircleType {
    fn collect_namespaces(&self, helper: &mut SerializeHelper, bytes: &mut BytesStart<'_>) {
        helper.write_xmlns(bytes, Some(&PREFIX_O), &NS_O);
        self.radius.collect_namespaces(helper, bytes);
    }
}
impl WithSerializer for CircleType {
    type Serializer<'x> = quick_xml_serialize::CircleTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::CircleTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::CircleTypeSerializerState::Init__),
            name: name.unwrap_or("o:Circle"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for CircleType {
    type Deserializer = quick_xml_deserialize::CircleTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::{
        quick_xml::{
            BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
            QName, RawByteStr, WithDeserializer,
        },
        xml::{AnyElement, AnySimpleType, Nillable},
    };
    #[derive(Debug)]
    pub struct FooTypeDeserializer {
        child: Option<super::ChildType>,
        item: Vec<String>,
        ref_: Option<String>,
        shape: Option<super::Shape>,
        n: Option<Nillable<String>>,
        any_value: Option<AnySimpleType>,
        node: Option<super::NodeType>,
        any: Option<AnyElement>,
        state__: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Init__,
        Child(Option<<super::ChildType as WithDeserializer>::Deserializer>),
        Item(Option<<String as WithDeserializer>::Deserializer>),
        Ref(Option<<String as WithDeserializer>::Deserializer>),
        Shape(Option<<super::Shape as WithDeserializer>::Deserializer>),
        N(Option<<Nillable<String> as WithDeserializer>::Deserializer>),
        AnyValue(Option<<AnySimpleType as WithDeserializer>::Deserializer>),
        Node(Option<<super::NodeType as WithDeserializer>::Deserializer>),
        Any(Option<<AnyElement as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl FooTypeDeserializer {
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
                item: Vec::new(),
                ref_: None,
                shape: None,
                n: None,
                any_value: None,
                node: None,
                any: None,
                state__: Box::new(FooTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: FooTypeDeserializerState,
        ) -> Result<(), Error> {
            use FooTypeDeserializerState as S;
            match state {
                S::Child(Some(deserializer)) => self.store_child(deserializer.finish(helper)?)?,
                S::Item(Some(deserializer)) => self.store_item(deserializer.finish(helper)?)?,
                S::Ref(Some(deserializer)) => self.store_ref_(deserializer.finish(helper)?)?,
                S::Shape(Some(deserializer)) => self.store_shape(deserializer.finish(helper)?)?,
                S::N(Some(deserializer)) => self.store_n(deserializer.finish(helper)?)?,
                S::AnyValue(Some(deserializer)) => {
                    self.store_any_value(deserializer.finish(helper)?)?
                }
                S::Node(Some(deserializer)) => self.store_node(deserializer.finish(helper)?)?,
                S::Any(Some(deserializer)) => self.store_any(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_child(&mut self, value: super::ChildType) -> Result<(), Error> {
            if self.child.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"child",
                )))?;
            }
            self.child = Some(value);
            Ok(())
        }
        fn store_item(&mut self, value: String) -> Result<(), Error> {
            self.item.push(value);
            Ok(())
        }
        fn store_ref_(&mut self, value: String) -> Result<(), Error> {
            if self.ref_.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Ref")))?;
            }
            self.ref_ = Some(value);
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
        fn store_n(&mut self, value: Nillable<String>) -> Result<(), Error> {
            if self.n.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"n")))?;
            }
            self.n = Some(value);
            Ok(())
        }
        fn store_any_value(&mut self, value: AnySimpleType) -> Result<(), Error> {
            if self.any_value.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"anyValue",
                )))?;
            }
            self.any_value = Some(value);
            Ok(())
        }
        fn store_node(&mut self, value: super::NodeType) -> Result<(), Error> {
            if self.node.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"node")))?;
            }
            self.node = Some(value);
            Ok(())
        }
        fn store_any(&mut self, value: AnyElement) -> Result<(), Error> {
            if self.any.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"any4")))?;
            }
            self.any = Some(value);
            Ok(())
        }
        fn handle_child<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ChildType>,
            fallback: &mut Option<FooTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FooTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Item(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_child(data)?;
                    *self.state__ = S::Item(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Child(Some(deserializer)));
                    *self.state__ = S::Item(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_item<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<FooTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FooTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
                if let Some(fallback) = fallback.take() {
                    self.finish_state(helper, fallback)?;
                }
                if self.item.len() < 1usize {
                    fallback.get_or_insert(S::Item(None));
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else {
                    *self.state__ = S::Ref(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_item(data)?;
                    if self.item.len() < 2usize {
                        *self.state__ = S::Item(None);
                    } else {
                        *self.state__ = S::Ref(None);
                    }
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Item(Some(deserializer)));
                    if self.item.len() < 1usize {
                        *self.state__ = S::Item(None);
                    } else {
                        *self.state__ = S::Ref(None);
                    }
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_ref_<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<FooTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FooTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Ref(None));
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
                    self.store_ref_(data)?;
                    *self.state__ = S::Shape(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Ref(Some(deserializer)));
                    *self.state__ = S::Shape(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_shape<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::Shape>,
            fallback: &mut Option<FooTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FooTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Shape(None));
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
                    self.store_shape(data)?;
                    *self.state__ = S::N(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Shape(Some(deserializer)));
                    *self.state__ = S::N(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_n<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, Nillable<String>>,
            fallback: &mut Option<FooTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FooTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::N(None));
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
                    self.store_n(data)?;
                    *self.state__ = S::AnyValue(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::N(Some(deserializer)));
                    *self.state__ = S::AnyValue(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_any_value<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, AnySimpleType>,
            fallback: &mut Option<FooTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FooTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::AnyValue(None));
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
                    self.store_any_value(data)?;
                    *self.state__ = S::Node(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::AnyValue(Some(deserializer)));
                    *self.state__ = S::Node(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_node<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::NodeType>,
            fallback: &mut Option<FooTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FooTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Node(None));
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
                    self.store_node(data)?;
                    *self.state__ = S::Any(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Node(Some(deserializer)));
                    *self.state__ = S::Any(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_any<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, AnyElement>,
            fallback: &mut Option<FooTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FooTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Any(None));
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
                    self.store_any(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Any(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::FooType> for FooTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooType> {
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
                    (S::Item(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_item(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Ref(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_ref_(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
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
                    (S::N(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_n(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::AnyValue(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_any_value(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Node(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_node(helper, output, &mut fallback)? {
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
                        let output = deserializer.next(helper, event)?;
                        match self.handle_any(helper, output, &mut fallback)? {
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
                            Some(&super::NS_UNNAMED_5),
                            b"child",
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
                    (S::Item(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_5),
                            b"item",
                            false,
                        )?;
                        match self.handle_item(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Ref(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_5),
                            b"Ref",
                            false,
                        )?;
                        match self.handle_ref_(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
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
                    (S::N(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_5),
                            b"n",
                            false,
                        )?;
                        match self.handle_n(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::AnyValue(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_5),
                            b"anyValue",
                            false,
                        )?;
                        match self.handle_any_value(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Node(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_5),
                            b"node",
                            false,
                        )?;
                        match self.handle_node(helper, output, &mut fallback)? {
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
                            let output = <AnyElement as WithDeserializer>::init(helper, event)?;
                            match self.handle_any(helper, output, &mut fallback)? {
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
                            *self.state__ = S::Done__;
                            break (DeserializerEvent::Continue(event), allow_any_element);
                        }
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::FooType, Error> {
            let state = replace(&mut *self.state__, FooTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::FooType {
                child: self.child,
                item: helper.finish_vec(1usize, Some(2usize), self.item)?,
                ref_: helper.finish_element("Ref", self.ref_)?,
                shape: helper.finish_element("Shape", self.shape)?,
                n: helper.finish_element("n", self.n)?,
                any_value: helper.finish_element("anyValue", self.any_value)?,
                node: helper.finish_element("node", self.node)?,
                any: helper.finish_element("any4", self.any)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ChildTypeDeserializer {
        value: Option<String>,
        state__: Box<ChildTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ChildTypeDeserializerState {
        Init__,
        Value(Option<<String as WithDeserializer>::Deserializer>),
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
                value: None,
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
                S::Value(Some(deserializer)) => self.store_value(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_value(&mut self, value: String) -> Result<(), Error> {
            if self.value.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"value",
                )))?;
            }
            self.value = Some(value);
            Ok(())
        }
        fn handle_value<'de>(
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
                fallback.get_or_insert(S::Value(None));
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
                    self.store_value(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Value(Some(deserializer)));
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
                    (S::Value(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_value(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Value(None);
                        event
                    }
                    (S::Value(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_5),
                            b"value",
                            false,
                        )?;
                        match self.handle_value(helper, output, &mut fallback)? {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ChildType, Error> {
            let state = replace(&mut *self.state__, ChildTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::ChildType {
                value: helper.finish_element("value", self.value)?,
            })
        }
    }
    #[derive(Debug)]
    pub enum ShapeDeserializer {
        Circle(<super::CircleType as WithDeserializer>::Deserializer),
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
                helper.resolve_element_local_name(QName(&type_name), &super::NS_O),
                Some(b"Circle")
            ) {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = <super::CircleType as WithDeserializer>::init(helper, event)?;
                return Ok(DeserializerOutput {
                    artifact: artifact.map(|x| super::Shape(Box::new(x)), |x| Self::Circle(x)),
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
            }
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::Shape, Error> {
            match self {
                Self::Circle(x) => Ok(super::Shape(Box::new(x.finish(helper)?))),
            }
        }
    }
    #[derive(Debug)]
    pub struct NodeTypeDeserializer {
        name: Option<String>,
        node: Option<super::NodeType>,
        state__: Box<NodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum NodeTypeDeserializerState {
        Init__,
        Name(Option<<String as WithDeserializer>::Deserializer>),
        Node(Option<<super::NodeType as WithDeserializer>::Deserializer>),
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
                node: None,
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
                S::Node(Some(deserializer)) => self.store_node(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_name(&mut self, value: String) -> Result<(), Error> {
            if self.name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"name")))?;
            }
            self.name = Some(value);
            Ok(())
        }
        fn store_node(&mut self, value: super::NodeType) -> Result<(), Error> {
            if self.node.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"node")))?;
            }
            self.node = Some(value);
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
                    *self.state__ = S::Node(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Name(Some(deserializer)));
                    *self.state__ = S::Node(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_node<'de>(
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
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_node(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Node(Some(deserializer)));
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
                    (S::Node(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_node(helper, output, &mut fallback)? {
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
                            Some(&super::NS_UNNAMED_5),
                            b"name",
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
                    (S::Node(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_5),
                            b"node",
                            false,
                        )?;
                        match self.handle_node(helper, output, &mut fallback)? {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::NodeType, Error> {
            let state = replace(&mut *self.state__, NodeTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::NodeType {
                name: helper.finish_element("name", self.name)?,
                node: self.node.map(Box::new),
            })
        }
    }
    #[derive(Debug)]
    pub struct CircleTypeDeserializer {
        radius: Option<i32>,
        state__: Box<CircleTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CircleTypeDeserializerState {
        Init__,
        Radius(Option<<i32 as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl CircleTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                radius: None,
                state__: Box::new(CircleTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: CircleTypeDeserializerState,
        ) -> Result<(), Error> {
            use CircleTypeDeserializerState as S;
            match state {
                S::Radius(Some(deserializer)) => self.store_radius(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_radius(&mut self, value: i32) -> Result<(), Error> {
            if self.radius.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"radius",
                )))?;
            }
            self.radius = Some(value);
            Ok(())
        }
        fn handle_radius<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<CircleTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CircleTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Radius(None));
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
                    self.store_radius(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Radius(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::CircleType> for CircleTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CircleType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CircleType> {
            use CircleTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Radius(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_radius(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Radius(None);
                        event
                    }
                    (S::Radius(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_O),
                            b"radius",
                            false,
                        )?;
                        match self.handle_radius(helper, output, &mut fallback)? {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::CircleType, Error> {
            let state = replace(&mut *self.state__, CircleTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::CircleType {
                radius: helper.finish_element("radius", self.radius)?,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::{
        quick_xml::{
            BytesEnd, BytesStart, CollectNamespaces, ElementNamespace, Error, Event,
            IterSerializer, SerializeHelper, Serializer, WithSerializer,
        },
        xml::{AnyElement, AnySimpleType, Nillable},
    };
    #[derive(Debug)]
    pub struct FooTypeSerializer<'ser> {
        pub(super) value: &'ser super::FooType,
        pub(super) state: Box<FooTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum FooTypeSerializerState<'ser> {
        Init__,
        Child(IterSerializer<'ser, Option<&'ser super::ChildType>, super::ChildType>),
        Item(IterSerializer<'ser, &'ser [String], String>),
        Ref(<String as WithSerializer>::Serializer<'ser>),
        Shape(<super::Shape as WithSerializer>::Serializer<'ser>),
        N(<Nillable<String> as WithSerializer>::Serializer<'ser>),
        AnyValue(<AnySimpleType as WithSerializer>::Serializer<'ser>),
        Node(<super::NodeType as WithSerializer>::Serializer<'ser>),
        Any(<AnyElement as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FooTypeSerializerState::Init__ => {
                        *self.state = FooTypeSerializerState::Child(
                            IterSerializer::new(self.value.child.as_ref(), Some("child"), false)
                                .with_namespace(ElementNamespace::None),
                        );
                        let namespace = if self.is_root {
                            ElementNamespace::Namespace(super::NS_UNNAMED_5)
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
                            CollectNamespaces::collect_namespaces(self.value, helper, &mut bytes);
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    FooTypeSerializerState::Child(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = FooTypeSerializerState::Item(
                                IterSerializer::new(&self.value.item[..], Some("item"), false)
                                    .with_namespace(ElementNamespace::None),
                            )
                        }
                    },
                    FooTypeSerializerState::Item(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = FooTypeSerializerState::Ref(
                                WithSerializer::serializer_with_namespace(
                                    &self.value.ref_,
                                    Some("Ref"),
                                    ElementNamespace::Namespace(super::NS_UNNAMED_5),
                                    false,
                                )?,
                            )
                        }
                    },
                    FooTypeSerializerState::Ref(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = FooTypeSerializerState::Shape(
                                WithSerializer::serializer_with_namespace(
                                    &self.value.shape,
                                    Some("Shape"),
                                    ElementNamespace::Namespace(super::NS_UNNAMED_5),
                                    false,
                                )?,
                            )
                        }
                    },
                    FooTypeSerializerState::Shape(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = FooTypeSerializerState::N(
                                WithSerializer::serializer_with_namespace(
                                    &self.value.n,
                                    Some("n"),
                                    ElementNamespace::None,
                                    false,
                                )?,
                            )
                        }
                    },
                    FooTypeSerializerState::N(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = FooTypeSerializerState::AnyValue(
                                WithSerializer::serializer_with_namespace(
                                    &self.value.any_value,
                                    Some("anyValue"),
                                    ElementNamespace::None,
                                    false,
                                )?,
                            )
                        }
                    },
                    FooTypeSerializerState::AnyValue(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = FooTypeSerializerState::Node(
                                WithSerializer::serializer_with_namespace(
                                    &self.value.node,
                                    Some("node"),
                                    ElementNamespace::None,
                                    false,
                                )?,
                            )
                        }
                    },
                    FooTypeSerializerState::Node(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = FooTypeSerializerState::Any(
                                WithSerializer::serializer_with_namespace(
                                    &self.value.any,
                                    None,
                                    ElementNamespace::Unknown,
                                    false,
                                )?,
                            )
                        }
                    },
                    FooTypeSerializerState::Any(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooTypeSerializerState::End__,
                    },
                    FooTypeSerializerState::End__ => {
                        *self.state = FooTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    FooTypeSerializerState::Done__ => return Ok(None),
                    FooTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for FooTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
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
        Value(<String as WithSerializer>::Serializer<'ser>),
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
                        *self.state = ChildTypeSerializerState::Value(
                            WithSerializer::serializer_with_namespace(
                                &self.value.value,
                                Some("value"),
                                ElementNamespace::None,
                                false,
                            )?,
                        );
                        let namespace = if self.is_root {
                            ElementNamespace::Namespace(super::NS_UNNAMED_5)
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
                            CollectNamespaces::collect_namespaces(self.value, helper, &mut bytes);
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ChildTypeSerializerState::Value(x) => match x.next(helper).transpose()? {
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
        Node(IterSerializer<'ser, Option<&'ser super::NodeType>, super::NodeType>),
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
                                Some("name"),
                                ElementNamespace::None,
                                false,
                            )?,
                        );
                        let namespace = if self.is_root {
                            ElementNamespace::Namespace(super::NS_UNNAMED_5)
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
                            CollectNamespaces::collect_namespaces(self.value, helper, &mut bytes);
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    NodeTypeSerializerState::Name(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = NodeTypeSerializerState::Node(
                                IterSerializer::new(
                                    self.value.node.as_deref(),
                                    Some("node"),
                                    false,
                                )
                                .with_namespace(ElementNamespace::None),
                            )
                        }
                    },
                    NodeTypeSerializerState::Node(x) => match x.next(helper).transpose()? {
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
    pub struct CircleTypeSerializer<'ser> {
        pub(super) value: &'ser super::CircleType,
        pub(super) state: Box<CircleTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum CircleTypeSerializerState<'ser> {
        Init__,
        Radius(<i32 as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CircleTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CircleTypeSerializerState::Init__ => {
                        *self.state = CircleTypeSerializerState::Radius(
                            WithSerializer::serializer_with_namespace(
                                &self.value.radius,
                                Some("radius"),
                                ElementNamespace::None,
                                false,
                            )?,
                        );
                        let namespace = if self.is_root {
                            ElementNamespace::Namespace(super::NS_O)
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
                            CollectNamespaces::collect_namespaces(self.value, helper, &mut bytes);
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    CircleTypeSerializerState::Radius(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CircleTypeSerializerState::End__,
                    },
                    CircleTypeSerializerState::End__ => {
                        *self.state = CircleTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    CircleTypeSerializerState::Done__ => return Ok(None),
                    CircleTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for CircleTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CircleTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
