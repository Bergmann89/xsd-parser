use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub type Container = ContainerType;
#[derive(Debug)]
pub struct ContainerType {
    pub content_2: Vec<ContainerContent2Type>,
}
impl WithSerializer for ContainerType {
    type Serializer<'x> = quick_xml_serialize::ContainerTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ContainerTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ContainerTypeSerializerState::Init__),
            name: name.unwrap_or("Container"),
            is_root,
        })
    }
}
impl WithDeserializer for ContainerType {
    type Deserializer = quick_xml_deserialize::ContainerTypeDeserializer;
}
#[derive(Debug)]
pub enum ContainerContent2Type {
    Foo(String),
    Content3(ContainerContent3Type),
}
impl WithSerializer for ContainerContent2Type {
    type Serializer<'x> = quick_xml_serialize::ContainerContent2TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        Ok(quick_xml_serialize::ContainerContent2TypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ContainerContent2TypeSerializerState::Init__),
            is_root,
        })
    }
}
impl WithDeserializer for ContainerContent2Type {
    type Deserializer = quick_xml_deserialize::ContainerContent2TypeDeserializer;
}
#[derive(Debug)]
pub struct ContainerContent3Type {
    pub content_4: Vec<ContainerContent4Type>,
}
impl WithSerializer for ContainerContent3Type {
    type Serializer<'x> = quick_xml_serialize::ContainerContent3TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::ContainerContent3TypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ContainerContent3TypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for ContainerContent3Type {
    type Deserializer = quick_xml_deserialize::ContainerContent3TypeDeserializer;
}
#[derive(Debug)]
pub enum ContainerContent4Type {
    Bar(String),
}
impl WithSerializer for ContainerContent4Type {
    type Serializer<'x> = quick_xml_serialize::ContainerContent4TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        Ok(quick_xml_serialize::ContainerContent4TypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ContainerContent4TypeSerializerState::Init__),
            is_root,
        })
    }
}
impl WithDeserializer for ContainerContent4Type {
    type Deserializer = quick_xml_deserialize::ContainerContent4TypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct ContainerTypeDeserializer {
        content_2: Vec<super::ContainerContent2Type>,
        state__: Box<ContainerTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ContainerTypeDeserializerState {
        Init__,
        Content2(Option<<super::ContainerContent2Type as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ContainerTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content_2: Vec::new(),
                state__: Box::new(ContainerTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ContainerTypeDeserializerState,
        ) -> Result<(), Error> {
            use ContainerTypeDeserializerState as S;
            match state {
                S::Content2(Some(deserializer)) => {
                    self.store_content_2(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_content_2(&mut self, value: super::ContainerContent2Type) -> Result<(), Error> {
            self.content_2.push(value);
            Ok(())
        }
        fn handle_content_2<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ContainerContent2Type>,
            fallback: &mut Option<ContainerTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if matches!(&fallback, Some(ContainerTypeDeserializerState::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else if self.content_2.len() < 1usize {
                    fallback.get_or_insert(ContainerTypeDeserializerState::Content2(None));
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else {
                    fallback.get_or_insert(ContainerTypeDeserializerState::Content2(None));
                    *self.state__ = ContainerTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content_2(data)?;
                    *self.state__ = ContainerTypeDeserializerState::Content2(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(ContainerTypeDeserializerState::Content2(Some(
                        deserializer,
                    )));
                    *self.state__ = ContainerTypeDeserializerState::Content2(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ContainerType> for ContainerTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ContainerType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ContainerType> {
            use ContainerTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content2(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content_2(helper, output, &mut fallback)? {
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
                        *self.state__ = ContainerTypeDeserializerState::Content2(None);
                        event
                    }
                    (S::Content2(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = <super::ContainerContent2Type as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_content_2(helper, output, &mut fallback)? {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ContainerType, Error> {
            let state = replace(
                &mut *self.state__,
                ContainerTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ContainerType {
                content_2: helper.finish_vec(1usize, None, self.content_2)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ContainerContent2TypeDeserializer {
        state__: Box<ContainerContent2TypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ContainerContent2TypeDeserializerState {
        Init__,
        Foo(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Content3(
            Option<super::ContainerContent3Type>,
            Option<<super::ContainerContent3Type as WithDeserializer>::Deserializer>,
            Option<<super::ContainerContent3Type as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ContainerContent2Type),
        Unknown__,
    }
    impl ContainerContent2TypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let mut event = event;
            let mut allow_any_element = false;
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"Foo" {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_foo(helper, Default::default(), None, output);
                }
                event = {
                    let output =
                        <super::ContainerContent3Type as WithDeserializer>::init(helper, event)?;
                    match self.handle_content_3(helper, Default::default(), None, output)? {
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
            *self.state__ = ContainerContent2TypeDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: ContainerContent2TypeDeserializerState,
        ) -> Result<super::ContainerContent2Type, Error> {
            use ContainerContent2TypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Foo(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_foo(&mut values, value)?;
                    }
                    Ok(super::ContainerContent2Type::Foo(
                        helper.finish_element("Foo", values)?,
                    ))
                }
                S::Content3(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_content_3(&mut values, value)?;
                    }
                    Ok(super::ContainerContent2Type::Content3(
                        helper.finish_element("Content3", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_foo(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Foo")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_content_3(
            values: &mut Option<super::ContainerContent3Type>,
            value: super::ContainerContent3Type,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Content3",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_foo<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
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
                Self::store_foo(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_foo(&mut values, data)?;
                    let data = Self::finish_state(
                        helper,
                        ContainerContent2TypeDeserializerState::Foo(values, None, None),
                    )?;
                    *self.state__ = ContainerContent2TypeDeserializerState::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = ContainerContent2TypeDeserializerState::Foo(
                        values,
                        None,
                        Some(deserializer),
                    );
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_content_3<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ContainerContent3Type>,
            fallback: Option<<super::ContainerContent3Type as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ContainerContent3Type>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_content_3(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_content_3(&mut values, data)?;
                    let data = Self::finish_state(
                        helper,
                        ContainerContent2TypeDeserializerState::Content3(values, None, None),
                    )?;
                    *self.state__ = ContainerContent2TypeDeserializerState::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = ContainerContent2TypeDeserializerState::Content3(
                        values,
                        None,
                        Some(deserializer),
                    );
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ContainerContent2Type> for ContainerContent2TypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ContainerContent2Type> {
            let deserializer = Self {
                state__: Box::new(ContainerContent2TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, ContainerContent2TypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::ContainerContent2Type> {
            use ContainerContent2TypeDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Foo(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_foo(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Content3(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content_3(helper, values, fallback, output)? {
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
                        S::Foo(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"Foo", false)?;
                        match self.handle_foo(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Content3(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = <super::ContainerContent3Type as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_content_3(helper, values, fallback, output)? {
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
        ) -> Result<super::ContainerContent2Type, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct ContainerContent3TypeDeserializer {
        content_4: Vec<super::ContainerContent4Type>,
        state__: Box<ContainerContent3TypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ContainerContent3TypeDeserializerState {
        Init__,
        Content4(Option<<super::ContainerContent4Type as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ContainerContent3TypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ContainerContent3TypeDeserializerState,
        ) -> Result<(), Error> {
            use ContainerContent3TypeDeserializerState as S;
            match state {
                S::Content4(Some(deserializer)) => {
                    self.store_content_4(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_content_4(&mut self, value: super::ContainerContent4Type) -> Result<(), Error> {
            self.content_4.push(value);
            Ok(())
        }
        fn handle_content_4<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ContainerContent4Type>,
            fallback: &mut Option<ContainerContent3TypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if matches!(
                    &fallback,
                    Some(ContainerContent3TypeDeserializerState::Init__)
                ) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else if self.content_4.len() < 1usize {
                    fallback.get_or_insert(ContainerContent3TypeDeserializerState::Content4(None));
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else {
                    fallback.get_or_insert(ContainerContent3TypeDeserializerState::Content4(None));
                    *self.state__ = ContainerContent3TypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content_4(data)?;
                    *self.state__ = ContainerContent3TypeDeserializerState::Content4(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(ContainerContent3TypeDeserializerState::Content4(Some(
                        deserializer,
                    )));
                    *self.state__ = ContainerContent3TypeDeserializerState::Content4(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ContainerContent3Type> for ContainerContent3TypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ContainerContent3Type> {
            let deserializer = Self {
                content_4: Vec::new(),
                state__: Box::new(ContainerContent3TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, ContainerContent3TypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::ContainerContent3Type> {
            use ContainerContent3TypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content4(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content_4(helper, output, &mut fallback)? {
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
                        *self.state__ = ContainerContent3TypeDeserializerState::Content4(None);
                        event
                    }
                    (S::Content4(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = <super::ContainerContent4Type as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_content_4(helper, output, &mut fallback)? {
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
        ) -> Result<super::ContainerContent3Type, Error> {
            let state = replace(
                &mut *self.state__,
                ContainerContent3TypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ContainerContent3Type {
                content_4: helper.finish_vec(1usize, None, self.content_4)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ContainerContent4TypeDeserializer {
        state__: Box<ContainerContent4TypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ContainerContent4TypeDeserializerState {
        Init__,
        Bar(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ContainerContent4Type),
        Unknown__,
    }
    impl ContainerContent4TypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"Bar" {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_bar(helper, Default::default(), None, output);
                }
            }
            *self.state__ = ContainerContent4TypeDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: ContainerContent4TypeDeserializerState,
        ) -> Result<super::ContainerContent4Type, Error> {
            use ContainerContent4TypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Bar(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_bar(&mut values, value)?;
                    }
                    Ok(super::ContainerContent4Type::Bar(
                        helper.finish_element("Bar", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_bar(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Bar")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_bar<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
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
                Self::store_bar(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bar(&mut values, data)?;
                    let data = Self::finish_state(
                        helper,
                        ContainerContent4TypeDeserializerState::Bar(values, None, None),
                    )?;
                    *self.state__ = ContainerContent4TypeDeserializerState::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = ContainerContent4TypeDeserializerState::Bar(
                        values,
                        None,
                        Some(deserializer),
                    );
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ContainerContent4Type> for ContainerContent4TypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ContainerContent4Type> {
            let deserializer = Self {
                state__: Box::new(ContainerContent4TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, ContainerContent4TypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::ContainerContent4Type> {
            use ContainerContent4TypeDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Bar(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_bar(helper, values, fallback, output)? {
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
                        S::Bar(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"Bar", false)?;
                        match self.handle_bar(helper, values, fallback, output)? {
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
        ) -> Result<super::ContainerContent4Type, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, Error, Event, IterSerializer, SerializeHelper, Serializer,
        WithSerializer,
    };
    #[derive(Debug)]
    pub struct ContainerTypeSerializer<'ser> {
        pub(super) value: &'ser super::ContainerType,
        pub(super) state: Box<ContainerTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ContainerTypeSerializerState<'ser> {
        Init__,
        Content2(
            IterSerializer<
                'ser,
                &'ser [super::ContainerContent2Type],
                super::ContainerContent2Type,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ContainerTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ContainerTypeSerializerState::Init__ => {
                        *self.state = ContainerTypeSerializerState::Content2(IterSerializer::new(
                            &self.value.content_2[..],
                            Some("Content2"),
                            false,
                        ));
                        let bytes = BytesStart::new(self.name);
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ContainerTypeSerializerState::Content2(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ContainerTypeSerializerState::End__,
                        }
                    }
                    ContainerTypeSerializerState::End__ => {
                        *self.state = ContainerTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ContainerTypeSerializerState::Done__ => return Ok(None),
                    ContainerTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ContainerTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ContainerTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ContainerContent2TypeSerializer<'ser> {
        pub(super) value: &'ser super::ContainerContent2Type,
        pub(super) state: Box<ContainerContent2TypeSerializerState<'ser>>,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ContainerContent2TypeSerializerState<'ser> {
        Init__,
        Foo(<String as WithSerializer>::Serializer<'ser>),
        Content3(<super::ContainerContent3Type as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ContainerContent2TypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ContainerContent2TypeSerializerState::Init__ => match self.value {
                        super::ContainerContent2Type::Foo(x) => {
                            *self.state = ContainerContent2TypeSerializerState::Foo(
                                WithSerializer::serializer(x, Some("Foo"), self.is_root)?,
                            )
                        }
                        super::ContainerContent2Type::Content3(x) => {
                            *self.state = ContainerContent2TypeSerializerState::Content3(
                                WithSerializer::serializer(x, Some("Content3"), self.is_root)?,
                            )
                        }
                    },
                    ContainerContent2TypeSerializerState::Foo(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ContainerContent2TypeSerializerState::Done__,
                        }
                    }
                    ContainerContent2TypeSerializerState::Content3(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ContainerContent2TypeSerializerState::Done__,
                        }
                    }
                    ContainerContent2TypeSerializerState::Done__ => return Ok(None),
                    ContainerContent2TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ContainerContent2TypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ContainerContent2TypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ContainerContent3TypeSerializer<'ser> {
        pub(super) value: &'ser super::ContainerContent3Type,
        pub(super) state: Box<ContainerContent3TypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum ContainerContent3TypeSerializerState<'ser> {
        Init__,
        Content4(
            IterSerializer<
                'ser,
                &'ser [super::ContainerContent4Type],
                super::ContainerContent4Type,
            >,
        ),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ContainerContent3TypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ContainerContent3TypeSerializerState::Init__ => {
                        *self.state = ContainerContent3TypeSerializerState::Content4(
                            IterSerializer::new(&self.value.content_4[..], Some("Content4"), false),
                        );
                    }
                    ContainerContent3TypeSerializerState::Content4(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ContainerContent3TypeSerializerState::Done__,
                        }
                    }
                    ContainerContent3TypeSerializerState::Done__ => return Ok(None),
                    ContainerContent3TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ContainerContent3TypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ContainerContent3TypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ContainerContent4TypeSerializer<'ser> {
        pub(super) value: &'ser super::ContainerContent4Type,
        pub(super) state: Box<ContainerContent4TypeSerializerState<'ser>>,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ContainerContent4TypeSerializerState<'ser> {
        Init__,
        Bar(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ContainerContent4TypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ContainerContent4TypeSerializerState::Init__ => match self.value {
                        super::ContainerContent4Type::Bar(x) => {
                            *self.state = ContainerContent4TypeSerializerState::Bar(
                                WithSerializer::serializer(x, Some("Bar"), self.is_root)?,
                            )
                        }
                    },
                    ContainerContent4TypeSerializerState::Bar(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ContainerContent4TypeSerializerState::Done__,
                        }
                    }
                    ContainerContent4TypeSerializerState::Done__ => return Ok(None),
                    ContainerContent4TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ContainerContent4TypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ContainerContent4TypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
