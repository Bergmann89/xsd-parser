use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub outer_1: FooOuterType,
    pub outer_2: FooOuterType,
}
impl WithSerializer for FooType {
    type Serializer<'x> = quick_xml_serialize::FooTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::FooTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::FooTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Foo"),
            is_root,
        })
    }
}
impl WithDeserializer for FooType {
    type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
}
#[derive(Debug)]
pub enum FooOuterType {
    Bar(String),
    Baz(i32),
    Inner(FooInnerType),
}
impl WithSerializer for FooOuterType {
    type Serializer<'x> = quick_xml_serialize::FooOuterTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        Ok(quick_xml_serialize::FooOuterTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::FooOuterTypeSerializerState::Init__),
            is_root,
        })
    }
}
impl WithDeserializer for FooOuterType {
    type Deserializer = quick_xml_deserialize::FooOuterTypeDeserializer;
}
#[derive(Debug)]
pub struct FooInnerType {
    pub fizz: String,
    pub buzz: i32,
}
impl WithSerializer for FooInnerType {
    type Serializer<'x> = quick_xml_serialize::FooInnerTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::FooInnerTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::FooInnerTypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for FooInnerType {
    type Deserializer = quick_xml_deserialize::FooInnerTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct FooTypeDeserializer {
        outer_1: Option<super::FooOuterType>,
        outer_2: Option<super::FooOuterType>,
        state__: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Init__,
        Outer1(Option<<super::FooOuterType as WithDeserializer>::Deserializer>),
        Outer2(Option<<super::FooOuterType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl FooTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                outer_1: None,
                outer_2: None,
                state__: Box::new(FooTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: FooTypeDeserializerState,
        ) -> Result<(), Error> {
            use FooTypeDeserializerState as S;
            match state {
                S::Outer1(Some(deserializer)) => {
                    self.store_outer_1(deserializer.finish(helper)?)?
                }
                S::Outer2(Some(deserializer)) => {
                    self.store_outer_2(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_outer_1(&mut self, value: super::FooOuterType) -> Result<(), Error> {
            if self.outer_1.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Outer",
                )))?;
            }
            self.outer_1 = Some(value);
            Ok(())
        }
        fn store_outer_2(&mut self, value: super::FooOuterType) -> Result<(), Error> {
            if self.outer_2.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Outer_2",
                )))?;
            }
            self.outer_2 = Some(value);
            Ok(())
        }
        fn handle_outer_1<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::FooOuterType>,
            fallback: &mut Option<FooTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(FooTypeDeserializerState::Outer1(None));
                if matches!(&fallback, Some(FooTypeDeserializerState::Init__)) {
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
                    self.store_outer_1(data)?;
                    *self.state__ = FooTypeDeserializerState::Outer2(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(FooTypeDeserializerState::Outer1(Some(deserializer)));
                    *self.state__ = FooTypeDeserializerState::Outer2(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_outer_2<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::FooOuterType>,
            fallback: &mut Option<FooTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(FooTypeDeserializerState::Outer2(None));
                if matches!(&fallback, Some(FooTypeDeserializerState::Init__)) {
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
                    self.store_outer_2(data)?;
                    *self.state__ = FooTypeDeserializerState::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(FooTypeDeserializerState::Outer2(Some(deserializer)));
                    *self.state__ = FooTypeDeserializerState::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::FooType> for FooTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooType> {
            use FooTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Outer1(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_outer_1(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Outer2(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_outer_2(helper, output, &mut fallback)? {
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
                        *self.state__ = FooTypeDeserializerState::Outer1(None);
                        event
                    }
                    (S::Outer1(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::FooOuterType as WithDeserializer>::init(helper, event)?;
                        match self.handle_outer_1(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Outer2(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::FooOuterType as WithDeserializer>::init(helper, event)?;
                        match self.handle_outer_2(helper, output, &mut fallback)? {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::FooType, Error> {
            let state = replace(&mut *self.state__, FooTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::FooType {
                outer_1: helper.finish_element("Outer", self.outer_1)?,
                outer_2: helper.finish_element("Outer_2", self.outer_2)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct FooOuterTypeDeserializer {
        state__: Box<FooOuterTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum FooOuterTypeDeserializerState {
        Init__,
        Bar(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Baz(
            Option<i32>,
            Option<<i32 as WithDeserializer>::Deserializer>,
            Option<<i32 as WithDeserializer>::Deserializer>,
        ),
        Inner(
            Option<super::FooInnerType>,
            Option<<super::FooInnerType as WithDeserializer>::Deserializer>,
            Option<<super::FooInnerType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::FooOuterType),
        Unknown__,
    }
    impl FooOuterTypeDeserializer {
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
                    Some(b"Bar")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_bar(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Baz")
                ) {
                    let output = <i32 as WithDeserializer>::init(helper, event)?;
                    return self.handle_baz(helper, Default::default(), None, output);
                }
                event = {
                    let output = <super::FooInnerType as WithDeserializer>::init(helper, event)?;
                    match self.handle_inner(helper, Default::default(), None, output)? {
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
            *self.state__ = FooOuterTypeDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: FooOuterTypeDeserializerState,
        ) -> Result<super::FooOuterType, Error> {
            use FooOuterTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Bar(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_bar(&mut values, value)?;
                    }
                    Ok(super::FooOuterType::Bar(
                        helper.finish_element("Bar", values)?,
                    ))
                }
                S::Baz(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_baz(&mut values, value)?;
                    }
                    Ok(super::FooOuterType::Baz(
                        helper.finish_element("Baz", values)?,
                    ))
                }
                S::Inner(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_inner(&mut values, value)?;
                    }
                    Ok(super::FooOuterType::Inner(
                        helper.finish_element("Inner", values)?,
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
        fn store_baz(values: &mut Option<i32>, value: i32) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Baz")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_inner(
            values: &mut Option<super::FooInnerType>,
            value: super::FooInnerType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Inner",
                )))?;
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
                        FooOuterTypeDeserializerState::Bar(values, None, None),
                    )?;
                    *self.state__ = FooOuterTypeDeserializerState::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ =
                        FooOuterTypeDeserializerState::Bar(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_baz<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<i32>,
            fallback: Option<<i32 as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, i32>,
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
                Self::store_baz(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_baz(&mut values, data)?;
                    let data = Self::finish_state(
                        helper,
                        FooOuterTypeDeserializerState::Baz(values, None, None),
                    )?;
                    *self.state__ = FooOuterTypeDeserializerState::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ =
                        FooOuterTypeDeserializerState::Baz(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_inner<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::FooInnerType>,
            fallback: Option<<super::FooInnerType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::FooInnerType>,
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
                Self::store_inner(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_inner(&mut values, data)?;
                    let data = Self::finish_state(
                        helper,
                        FooOuterTypeDeserializerState::Inner(values, None, None),
                    )?;
                    *self.state__ = FooOuterTypeDeserializerState::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ =
                        FooOuterTypeDeserializerState::Inner(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::FooOuterType> for FooOuterTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooOuterType> {
            let deserializer = Self {
                state__: Box::new(FooOuterTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, FooOuterTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::FooOuterType> {
            use FooOuterTypeDeserializerState as S;
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
                    (S::Baz(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_baz(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Inner(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_inner(helper, values, fallback, output)? {
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
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Bar",
                            false,
                        )?;
                        match self.handle_bar(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Baz(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Baz",
                            false,
                        )?;
                        match self.handle_baz(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Inner(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            <super::FooInnerType as WithDeserializer>::init(helper, event)?;
                        match self.handle_inner(helper, values, fallback, output)? {
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::FooOuterType, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct FooInnerTypeDeserializer {
        fizz: Option<String>,
        buzz: Option<i32>,
        state__: Box<FooInnerTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooInnerTypeDeserializerState {
        Init__,
        Fizz(Option<<String as WithDeserializer>::Deserializer>),
        Buzz(Option<<i32 as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl FooInnerTypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: FooInnerTypeDeserializerState,
        ) -> Result<(), Error> {
            use FooInnerTypeDeserializerState as S;
            match state {
                S::Fizz(Some(deserializer)) => self.store_fizz(deserializer.finish(helper)?)?,
                S::Buzz(Some(deserializer)) => self.store_buzz(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_fizz(&mut self, value: String) -> Result<(), Error> {
            if self.fizz.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Fizz")))?;
            }
            self.fizz = Some(value);
            Ok(())
        }
        fn store_buzz(&mut self, value: i32) -> Result<(), Error> {
            if self.buzz.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Buzz")))?;
            }
            self.buzz = Some(value);
            Ok(())
        }
        fn handle_fizz<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<FooInnerTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(FooInnerTypeDeserializerState::Fizz(None));
                if matches!(&fallback, Some(FooInnerTypeDeserializerState::Init__)) {
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
                    self.store_fizz(data)?;
                    *self.state__ = FooInnerTypeDeserializerState::Buzz(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(FooInnerTypeDeserializerState::Fizz(Some(deserializer)));
                    *self.state__ = FooInnerTypeDeserializerState::Buzz(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_buzz<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<FooInnerTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(FooInnerTypeDeserializerState::Buzz(None));
                if matches!(&fallback, Some(FooInnerTypeDeserializerState::Init__)) {
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
                    self.store_buzz(data)?;
                    *self.state__ = FooInnerTypeDeserializerState::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(FooInnerTypeDeserializerState::Buzz(Some(deserializer)));
                    *self.state__ = FooInnerTypeDeserializerState::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::FooInnerType> for FooInnerTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooInnerType> {
            let deserializer = Self {
                fizz: None,
                buzz: None,
                state__: Box::new(FooInnerTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, FooInnerTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::FooInnerType> {
            use FooInnerTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Fizz(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fizz(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Buzz(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_buzz(helper, output, &mut fallback)? {
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
                        *self.state__ = FooInnerTypeDeserializerState::Fizz(None);
                        event
                    }
                    (S::Fizz(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Fizz",
                            false,
                        )?;
                        match self.handle_fizz(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Buzz(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Buzz",
                            false,
                        )?;
                        match self.handle_buzz(helper, output, &mut fallback)? {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::FooInnerType, Error> {
            let state = replace(&mut *self.state__, FooInnerTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::FooInnerType {
                fizz: helper.finish_element("Fizz", self.fizz)?,
                buzz: helper.finish_element("Buzz", self.buzz)?,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, Error, Event, SerializeHelper, Serializer, WithSerializer,
    };
    #[derive(Debug)]
    pub struct FooTypeSerializer<'ser> {
        pub(super) value: &'ser super::FooType,
        pub(super) state: Box<FooTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum FooTypeSerializerState<'ser> {
        Init__,
        Outer1(<super::FooOuterType as WithSerializer>::Serializer<'ser>),
        Outer2(<super::FooOuterType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FooTypeSerializerState::Init__ => {
                        *self.state = FooTypeSerializerState::Outer1(WithSerializer::serializer(
                            &self.value.outer_1,
                            Some("Outer"),
                            false,
                        )?);
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    FooTypeSerializerState::Outer1(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                FooTypeSerializerState::Outer2(WithSerializer::serializer(
                                    &self.value.outer_2,
                                    Some("Outer_2"),
                                    false,
                                )?)
                        }
                    },
                    FooTypeSerializerState::Outer2(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooTypeSerializerState::End__,
                    },
                    FooTypeSerializerState::End__ => {
                        *self.state = FooTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    FooTypeSerializerState::Done__ => return Ok(None),
                    FooTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for FooTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = FooTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct FooOuterTypeSerializer<'ser> {
        pub(super) value: &'ser super::FooOuterType,
        pub(super) state: Box<FooOuterTypeSerializerState<'ser>>,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum FooOuterTypeSerializerState<'ser> {
        Init__,
        Bar(<String as WithSerializer>::Serializer<'ser>),
        Baz(<i32 as WithSerializer>::Serializer<'ser>),
        Inner(<super::FooInnerType as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooOuterTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FooOuterTypeSerializerState::Init__ => match self.value {
                        super::FooOuterType::Bar(x) => {
                            *self.state = FooOuterTypeSerializerState::Bar(
                                WithSerializer::serializer(x, Some("tns:Bar"), self.is_root)?,
                            )
                        }
                        super::FooOuterType::Baz(x) => {
                            *self.state = FooOuterTypeSerializerState::Baz(
                                WithSerializer::serializer(x, Some("tns:Baz"), self.is_root)?,
                            )
                        }
                        super::FooOuterType::Inner(x) => {
                            *self.state = FooOuterTypeSerializerState::Inner(
                                WithSerializer::serializer(x, Some("Inner"), self.is_root)?,
                            )
                        }
                    },
                    FooOuterTypeSerializerState::Bar(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooOuterTypeSerializerState::Done__,
                    },
                    FooOuterTypeSerializerState::Baz(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooOuterTypeSerializerState::Done__,
                    },
                    FooOuterTypeSerializerState::Inner(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooOuterTypeSerializerState::Done__,
                    },
                    FooOuterTypeSerializerState::Done__ => return Ok(None),
                    FooOuterTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for FooOuterTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = FooOuterTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct FooInnerTypeSerializer<'ser> {
        pub(super) value: &'ser super::FooInnerType,
        pub(super) state: Box<FooInnerTypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum FooInnerTypeSerializerState<'ser> {
        Init__,
        Fizz(<String as WithSerializer>::Serializer<'ser>),
        Buzz(<i32 as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooInnerTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FooInnerTypeSerializerState::Init__ => {
                        *self.state = FooInnerTypeSerializerState::Fizz(
                            WithSerializer::serializer(&self.value.fizz, Some("tns:Fizz"), false)?,
                        );
                    }
                    FooInnerTypeSerializerState::Fizz(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                FooInnerTypeSerializerState::Buzz(WithSerializer::serializer(
                                    &self.value.buzz,
                                    Some("tns:Buzz"),
                                    false,
                                )?)
                        }
                    },
                    FooInnerTypeSerializerState::Buzz(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooInnerTypeSerializerState::Done__,
                    },
                    FooInnerTypeSerializerState::Done__ => return Ok(None),
                    FooInnerTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for FooInnerTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = FooInnerTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
