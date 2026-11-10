use xsd_parser_types::misc::{Namespace, NamespacePrefix};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub mod tns {
    pub use self::type_animal_dyn::AnimalDyn;
    pub use self::type_animal_type::{AnimalType, AnimalTypeTrait};
    pub use self::type_dog_type::DogType;
    pub use self::type_labrador_type::LabradorType;
    pub use self::type_list::List;
    pub use self::type_list_type::ListType;
    pub mod type_animal_dyn {
        use xsd_parser_types::quick_xml::{
            ElementNamespace, Error, WithDeserializer, WithSerializer,
        };
        #[derive(Debug)]
        pub struct AnimalDyn {
            pub id: i32,
        }
        impl super::type_animal_type::AnimalTypeTrait for AnimalDyn {}
        impl WithSerializer for AnimalDyn {
            type Serializer<'x> = quick_xml_serialize::AnimalDynSerializer<'x>;
            fn serializer<'ser>(
                &'ser self,
                name: Option<&'ser str>,
                is_root: bool,
            ) -> Result<Self::Serializer<'ser>, Error> {
                WithSerializer::serializer_with_namespace(
                    self,
                    name,
                    ElementNamespace::Unknown,
                    is_root,
                )
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
        mod quick_xml_serialize {
            use xsd_parser_types::quick_xml::{
                BytesStart, ElementNamespace, Error, Event, SerializeHelper, Serializer,
            };
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
                                    ElementNamespace::Namespace(super::super::super::NS_TNS)
                                } else {
                                    self.namespace.clone()
                                };
                                self.name = helper.element_name(self.name, &namespace);
                                let mut bytes = BytesStart::new(self.name);
                                helper.begin_ns_scope();
                                helper.write_xmlns_for_element(&mut bytes, self.name, &namespace);
                                if self.is_root {
                                    helper
                                        .write_default_namespace(&mut bytes, self.name, &namespace);
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
                fn next(
                    &mut self,
                    helper: &mut SerializeHelper,
                ) -> Option<Result<Event<'ser>, Error>> {
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
        }
        mod quick_xml_deserialize {
            use core::mem::replace;
            use xsd_parser_types::quick_xml::{
                BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact,
                DeserializerEvent, DeserializerOutput, DeserializerResult, Error, ErrorKind, Event,
            };
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
                            helper.resolve_attrib_local_name(
                                attrib.key,
                                &super::super::super::NS_TNS
                            ),
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
                fn finish(
                    mut self,
                    helper: &mut DeserializeHelper,
                ) -> Result<super::AnimalDyn, Error> {
                    let state = replace(&mut *self.state__, AnimalDynDeserializerState::Unknown__);
                    self.finish_state(helper, state)?;
                    Ok(super::AnimalDyn { id: self.id })
                }
            }
        }
    }
    pub mod type_animal_type {
        use core::fmt::Debug;
        use xsd_parser_types::{
            quick_xml::{
                BoxedSerializer, ElementNamespace, Error, WithBoxedSerializer, WithDeserializer,
                WithSerializer, XsiTypeSerializer,
            },
            AsAny,
        };
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
                WithSerializer::serializer_with_namespace(
                    self,
                    name,
                    ElementNamespace::Unknown,
                    is_root,
                )
            }
            fn serializer_with_namespace<'ser>(
                &'ser self,
                name: Option<&'ser str>,
                namespace: ElementNamespace,
                is_root: bool,
            ) -> Result<Self::Serializer<'ser>, Error> {
                if (*self.0).as_any().is::<super::type_dog_type::DogType>() {
                    return Ok(Box::new(
                        XsiTypeSerializer::new(
                            self.0.serializer(Some("tns:dog"), is_root)?,
                            name.unwrap_or("animal"),
                            is_root,
                        )
                        .with_namespace(namespace)
                        .with_type_namespace(super::super::NS_TNS),
                    ));
                }
                if (*self.0)
                    .as_any()
                    .is::<super::type_labrador_type::LabradorType>()
                {
                    return Ok(Box::new(
                        XsiTypeSerializer::new(
                            self.0.serializer(Some("tns:labrador"), is_root)?,
                            name.unwrap_or("animal"),
                            is_root,
                        )
                        .with_namespace(namespace)
                        .with_type_namespace(super::super::NS_TNS),
                    ));
                }
                self.0.serializer_with_namespace(name, namespace, is_root)
            }
        }
        impl WithDeserializer for AnimalType {
            type Deserializer = quick_xml_deserialize::AnimalTypeDeserializer;
        }
        mod quick_xml_deserialize {
            use std::borrow::Cow;
            use xsd_parser_types::quick_xml::{
                DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
                DeserializerOutput, DeserializerResult, Error, Event, QName, WithDeserializer,
            };
            #[derive(Debug)]
            pub enum AnimalTypeDeserializer {
                Animal (< super :: super :: type_animal_dyn :: AnimalDyn as WithDeserializer > :: Deserializer) , Dog (< super :: super :: type_dog_type :: DogType as WithDeserializer > :: Deserializer) , Labrador (< super :: super :: type_labrador_type :: LabradorType as WithDeserializer > :: Deserializer) , }
            impl<'de> Deserializer<'de, super::AnimalType> for AnimalTypeDeserializer {
                fn init(
                    helper: &mut DeserializeHelper,
                    event: Event<'de>,
                ) -> DeserializerResult<'de, super::AnimalType> {
                    Self::init_with_name(
                        helper,
                        event,
                        Some(&super::super::super::NS_TNS),
                        b"animal",
                    )
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
                            helper.resolve_element_local_name(
                                QName(&type_name),
                                &super::super::super::NS_TNS
                            ),
                            Some(b"animal")
                        ) {
                            let DeserializerOutput { artifact , event , allow_any , } = < super :: super :: type_animal_dyn :: AnimalDyn as WithDeserializer > :: init (helper , event) ? ;
                            return Ok(DeserializerOutput {
                                artifact: artifact
                                    .map(|x| super::AnimalType(Box::new(x)), |x| Self::Animal(x)),
                                event,
                                allow_any,
                            });
                        }
                        if matches!(
                            helper.resolve_element_local_name(
                                QName(&type_name),
                                &super::super::super::NS_TNS
                            ),
                            Some(b"dog")
                        ) {
                            let DeserializerOutput {
                                artifact,
                                event,
                                allow_any,
                            } = <super::super::type_dog_type::DogType as WithDeserializer>::init(
                                helper, event,
                            )?;
                            return Ok(DeserializerOutput {
                                artifact: artifact
                                    .map(|x| super::AnimalType(Box::new(x)), |x| Self::Dog(x)),
                                event,
                                allow_any,
                            });
                        }
                        if matches!(
                            helper.resolve_element_local_name(
                                QName(&type_name),
                                &super::super::super::NS_TNS
                            ),
                            Some(b"labrador")
                        ) {
                            let DeserializerOutput { artifact , event , allow_any , } = < super :: super :: type_labrador_type :: LabradorType as WithDeserializer > :: init (helper , event) ? ;
                            return Ok(DeserializerOutput {
                                artifact: artifact
                                    .map(|x| super::AnimalType(Box::new(x)), |x| Self::Labrador(x)),
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
                        Self::Labrador(x) => {
                            let DeserializerOutput {
                                artifact,
                                event,
                                allow_any,
                            } = x.next(helper, event)?;
                            Ok(DeserializerOutput {
                                artifact: artifact
                                    .map(|x| super::AnimalType(Box::new(x)), |x| Self::Labrador(x)),
                                event,
                                allow_any,
                            })
                        }
                    }
                }
                fn finish(
                    self,
                    helper: &mut DeserializeHelper,
                ) -> Result<super::AnimalType, Error> {
                    match self {
                        Self::Animal(x) => Ok(super::AnimalType(Box::new(x.finish(helper)?))),
                        Self::Dog(x) => Ok(super::AnimalType(Box::new(x.finish(helper)?))),
                        Self::Labrador(x) => Ok(super::AnimalType(Box::new(x.finish(helper)?))),
                    }
                }
            }
        }
    }
    pub mod type_dog_type {
        use xsd_parser_types::quick_xml::{
            ElementNamespace, Error, WithDeserializer, WithSerializer,
        };
        #[derive(Debug)]
        pub struct DogType {
            pub id: i32,
            pub name: String,
        }
        impl super::type_animal_type::AnimalTypeTrait for DogType {}
        impl WithSerializer for DogType {
            type Serializer<'x> = quick_xml_serialize::DogTypeSerializer<'x>;
            fn serializer<'ser>(
                &'ser self,
                name: Option<&'ser str>,
                is_root: bool,
            ) -> Result<Self::Serializer<'ser>, Error> {
                WithSerializer::serializer_with_namespace(
                    self,
                    name,
                    ElementNamespace::Unknown,
                    is_root,
                )
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
        mod quick_xml_serialize {
            use xsd_parser_types::quick_xml::{
                BytesStart, ElementNamespace, Error, Event, SerializeHelper, Serializer,
            };
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
                                    ElementNamespace::Namespace(super::super::super::NS_TNS)
                                } else {
                                    self.namespace.clone()
                                };
                                self.name = helper.element_name(self.name, &namespace);
                                let mut bytes = BytesStart::new(self.name);
                                helper.begin_ns_scope();
                                helper.write_xmlns_for_element(&mut bytes, self.name, &namespace);
                                if self.is_root {
                                    helper
                                        .write_default_namespace(&mut bytes, self.name, &namespace);
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
                fn next(
                    &mut self,
                    helper: &mut SerializeHelper,
                ) -> Option<Result<Event<'ser>, Error>> {
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
        mod quick_xml_deserialize {
            use core::mem::replace;
            use xsd_parser_types::quick_xml::{
                BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact,
                DeserializerEvent, DeserializerOutput, DeserializerResult, Error, ErrorKind, Event,
            };
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
                            helper.resolve_attrib_local_name(
                                attrib.key,
                                &super::super::super::NS_TNS
                            ),
                            Some(b"id")
                        ) {
                            helper.read_attrib(&mut id, b"id", &attrib.value)?;
                        } else if matches!(
                            helper.resolve_attrib_local_name(
                                attrib.key,
                                &super::super::super::NS_TNS
                            ),
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
                fn finish(
                    mut self,
                    helper: &mut DeserializeHelper,
                ) -> Result<super::DogType, Error> {
                    let state = replace(&mut *self.state__, DogTypeDeserializerState::Unknown__);
                    self.finish_state(helper, state)?;
                    Ok(super::DogType {
                        id: self.id,
                        name: self.name,
                    })
                }
            }
        }
    }
    pub mod type_labrador_type {
        use xsd_parser_types::quick_xml::{
            ElementNamespace, Error, WithDeserializer, WithSerializer,
        };
        #[derive(Debug)]
        pub struct LabradorType {
            pub id: i32,
            pub name: String,
            pub color: String,
        }
        impl super::type_animal_type::AnimalTypeTrait for LabradorType {}
        impl WithSerializer for LabradorType {
            type Serializer<'x> = quick_xml_serialize::LabradorTypeSerializer<'x>;
            fn serializer<'ser>(
                &'ser self,
                name: Option<&'ser str>,
                is_root: bool,
            ) -> Result<Self::Serializer<'ser>, Error> {
                WithSerializer::serializer_with_namespace(
                    self,
                    name,
                    ElementNamespace::Unknown,
                    is_root,
                )
            }
            fn serializer_with_namespace<'ser>(
                &'ser self,
                name: Option<&'ser str>,
                namespace: ElementNamespace,
                is_root: bool,
            ) -> Result<Self::Serializer<'ser>, Error> {
                Ok(quick_xml_serialize::LabradorTypeSerializer {
                    value: self,
                    state: Box::new(quick_xml_serialize::LabradorTypeSerializerState::Init__),
                    name: name.unwrap_or("labrador"),
                    namespace,
                    is_root,
                })
            }
        }
        impl WithDeserializer for LabradorType {
            type Deserializer = quick_xml_deserialize::LabradorTypeDeserializer;
        }
        mod quick_xml_serialize {
            use xsd_parser_types::quick_xml::{
                BytesStart, ElementNamespace, Error, Event, SerializeHelper, Serializer,
            };
            #[derive(Debug)]
            pub struct LabradorTypeSerializer<'ser> {
                pub(super) value: &'ser super::LabradorType,
                pub(super) state: Box<LabradorTypeSerializerState<'ser>>,
                pub(super) name: &'ser str,
                pub(super) namespace: ElementNamespace,
                pub(super) is_root: bool,
            }
            #[derive(Debug)]
            pub(super) enum LabradorTypeSerializerState<'ser> {
                Init__,
                Done__,
                Phantom__(&'ser ()),
            }
            impl<'ser> LabradorTypeSerializer<'ser> {
                fn next_event(
                    &mut self,
                    helper: &mut SerializeHelper,
                ) -> Result<Option<Event<'ser>>, Error> {
                    loop {
                        match &mut *self.state {
                            LabradorTypeSerializerState::Init__ => {
                                *self.state = LabradorTypeSerializerState::Done__;
                                let namespace = if self.is_root {
                                    ElementNamespace::Namespace(super::super::super::NS_TNS)
                                } else {
                                    self.namespace.clone()
                                };
                                self.name = helper.element_name(self.name, &namespace);
                                let mut bytes = BytesStart::new(self.name);
                                helper.begin_ns_scope();
                                helper.write_xmlns_for_element(&mut bytes, self.name, &namespace);
                                if self.is_root {
                                    helper
                                        .write_default_namespace(&mut bytes, self.name, &namespace);
                                }
                                helper.write_attrib(&mut bytes, "id", &self.value.id)?;
                                helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                                helper.write_attrib(&mut bytes, "color", &self.value.color)?;
                                helper.end_ns_scope();
                                return Ok(Some(Event::Empty(bytes)));
                            }
                            LabradorTypeSerializerState::Done__ => return Ok(None),
                            LabradorTypeSerializerState::Phantom__(_) => unreachable!(),
                        }
                    }
                }
            }
            impl<'ser> Serializer<'ser> for LabradorTypeSerializer<'ser> {
                fn next(
                    &mut self,
                    helper: &mut SerializeHelper,
                ) -> Option<Result<Event<'ser>, Error>> {
                    match self.next_event(helper) {
                        Ok(Some(event)) => Some(Ok(event)),
                        Ok(None) => None,
                        Err(error) => {
                            *self.state = LabradorTypeSerializerState::Done__;
                            Some(Err(error))
                        }
                    }
                }
            }
        }
        mod quick_xml_deserialize {
            use core::mem::replace;
            use xsd_parser_types::quick_xml::{
                BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact,
                DeserializerEvent, DeserializerOutput, DeserializerResult, Error, ErrorKind, Event,
            };
            #[derive(Debug)]
            pub struct LabradorTypeDeserializer {
                id: i32,
                name: String,
                color: String,
                state__: Box<LabradorTypeDeserializerState>,
            }
            #[derive(Debug)]
            enum LabradorTypeDeserializerState {
                Init__,
                Unknown__,
            }
            impl LabradorTypeDeserializer {
                fn from_bytes_start(
                    helper: &mut DeserializeHelper,
                    bytes_start: &BytesStart<'_>,
                ) -> Result<Self, Error> {
                    let mut id: Option<i32> = None;
                    let mut name: Option<String> = None;
                    let mut color: Option<String> = None;
                    for attrib in helper.filter_xmlns_attributes(bytes_start) {
                        let attrib = attrib?;
                        if matches!(
                            helper.resolve_attrib_local_name(
                                attrib.key,
                                &super::super::super::NS_TNS
                            ),
                            Some(b"id")
                        ) {
                            helper.read_attrib(&mut id, b"id", &attrib.value)?;
                        } else if matches!(
                            helper.resolve_attrib_local_name(
                                attrib.key,
                                &super::super::super::NS_TNS
                            ),
                            Some(b"name")
                        ) {
                            helper.read_attrib(&mut name, b"name", &attrib.value)?;
                        } else if matches!(
                            helper.resolve_attrib_local_name(
                                attrib.key,
                                &super::super::super::NS_TNS
                            ),
                            Some(b"color")
                        ) {
                            helper.read_attrib(&mut color, b"color", &attrib.value)?;
                        } else {
                            helper.raise_unexpected_attrib_checked(&attrib)?;
                        }
                    }
                    Ok(Self {
                        id: id.ok_or_else(|| ErrorKind::MissingAttribute("id".into()))?,
                        name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                        color: color.ok_or_else(|| ErrorKind::MissingAttribute("color".into()))?,
                        state__: Box::new(LabradorTypeDeserializerState::Init__),
                    })
                }
                fn finish_state(
                    &mut self,
                    helper: &mut DeserializeHelper,
                    state: LabradorTypeDeserializerState,
                ) -> Result<(), Error> {
                    Ok(())
                }
            }
            impl<'de> Deserializer<'de, super::LabradorType> for LabradorTypeDeserializer {
                fn init(
                    helper: &mut DeserializeHelper,
                    event: Event<'de>,
                ) -> DeserializerResult<'de, super::LabradorType> {
                    helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
                }
                fn next(
                    mut self,
                    helper: &mut DeserializeHelper,
                    event: Event<'de>,
                ) -> DeserializerResult<'de, super::LabradorType> {
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
                fn finish(
                    mut self,
                    helper: &mut DeserializeHelper,
                ) -> Result<super::LabradorType, Error> {
                    let state =
                        replace(&mut *self.state__, LabradorTypeDeserializerState::Unknown__);
                    self.finish_state(helper, state)?;
                    Ok(super::LabradorType {
                        id: self.id,
                        name: self.name,
                        color: self.color,
                    })
                }
            }
        }
    }
    pub mod type_list {
        pub type List = super::type_list_type::ListType;
    }
    pub mod type_list_type {
        use xsd_parser_types::quick_xml::{
            ElementNamespace, Error, WithDeserializer, WithSerializer,
        };
        #[derive(Debug)]
        pub struct ListType {
            pub animal: Vec<super::type_animal_type::AnimalType>,
        }
        impl WithSerializer for ListType {
            type Serializer<'x> = quick_xml_serialize::ListTypeSerializer<'x>;
            fn serializer<'ser>(
                &'ser self,
                name: Option<&'ser str>,
                is_root: bool,
            ) -> Result<Self::Serializer<'ser>, Error> {
                WithSerializer::serializer_with_namespace(
                    self,
                    name,
                    ElementNamespace::Unknown,
                    is_root,
                )
            }
            fn serializer_with_namespace<'ser>(
                &'ser self,
                name: Option<&'ser str>,
                namespace: ElementNamespace,
                is_root: bool,
            ) -> Result<Self::Serializer<'ser>, Error> {
                Ok(quick_xml_serialize::ListTypeSerializer {
                    value: self,
                    state: Box::new(quick_xml_serialize::ListTypeSerializerState::Init__),
                    name: name.unwrap_or("tns:list"),
                    namespace,
                    is_root,
                })
            }
        }
        impl WithDeserializer for ListType {
            type Deserializer = quick_xml_deserialize::ListTypeDeserializer;
        }
        mod quick_xml_serialize {
            use xsd_parser_types::quick_xml::{
                BytesEnd, BytesStart, ElementNamespace, Error, Event, IterSerializer,
                SerializeHelper, Serializer,
            };
            #[derive(Debug)]
            pub struct ListTypeSerializer<'ser> {
                pub(super) value: &'ser super::ListType,
                pub(super) state: Box<ListTypeSerializerState<'ser>>,
                pub(super) name: &'ser str,
                pub(super) namespace: ElementNamespace,
                pub(super) is_root: bool,
            }
            #[derive(Debug)]
            pub(super) enum ListTypeSerializerState<'ser> {
                Init__,
                Animal(
                    IterSerializer<
                        'ser,
                        &'ser [super::super::type_animal_type::AnimalType],
                        super::super::type_animal_type::AnimalType,
                    >,
                ),
                End__,
                Done__,
                Phantom__(&'ser ()),
            }
            impl<'ser> ListTypeSerializer<'ser> {
                fn next_event(
                    &mut self,
                    helper: &mut SerializeHelper,
                ) -> Result<Option<Event<'ser>>, Error> {
                    loop {
                        match &mut *self.state {
                            ListTypeSerializerState::Init__ => {
                                *self.state = ListTypeSerializerState::Animal(
                                    IterSerializer::new(
                                        &self.value.animal[..],
                                        Some("tns:animal"),
                                        false,
                                    )
                                    .with_namespace(
                                        ElementNamespace::Namespace(super::super::super::NS_TNS),
                                    ),
                                );
                                let namespace = if self.is_root {
                                    ElementNamespace::Namespace(super::super::super::NS_TNS)
                                } else {
                                    self.namespace.clone()
                                };
                                self.name = helper.element_name(self.name, &namespace);
                                let mut bytes = BytesStart::new(self.name);
                                helper.begin_ns_scope();
                                helper.write_xmlns_for_element(&mut bytes, self.name, &namespace);
                                if self.is_root {
                                    helper
                                        .write_default_namespace(&mut bytes, self.name, &namespace);
                                }
                                if self.is_root {
                                    helper.write_xmlns(
                                        &mut bytes,
                                        Some(&super::super::super::PREFIX_XSI),
                                        &super::super::super::NS_XSI,
                                    );
                                    helper.write_xmlns(
                                        &mut bytes,
                                        Some(&super::super::super::PREFIX_TNS),
                                        &super::super::super::NS_TNS,
                                    );
                                }
                                return Ok(Some(Event::Start(bytes)));
                            }
                            ListTypeSerializerState::Animal(x) => {
                                match x.next(helper).transpose()? {
                                    Some(event) => return Ok(Some(event)),
                                    None => *self.state = ListTypeSerializerState::End__,
                                }
                            }
                            ListTypeSerializerState::End__ => {
                                *self.state = ListTypeSerializerState::Done__;
                                helper.end_ns_scope();
                                return Ok(Some(Event::End(BytesEnd::new(self.name))));
                            }
                            ListTypeSerializerState::Done__ => return Ok(None),
                            ListTypeSerializerState::Phantom__(_) => unreachable!(),
                        }
                    }
                }
            }
            impl<'ser> Serializer<'ser> for ListTypeSerializer<'ser> {
                fn next(
                    &mut self,
                    helper: &mut SerializeHelper,
                ) -> Option<Result<Event<'ser>, Error>> {
                    match self.next_event(helper) {
                        Ok(Some(event)) => Some(Ok(event)),
                        Ok(None) => None,
                        Err(error) => {
                            *self.state = ListTypeSerializerState::Done__;
                            Some(Err(error))
                        }
                    }
                }
            }
        }
        mod quick_xml_deserialize {
            use core::mem::replace;
            use xsd_parser_types::quick_xml::{
                BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact,
                DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput,
                Error, Event, WithDeserializer,
            };
            #[derive(Debug)]
            pub struct ListTypeDeserializer {
                animal: Vec<super::super::type_animal_type::AnimalType>,
                state__: Box<ListTypeDeserializerState>,
            }
            #[derive(Debug)]
            enum ListTypeDeserializerState {
                Init__ , Animal (Option << super :: super :: type_animal_type :: AnimalType as WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
            impl ListTypeDeserializer {
                fn from_bytes_start(
                    helper: &mut DeserializeHelper,
                    bytes_start: &BytesStart<'_>,
                ) -> Result<Self, Error> {
                    for attrib in helper.filter_xmlns_attributes(bytes_start) {
                        let attrib = attrib?;
                        helper.raise_unexpected_attrib_checked(&attrib)?;
                    }
                    Ok(Self {
                        animal: Vec::new(),
                        state__: Box::new(ListTypeDeserializerState::Init__),
                    })
                }
                fn finish_state(
                    &mut self,
                    helper: &mut DeserializeHelper,
                    state: ListTypeDeserializerState,
                ) -> Result<(), Error> {
                    use ListTypeDeserializerState as S;
                    match state {
                        S::Animal(Some(deserializer)) => {
                            self.store_animal(deserializer.finish(helper)?)?
                        }
                        _ => (),
                    }
                    Ok(())
                }
                fn store_animal(
                    &mut self,
                    value: super::super::type_animal_type::AnimalType,
                ) -> Result<(), Error> {
                    self.animal.push(value);
                    Ok(())
                }
                fn handle_animal<'de>(
                    &mut self,
                    helper: &mut DeserializeHelper,
                    output: DeserializerOutput<'de, super::super::type_animal_type::AnimalType>,
                    fallback: &mut Option<ListTypeDeserializerState>,
                ) -> Result<ElementHandlerOutput<'de>, Error> {
                    use ListTypeDeserializerState as S;
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
                            self.store_animal(data)?;
                            *self.state__ = S::Animal(None);
                            Ok(ElementHandlerOutput::from_event(event, allow_any))
                        }
                        DeserializerArtifact::Deserializer(deserializer) => {
                            fallback.get_or_insert(S::Animal(Some(deserializer)));
                            *self.state__ = S::Animal(None);
                            Ok(ElementHandlerOutput::from_event(event, allow_any))
                        }
                    }
                }
            }
            impl<'de> Deserializer<'de, super::ListType> for ListTypeDeserializer {
                fn init(
                    helper: &mut DeserializeHelper,
                    event: Event<'de>,
                ) -> DeserializerResult<'de, super::ListType> {
                    helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
                }
                fn next(
                    mut self,
                    helper: &mut DeserializeHelper,
                    event: Event<'de>,
                ) -> DeserializerResult<'de, super::ListType> {
                    use ListTypeDeserializerState as S;
                    let mut event = event;
                    let mut fallback = None;
                    let mut allow_any_element = false;
                    let (event, allow_any) = loop {
                        let state = replace(&mut *self.state__, S::Unknown__);
                        event = match (state, event) {
                            (S::Unknown__, _) => unreachable!(),
                            (S::Animal(Some(deserializer)), event) => {
                                let output = deserializer.next(helper, event)?;
                                match self.handle_animal(helper, output, &mut fallback)? {
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
                                *self.state__ = S::Animal(None);
                                event
                            }
                            (S::Animal(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                                let output = < super :: super :: type_animal_type :: AnimalType as WithDeserializer > :: init_with_name (helper , event , Some (& super :: super :: super :: NS_TNS) , b"animal") ? ;
                                match self.handle_animal(helper, output, &mut fallback)? {
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
                fn finish(
                    mut self,
                    helper: &mut DeserializeHelper,
                ) -> Result<super::ListType, Error> {
                    let state = replace(&mut *self.state__, ListTypeDeserializerState::Unknown__);
                    self.finish_state(helper, state)?;
                    Ok(super::ListType {
                        animal: self.animal,
                    })
                }
            }
        }
    }
}
