use core::fmt::Debug;
use xsd_parser::{
    quick_xml::{BoxedSerializer, Error, WithBoxedSerializer, WithDeserializer, WithSerializer},
    schema::Namespace,
    AsAny,
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
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
            name: name.unwrap_or("tns:list"),
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
#[derive(Debug)]
pub struct IntermediateType {
    pub base_value: Option<i32>,
    pub intermediate_value: Option<i32>,
}
impl BaseTrait for IntermediateType {}
impl IntermediateTrait for IntermediateType {}
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
            name: name.unwrap_or("tns:intermediate"),
            is_root,
        })
    }
}
impl WithDeserializer for IntermediateType {
    type Deserializer = quick_xml_deserialize::IntermediateTypeDeserializer;
}
#[derive(Debug)]
pub struct FinalType {
    pub base_value: Option<i32>,
    pub intermediate_value: Option<i32>,
    pub final_value: Option<i32>,
}
impl BaseTrait for FinalType {}
impl IntermediateTrait for FinalType {}
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
            name: name.unwrap_or("tns:final"),
            is_root,
        })
    }
}
impl WithDeserializer for FinalType {
    type Deserializer = quick_xml_deserialize::FinalTypeDeserializer;
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
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
        Event, QName, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct ListTypeDeserializer {
        base: Vec<super::Base>,
        state: Box<ListTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ListTypeDeserializerState {
        Init__,
        Base(Option<<super::Base as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ListTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                base: Vec::new(),
                state: Box::new(ListTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ListTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use ListTypeDeserializerState as S;
            match state {
                S::Base(Some(deserializer)) => self.store_base(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_base(&mut self, value: super::Base) -> Result<(), Error> {
            self.base.push(value);
            Ok(())
        }
        fn handle_base<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::Base>,
            fallback: &mut Option<ListTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(ListTypeDeserializerState::Base(None));
                *self.state = ListTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_base(data)?;
                    *self.state = ListTypeDeserializerState::Base(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(ListTypeDeserializerState::Base(Some(deserializer)));
                            *self.state = ListTypeDeserializerState::Base(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ListTypeDeserializerState::Base(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ListType> for ListTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ListType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ListType>
        where
            R: DeserializeReader,
        {
            use ListTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Base(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_base(reader, output, &mut fallback)? {
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
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = ListTypeDeserializerState::Base(None);
                        event
                    }
                    (S::Base(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::Base as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_base(reader, output, &mut fallback)? {
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
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ListType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, ListTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::ListType { base: self.base })
        }
    }
    #[derive(Debug)]
    pub enum BaseDeserializer {
        Intermediate(<super::IntermediateType as WithDeserializer>::Deserializer),
        Final(<super::FinalType as WithDeserializer>::Deserializer),
    }
    impl<'de> Deserializer<'de, super::Base> for BaseDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Base>
        where
            R: DeserializeReader,
        {
            let Some(type_name) = reader.get_dynamic_type_name(&event)? else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::None,
                    allow_any: false,
                });
            };
            let type_name = type_name.into_owned();
            if matches!(
                reader.resolve_local_name(QName(&type_name), &super::NS_TNS),
                Some(b"intermediate")
            ) {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = <super::IntermediateType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return Ok(DeserializerOutput {
                    artifact: artifact.map(|x| super::Base(Box::new(x)), |x| Self::Intermediate(x)),
                    event,
                    allow_any,
                });
            }
            if matches!(
                reader.resolve_local_name(QName(&type_name), &super::NS_TNS),
                Some(b"final")
            ) {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = <super::FinalType as WithDeserializer>::Deserializer::init(reader, event)?;
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
        fn next<R>(self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Base>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Intermediate(x) => {
                    let DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = x.next(reader, event)?;
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
                    } = x.next(reader, event)?;
                    Ok(DeserializerOutput {
                        artifact: artifact.map(|x| super::Base(Box::new(x)), |x| Self::Final(x)),
                        event,
                        allow_any,
                    })
                }
            }
        }
        fn finish<R>(self, reader: &R) -> Result<super::Base, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Intermediate(x) => Ok(super::Base(Box::new(x.finish(reader)?))),
                Self::Final(x) => Ok(super::Base(Box::new(x.finish(reader)?))),
            }
        }
    }
    #[derive(Debug)]
    pub struct IntermediateTypeDeserializer {
        base_value: Option<i32>,
        intermediate_value: Option<i32>,
        state: Box<IntermediateTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum IntermediateTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl IntermediateTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut base_value: Option<i32> = None;
            let mut intermediate_value: Option<i32> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"baseValue")
                ) {
                    reader.read_attrib(&mut base_value, b"baseValue", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"intermediateValue")
                ) {
                    reader.read_attrib(
                        &mut intermediate_value,
                        b"intermediateValue",
                        &attrib.value,
                    )?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
            Ok(Self {
                base_value: base_value,
                intermediate_value: intermediate_value,
                state: Box::new(IntermediateTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: IntermediateTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::IntermediateType> for IntermediateTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IntermediateType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IntermediateType>
        where
            R: DeserializeReader,
        {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(reader)?),
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
        fn finish<R>(mut self, reader: &R) -> Result<super::IntermediateType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                IntermediateTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::IntermediateType {
                base_value: self.base_value,
                intermediate_value: self.intermediate_value,
            })
        }
    }
    #[derive(Debug)]
    pub struct FinalTypeDeserializer {
        base_value: Option<i32>,
        intermediate_value: Option<i32>,
        final_value: Option<i32>,
        state: Box<FinalTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FinalTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl FinalTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut base_value: Option<i32> = None;
            let mut intermediate_value: Option<i32> = None;
            let mut final_value: Option<i32> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"baseValue")
                ) {
                    reader.read_attrib(&mut base_value, b"baseValue", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"intermediateValue")
                ) {
                    reader.read_attrib(
                        &mut intermediate_value,
                        b"intermediateValue",
                        &attrib.value,
                    )?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"finalValue")
                ) {
                    reader.read_attrib(&mut final_value, b"finalValue", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
            Ok(Self {
                base_value: base_value,
                intermediate_value: intermediate_value,
                final_value: final_value,
                state: Box::new(FinalTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: FinalTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::FinalType> for FinalTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::FinalType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FinalType>
        where
            R: DeserializeReader,
        {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(reader)?),
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
        fn finish<R>(mut self, reader: &R) -> Result<super::FinalType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, FinalTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::FinalType {
                base_value: self.base_value,
                intermediate_value: self.intermediate_value,
                final_value: self.final_value,
            })
        }
    }
    #[derive(Debug)]
    pub enum IntermediateDeserializer {
        Intermediate(<super::IntermediateType as WithDeserializer>::Deserializer),
        Final(<super::FinalType as WithDeserializer>::Deserializer),
    }
    impl<'de> Deserializer<'de, super::Intermediate> for IntermediateDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Intermediate>
        where
            R: DeserializeReader,
        {
            let Some(type_name) = reader.get_dynamic_type_name(&event)? else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::None,
                    allow_any: false,
                });
            };
            let type_name = type_name.into_owned();
            if matches!(
                reader.resolve_local_name(QName(&type_name), &super::NS_TNS),
                Some(b"intermediate")
            ) {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = <super::IntermediateType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
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
                reader.resolve_local_name(QName(&type_name), &super::NS_TNS),
                Some(b"final")
            ) {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = <super::FinalType as WithDeserializer>::Deserializer::init(reader, event)?;
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
        fn next<R>(
            self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Intermediate>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Intermediate(x) => {
                    let DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = x.next(reader, event)?;
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
                    } = x.next(reader, event)?;
                    Ok(DeserializerOutput {
                        artifact: artifact
                            .map(|x| super::Intermediate(Box::new(x)), |x| Self::Final(x)),
                        event,
                        allow_any,
                    })
                }
            }
        }
        fn finish<R>(self, reader: &R) -> Result<super::Intermediate, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Intermediate(x) => Ok(super::Intermediate(Box::new(x.finish(reader)?))),
                Self::Final(x) => Ok(super::Intermediate(Box::new(x.finish(reader)?))),
            }
        }
    }
}
pub mod quick_xml_serialize {
    use core::iter::Iterator;
    use xsd_parser::quick_xml::{
        write_attrib_opt, BytesEnd, BytesStart, Error, Event, IterSerializer,
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ListTypeSerializerState::Init__ => {
                        *self.state = ListTypeSerializerState::Base(IterSerializer::new(
                            &self.value.base[..],
                            Some("tns:base"),
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ListTypeSerializerState::Base(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ListTypeSerializerState::End__,
                    },
                    ListTypeSerializerState::End__ => {
                        *self.state = ListTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ListTypeSerializerState::Done__ => return Ok(None),
                    ListTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ListTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    IntermediateTypeSerializerState::Init__ => {
                        *self.state = IntermediateTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        write_attrib_opt(&mut bytes, "tns:baseValue", &self.value.base_value)?;
                        write_attrib_opt(
                            &mut bytes,
                            "tns:intermediateValue",
                            &self.value.intermediate_value,
                        )?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    IntermediateTypeSerializerState::Done__ => return Ok(None),
                    IntermediateTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for IntermediateTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FinalTypeSerializerState::Init__ => {
                        *self.state = FinalTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        write_attrib_opt(&mut bytes, "tns:baseValue", &self.value.base_value)?;
                        write_attrib_opt(
                            &mut bytes,
                            "tns:intermediateValue",
                            &self.value.intermediate_value,
                        )?;
                        write_attrib_opt(&mut bytes, "tns:finalValue", &self.value.final_value)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    FinalTypeSerializerState::Done__ => return Ok(None),
                    FinalTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for FinalTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
