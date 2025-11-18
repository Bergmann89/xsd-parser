use xsd_parser_types::{
    misc::Namespace,
    quick_xml::{Error, WithDeserializer, WithSerializer},
    xml::Nillable,
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
            name: name.unwrap_or("list"),
            is_root,
        })
    }
}
impl WithDeserializer for ListType {
    type Deserializer = quick_xml_deserialize::ListTypeDeserializer;
}
#[derive(Debug)]
pub enum Base {
    Intermediate(IntermediateDyn),
    Final(FinalDyn),
}
impl WithSerializer for Base {
    type Serializer<'x> = quick_xml_serialize::BaseSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        Ok(quick_xml_serialize::BaseSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::BaseSerializerState::Init__),
            is_root,
        })
    }
}
impl WithDeserializer for Base {
    type Deserializer = quick_xml_deserialize::BaseDeserializer;
}
pub type IntermediateDyn = Nillable<IntermediateType>;
pub type FinalDyn = Nillable<FinalType>;
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
        filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
        ErrorKind, Event, RawByteStr, WithDeserializer,
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
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                base: Vec::new(),
                state__: Box::new(ListTypeDeserializerState::Init__),
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
                *self.state__ = ListTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_base(data)?;
                    *self.state__ = ListTypeDeserializerState::Base(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(ListTypeDeserializerState::Base(Some(deserializer)));
                            *self.state__ = ListTypeDeserializerState::Base(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ = ListTypeDeserializerState::Base(Some(deserializer));
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
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
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
                        *self.state__ = ListTypeDeserializerState::Base(None);
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
        fn finish<R>(mut self, reader: &R) -> Result<super::ListType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state__, ListTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::ListType { base: self.base })
        }
    }
    #[derive(Debug)]
    pub struct BaseDeserializer {
        state__: Box<BaseDeserializerState>,
    }
    #[derive(Debug)]
    pub enum BaseDeserializerState {
        Init__,
        Intermediate(
            Option<super::IntermediateDyn>,
            Option<<super::IntermediateDyn as WithDeserializer>::Deserializer>,
        ),
        Final(
            Option<super::FinalDyn>,
            Option<<super::FinalDyn as WithDeserializer>::Deserializer>,
        ),
        Done__(super::Base),
        Unknown__,
    }
    impl BaseDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<BaseDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"intermediate")
                ) {
                    let output = <super::IntermediateDyn as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_intermediate(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"final")
                ) {
                    let output =
                        <super::FinalDyn as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_final_(reader, Default::default(), output, &mut *fallback);
                }
            }
            *self.state__ = fallback.take().unwrap_or(BaseDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(reader: &R, state: BaseDeserializerState) -> Result<super::Base, Error>
        where
            R: DeserializeReader,
        {
            use BaseDeserializerState as S;
            match state {
                S::Unknown__ => unreachable!(),
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Intermediate(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_intermediate(&mut values, value)?;
                    }
                    Ok(super::Base::Intermediate(values.ok_or_else(|| {
                        ErrorKind::MissingElement("intermediate".into())
                    })?))
                }
                S::Final(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_final_(&mut values, value)?;
                    }
                    Ok(super::Base::Final(values.ok_or_else(|| {
                        ErrorKind::MissingElement("final".into())
                    })?))
                }
                S::Done__(data) => Ok(data),
            }
        }
        fn store_intermediate(
            values: &mut Option<super::IntermediateDyn>,
            value: super::IntermediateDyn,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"intermediate",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_final_(
            values: &mut Option<super::FinalDyn>,
            value: super::FinalDyn,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"final",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_intermediate<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::IntermediateDyn>,
            output: DeserializerOutput<'de, super::IntermediateDyn>,
            fallback: &mut Option<BaseDeserializerState>,
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
                *self.state__ = match fallback.take() {
                    None if values.is_none() => {
                        *self.state__ = BaseDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => BaseDeserializerState::Intermediate(values, None),
                    Some(BaseDeserializerState::Intermediate(_, Some(deserializer))) => {
                        BaseDeserializerState::Intermediate(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(BaseDeserializerState::Intermediate(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_intermediate(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_intermediate(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        BaseDeserializerState::Intermediate(values, None),
                    )?;
                    *self.state__ = BaseDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = BaseDeserializerState::Intermediate(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_final_<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::FinalDyn>,
            output: DeserializerOutput<'de, super::FinalDyn>,
            fallback: &mut Option<BaseDeserializerState>,
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
                *self.state__ = match fallback.take() {
                    None if values.is_none() => {
                        *self.state__ = BaseDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => BaseDeserializerState::Final(values, None),
                    Some(BaseDeserializerState::Final(_, Some(deserializer))) => {
                        BaseDeserializerState::Final(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(BaseDeserializerState::Final(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_final_(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_final_(&mut values, data)?;
                    let data =
                        Self::finish_state(reader, BaseDeserializerState::Final(values, None))?;
                    *self.state__ = BaseDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = BaseDeserializerState::Final(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::Base> for BaseDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Base>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state__: Box::new(BaseDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, BaseDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Base>
        where
            R: DeserializeReader,
        {
            use BaseDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Intermediate(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_intermediate(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Final(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_final_(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Intermediate(values, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"intermediate",
                            false,
                        )?;
                        match self.handle_intermediate(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Final(values, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"final",
                            false,
                        )?;
                        match self.handle_final_(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state__ = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::Base, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state__)
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
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                base_value: base_value,
                intermediate_value: intermediate_value,
                state__: Box::new(IntermediateTypeDeserializerState::Init__),
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
                &mut *self.state__,
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
        state__: Box<FinalTypeDeserializerState>,
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
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                base_value: base_value,
                intermediate_value: intermediate_value,
                final_value: final_value,
                state__: Box::new(FinalTypeDeserializerState::Init__),
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
            let state = replace(&mut *self.state__, FinalTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
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
        misc::Namespace,
        quick_xml::{
            write_attrib_opt, BytesEnd, BytesStart, Error, Event, IterSerializer, WithSerializer,
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ListTypeSerializerState::Init__ => {
                        *self.state = ListTypeSerializerState::Base(IterSerializer::new(
                            &self.value.base[..],
                            Some("base"),
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                            bytes.push_attribute((&b"xmlns:xsi"[..], &Namespace::XSI[..]));
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
    pub struct BaseSerializer<'ser> {
        pub(super) value: &'ser super::Base,
        pub(super) state: Box<BaseSerializerState<'ser>>,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum BaseSerializerState<'ser> {
        Init__,
        Intermediate(<super::IntermediateDyn as WithSerializer>::Serializer<'ser>),
        Final(<super::FinalDyn as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> BaseSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    BaseSerializerState::Init__ => match self.value {
                        super::Base::Intermediate(x) => {
                            *self.state = BaseSerializerState::Intermediate(
                                WithSerializer::serializer(x, Some("intermediate"), self.is_root)?,
                            )
                        }
                        super::Base::Final(x) => {
                            *self.state = BaseSerializerState::Final(WithSerializer::serializer(
                                x,
                                Some("final"),
                                self.is_root,
                            )?)
                        }
                    },
                    BaseSerializerState::Intermediate(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = BaseSerializerState::Done__,
                    },
                    BaseSerializerState::Final(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = BaseSerializerState::Done__,
                    },
                    BaseSerializerState::Done__ => return Ok(None),
                    BaseSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for BaseSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = BaseSerializerState::Done__;
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
                            bytes.push_attribute((&b"xmlns:xsi"[..], &Namespace::XSI[..]));
                        }
                        write_attrib_opt(&mut bytes, "baseValue", &self.value.base_value)?;
                        write_attrib_opt(
                            &mut bytes,
                            "intermediateValue",
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
                            bytes.push_attribute((&b"xmlns:xsi"[..], &Namespace::XSI[..]));
                        }
                        write_attrib_opt(&mut bytes, "baseValue", &self.value.base_value)?;
                        write_attrib_opt(
                            &mut bytes,
                            "intermediateValue",
                            &self.value.intermediate_value,
                        )?;
                        write_attrib_opt(&mut bytes, "finalValue", &self.value.final_value)?;
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
