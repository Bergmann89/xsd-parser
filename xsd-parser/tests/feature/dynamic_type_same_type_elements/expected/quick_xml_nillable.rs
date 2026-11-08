use core::fmt::Debug;
use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{
        BoxedSerializer, ElementNamespace, Error, WithBoxedSerializer, WithDeserializer,
        WithSerializer, XsiTypeSerializer,
    },
    xml::Nillable,
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
pub type Bag = BagType;
#[derive(Debug)]
pub struct BagType {
    pub pet: Pet,
}
impl WithSerializer for BagType {
    type Serializer<'x> = quick_xml_serialize::BagTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::BagTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::BagTypeSerializerState::Init__),
            name: name.unwrap_or("tns:bag"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for BagType {
    type Deserializer = quick_xml_deserialize::BagTypeDeserializer;
}
pub type Pet = Nillable<AnimalType>;
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
        if (*self.0).as_any().is::<DogType>() {
            return Ok(Box::new(
                XsiTypeSerializer::new(
                    self.0.serializer(Some("tns:dog"), is_root)?,
                    name.unwrap_or("animal"),
                    is_root,
                )
                .with_namespace(namespace)
                .with_type_namespace(NS_TNS),
            ));
        }
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
            name: name.unwrap_or("animal"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for AnimalDyn {
    type Deserializer = quick_xml_deserialize::AnimalDynDeserializer;
}
#[derive(Debug)]
pub struct DogType {
    pub id: i32,
    pub name: String,
}
impl AnimalTypeTrait for DogType {}
impl WithSerializer for DogType {
    type Serializer<'x> = quick_xml_serialize::DogTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::DogTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DogTypeSerializerState::Init__),
            name: name.unwrap_or("dog"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for DogType {
    type Deserializer = quick_xml_deserialize::DogTypeDeserializer;
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
    pub struct BagTypeDeserializer {
        pet: Option<super::Pet>,
        state__: Box<BagTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum BagTypeDeserializerState {
        Init__,
        Pet(Option<<super::Pet as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl BagTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                pet: None,
                state__: Box::new(BagTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: BagTypeDeserializerState,
        ) -> Result<(), Error> {
            use BagTypeDeserializerState as S;
            match state {
                S::Pet(Some(deserializer)) => self.store_pet(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_pet(&mut self, value: super::Pet) -> Result<(), Error> {
            if self.pet.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"pet")))?;
            }
            self.pet = Some(value);
            Ok(())
        }
        fn handle_pet<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::Pet>,
            fallback: &mut Option<BagTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use BagTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Pet(None));
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
                    self.store_pet(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Pet(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::BagType> for BagTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::BagType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::BagType> {
            use BagTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Pet(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_pet(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Pet(None);
                        event
                    }
                    (S::Pet(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = <super::Pet as WithDeserializer>::init_with_name(
                            helper,
                            event,
                            Some(&super::NS_TNS),
                            b"pet",
                        )?;
                        match self.handle_pet(helper, output, &mut fallback)? {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::BagType, Error> {
            let state = replace(&mut *self.state__, BagTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::BagType {
                pet: helper.finish_element("pet", self.pet)?,
            })
        }
    }
    #[derive(Debug)]
    pub enum AnimalTypeDeserializer {
        Animal(<super::AnimalDyn as WithDeserializer>::Deserializer),
        Dog(<super::DogType as WithDeserializer>::Deserializer),
    }
    impl<'de> Deserializer<'de, super::AnimalType> for AnimalTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnimalType> {
            Self::init_with_name(helper, event, Some(&super::NS_TNS), b"animal")
        }
        fn init_with_name(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
            ns: Option<&[u8]>,
            name: &[u8],
        ) -> DeserializerResult<'de, super::AnimalType> {
            let Some(type_name) = helper.get_dynamic_type_from_tag(&event) else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::None,
                    allow_any: false,
                });
            };
            let type_name = type_name.into_owned();
            if helper.check_start_tag_name(&event, ns, name) {
                let type_name = helper
                    .get_dynamic_type_from_attrib(&event)?
                    .unwrap_or(Cow::Borrowed("animal"))
                    .into_owned();
                if matches!(
                    helper.resolve_element_local_name(QName(&type_name), &super::NS_TNS),
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
                if matches!(
                    helper.resolve_element_local_name(QName(&type_name), &super::NS_TNS),
                    Some(b"dog")
                ) {
                    let DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = <super::DogType as WithDeserializer>::init(helper, event)?;
                    return Ok(DeserializerOutput {
                        artifact: artifact
                            .map(|x| super::AnimalType(Box::new(x)), |x| Self::Dog(x)),
                        event,
                        allow_any,
                    });
                }
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::None,
                event: DeserializerEvent::Continue(event),
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
                Self::Dog(x) => {
                    let DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = x.next(helper, event)?;
                    Ok(DeserializerOutput {
                        artifact: artifact
                            .map(|x| super::AnimalType(Box::new(x)), |x| Self::Dog(x)),
                        event,
                        allow_any,
                    })
                }
            }
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::AnimalType, Error> {
            match self {
                Self::Animal(x) => Ok(super::AnimalType(Box::new(x.finish(helper)?))),
                Self::Dog(x) => Ok(super::AnimalType(Box::new(x.finish(helper)?))),
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
                    helper.resolve_attrib_local_name(attrib.key, &super::NS_TNS),
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
    pub struct DogTypeDeserializer {
        id: i32,
        name: String,
        state__: Box<DogTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DogTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl DogTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut id: Option<i32> = None;
            let mut name: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_attrib_local_name(attrib.key, &super::NS_TNS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_attrib_local_name(attrib.key, &super::NS_TNS),
                    Some(b"name")
                ) {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id.ok_or_else(|| ErrorKind::MissingAttribute("id".into()))?,
                name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                state__: Box::new(DogTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: DogTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::DogType> for DogTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DogType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DogType> {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::DogType, Error> {
            let state = replace(&mut *self.state__, DogTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::DogType {
                id: self.id,
                name: self.name,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, ElementNamespace, Error, Event, SerializeHelper, Serializer,
        WithSerializer,
    };
    #[derive(Debug)]
    pub struct BagTypeSerializer<'ser> {
        pub(super) value: &'ser super::BagType,
        pub(super) state: Box<BagTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum BagTypeSerializerState<'ser> {
        Init__,
        Pet(<super::Pet as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> BagTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    BagTypeSerializerState::Init__ => {
                        *self.state =
                            BagTypeSerializerState::Pet(WithSerializer::serializer_with_namespace(
                                &self.value.pet,
                                Some("tns:pet"),
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
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_XSI),
                                &super::NS_XSI,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    BagTypeSerializerState::Pet(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = BagTypeSerializerState::End__,
                    },
                    BagTypeSerializerState::End__ => {
                        *self.state = BagTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    BagTypeSerializerState::Done__ => return Ok(None),
                    BagTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for BagTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = BagTypeSerializerState::Done__;
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
    pub struct DogTypeSerializer<'ser> {
        pub(super) value: &'ser super::DogType,
        pub(super) state: Box<DogTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DogTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DogTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DogTypeSerializerState::Init__ => {
                        *self.state = DogTypeSerializerState::Done__;
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
                        helper.write_attrib(&mut bytes, "id", &self.value.id)?;
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        helper.end_ns_scope();
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    DogTypeSerializerState::Done__ => return Ok(None),
                    DogTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for DogTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DogTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
