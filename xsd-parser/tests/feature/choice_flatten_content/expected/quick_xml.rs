use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub type Foo = FooType;
#[derive(Debug)]
pub enum FooType {
    Once(i32),
    Optional(Option<i32>),
    OnceSpecify(i32),
    TwiceOrMore(Vec<i32>),
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
            name: name.unwrap_or("tns:FooType"),
            is_root,
        })
    }
}
impl WithDeserializer for FooType {
    type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
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
        state__: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum FooTypeDeserializerState {
        Init__,
        Once(
            Option<i32>,
            Option<<i32 as WithDeserializer>::Deserializer>,
            Option<<i32 as WithDeserializer>::Deserializer>,
        ),
        Optional(
            Option<i32>,
            Option<<i32 as WithDeserializer>::Deserializer>,
            Option<<i32 as WithDeserializer>::Deserializer>,
        ),
        OnceSpecify(
            Option<i32>,
            Option<<i32 as WithDeserializer>::Deserializer>,
            Option<<i32 as WithDeserializer>::Deserializer>,
        ),
        TwiceOrMore(
            Vec<i32>,
            Option<<i32 as WithDeserializer>::Deserializer>,
            Option<<i32 as WithDeserializer>::Deserializer>,
        ),
        Done__(super::FooType),
        Unknown__,
    }
    impl FooTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Once")
                ) {
                    let output = <i32 as WithDeserializer>::init(helper, event)?;
                    return self.handle_once(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Optional")
                ) {
                    let output = <i32 as WithDeserializer>::init(helper, event)?;
                    return self.handle_optional(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"OnceSpecify")
                ) {
                    let output = <i32 as WithDeserializer>::init(helper, event)?;
                    return self.handle_once_specify(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"TwiceOrMore")
                ) {
                    let output = <i32 as WithDeserializer>::init(helper, event)?;
                    return self.handle_twice_or_more(helper, Default::default(), None, output);
                }
            }
            *self.state__ = FooTypeDeserializerState::Init__;
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
                state__: Box::new(FooTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: FooTypeDeserializerState,
        ) -> Result<super::FooType, Error> {
            use FooTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Once(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_once(&mut values, value)?;
                    }
                    Ok(super::FooType::Once(helper.finish_element("Once", values)?))
                }
                S::Optional(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_optional(&mut values, value)?;
                    }
                    Ok(super::FooType::Optional(values))
                }
                S::OnceSpecify(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_once_specify(&mut values, value)?;
                    }
                    Ok(super::FooType::OnceSpecify(
                        helper.finish_element("OnceSpecify", values)?,
                    ))
                }
                S::TwiceOrMore(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_twice_or_more(&mut values, value)?;
                    }
                    Ok(super::FooType::TwiceOrMore(helper.finish_vec(
                        2usize,
                        Some(100usize),
                        values,
                    )?))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_once(values: &mut Option<i32>, value: i32) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Once")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_optional(values: &mut Option<i32>, value: i32) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Optional",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_once_specify(values: &mut Option<i32>, value: i32) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"OnceSpecify",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_twice_or_more(values: &mut Vec<i32>, value: i32) -> Result<(), Error> {
            values.push(value);
            Ok(())
        }
        fn handle_once<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<i32>,
            fallback: Option<<i32 as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, i32>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FooTypeDeserializerState as S;
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
                Self::store_once(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_once(&mut values, data)?;
                    *self.state__ = S::Once(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Once(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
        fn handle_optional<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<i32>,
            fallback: Option<<i32 as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, i32>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FooTypeDeserializerState as S;
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
                Self::store_optional(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_optional(&mut values, data)?;
                    *self.state__ = S::Optional(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Optional(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
        fn handle_once_specify<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<i32>,
            fallback: Option<<i32 as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, i32>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FooTypeDeserializerState as S;
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
                Self::store_once_specify(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_once_specify(&mut values, data)?;
                    *self.state__ = S::OnceSpecify(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::OnceSpecify(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
        fn handle_twice_or_more<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Vec<i32>,
            fallback: Option<<i32 as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, i32>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FooTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if fallback.is_none() && values.is_empty() {
                    *self.state__ = S::Init__;
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else if values.len() + usize::from(fallback.is_some()) < 2usize {
                    *self.state__ = S::TwiceOrMore(values, None, fallback);
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else {
                    *self.state__ = S::TwiceOrMore(values, None, fallback);
                    return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
                }
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_twice_or_more(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_twice_or_more(&mut values, data)?;
                    *self.state__ = S::TwiceOrMore(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let can_have_more = values.len() < 99usize;
                    let ret = if can_have_more {
                        ElementHandlerOutput::from_event(event, allow_any)
                    } else {
                        ElementHandlerOutput::from_event_end(event, allow_any)
                    };
                    if can_have_more && ret.is_continue_start_or_empty() {
                        *self.state__ = S::TwiceOrMore(values, Some(deserializer), None);
                    } else {
                        *self.state__ = S::TwiceOrMore(values, None, Some(deserializer));
                    }
                    Ok(ret)
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
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Once(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_once(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Optional(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_optional(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::OnceSpecify(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_once_specify(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::TwiceOrMore(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_twice_or_more(helper, values, fallback, output)? {
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
                        S::Once(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Once",
                            false,
                        )?;
                        match self.handle_once(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Optional(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Optional",
                            false,
                        )?;
                        match self.handle_optional(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::OnceSpecify(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"OnceSpecify",
                            false,
                        )?;
                        match self.handle_once_specify(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::TwiceOrMore(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"TwiceOrMore",
                            false,
                        )?;
                        match self.handle_twice_or_more(helper, values, fallback, output)? {
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::FooType, Error> {
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
    pub struct FooTypeSerializer<'ser> {
        pub(super) value: &'ser super::FooType,
        pub(super) state: Box<FooTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum FooTypeSerializerState<'ser> {
        Init__,
        Once(<i32 as WithSerializer>::Serializer<'ser>),
        Optional(IterSerializer<'ser, Option<&'ser i32>, i32>),
        OnceSpecify(<i32 as WithSerializer>::Serializer<'ser>),
        TwiceOrMore(IterSerializer<'ser, &'ser [i32], i32>),
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
                        match self.value {
                            super::FooType::Once(x) => {
                                *self.state = FooTypeSerializerState::Once(
                                    WithSerializer::serializer(x, Some("tns:Once"), self.is_root)?,
                                )
                            }
                            super::FooType::Optional(x) => {
                                *self.state = FooTypeSerializerState::Optional(IterSerializer::new(
                                    x.as_ref(),
                                    Some("tns:Optional"),
                                    self.is_root,
                                ))
                            }
                            super::FooType::OnceSpecify(x) => {
                                *self.state =
                                    FooTypeSerializerState::OnceSpecify(WithSerializer::serializer(
                                        x,
                                        Some("tns:OnceSpecify"),
                                        self.is_root,
                                    )?)
                            }
                            super::FooType::TwiceOrMore(x) => {
                                *self.state =
                                    FooTypeSerializerState::TwiceOrMore(IterSerializer::new(
                                        &x[..],
                                        Some("tns:TwiceOrMore"),
                                        self.is_root,
                                    ))
                            }
                        }
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
                    FooTypeSerializerState::Once(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooTypeSerializerState::End__,
                    },
                    FooTypeSerializerState::Optional(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooTypeSerializerState::End__,
                    },
                    FooTypeSerializerState::OnceSpecify(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooTypeSerializerState::End__,
                    },
                    FooTypeSerializerState::TwiceOrMore(x) => match x.next(helper).transpose()? {
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
}
