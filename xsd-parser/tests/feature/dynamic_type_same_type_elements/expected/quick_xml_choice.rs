use core::fmt::Debug;
use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{
        BoxedSerializer, ElementNamespace, Error, WithBoxedSerializer, WithDeserializer,
        WithSerializer, XsiTypeSerializer,
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
pub type Zoo = ZooType;
#[derive(Debug)]
pub enum ZooType {
    Pet(AnimalType),
    Note(String),
}
impl WithSerializer for ZooType {
    type Serializer<'x> = quick_xml_serialize::ZooTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::ZooTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ZooTypeSerializerState::Init__),
            name: name.unwrap_or("tns:zoo"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for ZooType {
    type Deserializer = quick_xml_deserialize::ZooTypeDeserializer;
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
    pub struct ZooTypeDeserializer {
        state__: Box<ZooTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ZooTypeDeserializerState {
        Init__,
        Pet(
            Option<super::AnimalType>,
            Option<<super::AnimalType as WithDeserializer>::Deserializer>,
            Option<<super::AnimalType as WithDeserializer>::Deserializer>,
        ),
        Note(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ZooType),
        Unknown__,
    }
    impl ZooTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let mut event = event;
            let mut allow_any_element = false;
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                    Some(b"note")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_note(helper, Default::default(), None, output);
                }
                event = {
                    let output = <super::AnimalType as WithDeserializer>::init_with_name(
                        helper,
                        event,
                        Some(&super::NS_TNS),
                        b"pet",
                    )?;
                    match self.handle_pet(helper, Default::default(), None, output)? {
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
            *self.state__ = ZooTypeDeserializerState::Init__;
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
                state__: Box::new(ZooTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: ZooTypeDeserializerState,
        ) -> Result<super::ZooType, Error> {
            use ZooTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Pet(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_pet(&mut values, value)?;
                    }
                    Ok(super::ZooType::Pet(helper.finish_element("pet", values)?))
                }
                S::Note(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_note(&mut values, value)?;
                    }
                    Ok(super::ZooType::Note(helper.finish_element("note", values)?))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_pet(
            values: &mut Option<super::AnimalType>,
            value: super::AnimalType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"pet")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_note(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"note")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_pet<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnimalType>,
            fallback: Option<<super::AnimalType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnimalType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ZooTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let fresh = values.is_none() && fallback.is_none();
                *self.state__ = S::Pet(values, None, fallback);
                if !fresh {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_pet(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pet(&mut values, data)?;
                    *self.state__ = S::Pet(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Pet(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
        fn handle_note<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ZooTypeDeserializerState as S;
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
                Self::store_note(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_note(&mut values, data)?;
                    *self.state__ = S::Note(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Note(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ZooType> for ZooTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ZooType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ZooType> {
            use ZooTypeDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Pet(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_pet(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Note(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_note(helper, values, fallback, output)? {
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
                        S::Pet(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = <super::AnimalType as WithDeserializer>::init_with_name(
                            helper,
                            event,
                            Some(&super::NS_TNS),
                            b"pet",
                        )?;
                        match self.handle_pet(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Note(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"note",
                            false,
                        )?;
                        match self.handle_note(helper, values, fallback, output)? {
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::ZooType, Error> {
            Self::finish_state(helper, *self.state__)
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
    pub struct ZooTypeSerializer<'ser> {
        pub(super) value: &'ser super::ZooType,
        pub(super) state: Box<ZooTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ZooTypeSerializerState<'ser> {
        Init__,
        Pet(<super::AnimalType as WithSerializer>::Serializer<'ser>),
        Note(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ZooTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ZooTypeSerializerState::Init__ => {
                        match self.value {
                            super::ZooType::Pet(x) => {
                                *self.state = ZooTypeSerializerState::Pet(
                                    WithSerializer::serializer_with_namespace(
                                        x,
                                        Some("tns:pet"),
                                        ElementNamespace::Namespace(super::NS_TNS),
                                        self.is_root,
                                    )?,
                                )
                            }
                            super::ZooType::Note(x) => {
                                *self.state = ZooTypeSerializerState::Note(
                                    WithSerializer::serializer_with_namespace(
                                        x,
                                        Some("tns:note"),
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
                    ZooTypeSerializerState::Pet(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ZooTypeSerializerState::End__,
                    },
                    ZooTypeSerializerState::Note(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ZooTypeSerializerState::End__,
                    },
                    ZooTypeSerializerState::End__ => {
                        *self.state = ZooTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ZooTypeSerializerState::Done__ => return Ok(None),
                    ZooTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ZooTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ZooTypeSerializerState::Done__;
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
