use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{Error, WithDeserializer, WithSerializer},
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
        Ok(quick_xml_serialize::RootTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::RootTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Root"),
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
        Ok(quick_xml_serialize::MaxTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::MaxTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Max"),
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
        Ok(quick_xml_serialize::OuterTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::OuterTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Outer"),
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
        Ok(quick_xml_serialize::ChildTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ChildTypeSerializerState::Init__),
            name: name.unwrap_or("tns:ChildType"),
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
        Ok(quick_xml_serialize::NodeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::NodeTypeSerializerState::Init__),
            name: name.unwrap_or("tns:NodeType"),
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
        Ok(quick_xml_serialize::GRootTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::GRootTypeSerializerState::Init__),
            name: name.unwrap_or("tns:GRoot"),
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
        let _name = name;
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
        Ok(quick_xml_serialize::ViaTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ViaTypeSerializerState::Init__),
            name: name.unwrap_or("tns:ViaType"),
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
        let _name = name;
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
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        RawByteStr, WithDeserializer,
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
            let entry_state__ = match &*self.state__ {
                S::Init__ => Some(S::Init__),
                S::A(None) => Some(S::A(None)),
                S::Pos(None) => Some(S::Pos(None)),
                _ => None,
            };
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
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            } else if !matches!(event, DeserializerEvent::None) {
                if let Some(entry_state) = entry_state__ {
                    *self.state__ = entry_state;
                }
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
            let _ = helper;
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"A")
            ) {
                return true;
            }
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Pos")
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
            let entry_state__ = match &*self.state__ {
                S::Init__ => Some(S::Init__),
                S::A(None) => Some(S::A(None)),
                _ => None,
            };
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
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            } else if !matches!(event, DeserializerEvent::None) {
                if let Some(entry_state) = entry_state__ {
                    *self.state__ = entry_state;
                }
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
            let _ = helper;
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
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
            let entry_state__ = match &*self.state__ {
                S::Init__ => Some(S::Init__),
                S::Child(None) => Some(S::Child(None)),
                S::B(None) => Some(S::B(None)),
                _ => None,
            };
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
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            } else if !matches!(event, DeserializerEvent::None) {
                if let Some(entry_state) = entry_state__ {
                    *self.state__ = entry_state;
                }
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
            let _ = helper;
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Child")
            ) {
                return true;
            }
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"B")
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
            let entry_state__ = match &*self.state__ {
                S::Init__ => Some(S::Init__),
                S::Inner(None) => Some(S::Inner(None)),
                S::Extra(None) => Some(S::Extra(None)),
                _ => None,
            };
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
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            } else if !matches!(event, DeserializerEvent::None) {
                if let Some(entry_state) = entry_state__ {
                    *self.state__ = entry_state;
                }
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
            let _ = helper;
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Inner")
            ) {
                return true;
            }
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Extra")
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
            let entry_state__ = match &*self.state__ {
                S::Init__ => Some(S::Init__),
                S::Name(None) => Some(S::Name(None)),
                S::Kid(None) => Some(S::Kid(None)),
                _ => None,
            };
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
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            } else if !matches!(event, DeserializerEvent::None) {
                if let Some(entry_state) = entry_state__ {
                    *self.state__ = entry_state;
                }
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
            let _ = helper;
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Name")
            ) {
                return true;
            }
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Kid")
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
            let entry_state__ = match &*self.state__ {
                S::Init__ => Some(S::Init__),
                S::RecGroup(None) => Some(S::RecGroup(None)),
                _ => None,
            };
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
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            } else if !matches!(event, DeserializerEvent::None) {
                if let Some(entry_state) = entry_state__ {
                    *self.state__ = entry_state;
                }
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
            let _ = helper;
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
            let entry_state__ = match &*self.state__ {
                S::Init__ => Some(S::Init__),
                S::V(None) => Some(S::V(None)),
                S::Via(None) => Some(S::Via(None)),
                _ => None,
            };
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
            } else if !matches!(event, DeserializerEvent::None) {
                if let Some(entry_state) = entry_state__ {
                    *self.state__ = entry_state;
                }
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
            let _ = helper;
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"V")
            ) {
                return true;
            }
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Via")
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
            let entry_state__ = match &*self.state__ {
                S::Init__ => Some(S::Init__),
                S::RecGroup(None) => Some(S::RecGroup(None)),
                _ => None,
            };
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
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            } else if !matches!(event, DeserializerEvent::None) {
                if let Some(entry_state) = entry_state__ {
                    *self.state__ = entry_state;
                }
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
            let _ = helper;
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
            let entry_state__ = match &*self.state__ {
                S::Init__ => Some(S::Init__),
                S::V(None) => Some(S::V(None)),
                S::Via(None) => Some(S::Via(None)),
                _ => None,
            };
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
            } else if !matches!(event, DeserializerEvent::None) {
                if let Some(entry_state) = entry_state__ {
                    *self.state__ = entry_state;
                }
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
            let _ = helper;
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"V")
            ) {
                return true;
            }
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Via")
            ) {
                return true;
            }
            false
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, Error, Event, IterSerializer, SerializeHelper, Serializer,
        WithSerializer,
    };
    #[derive(Debug)]
    pub struct RootTypeSerializer<'ser> {
        pub(super) value: &'ser super::RootType,
        pub(super) state: Box<RootTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
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
                        *self.state = RootTypeSerializerState::A(WithSerializer::serializer(
                            &self.value.a,
                            Some("tns:A"),
                            false,
                        )?);
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
                    RootTypeSerializerState::A(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = RootTypeSerializerState::Pos(IterSerializer::new(
                                self.value.pos.as_ref(),
                                Some("tns:Pos"),
                                false,
                            ))
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
                        *self.state = MaxTypeSerializerState::A(IterSerializer::new(
                            &self.value.a[..],
                            Some("tns:A"),
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
                        *self.state = OuterTypeSerializerState::Child(WithSerializer::serializer(
                            &self.value.child,
                            Some("tns:Child"),
                            false,
                        )?);
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
                    OuterTypeSerializerState::Child(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = OuterTypeSerializerState::B(IterSerializer::new(
                                self.value.b.as_ref(),
                                Some("tns:B"),
                                false,
                            ))
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
                        *self.state = ChildTypeSerializerState::Inner(WithSerializer::serializer(
                            &self.value.inner,
                            Some("tns:Inner"),
                            false,
                        )?);
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
                    ChildTypeSerializerState::Inner(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = ChildTypeSerializerState::Extra(IterSerializer::new(
                                self.value.extra.as_ref(),
                                Some("tns:Extra"),
                                false,
                            ))
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
                        *self.state = NodeTypeSerializerState::Name(WithSerializer::serializer(
                            &self.value.name,
                            Some("tns:Name"),
                            false,
                        )?);
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
                    NodeTypeSerializerState::Name(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = NodeTypeSerializerState::Kid(IterSerializer::new(
                                self.value.kid.as_deref(),
                                Some("tns:Kid"),
                                false,
                            ))
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
                        *self.state =
                            GRootTypeSerializerState::RecGroup(WithSerializer::serializer(
                                &self.value.rec_group,
                                Some("RecGroup"),
                                false,
                            )?);
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
                            WithSerializer::serializer(&self.value.v, Some("tns:V"), false)?,
                        );
                    }
                    GRootRecGroupTypeSerializerState::V(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                GRootRecGroupTypeSerializerState::Via(IterSerializer::new(
                                    self.value.via.as_ref(),
                                    Some("tns:Via"),
                                    false,
                                ))
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
                        *self.state = ViaTypeSerializerState::RecGroup(WithSerializer::serializer(
                            &self.value.rec_group,
                            Some("RecGroup"),
                            false,
                        )?);
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
                            WithSerializer::serializer(&self.value.v, Some("tns:V"), false)?,
                        );
                    }
                    ViaRecGroupTypeSerializerState::V(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = ViaRecGroupTypeSerializerState::Via(IterSerializer::new(
                                self.value.via.as_deref(),
                                Some("tns:Via"),
                                false,
                            ))
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
}
