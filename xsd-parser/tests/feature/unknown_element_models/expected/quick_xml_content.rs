use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{ElementNamespace, Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub type Seq = SeqType;
#[derive(Debug)]
pub struct SeqType {
    pub a: String,
    pub child: ChildType,
    pub b: Option<String>,
}
impl WithSerializer for SeqType {
    type Serializer<'x> = quick_xml_serialize::SeqTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::SeqTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SeqTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Seq"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for SeqType {
    type Deserializer = quick_xml_deserialize::SeqTypeDeserializer;
}
#[derive(Debug)]
pub struct ChildType {
    pub inner: String,
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
pub type Many = ManyType;
#[derive(Debug)]
pub struct ManyType {
    pub content: Vec<ManyTypeContent>,
}
#[derive(Debug)]
pub enum ManyTypeContent {
    X(String),
    Y(String),
}
impl WithSerializer for ManyType {
    type Serializer<'x> = quick_xml_serialize::ManyTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::ManyTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ManyTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Many"),
            namespace,
            is_root,
        })
    }
}
impl WithSerializer for ManyTypeContent {
    type Serializer<'x> = quick_xml_serialize::ManyTypeContentSerializer<'x>;
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
        Ok(quick_xml_serialize::ManyTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ManyTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for ManyType {
    type Deserializer = quick_xml_deserialize::ManyTypeDeserializer;
}
impl WithDeserializer for ManyTypeContent {
    type Deserializer = quick_xml_deserialize::ManyTypeContentDeserializer;
}
pub type GroupPick = GroupPickType;
#[derive(Debug)]
pub struct GroupPickType {
    pub content: GroupPickTypeContent,
}
#[derive(Debug)]
pub enum GroupPickTypeContent {
    Pq(GroupPickPqType),
    Y(String),
}
impl WithSerializer for GroupPickType {
    type Serializer<'x> = quick_xml_serialize::GroupPickTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::GroupPickTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::GroupPickTypeSerializerState::Init__),
            name: name.unwrap_or("tns:GroupPick"),
            namespace,
            is_root,
        })
    }
}
impl WithSerializer for GroupPickTypeContent {
    type Serializer<'x> = quick_xml_serialize::GroupPickTypeContentSerializer<'x>;
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
        Ok(quick_xml_serialize::GroupPickTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::GroupPickTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for GroupPickType {
    type Deserializer = quick_xml_deserialize::GroupPickTypeDeserializer;
}
impl WithDeserializer for GroupPickTypeContent {
    type Deserializer = quick_xml_deserialize::GroupPickTypeContentDeserializer;
}
#[derive(Debug)]
pub struct GroupPickPqType {
    pub p: String,
    pub q: String,
}
impl WithSerializer for GroupPickPqType {
    type Serializer<'x> = quick_xml_serialize::GroupPickPqTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::GroupPickPqTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::GroupPickPqTypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for GroupPickPqType {
    type Deserializer = quick_xml_deserialize::GroupPickPqTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct SeqTypeDeserializer {
        a: Option<String>,
        child: Option<super::ChildType>,
        b: Option<String>,
        state__: Box<SeqTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SeqTypeDeserializerState {
        Init__,
        A(Option<<String as WithDeserializer>::Deserializer>),
        Child(Option<<super::ChildType as WithDeserializer>::Deserializer>),
        B(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SeqTypeDeserializer {
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
                child: None,
                b: None,
                state__: Box::new(SeqTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SeqTypeDeserializerState,
        ) -> Result<(), Error> {
            use SeqTypeDeserializerState as S;
            match state {
                S::A(Some(deserializer)) => self.store_a(deserializer.finish(helper)?)?,
                S::Child(Some(deserializer)) => self.store_child(deserializer.finish(helper)?)?,
                S::B(Some(deserializer)) => self.store_b(deserializer.finish(helper)?)?,
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
        fn handle_a<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<SeqTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SeqTypeDeserializerState as S;
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
                    *self.state__ = S::Child(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::A(Some(deserializer)));
                    *self.state__ = S::Child(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_child<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ChildType>,
            fallback: &mut Option<SeqTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SeqTypeDeserializerState as S;
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
            fallback: &mut Option<SeqTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SeqTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::SeqType> for SeqTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SeqType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SeqType> {
            use SeqTypeDeserializerState as S;
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
                helper.decide_unknown_element::<super::SeqType>(event, allow_any)?;
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::SeqType, Error> {
            let state = replace(&mut *self.state__, SeqTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::SeqType {
                a: helper.finish_element("A", self.a)?,
                child: helper.finish_element("Child", self.child)?,
                b: self.b,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"A" | b"Child" | b"B")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct ChildTypeDeserializer {
        inner: Option<String>,
        state__: Box<ChildTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ChildTypeDeserializerState {
        Init__,
        Inner(Option<<String as WithDeserializer>::Deserializer>),
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
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Inner(Some(deserializer)));
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
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Inner")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct ManyTypeDeserializer {
        content: Vec<super::ManyTypeContent>,
        state__: Box<ManyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ManyTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::ManyTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ManyTypeDeserializer {
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
                state__: Box::new(ManyTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ManyTypeDeserializerState,
        ) -> Result<(), Error> {
            if let ManyTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ManyTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ManyTypeContent>,
            fallback: &mut Option<ManyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ManyTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::ManyType> for ManyTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ManyType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ManyType> {
            use ManyTypeDeserializerState as S;
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
                            <super::ManyTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            let (event, allow_any) =
                helper.decide_unknown_element::<super::ManyType>(event, allow_any)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ManyType, Error> {
            let state = replace(&mut *self.state__, ManyTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::ManyType {
                content: helper.finish_vec(1usize, None, self.content)?,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if <super::ManyTypeContent as WithDeserializer>::Deserializer::is_known_start_tag(
                helper, x,
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct ManyTypeContentDeserializer {
        state__: Box<ManyTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ManyTypeContentDeserializerState {
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
        Done__(super::ManyTypeContent),
        Unknown__,
    }
    impl ManyTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"X")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_x(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Y")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_y(helper, Default::default(), None, output);
                }
            }
            *self.state__ = ManyTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: ManyTypeContentDeserializerState,
        ) -> Result<super::ManyTypeContent, Error> {
            use ManyTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::X(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_x(&mut values, value)?;
                    }
                    Ok(super::ManyTypeContent::X(
                        helper.finish_element("X", values)?,
                    ))
                }
                S::Y(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_y(&mut values, value)?;
                    }
                    Ok(super::ManyTypeContent::Y(
                        helper.finish_element("Y", values)?,
                    ))
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
            use ManyTypeContentDeserializerState as S;
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
                    let data = Self::finish_state(helper, S::X(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::X(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
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
            use ManyTypeContentDeserializerState as S;
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
                    let data = Self::finish_state(helper, S::Y(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Y(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ManyTypeContent> for ManyTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ManyTypeContent> {
            let deserializer = Self {
                state__: Box::new(ManyTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, ManyTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::ManyTypeContent> {
            use ManyTypeContentDeserializerState as S;
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::ManyTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"X" | b"Y")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct GroupPickTypeDeserializer {
        content: Option<super::GroupPickTypeContent>,
        state__: Box<GroupPickTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum GroupPickTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::GroupPickTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl GroupPickTypeDeserializer {
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
                state__: Box::new(GroupPickTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: GroupPickTypeDeserializerState,
        ) -> Result<(), Error> {
            if let GroupPickTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::GroupPickTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::GroupPickTypeContent>,
            fallback: &mut Option<GroupPickTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use GroupPickTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::GroupPickType> for GroupPickTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GroupPickType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GroupPickType> {
            use GroupPickTypeDeserializerState as S;
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
                            <super::GroupPickTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            let (event, allow_any) =
                helper.decide_unknown_element::<super::GroupPickType>(event, allow_any)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::GroupPickType, Error> {
            let state = replace(
                &mut *self.state__,
                GroupPickTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::GroupPickType {
                content: helper.finish_content(self.content)?,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if <super::GroupPickTypeContent as WithDeserializer>::Deserializer::is_known_start_tag(
                helper, x,
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct GroupPickTypeContentDeserializer {
        state__: Box<GroupPickTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum GroupPickTypeContentDeserializerState {
        Init__,
        Pq(
            Option<super::GroupPickPqType>,
            Option<<super::GroupPickPqType as WithDeserializer>::Deserializer>,
            Option<<super::GroupPickPqType as WithDeserializer>::Deserializer>,
        ),
        Y(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::GroupPickTypeContent),
        Unknown__,
    }
    impl GroupPickTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let mut event = event;
            let mut allow_any_element = false;
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Y")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_y(helper, Default::default(), None, output);
                }
                event = {
                    let output = <super::GroupPickPqType as WithDeserializer>::init(helper, event)?;
                    match self.handle_pq(helper, Default::default(), None, output)? {
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
            *self.state__ = GroupPickTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: GroupPickTypeContentDeserializerState,
        ) -> Result<super::GroupPickTypeContent, Error> {
            use GroupPickTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Pq(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_pq(&mut values, value)?;
                    }
                    Ok(super::GroupPickTypeContent::Pq(
                        helper.finish_element("PQ", values)?,
                    ))
                }
                S::Y(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_y(&mut values, value)?;
                    }
                    Ok(super::GroupPickTypeContent::Y(
                        helper.finish_element("Y", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_pq(
            values: &mut Option<super::GroupPickPqType>,
            value: super::GroupPickPqType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"PQ")))?;
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
        fn handle_pq<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupPickPqType>,
            fallback: Option<<super::GroupPickPqType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupPickPqType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use GroupPickTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let fresh = values.is_none() && fallback.is_none();
                *self.state__ = S::Pq(values, None, fallback);
                if !fresh {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_pq(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pq(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Pq(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Pq(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
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
            use GroupPickTypeContentDeserializerState as S;
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
                    let data = Self::finish_state(helper, S::Y(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Y(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::GroupPickTypeContent> for GroupPickTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GroupPickTypeContent> {
            let deserializer = Self {
                state__: Box::new(GroupPickTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, GroupPickTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::GroupPickTypeContent> {
            use GroupPickTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Pq(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_pq(helper, values, fallback, output)? {
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
                        S::Pq(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            <super::GroupPickPqType as WithDeserializer>::init(helper, event)?;
                        match self.handle_pq(helper, values, fallback, output)? {
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
        ) -> Result<super::GroupPickTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Y")
            ) {
                return true;
            }
            if <super::GroupPickPqType as WithDeserializer>::Deserializer::is_known_start_tag(
                helper, x,
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct GroupPickPqTypeDeserializer {
        p: Option<String>,
        q: Option<String>,
        state__: Box<GroupPickPqTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum GroupPickPqTypeDeserializerState {
        Init__,
        P(Option<<String as WithDeserializer>::Deserializer>),
        Q(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl GroupPickPqTypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: GroupPickPqTypeDeserializerState,
        ) -> Result<(), Error> {
            use GroupPickPqTypeDeserializerState as S;
            match state {
                S::P(Some(deserializer)) => self.store_p(deserializer.finish(helper)?)?,
                S::Q(Some(deserializer)) => self.store_q(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_p(&mut self, value: String) -> Result<(), Error> {
            if self.p.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"P")))?;
            }
            self.p = Some(value);
            Ok(())
        }
        fn store_q(&mut self, value: String) -> Result<(), Error> {
            if self.q.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Q")))?;
            }
            self.q = Some(value);
            Ok(())
        }
        fn handle_p<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<GroupPickPqTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use GroupPickPqTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::P(None));
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
                    self.store_p(data)?;
                    *self.state__ = S::Q(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::P(Some(deserializer)));
                    *self.state__ = S::Q(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_q<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<GroupPickPqTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use GroupPickPqTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Q(None));
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
                    self.store_q(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Q(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::GroupPickPqType> for GroupPickPqTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GroupPickPqType> {
            let deserializer = Self {
                p: None,
                q: None,
                state__: Box::new(GroupPickPqTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, GroupPickPqTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::GroupPickPqType> {
            use GroupPickPqTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::P(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_p(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Q(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_q(helper, output, &mut fallback)? {
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
                        *self.state__ = S::P(None);
                        event
                    }
                    (S::P(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"P",
                            false,
                        )?;
                        match self.handle_p(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Q(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Q",
                            false,
                        )?;
                        match self.handle_q(helper, output, &mut fallback)? {
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
        ) -> Result<super::GroupPickPqType, Error> {
            let state = replace(
                &mut *self.state__,
                GroupPickPqTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::GroupPickPqType {
                p: helper.finish_element("P", self.p)?,
                q: helper.finish_element("Q", self.q)?,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"P" | b"Q")
            ) {
                return true;
            }
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
    pub struct SeqTypeSerializer<'ser> {
        pub(super) value: &'ser super::SeqType,
        pub(super) state: Box<SeqTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SeqTypeSerializerState<'ser> {
        Init__,
        A(<String as WithSerializer>::Serializer<'ser>),
        Child(<super::ChildType as WithSerializer>::Serializer<'ser>),
        B(IterSerializer<'ser, Option<&'ser String>, String>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SeqTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SeqTypeSerializerState::Init__ => {
                        *self.state =
                            SeqTypeSerializerState::A(WithSerializer::serializer_with_namespace(
                                &self.value.a,
                                Some("tns:A"),
                                ElementNamespace::Namespace(super::NS_TNS),
                                false,
                            )?);
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns_for_element(&mut bytes, self.name, &self.namespace);
                        if self.is_root {
                            helper.write_xmlns_for_tag(&mut bytes, self.name, &super::NS_TNS);
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SeqTypeSerializerState::A(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = SeqTypeSerializerState::Child(
                                WithSerializer::serializer_with_namespace(
                                    &self.value.child,
                                    Some("tns:Child"),
                                    ElementNamespace::Namespace(super::NS_TNS),
                                    false,
                                )?,
                            )
                        }
                    },
                    SeqTypeSerializerState::Child(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = SeqTypeSerializerState::B(
                                IterSerializer::new(self.value.b.as_ref(), Some("tns:B"), false)
                                    .with_namespace(ElementNamespace::Namespace(super::NS_TNS)),
                            )
                        }
                    },
                    SeqTypeSerializerState::B(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = SeqTypeSerializerState::End__,
                    },
                    SeqTypeSerializerState::End__ => {
                        *self.state = SeqTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SeqTypeSerializerState::Done__ => return Ok(None),
                    SeqTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for SeqTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SeqTypeSerializerState::Done__;
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
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns_for_element(&mut bytes, self.name, &self.namespace);
                        if self.is_root {
                            helper.write_xmlns_for_tag(&mut bytes, self.name, &super::NS_TNS);
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
    pub struct ManyTypeSerializer<'ser> {
        pub(super) value: &'ser super::ManyType,
        pub(super) state: Box<ManyTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ManyTypeSerializerState<'ser> {
        Init__,
        Content__(IterSerializer<'ser, &'ser [super::ManyTypeContent], super::ManyTypeContent>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ManyTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ManyTypeSerializerState::Init__ => {
                        *self.state = ManyTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns_for_element(&mut bytes, self.name, &self.namespace);
                        if self.is_root {
                            helper.write_xmlns_for_tag(&mut bytes, self.name, &super::NS_TNS);
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ManyTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ManyTypeSerializerState::End__,
                    },
                    ManyTypeSerializerState::End__ => {
                        *self.state = ManyTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ManyTypeSerializerState::Done__ => return Ok(None),
                    ManyTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ManyTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ManyTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ManyTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::ManyTypeContent,
        pub(super) state: Box<ManyTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum ManyTypeContentSerializerState<'ser> {
        Init__,
        X(<String as WithSerializer>::Serializer<'ser>),
        Y(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ManyTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ManyTypeContentSerializerState::Init__ => match self.value {
                        super::ManyTypeContent::X(x) => {
                            *self.state = ManyTypeContentSerializerState::X(
                                WithSerializer::serializer_with_namespace(
                                    x,
                                    Some("tns:X"),
                                    ElementNamespace::Namespace(super::NS_TNS),
                                    false,
                                )?,
                            )
                        }
                        super::ManyTypeContent::Y(x) => {
                            *self.state = ManyTypeContentSerializerState::Y(
                                WithSerializer::serializer_with_namespace(
                                    x,
                                    Some("tns:Y"),
                                    ElementNamespace::Namespace(super::NS_TNS),
                                    false,
                                )?,
                            )
                        }
                    },
                    ManyTypeContentSerializerState::X(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ManyTypeContentSerializerState::Done__,
                    },
                    ManyTypeContentSerializerState::Y(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ManyTypeContentSerializerState::Done__,
                    },
                    ManyTypeContentSerializerState::Done__ => return Ok(None),
                    ManyTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ManyTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ManyTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct GroupPickTypeSerializer<'ser> {
        pub(super) value: &'ser super::GroupPickType,
        pub(super) state: Box<GroupPickTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum GroupPickTypeSerializerState<'ser> {
        Init__,
        Content__(<super::GroupPickTypeContent as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> GroupPickTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    GroupPickTypeSerializerState::Init__ => {
                        *self.state = GroupPickTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns_for_element(&mut bytes, self.name, &self.namespace);
                        if self.is_root {
                            helper.write_xmlns_for_tag(&mut bytes, self.name, &super::NS_TNS);
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    GroupPickTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = GroupPickTypeSerializerState::End__,
                        }
                    }
                    GroupPickTypeSerializerState::End__ => {
                        *self.state = GroupPickTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    GroupPickTypeSerializerState::Done__ => return Ok(None),
                    GroupPickTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for GroupPickTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = GroupPickTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct GroupPickTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::GroupPickTypeContent,
        pub(super) state: Box<GroupPickTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum GroupPickTypeContentSerializerState<'ser> {
        Init__,
        Pq(<super::GroupPickPqType as WithSerializer>::Serializer<'ser>),
        Y(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> GroupPickTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    GroupPickTypeContentSerializerState::Init__ => match self.value {
                        super::GroupPickTypeContent::Pq(x) => {
                            *self.state = GroupPickTypeContentSerializerState::Pq(
                                WithSerializer::serializer_with_namespace(
                                    x,
                                    Some("PQ"),
                                    ElementNamespace::None,
                                    false,
                                )?,
                            )
                        }
                        super::GroupPickTypeContent::Y(x) => {
                            *self.state = GroupPickTypeContentSerializerState::Y(
                                WithSerializer::serializer_with_namespace(
                                    x,
                                    Some("tns:Y"),
                                    ElementNamespace::Namespace(super::NS_TNS),
                                    false,
                                )?,
                            )
                        }
                    },
                    GroupPickTypeContentSerializerState::Pq(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = GroupPickTypeContentSerializerState::Done__,
                        }
                    }
                    GroupPickTypeContentSerializerState::Y(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = GroupPickTypeContentSerializerState::Done__,
                        }
                    }
                    GroupPickTypeContentSerializerState::Done__ => return Ok(None),
                    GroupPickTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for GroupPickTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = GroupPickTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct GroupPickPqTypeSerializer<'ser> {
        pub(super) value: &'ser super::GroupPickPqType,
        pub(super) state: Box<GroupPickPqTypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum GroupPickPqTypeSerializerState<'ser> {
        Init__,
        P(<String as WithSerializer>::Serializer<'ser>),
        Q(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> GroupPickPqTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    GroupPickPqTypeSerializerState::Init__ => {
                        *self.state = GroupPickPqTypeSerializerState::P(
                            WithSerializer::serializer_with_namespace(
                                &self.value.p,
                                Some("tns:P"),
                                ElementNamespace::Namespace(super::NS_TNS),
                                false,
                            )?,
                        );
                    }
                    GroupPickPqTypeSerializerState::P(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = GroupPickPqTypeSerializerState::Q(
                                WithSerializer::serializer_with_namespace(
                                    &self.value.q,
                                    Some("tns:Q"),
                                    ElementNamespace::Namespace(super::NS_TNS),
                                    false,
                                )?,
                            )
                        }
                    },
                    GroupPickPqTypeSerializerState::Q(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = GroupPickPqTypeSerializerState::Done__,
                    },
                    GroupPickPqTypeSerializerState::Done__ => return Ok(None),
                    GroupPickPqTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for GroupPickPqTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = GroupPickPqTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
