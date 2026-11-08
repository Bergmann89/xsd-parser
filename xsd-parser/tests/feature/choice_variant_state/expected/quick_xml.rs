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
pub type GroupPick = GroupPickType;
#[derive(Debug)]
pub enum GroupPickType {
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
impl WithDeserializer for GroupPickType {
    type Deserializer = quick_xml_deserialize::GroupPickTypeDeserializer;
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
pub type ChoiceGroupPick = ChoiceGroupPickType;
#[derive(Debug)]
pub enum ChoiceGroupPickType {
    Uv(ChoiceGroupPickUvType),
    Y(String),
}
impl WithSerializer for ChoiceGroupPickType {
    type Serializer<'x> = quick_xml_serialize::ChoiceGroupPickTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::ChoiceGroupPickTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ChoiceGroupPickTypeSerializerState::Init__),
            name: name.unwrap_or("tns:ChoiceGroupPick"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for ChoiceGroupPickType {
    type Deserializer = quick_xml_deserialize::ChoiceGroupPickTypeDeserializer;
}
#[derive(Debug)]
pub enum ChoiceGroupPickUvType {
    U(String),
    V(String),
}
impl WithSerializer for ChoiceGroupPickUvType {
    type Serializer<'x> = quick_xml_serialize::ChoiceGroupPickUvTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::ChoiceGroupPickUvTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ChoiceGroupPickUvTypeSerializerState::Init__),
            is_root,
        })
    }
}
impl WithDeserializer for ChoiceGroupPickUvType {
    type Deserializer = quick_xml_deserialize::ChoiceGroupPickUvTypeDeserializer;
}
pub type DynPick = DynPickType;
#[derive(Debug)]
pub enum DynPickType {
    Animal(AnimalType),
    Y(String),
}
impl WithSerializer for DynPickType {
    type Serializer<'x> = quick_xml_serialize::DynPickTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::DynPickTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DynPickTypeSerializerState::Init__),
            name: name.unwrap_or("tns:DynPick"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for DynPickType {
    type Deserializer = quick_xml_deserialize::DynPickTypeDeserializer;
}
#[derive(Debug)]
pub struct AnimalType(pub Box<dyn AnimalTypeTrait>);
pub trait AnimalTypeTrait: Debug + AsAny + WithBoxedSerializer {}
impl AnimalType {
    pub fn new<T: AnimalTypeTrait + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
impl WithSerializer for AnimalType {
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
        self.0.serializer_with_namespace(name, namespace, is_root)
    }
}
impl WithDeserializer for AnimalType {
    type Deserializer = quick_xml_deserialize::AnimalTypeDeserializer;
}
#[derive(Debug)]
pub struct AnimalDyn {
    pub id: i32,
}
impl AnimalTypeTrait for AnimalDyn {}
impl WithSerializer for AnimalDyn {
    type Serializer<'x> = quick_xml_serialize::AnimalDynSerializer<'x>;
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
        Ok(quick_xml_serialize::AnimalDynSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::AnimalDynSerializerState::Init__),
            name: name.unwrap_or("tns:animal"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for AnimalDyn {
    type Deserializer = quick_xml_deserialize::AnimalDynDeserializer;
}
pub type MixedPick = MixedPickType;
#[derive(Debug)]
pub enum MixedPickType {
    A(String),
    B(String),
}
impl WithSerializer for MixedPickType {
    type Serializer<'x> = quick_xml_serialize::MixedPickTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::MixedPickTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::MixedPickTypeSerializerState::Init__),
            name: name.unwrap_or("tns:MixedPick"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for MixedPickType {
    type Deserializer = quick_xml_deserialize::MixedPickTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use std::borrow::Cow;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        QName, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct GroupPickTypeDeserializer {
        state__: Box<GroupPickTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum GroupPickTypeDeserializerState {
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
        Done__(super::GroupPickType),
        Unknown__,
    }
    impl GroupPickTypeDeserializer {
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
            *self.state__ = GroupPickTypeDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
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
                state__: Box::new(GroupPickTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: GroupPickTypeDeserializerState,
        ) -> Result<super::GroupPickType, Error> {
            use GroupPickTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Pq(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_pq(&mut values, value)?;
                    }
                    Ok(super::GroupPickType::Pq(
                        helper.finish_element("PQ", values)?,
                    ))
                }
                S::Y(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_y(&mut values, value)?;
                    }
                    Ok(super::GroupPickType::Y(helper.finish_element("Y", values)?))
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
            use GroupPickTypeDeserializerState as S;
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
                    *self.state__ = S::Pq(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Pq(values, None, Some(deserializer));
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
            use GroupPickTypeDeserializerState as S;
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
            let event = event.into_break();
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::GroupPickType, Error> {
            Self::finish_state(helper, *self.state__)
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
    }
    #[derive(Debug)]
    pub struct ChoiceGroupPickTypeDeserializer {
        state__: Box<ChoiceGroupPickTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ChoiceGroupPickTypeDeserializerState {
        Init__,
        Uv(
            Option<super::ChoiceGroupPickUvType>,
            Option<<super::ChoiceGroupPickUvType as WithDeserializer>::Deserializer>,
            Option<<super::ChoiceGroupPickUvType as WithDeserializer>::Deserializer>,
        ),
        Y(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ChoiceGroupPickType),
        Unknown__,
    }
    impl ChoiceGroupPickTypeDeserializer {
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
                    let output =
                        <super::ChoiceGroupPickUvType as WithDeserializer>::init(helper, event)?;
                    match self.handle_uv(helper, Default::default(), None, output)? {
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
            *self.state__ = ChoiceGroupPickTypeDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
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
                state__: Box::new(ChoiceGroupPickTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: ChoiceGroupPickTypeDeserializerState,
        ) -> Result<super::ChoiceGroupPickType, Error> {
            use ChoiceGroupPickTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Uv(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_uv(&mut values, value)?;
                    }
                    Ok(super::ChoiceGroupPickType::Uv(
                        helper.finish_element("UV", values)?,
                    ))
                }
                S::Y(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_y(&mut values, value)?;
                    }
                    Ok(super::ChoiceGroupPickType::Y(
                        helper.finish_element("Y", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_uv(
            values: &mut Option<super::ChoiceGroupPickUvType>,
            value: super::ChoiceGroupPickUvType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"UV")))?;
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
        fn handle_uv<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ChoiceGroupPickUvType>,
            fallback: Option<<super::ChoiceGroupPickUvType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ChoiceGroupPickUvType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ChoiceGroupPickTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let fresh = values.is_none() && fallback.is_none();
                *self.state__ = S::Uv(values, None, fallback);
                if !fresh {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_uv(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_uv(&mut values, data)?;
                    *self.state__ = S::Uv(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Uv(values, None, Some(deserializer));
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
            use ChoiceGroupPickTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::ChoiceGroupPickType> for ChoiceGroupPickTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ChoiceGroupPickType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ChoiceGroupPickType> {
            use ChoiceGroupPickTypeDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Uv(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_uv(helper, values, fallback, output)? {
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
                    (
                        S::Uv(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = <super::ChoiceGroupPickUvType as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_uv(helper, values, fallback, output)? {
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
            let event = event.into_break();
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
        ) -> Result<super::ChoiceGroupPickType, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct ChoiceGroupPickUvTypeDeserializer {
        state__: Box<ChoiceGroupPickUvTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ChoiceGroupPickUvTypeDeserializerState {
        Init__,
        U(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        V(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ChoiceGroupPickUvType),
        Unknown__,
    }
    impl ChoiceGroupPickUvTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"U")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_u(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"V")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_v(helper, Default::default(), None, output);
                }
            }
            *self.state__ = ChoiceGroupPickUvTypeDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: ChoiceGroupPickUvTypeDeserializerState,
        ) -> Result<super::ChoiceGroupPickUvType, Error> {
            use ChoiceGroupPickUvTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::U(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_u(&mut values, value)?;
                    }
                    Ok(super::ChoiceGroupPickUvType::U(
                        helper.finish_element("U", values)?,
                    ))
                }
                S::V(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_v(&mut values, value)?;
                    }
                    Ok(super::ChoiceGroupPickUvType::V(
                        helper.finish_element("V", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_u(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"U")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_v(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"V")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_u<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ChoiceGroupPickUvTypeDeserializerState as S;
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
                Self::store_u(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_u(&mut values, data)?;
                    let data = Self::finish_state(helper, S::U(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::U(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_v<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ChoiceGroupPickUvTypeDeserializerState as S;
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
                Self::store_v(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_v(&mut values, data)?;
                    let data = Self::finish_state(helper, S::V(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::V(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ChoiceGroupPickUvType> for ChoiceGroupPickUvTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ChoiceGroupPickUvType> {
            let deserializer = Self {
                state__: Box::new(ChoiceGroupPickUvTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, ChoiceGroupPickUvTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::ChoiceGroupPickUvType> {
            use ChoiceGroupPickUvTypeDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::U(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_u(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::V(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_v(helper, values, fallback, output)? {
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
                    (S::U(values, fallback, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"U",
                            false,
                        )?;
                        match self.handle_u(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::V(values, fallback, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"V",
                            false,
                        )?;
                        match self.handle_v(helper, values, fallback, output)? {
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
        ) -> Result<super::ChoiceGroupPickUvType, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct DynPickTypeDeserializer {
        state__: Box<DynPickTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum DynPickTypeDeserializerState {
        Init__,
        Animal(
            Option<super::AnimalType>,
            Option<<super::AnimalType as WithDeserializer>::Deserializer>,
            Option<<super::AnimalType as WithDeserializer>::Deserializer>,
        ),
        Y(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::DynPickType),
        Unknown__,
    }
    impl DynPickTypeDeserializer {
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
                    let output = <super::AnimalType as WithDeserializer>::init(helper, event)?;
                    match self.handle_animal(helper, Default::default(), None, output)? {
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
            *self.state__ = DynPickTypeDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
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
                state__: Box::new(DynPickTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: DynPickTypeDeserializerState,
        ) -> Result<super::DynPickType, Error> {
            use DynPickTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Animal(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_animal(&mut values, value)?;
                    }
                    Ok(super::DynPickType::Animal(
                        helper.finish_element("animal", values)?,
                    ))
                }
                S::Y(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_y(&mut values, value)?;
                    }
                    Ok(super::DynPickType::Y(helper.finish_element("Y", values)?))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_animal(
            values: &mut Option<super::AnimalType>,
            value: super::AnimalType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"animal",
                )))?;
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
        fn handle_animal<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnimalType>,
            fallback: Option<<super::AnimalType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnimalType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DynPickTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let fresh = values.is_none() && fallback.is_none();
                *self.state__ = S::Animal(values, None, fallback);
                if !fresh {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_animal(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_animal(&mut values, data)?;
                    *self.state__ = S::Animal(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Animal(values, None, Some(deserializer));
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
            use DynPickTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::DynPickType> for DynPickTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DynPickType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DynPickType> {
            use DynPickTypeDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Animal(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_animal(helper, values, fallback, output)? {
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
                    (
                        S::Animal(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = <super::AnimalType as WithDeserializer>::init(helper, event)?;
                        match self.handle_animal(helper, values, fallback, output)? {
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
            let event = event.into_break();
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::DynPickType, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub enum AnimalTypeDeserializer {
        Animal(<super::AnimalDyn as WithDeserializer>::Deserializer),
    }
    impl<'de> Deserializer<'de, super::AnimalType> for AnimalTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnimalType> {
            let Some(type_name) = helper.get_dynamic_type_from_tag(&event) else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::None,
                    allow_any: false,
                });
            };
            let type_name = type_name.into_owned();
            if matches!(
                helper.resolve_local_name(QName(&type_name), &super::NS_TNS),
                Some(b"animal")
            ) {
                let type_name = helper
                    .get_dynamic_type_from_attrib(&event)?
                    .unwrap_or(Cow::Borrowed("animal"))
                    .into_owned();
                if matches!(
                    helper.resolve_local_name(QName(&type_name), &super::NS_TNS),
                    Some(b"animal")
                ) {
                    let DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = <super::AnimalDyn as WithDeserializer>::init(helper, event)?;
                    return Ok(DeserializerOutput {
                        artifact: artifact
                            .map(|x| super::AnimalType(Box::new(x)), |x| Self::Animal(x)),
                        event,
                        allow_any,
                    });
                }
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
        ) -> DeserializerResult<'de, super::AnimalType> {
            match self {
                Self::Animal(x) => {
                    let DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = x.next(helper, event)?;
                    Ok(DeserializerOutput {
                        artifact: artifact
                            .map(|x| super::AnimalType(Box::new(x)), |x| Self::Animal(x)),
                        event,
                        allow_any,
                    })
                }
            }
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::AnimalType, Error> {
            match self {
                Self::Animal(x) => Ok(super::AnimalType(Box::new(x.finish(helper)?))),
            }
        }
    }
    #[derive(Debug)]
    pub struct AnimalDynDeserializer {
        id: i32,
        state__: Box<AnimalDynDeserializerState>,
    }
    #[derive(Debug)]
    enum AnimalDynDeserializerState {
        Init__,
        Unknown__,
    }
    impl AnimalDynDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut id: Option<i32> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id.ok_or_else(|| ErrorKind::MissingAttribute("id".into()))?,
                state__: Box::new(AnimalDynDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: AnimalDynDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::AnimalDyn> for AnimalDynDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnimalDyn> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnimalDyn> {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::AnimalDyn, Error> {
            let state = replace(&mut *self.state__, AnimalDynDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::AnimalDyn { id: self.id })
        }
    }
    #[derive(Debug)]
    pub struct MixedPickTypeDeserializer {
        state__: Box<MixedPickTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum MixedPickTypeDeserializerState {
        Init__,
        A(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        B(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::MixedPickType),
        Unknown__,
    }
    impl MixedPickTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"A")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_a(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"B")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_b(helper, Default::default(), None, output);
                }
            }
            *self.state__ = MixedPickTypeDeserializerState::Init__;
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
                state__: Box::new(MixedPickTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: MixedPickTypeDeserializerState,
        ) -> Result<super::MixedPickType, Error> {
            use MixedPickTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::A(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_a(&mut values, value)?;
                    }
                    Ok(super::MixedPickType::A(helper.finish_element("A", values)?))
                }
                S::B(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_b(&mut values, value)?;
                    }
                    Ok(super::MixedPickType::B(helper.finish_element("B", values)?))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_a(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"A")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_b(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"B")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_a<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedPickTypeDeserializerState as S;
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
                Self::store_a(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_a(&mut values, data)?;
                    *self.state__ = S::A(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::A(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
        fn handle_b<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedPickTypeDeserializerState as S;
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
                Self::store_b(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_b(&mut values, data)?;
                    *self.state__ = S::B(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::B(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::MixedPickType> for MixedPickTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedPickType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedPickType> {
            use MixedPickTypeDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::A(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_a(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::B(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_b(helper, values, fallback, output)? {
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
                    (state, Event::Text(_) | Event::CData(_)) => {
                        *self.state__ = state;
                        break (DeserializerEvent::None, false);
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::A(values, fallback, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"A",
                            false,
                        )?;
                        match self.handle_a(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::B(values, fallback, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"B",
                            false,
                        )?;
                        match self.handle_b(helper, values, fallback, output)? {
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
            let event = event.into_break();
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::MixedPickType, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, ElementNamespace, Error, Event, SerializeHelper, Serializer,
        WithSerializer,
    };
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
        Pq(<super::GroupPickPqType as WithSerializer>::Serializer<'ser>),
        Y(<String as WithSerializer>::Serializer<'ser>),
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
                        match self.value {
                            super::GroupPickType::Pq(x) => {
                                *self.state = GroupPickTypeSerializerState::Pq(
                                    WithSerializer::serializer_with_namespace(
                                        x,
                                        Some("PQ"),
                                        ElementNamespace::None,
                                        self.is_root,
                                    )?,
                                )
                            }
                            super::GroupPickType::Y(x) => {
                                *self.state = GroupPickTypeSerializerState::Y(
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
                    GroupPickTypeSerializerState::Pq(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = GroupPickTypeSerializerState::End__,
                    },
                    GroupPickTypeSerializerState::Y(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = GroupPickTypeSerializerState::End__,
                    },
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
    #[derive(Debug)]
    pub struct ChoiceGroupPickTypeSerializer<'ser> {
        pub(super) value: &'ser super::ChoiceGroupPickType,
        pub(super) state: Box<ChoiceGroupPickTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ChoiceGroupPickTypeSerializerState<'ser> {
        Init__,
        Uv(<super::ChoiceGroupPickUvType as WithSerializer>::Serializer<'ser>),
        Y(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ChoiceGroupPickTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ChoiceGroupPickTypeSerializerState::Init__ => {
                        match self.value {
                            super::ChoiceGroupPickType::Uv(x) => {
                                *self.state = ChoiceGroupPickTypeSerializerState::Uv(
                                    WithSerializer::serializer_with_namespace(
                                        x,
                                        Some("UV"),
                                        ElementNamespace::None,
                                        self.is_root,
                                    )?,
                                )
                            }
                            super::ChoiceGroupPickType::Y(x) => {
                                *self.state = ChoiceGroupPickTypeSerializerState::Y(
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
                    ChoiceGroupPickTypeSerializerState::Uv(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ChoiceGroupPickTypeSerializerState::End__,
                        }
                    }
                    ChoiceGroupPickTypeSerializerState::Y(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ChoiceGroupPickTypeSerializerState::End__,
                        }
                    }
                    ChoiceGroupPickTypeSerializerState::End__ => {
                        *self.state = ChoiceGroupPickTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ChoiceGroupPickTypeSerializerState::Done__ => return Ok(None),
                    ChoiceGroupPickTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ChoiceGroupPickTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ChoiceGroupPickTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ChoiceGroupPickUvTypeSerializer<'ser> {
        pub(super) value: &'ser super::ChoiceGroupPickUvType,
        pub(super) state: Box<ChoiceGroupPickUvTypeSerializerState<'ser>>,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ChoiceGroupPickUvTypeSerializerState<'ser> {
        Init__,
        U(<String as WithSerializer>::Serializer<'ser>),
        V(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ChoiceGroupPickUvTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ChoiceGroupPickUvTypeSerializerState::Init__ => match self.value {
                        super::ChoiceGroupPickUvType::U(x) => {
                            *self.state = ChoiceGroupPickUvTypeSerializerState::U(
                                WithSerializer::serializer_with_namespace(
                                    x,
                                    Some("tns:U"),
                                    ElementNamespace::Namespace(super::NS_TNS),
                                    self.is_root,
                                )?,
                            )
                        }
                        super::ChoiceGroupPickUvType::V(x) => {
                            *self.state = ChoiceGroupPickUvTypeSerializerState::V(
                                WithSerializer::serializer_with_namespace(
                                    x,
                                    Some("tns:V"),
                                    ElementNamespace::Namespace(super::NS_TNS),
                                    self.is_root,
                                )?,
                            )
                        }
                    },
                    ChoiceGroupPickUvTypeSerializerState::U(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ChoiceGroupPickUvTypeSerializerState::Done__,
                        }
                    }
                    ChoiceGroupPickUvTypeSerializerState::V(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ChoiceGroupPickUvTypeSerializerState::Done__,
                        }
                    }
                    ChoiceGroupPickUvTypeSerializerState::Done__ => return Ok(None),
                    ChoiceGroupPickUvTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ChoiceGroupPickUvTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ChoiceGroupPickUvTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DynPickTypeSerializer<'ser> {
        pub(super) value: &'ser super::DynPickType,
        pub(super) state: Box<DynPickTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DynPickTypeSerializerState<'ser> {
        Init__,
        Animal(<super::AnimalType as WithSerializer>::Serializer<'ser>),
        Y(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DynPickTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DynPickTypeSerializerState::Init__ => {
                        match self.value {
                            super::DynPickType::Animal(x) => {
                                *self.state = DynPickTypeSerializerState::Animal(
                                    WithSerializer::serializer_with_namespace(
                                        x,
                                        Some("tns:animal"),
                                        ElementNamespace::Namespace(super::NS_TNS),
                                        self.is_root,
                                    )?,
                                )
                            }
                            super::DynPickType::Y(x) => {
                                *self.state = DynPickTypeSerializerState::Y(
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
                    DynPickTypeSerializerState::Animal(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DynPickTypeSerializerState::End__,
                    },
                    DynPickTypeSerializerState::Y(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DynPickTypeSerializerState::End__,
                    },
                    DynPickTypeSerializerState::End__ => {
                        *self.state = DynPickTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DynPickTypeSerializerState::Done__ => return Ok(None),
                    DynPickTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for DynPickTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DynPickTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct AnimalDynSerializer<'ser> {
        pub(super) value: &'ser super::AnimalDyn,
        pub(super) state: Box<AnimalDynSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum AnimalDynSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> AnimalDynSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    AnimalDynSerializerState::Init__ => {
                        *self.state = AnimalDynSerializerState::Done__;
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
                        helper.end_ns_scope();
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    AnimalDynSerializerState::Done__ => return Ok(None),
                    AnimalDynSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for AnimalDynSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = AnimalDynSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct MixedPickTypeSerializer<'ser> {
        pub(super) value: &'ser super::MixedPickType,
        pub(super) state: Box<MixedPickTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum MixedPickTypeSerializerState<'ser> {
        Init__,
        A(<String as WithSerializer>::Serializer<'ser>),
        B(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> MixedPickTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MixedPickTypeSerializerState::Init__ => {
                        match self.value {
                            super::MixedPickType::A(x) => {
                                *self.state = MixedPickTypeSerializerState::A(
                                    WithSerializer::serializer_with_namespace(
                                        x,
                                        Some("tns:A"),
                                        ElementNamespace::Namespace(super::NS_TNS),
                                        self.is_root,
                                    )?,
                                )
                            }
                            super::MixedPickType::B(x) => {
                                *self.state = MixedPickTypeSerializerState::B(
                                    WithSerializer::serializer_with_namespace(
                                        x,
                                        Some("tns:B"),
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
                    MixedPickTypeSerializerState::A(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = MixedPickTypeSerializerState::End__,
                    },
                    MixedPickTypeSerializerState::B(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = MixedPickTypeSerializerState::End__,
                    },
                    MixedPickTypeSerializerState::End__ => {
                        *self.state = MixedPickTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    MixedPickTypeSerializerState::Done__ => return Ok(None),
                    MixedPickTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for MixedPickTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = MixedPickTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
