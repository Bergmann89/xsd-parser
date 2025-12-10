use core::fmt::Debug;
use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{BoxedSerializer, Error, WithBoxedSerializer, WithDeserializer, WithSerializer},
    xml::Nillable,
    AsAny,
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub type List = ListType;
#[derive(Debug)]
pub struct ListType {
    pub base: Vec<Base>,
}
impl WithSerializer for ListType {
    type Serializer<'x> = quick_xml_serialize::ListTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ListTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ListTypeSerializerState::Init__),
            name: name.unwrap_or("list"),
            is_root,
        })
    }
}
impl WithDeserializer for ListType {
    type Deserializer = quick_xml_deserialize::ListTypeDeserializer;
}
#[derive(Debug)]
pub struct Base(pub Box<dyn BaseTrait>);
pub trait BaseTrait: Debug + AsAny + WithBoxedSerializer {}
impl WithSerializer for Base {
    type Serializer<'x> = BoxedSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        self.0.serializer(None, is_root)
    }
}
impl WithDeserializer for Base {
    type Deserializer = quick_xml_deserialize::BaseDeserializer;
}
pub type IntermediateDyn = Nillable<IntermediateType>;
impl BaseTrait for IntermediateDyn {}
impl IntermediateTrait for IntermediateDyn {}
pub type FinalDyn = Nillable<FinalType>;
impl BaseTrait for FinalDyn {}
impl IntermediateTrait for FinalDyn {}
#[derive(Debug)]
pub struct IntermediateType {
    pub base_value: Option<i32>,
    pub intermediate_value: Option<i32>,
}
impl WithSerializer for IntermediateType {
    type Serializer<'x> = quick_xml_serialize::IntermediateTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::IntermediateTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::IntermediateTypeSerializerState::Init__),
            name: name.unwrap_or("intermediate"),
            is_root,
        })
    }
}
impl WithDeserializer for IntermediateType {
    type Deserializer = quick_xml_deserialize::IntermediateTypeDeserializer;
}
#[derive(Debug)]
pub struct Intermediate(pub Box<dyn IntermediateTrait>);
pub trait IntermediateTrait: BaseTrait {}
impl WithSerializer for Intermediate {
    type Serializer<'x> = BoxedSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        self.0.serializer(None, is_root)
    }
}
impl WithDeserializer for Intermediate {
    type Deserializer = quick_xml_deserialize::IntermediateDeserializer;
}
#[derive(Debug)]
pub struct FinalType {
    pub base_value: Option<i32>,
    pub intermediate_value: Option<i32>,
    pub final_value: Option<i32>,
}
impl WithSerializer for FinalType {
    type Serializer<'x> = quick_xml_serialize::FinalTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::FinalTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::FinalTypeSerializerState::Init__),
            name: name.unwrap_or("final"),
            is_root,
        })
    }
}
impl WithDeserializer for FinalType {
    type Deserializer = quick_xml_deserialize::FinalTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, Event, QName,
        WithDeserializer,
    };
    #[derive(Debug)]
    pub struct ListTypeDeserializer {
        base: Vec<super::Base>,
        state__: Box<ListTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ListTypeDeserializerState {
        Init__,
        Base(Option<<super::Base as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
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
                base: Vec::new(),
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
                S::Base(Some(deserializer)) => self.store_base(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_base(&mut self, value: super::Base) -> Result<(), Error> {
            self.base.push(value);
            Ok(())
        }
        fn handle_base<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::Base>,
            fallback: &mut Option<ListTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(ListTypeDeserializerState::Base(None));
                *self.state__ = ListTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_base(data)?;
                    *self.state__ = ListTypeDeserializerState::Base(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(ListTypeDeserializerState::Base(Some(deserializer)));
                    *self.state__ = ListTypeDeserializerState::Base(None);
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
                    (S::Base(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_base(helper, output, &mut fallback)? {
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
                        *self.state__ = ListTypeDeserializerState::Base(None);
                        event
                    }
                    (S::Base(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = <super::Base as WithDeserializer>::init(helper, event)?;
                        match self.handle_base(helper, output, &mut fallback)? {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ListType, Error> {
            let state = replace(&mut *self.state__, ListTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::ListType { base: self.base })
        }
    }
    #[derive(Debug)]
    pub enum BaseDeserializer {
        Intermediate(<super::IntermediateDyn as WithDeserializer>::Deserializer),
        Final(<super::FinalDyn as WithDeserializer>::Deserializer),
    }
    impl<'de> Deserializer<'de, super::Base> for BaseDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Base> {
            let Some(type_name) = helper.get_dynamic_type_name(&event)? else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::None,
                    allow_any: false,
                });
            };
            let type_name = type_name.into_owned();
            if matches!(
                helper.resolve_local_name(QName(&type_name), &super::NS_TNS),
                Some(b"intermediate")
            ) {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = <super::IntermediateDyn as WithDeserializer>::init(helper, event)?;
                return Ok(DeserializerOutput {
                    artifact: artifact.map(|x| super::Base(Box::new(x)), |x| Self::Intermediate(x)),
                    event,
                    allow_any,
                });
            }
            if matches!(
                helper.resolve_local_name(QName(&type_name), &super::NS_TNS),
                Some(b"final")
            ) {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = <super::FinalDyn as WithDeserializer>::init(helper, event)?;
                return Ok(DeserializerOutput {
                    artifact: artifact.map(|x| super::Base(Box::new(x)), |x| Self::Final(x)),
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
        ) -> DeserializerResult<'de, super::Base> {
            match self {
                Self::Intermediate(x) => {
                    let DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = x.next(helper, event)?;
                    Ok(DeserializerOutput {
                        artifact: artifact
                            .map(|x| super::Base(Box::new(x)), |x| Self::Intermediate(x)),
                        event,
                        allow_any,
                    })
                }
                Self::Final(x) => {
                    let DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = x.next(helper, event)?;
                    Ok(DeserializerOutput {
                        artifact: artifact.map(|x| super::Base(Box::new(x)), |x| Self::Final(x)),
                        event,
                        allow_any,
                    })
                }
            }
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::Base, Error> {
            match self {
                Self::Intermediate(x) => Ok(super::Base(Box::new(x.finish(helper)?))),
                Self::Final(x) => Ok(super::Base(Box::new(x.finish(helper)?))),
            }
        }
    }
    #[derive(Debug)]
    pub struct IntermediateTypeDeserializer {
        base_value: Option<i32>,
        intermediate_value: Option<i32>,
        state__: Box<IntermediateTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum IntermediateTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl IntermediateTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut base_value: Option<i32> = None;
            let mut intermediate_value: Option<i32> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"baseValue")
                ) {
                    helper.read_attrib(&mut base_value, b"baseValue", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"intermediateValue")
                ) {
                    helper.read_attrib(
                        &mut intermediate_value,
                        b"intermediateValue",
                        &attrib.value,
                    )?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                base_value: base_value,
                intermediate_value: intermediate_value,
                state__: Box::new(IntermediateTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: IntermediateTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::IntermediateType> for IntermediateTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IntermediateType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IntermediateType> {
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
        ) -> Result<super::IntermediateType, Error> {
            let state = replace(
                &mut *self.state__,
                IntermediateTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::IntermediateType {
                base_value: self.base_value,
                intermediate_value: self.intermediate_value,
            })
        }
    }
    #[derive(Debug)]
    pub enum IntermediateDeserializer {
        Intermediate(<super::IntermediateDyn as WithDeserializer>::Deserializer),
        Final(<super::FinalDyn as WithDeserializer>::Deserializer),
    }
    impl<'de> Deserializer<'de, super::Intermediate> for IntermediateDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Intermediate> {
            let Some(type_name) = helper.get_dynamic_type_name(&event)? else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::None,
                    allow_any: false,
                });
            };
            let type_name = type_name.into_owned();
            if matches!(
                helper.resolve_local_name(QName(&type_name), &super::NS_TNS),
                Some(b"intermediate")
            ) {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = <super::IntermediateDyn as WithDeserializer>::init(helper, event)?;
                return Ok(DeserializerOutput {
                    artifact: artifact.map(
                        |x| super::Intermediate(Box::new(x)),
                        |x| Self::Intermediate(x),
                    ),
                    event,
                    allow_any,
                });
            }
            if matches!(
                helper.resolve_local_name(QName(&type_name), &super::NS_TNS),
                Some(b"final")
            ) {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = <super::FinalDyn as WithDeserializer>::init(helper, event)?;
                return Ok(DeserializerOutput {
                    artifact: artifact
                        .map(|x| super::Intermediate(Box::new(x)), |x| Self::Final(x)),
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
        ) -> DeserializerResult<'de, super::Intermediate> {
            match self {
                Self::Intermediate(x) => {
                    let DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = x.next(helper, event)?;
                    Ok(DeserializerOutput {
                        artifact: artifact.map(
                            |x| super::Intermediate(Box::new(x)),
                            |x| Self::Intermediate(x),
                        ),
                        event,
                        allow_any,
                    })
                }
                Self::Final(x) => {
                    let DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = x.next(helper, event)?;
                    Ok(DeserializerOutput {
                        artifact: artifact
                            .map(|x| super::Intermediate(Box::new(x)), |x| Self::Final(x)),
                        event,
                        allow_any,
                    })
                }
            }
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::Intermediate, Error> {
            match self {
                Self::Intermediate(x) => Ok(super::Intermediate(Box::new(x.finish(helper)?))),
                Self::Final(x) => Ok(super::Intermediate(Box::new(x.finish(helper)?))),
            }
        }
    }
    #[derive(Debug)]
    pub struct FinalTypeDeserializer {
        base_value: Option<i32>,
        intermediate_value: Option<i32>,
        final_value: Option<i32>,
        state__: Box<FinalTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FinalTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl FinalTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut base_value: Option<i32> = None;
            let mut intermediate_value: Option<i32> = None;
            let mut final_value: Option<i32> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"baseValue")
                ) {
                    helper.read_attrib(&mut base_value, b"baseValue", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"intermediateValue")
                ) {
                    helper.read_attrib(
                        &mut intermediate_value,
                        b"intermediateValue",
                        &attrib.value,
                    )?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"finalValue")
                ) {
                    helper.read_attrib(&mut final_value, b"finalValue", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                base_value: base_value,
                intermediate_value: intermediate_value,
                final_value: final_value,
                state__: Box::new(FinalTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: FinalTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::FinalType> for FinalTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FinalType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FinalType> {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::FinalType, Error> {
            let state = replace(&mut *self.state__, FinalTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::FinalType {
                base_value: self.base_value,
                intermediate_value: self.intermediate_value,
                final_value: self.final_value,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::{
        misc::{Namespace, NamespacePrefix},
        quick_xml::{
            BytesEnd, BytesStart, Error, Event, IterSerializer, SerializeHelper, Serializer,
        },
    };
    #[derive(Debug)]
    pub struct ListTypeSerializer<'ser> {
        pub(super) value: &'ser super::ListType,
        pub(super) state: Box<ListTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ListTypeSerializerState<'ser> {
        Init__,
        Base(IterSerializer<'ser, &'ser [super::Base], super::Base>),
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
                        *self.state = ListTypeSerializerState::Base(IterSerializer::new(
                            &self.value.base[..],
                            Some("base"),
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&NamespacePrefix::XSI),
                                &Namespace::XSI,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ListTypeSerializerState::Base(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ListTypeSerializerState::End__,
                    },
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
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
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
    #[derive(Debug)]
    pub struct IntermediateTypeSerializer<'ser> {
        pub(super) value: &'ser super::IntermediateType,
        pub(super) state: Box<IntermediateTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum IntermediateTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> IntermediateTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    IntermediateTypeSerializerState::Init__ => {
                        *self.state = IntermediateTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&NamespacePrefix::XSI),
                                &Namespace::XSI,
                            );
                        }
                        helper.write_attrib_opt(&mut bytes, "baseValue", &self.value.base_value)?;
                        helper.write_attrib_opt(
                            &mut bytes,
                            "intermediateValue",
                            &self.value.intermediate_value,
                        )?;
                        helper.end_ns_scope();
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    IntermediateTypeSerializerState::Done__ => return Ok(None),
                    IntermediateTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for IntermediateTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = IntermediateTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct FinalTypeSerializer<'ser> {
        pub(super) value: &'ser super::FinalType,
        pub(super) state: Box<FinalTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum FinalTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FinalTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FinalTypeSerializerState::Init__ => {
                        *self.state = FinalTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&NamespacePrefix::XSI),
                                &Namespace::XSI,
                            );
                        }
                        helper.write_attrib_opt(&mut bytes, "baseValue", &self.value.base_value)?;
                        helper.write_attrib_opt(
                            &mut bytes,
                            "intermediateValue",
                            &self.value.intermediate_value,
                        )?;
                        helper.write_attrib_opt(
                            &mut bytes,
                            "finalValue",
                            &self.value.final_value,
                        )?;
                        helper.end_ns_scope();
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    FinalTypeSerializerState::Done__ => return Ok(None),
                    FinalTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for FinalTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = FinalTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
