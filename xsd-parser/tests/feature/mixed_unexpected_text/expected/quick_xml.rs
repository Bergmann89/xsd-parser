use xsd_parser_types::{
    misc::Namespace,
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub type Mixed = MixedType;
#[derive(Debug)]
pub struct MixedType {
    pub group: Vec<MixedGroupType>,
    pub baz: Vec<i32>,
}
impl WithSerializer for MixedType {
    type Serializer<'x> = quick_xml_serialize::MixedTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::MixedTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::MixedTypeSerializerState::Init__),
            name: name.unwrap_or("tns:MixedType"),
            is_root,
        })
    }
}
impl WithDeserializer for MixedType {
    type Deserializer = quick_xml_deserialize::MixedTypeDeserializer;
}
#[derive(Debug)]
pub enum MixedGroupType {
    Fuu(Vec<i32>),
    Bar(Vec<i32>),
}
impl WithSerializer for MixedGroupType {
    type Serializer<'x> = quick_xml_serialize::MixedGroupTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        Ok(quick_xml_serialize::MixedGroupTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::MixedGroupTypeSerializerState::Init__),
            is_root,
        })
    }
}
impl WithDeserializer for MixedGroupType {
    type Deserializer = quick_xml_deserialize::MixedGroupTypeDeserializer;
}
pub type Normal = NormalType;
#[derive(Debug)]
pub struct NormalType {
    pub group: Vec<NormalGroupType>,
    pub baz: Vec<i32>,
}
impl WithSerializer for NormalType {
    type Serializer<'x> = quick_xml_serialize::NormalTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::NormalTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::NormalTypeSerializerState::Init__),
            name: name.unwrap_or("tns:NormalType"),
            is_root,
        })
    }
}
impl WithDeserializer for NormalType {
    type Deserializer = quick_xml_deserialize::NormalTypeDeserializer;
}
#[derive(Debug)]
pub enum NormalGroupType {
    Fuu(Vec<i32>),
    Bar(Vec<i32>),
}
impl WithSerializer for NormalGroupType {
    type Serializer<'x> = quick_xml_serialize::NormalGroupTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        Ok(quick_xml_serialize::NormalGroupTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::NormalGroupTypeSerializerState::Init__),
            is_root,
        })
    }
}
impl WithDeserializer for NormalGroupType {
    type Deserializer = quick_xml_deserialize::NormalGroupTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        WithDeserializer,
    };
    #[derive(Debug)]
    pub struct MixedTypeDeserializer {
        group: Vec<super::MixedGroupType>,
        baz: Vec<i32>,
        state__: Box<MixedTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum MixedTypeDeserializerState {
        Init__,
        Group(Option<<super::MixedGroupType as WithDeserializer>::Deserializer>),
        Baz(Option<<i32 as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl MixedTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                group: Vec::new(),
                baz: Vec::new(),
                state__: Box::new(MixedTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: MixedTypeDeserializerState,
        ) -> Result<(), Error> {
            use MixedTypeDeserializerState as S;
            match state {
                S::Group(Some(deserializer)) => self.store_group(deserializer.finish(helper)?)?,
                S::Baz(Some(deserializer)) => self.store_baz(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_group(&mut self, value: super::MixedGroupType) -> Result<(), Error> {
            self.group.push(value);
            Ok(())
        }
        fn store_baz(&mut self, value: i32) -> Result<(), Error> {
            self.baz.push(value);
            Ok(())
        }
        fn handle_group<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::MixedGroupType>,
            fallback: &mut Option<MixedTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.group.len() < 1usize {
                    *self.state__ = MixedTypeDeserializerState::Group(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(MixedTypeDeserializerState::Group(None));
                    *self.state__ = MixedTypeDeserializerState::Baz(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_group(data)?;
                    *self.state__ = MixedTypeDeserializerState::Group(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(MixedTypeDeserializerState::Group(Some(
                                deserializer,
                            )));
                            *self.state__ = MixedTypeDeserializerState::Group(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ = MixedTypeDeserializerState::Group(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_baz<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<MixedTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.baz.len() < 1usize {
                    *self.state__ = MixedTypeDeserializerState::Baz(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(MixedTypeDeserializerState::Baz(None));
                    *self.state__ = MixedTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_baz(data)?;
                    *self.state__ = MixedTypeDeserializerState::Baz(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(MixedTypeDeserializerState::Baz(Some(deserializer)));
                            *self.state__ = MixedTypeDeserializerState::Baz(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ = MixedTypeDeserializerState::Baz(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::MixedType> for MixedTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedType> {
            use MixedTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Group(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_group(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Baz(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_baz(helper, output, &mut fallback)? {
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
                        *self.state__ = MixedTypeDeserializerState::Group(None);
                        event
                    }
                    (S::Group(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::MixedGroupType as WithDeserializer>::Deserializer::init(
                                helper, event,
                            )?;
                        match self.handle_group(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Baz(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Baz",
                            false,
                        )?;
                        match self.handle_baz(helper, output, &mut fallback)? {
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
                    (state, Event::Text(_) | Event::CData(_)) => {
                        *self.state__ = state;
                        break (DeserializerEvent::None, false);
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::MixedType, Error> {
            let state = replace(&mut *self.state__, MixedTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::MixedType {
                group: self.group,
                baz: self.baz,
            })
        }
    }
    #[derive(Debug)]
    pub struct MixedGroupTypeDeserializer {
        state__: Box<MixedGroupTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum MixedGroupTypeDeserializerState {
        Init__,
        Fuu(Vec<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Bar(Vec<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Done__(super::MixedGroupType),
        Unknown__,
    }
    impl MixedGroupTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
            fallback: &mut Option<MixedGroupTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Fuu")
                ) {
                    let output = <i32 as WithDeserializer>::Deserializer::init(helper, event)?;
                    return self.handle_fuu(helper, Default::default(), output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Bar")
                ) {
                    let output = <i32 as WithDeserializer>::Deserializer::init(helper, event)?;
                    return self.handle_bar(helper, Default::default(), output, &mut *fallback);
                }
            }
            *self.state__ = fallback
                .take()
                .unwrap_or(MixedGroupTypeDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: MixedGroupTypeDeserializerState,
        ) -> Result<super::MixedGroupType, Error> {
            use MixedGroupTypeDeserializerState as S;
            match state {
                S::Unknown__ => unreachable!(),
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Fuu(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_fuu(&mut values, value)?;
                    }
                    Ok(super::MixedGroupType::Fuu(values))
                }
                S::Bar(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_bar(&mut values, value)?;
                    }
                    Ok(super::MixedGroupType::Bar(values))
                }
                S::Done__(data) => Ok(data),
            }
        }
        fn store_fuu(values: &mut Vec<i32>, value: i32) -> Result<(), Error> {
            values.push(value);
            Ok(())
        }
        fn store_bar(values: &mut Vec<i32>, value: i32) -> Result<(), Error> {
            values.push(value);
            Ok(())
        }
        fn handle_fuu<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Vec<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<MixedGroupTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = match fallback.take() {
                    None if values.is_empty() => {
                        *self.state__ = MixedGroupTypeDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => MixedGroupTypeDeserializerState::Fuu(values, None),
                    Some(MixedGroupTypeDeserializerState::Fuu(_, Some(deserializer))) => {
                        MixedGroupTypeDeserializerState::Fuu(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(MixedGroupTypeDeserializerState::Fuu(_, Some(deserializer))) => {
                    let data = deserializer.finish(helper)?;
                    Self::store_fuu(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fuu(&mut values, data)?;
                    *self.state__ = MixedGroupTypeDeserializerState::Fuu(values, None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                MixedGroupTypeDeserializerState::Fuu(values, Some(deserializer));
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(MixedGroupTypeDeserializerState::Fuu(
                                Default::default(),
                                Some(deserializer),
                            ));
                            *self.state__ = MixedGroupTypeDeserializerState::Fuu(values, None);
                        }
                    }
                    ret
                }
            })
        }
        fn handle_bar<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Vec<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<MixedGroupTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = match fallback.take() {
                    None if values.is_empty() => {
                        *self.state__ = MixedGroupTypeDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => MixedGroupTypeDeserializerState::Bar(values, None),
                    Some(MixedGroupTypeDeserializerState::Bar(_, Some(deserializer))) => {
                        MixedGroupTypeDeserializerState::Bar(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(MixedGroupTypeDeserializerState::Bar(_, Some(deserializer))) => {
                    let data = deserializer.finish(helper)?;
                    Self::store_bar(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bar(&mut values, data)?;
                    *self.state__ = MixedGroupTypeDeserializerState::Bar(values, None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                MixedGroupTypeDeserializerState::Bar(values, Some(deserializer));
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(MixedGroupTypeDeserializerState::Bar(
                                Default::default(),
                                Some(deserializer),
                            ));
                            *self.state__ = MixedGroupTypeDeserializerState::Bar(values, None);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::MixedGroupType> for MixedGroupTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedGroupType> {
            let deserializer = Self {
                state__: Box::new(MixedGroupTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, MixedGroupTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::MixedGroupType> {
            use MixedGroupTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Fuu(values, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fuu(helper, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bar(values, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_bar(helper, values, output, &mut fallback)? {
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
                    (S::Init__, event) => match self.find_suitable(helper, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::Fuu(values, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Fuu",
                            false,
                        )?;
                        match self.handle_fuu(helper, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bar(values, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Bar",
                            false,
                        )?;
                        match self.handle_bar(helper, values, output, &mut fallback)? {
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
                    (state, Event::Text(_) | Event::CData(_)) => {
                        *self.state__ = state;
                        break (DeserializerEvent::None, false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::MixedGroupType, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct NormalTypeDeserializer {
        group: Vec<super::NormalGroupType>,
        baz: Vec<i32>,
        state__: Box<NormalTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum NormalTypeDeserializerState {
        Init__,
        Group(Option<<super::NormalGroupType as WithDeserializer>::Deserializer>),
        Baz(Option<<i32 as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl NormalTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                group: Vec::new(),
                baz: Vec::new(),
                state__: Box::new(NormalTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: NormalTypeDeserializerState,
        ) -> Result<(), Error> {
            use NormalTypeDeserializerState as S;
            match state {
                S::Group(Some(deserializer)) => self.store_group(deserializer.finish(helper)?)?,
                S::Baz(Some(deserializer)) => self.store_baz(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_group(&mut self, value: super::NormalGroupType) -> Result<(), Error> {
            self.group.push(value);
            Ok(())
        }
        fn store_baz(&mut self, value: i32) -> Result<(), Error> {
            self.baz.push(value);
            Ok(())
        }
        fn handle_group<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::NormalGroupType>,
            fallback: &mut Option<NormalTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.group.len() < 1usize {
                    *self.state__ = NormalTypeDeserializerState::Group(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(NormalTypeDeserializerState::Group(None));
                    *self.state__ = NormalTypeDeserializerState::Baz(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_group(data)?;
                    *self.state__ = NormalTypeDeserializerState::Group(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(NormalTypeDeserializerState::Group(Some(
                                deserializer,
                            )));
                            *self.state__ = NormalTypeDeserializerState::Group(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ = NormalTypeDeserializerState::Group(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_baz<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<NormalTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.baz.len() < 1usize {
                    *self.state__ = NormalTypeDeserializerState::Baz(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(NormalTypeDeserializerState::Baz(None));
                    *self.state__ = NormalTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_baz(data)?;
                    *self.state__ = NormalTypeDeserializerState::Baz(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(NormalTypeDeserializerState::Baz(Some(
                                deserializer,
                            )));
                            *self.state__ = NormalTypeDeserializerState::Baz(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ = NormalTypeDeserializerState::Baz(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::NormalType> for NormalTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NormalType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NormalType> {
            use NormalTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Group(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_group(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Baz(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_baz(helper, output, &mut fallback)? {
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
                        *self.state__ = NormalTypeDeserializerState::Group(None);
                        event
                    }
                    (S::Group(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::NormalGroupType as WithDeserializer>::Deserializer::init(
                                helper, event,
                            )?;
                        match self.handle_group(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Baz(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Baz",
                            false,
                        )?;
                        match self.handle_baz(helper, output, &mut fallback)? {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::NormalType, Error> {
            let state = replace(&mut *self.state__, NormalTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::NormalType {
                group: self.group,
                baz: self.baz,
            })
        }
    }
    #[derive(Debug)]
    pub struct NormalGroupTypeDeserializer {
        state__: Box<NormalGroupTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum NormalGroupTypeDeserializerState {
        Init__,
        Fuu(Vec<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Bar(Vec<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Done__(super::NormalGroupType),
        Unknown__,
    }
    impl NormalGroupTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
            fallback: &mut Option<NormalGroupTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Fuu")
                ) {
                    let output = <i32 as WithDeserializer>::Deserializer::init(helper, event)?;
                    return self.handle_fuu(helper, Default::default(), output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Bar")
                ) {
                    let output = <i32 as WithDeserializer>::Deserializer::init(helper, event)?;
                    return self.handle_bar(helper, Default::default(), output, &mut *fallback);
                }
            }
            *self.state__ = fallback
                .take()
                .unwrap_or(NormalGroupTypeDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: NormalGroupTypeDeserializerState,
        ) -> Result<super::NormalGroupType, Error> {
            use NormalGroupTypeDeserializerState as S;
            match state {
                S::Unknown__ => unreachable!(),
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Fuu(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_fuu(&mut values, value)?;
                    }
                    Ok(super::NormalGroupType::Fuu(values))
                }
                S::Bar(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_bar(&mut values, value)?;
                    }
                    Ok(super::NormalGroupType::Bar(values))
                }
                S::Done__(data) => Ok(data),
            }
        }
        fn store_fuu(values: &mut Vec<i32>, value: i32) -> Result<(), Error> {
            values.push(value);
            Ok(())
        }
        fn store_bar(values: &mut Vec<i32>, value: i32) -> Result<(), Error> {
            values.push(value);
            Ok(())
        }
        fn handle_fuu<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Vec<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<NormalGroupTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = match fallback.take() {
                    None if values.is_empty() => {
                        *self.state__ = NormalGroupTypeDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => NormalGroupTypeDeserializerState::Fuu(values, None),
                    Some(NormalGroupTypeDeserializerState::Fuu(_, Some(deserializer))) => {
                        NormalGroupTypeDeserializerState::Fuu(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(NormalGroupTypeDeserializerState::Fuu(_, Some(deserializer))) => {
                    let data = deserializer.finish(helper)?;
                    Self::store_fuu(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fuu(&mut values, data)?;
                    *self.state__ = NormalGroupTypeDeserializerState::Fuu(values, None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                NormalGroupTypeDeserializerState::Fuu(values, Some(deserializer));
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(NormalGroupTypeDeserializerState::Fuu(
                                Default::default(),
                                Some(deserializer),
                            ));
                            *self.state__ = NormalGroupTypeDeserializerState::Fuu(values, None);
                        }
                    }
                    ret
                }
            })
        }
        fn handle_bar<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Vec<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<NormalGroupTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = match fallback.take() {
                    None if values.is_empty() => {
                        *self.state__ = NormalGroupTypeDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => NormalGroupTypeDeserializerState::Bar(values, None),
                    Some(NormalGroupTypeDeserializerState::Bar(_, Some(deserializer))) => {
                        NormalGroupTypeDeserializerState::Bar(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(NormalGroupTypeDeserializerState::Bar(_, Some(deserializer))) => {
                    let data = deserializer.finish(helper)?;
                    Self::store_bar(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bar(&mut values, data)?;
                    *self.state__ = NormalGroupTypeDeserializerState::Bar(values, None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                NormalGroupTypeDeserializerState::Bar(values, Some(deserializer));
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(NormalGroupTypeDeserializerState::Bar(
                                Default::default(),
                                Some(deserializer),
                            ));
                            *self.state__ = NormalGroupTypeDeserializerState::Bar(values, None);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::NormalGroupType> for NormalGroupTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NormalGroupType> {
            let deserializer = Self {
                state__: Box::new(NormalGroupTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, NormalGroupTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::NormalGroupType> {
            use NormalGroupTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Fuu(values, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fuu(helper, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bar(values, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_bar(helper, values, output, &mut fallback)? {
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
                    (S::Init__, event) => match self.find_suitable(helper, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::Fuu(values, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Fuu",
                            false,
                        )?;
                        match self.handle_fuu(helper, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bar(values, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Bar",
                            false,
                        )?;
                        match self.handle_bar(helper, values, output, &mut fallback)? {
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::NormalGroupType, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{BytesEnd, BytesStart, Error, Event, IterSerializer};
    #[derive(Debug)]
    pub struct MixedTypeSerializer<'ser> {
        pub(super) value: &'ser super::MixedType,
        pub(super) state: Box<MixedTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum MixedTypeSerializerState<'ser> {
        Init__,
        Group(IterSerializer<'ser, &'ser [super::MixedGroupType], super::MixedGroupType>),
        Baz(IterSerializer<'ser, &'ser [i32], i32>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> MixedTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MixedTypeSerializerState::Init__ => {
                        *self.state = MixedTypeSerializerState::Group(IterSerializer::new(
                            &self.value.group[..],
                            Some("Group"),
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    MixedTypeSerializerState::Group(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = MixedTypeSerializerState::Baz(IterSerializer::new(
                                &self.value.baz[..],
                                Some("tns:Baz"),
                                false,
                            ))
                        }
                    },
                    MixedTypeSerializerState::Baz(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = MixedTypeSerializerState::End__,
                    },
                    MixedTypeSerializerState::End__ => {
                        *self.state = MixedTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    MixedTypeSerializerState::Done__ => return Ok(None),
                    MixedTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for MixedTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = MixedTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct MixedGroupTypeSerializer<'ser> {
        pub(super) value: &'ser super::MixedGroupType,
        pub(super) state: Box<MixedGroupTypeSerializerState<'ser>>,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum MixedGroupTypeSerializerState<'ser> {
        Init__,
        Fuu(IterSerializer<'ser, &'ser [i32], i32>),
        Bar(IterSerializer<'ser, &'ser [i32], i32>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> MixedGroupTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MixedGroupTypeSerializerState::Init__ => {
                        match self.value {
                            super::MixedGroupType::Fuu(x) => {
                                *self.state = MixedGroupTypeSerializerState::Fuu(
                                    IterSerializer::new(&x[..], Some("tns:Fuu"), self.is_root),
                                )
                            }
                            super::MixedGroupType::Bar(x) => {
                                *self.state = MixedGroupTypeSerializerState::Bar(
                                    IterSerializer::new(&x[..], Some("tns:Bar"), self.is_root),
                                )
                            }
                        }
                    }
                    MixedGroupTypeSerializerState::Fuu(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = MixedGroupTypeSerializerState::Done__,
                    },
                    MixedGroupTypeSerializerState::Bar(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = MixedGroupTypeSerializerState::Done__,
                    },
                    MixedGroupTypeSerializerState::Done__ => return Ok(None),
                    MixedGroupTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for MixedGroupTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = MixedGroupTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct NormalTypeSerializer<'ser> {
        pub(super) value: &'ser super::NormalType,
        pub(super) state: Box<NormalTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum NormalTypeSerializerState<'ser> {
        Init__,
        Group(IterSerializer<'ser, &'ser [super::NormalGroupType], super::NormalGroupType>),
        Baz(IterSerializer<'ser, &'ser [i32], i32>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> NormalTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    NormalTypeSerializerState::Init__ => {
                        *self.state = NormalTypeSerializerState::Group(IterSerializer::new(
                            &self.value.group[..],
                            Some("Group"),
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    NormalTypeSerializerState::Group(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = NormalTypeSerializerState::Baz(IterSerializer::new(
                                &self.value.baz[..],
                                Some("tns:Baz"),
                                false,
                            ))
                        }
                    },
                    NormalTypeSerializerState::Baz(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = NormalTypeSerializerState::End__,
                    },
                    NormalTypeSerializerState::End__ => {
                        *self.state = NormalTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    NormalTypeSerializerState::Done__ => return Ok(None),
                    NormalTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for NormalTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = NormalTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct NormalGroupTypeSerializer<'ser> {
        pub(super) value: &'ser super::NormalGroupType,
        pub(super) state: Box<NormalGroupTypeSerializerState<'ser>>,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum NormalGroupTypeSerializerState<'ser> {
        Init__,
        Fuu(IterSerializer<'ser, &'ser [i32], i32>),
        Bar(IterSerializer<'ser, &'ser [i32], i32>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> NormalGroupTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    NormalGroupTypeSerializerState::Init__ => {
                        match self.value {
                            super::NormalGroupType::Fuu(x) => {
                                *self.state = NormalGroupTypeSerializerState::Fuu(
                                    IterSerializer::new(&x[..], Some("tns:Fuu"), self.is_root),
                                )
                            }
                            super::NormalGroupType::Bar(x) => {
                                *self.state = NormalGroupTypeSerializerState::Bar(
                                    IterSerializer::new(&x[..], Some("tns:Bar"), self.is_root),
                                )
                            }
                        }
                    }
                    NormalGroupTypeSerializerState::Fuu(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = NormalGroupTypeSerializerState::Done__,
                    },
                    NormalGroupTypeSerializerState::Bar(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = NormalGroupTypeSerializerState::Done__,
                    },
                    NormalGroupTypeSerializerState::Done__ => return Ok(None),
                    NormalGroupTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for NormalGroupTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = NormalGroupTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
