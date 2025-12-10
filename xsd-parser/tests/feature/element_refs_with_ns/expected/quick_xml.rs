use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_UNNAMED_2: Namespace = Namespace::new_const(b"Foo");
pub const NS_BAR: Namespace = Namespace::new_const(b"Bar");
pub const NS_BAZ: Namespace = Namespace::new_const(b"Baz");
pub const NS_BIZ: Namespace = Namespace::new_const(b"Biz");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_BAR: NamespacePrefix = NamespacePrefix::new_const(b"bar");
pub const PREFIX_BAZ: NamespacePrefix = NamespacePrefix::new_const(b"baz");
pub const PREFIX_BIZ: NamespacePrefix = NamespacePrefix::new_const(b"biz");
pub type Outer = OuterType;
#[derive(Debug)]
pub struct OuterType {
    pub bar_inner: bar::InnerType,
    pub baz_inner: baz::InnerType,
    pub biz_inner: biz::InnerType,
}
impl WithSerializer for OuterType {
    type Serializer<'x> = quick_xml_serialize::OuterTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::OuterTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::OuterTypeSerializerState::Init__),
            name: name.unwrap_or("Outer"),
            is_root,
        })
    }
}
impl WithDeserializer for OuterType {
    type Deserializer = quick_xml_deserialize::OuterTypeDeserializer;
}
pub mod bar {
    use xsd_parser_types::quick_xml::{Error, WithDeserializer, WithSerializer};
    #[derive(Debug)]
    pub struct InnerType {
        pub a: String,
    }
    impl WithSerializer for InnerType {
        type Serializer<'x> = quick_xml_serialize::InnerTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::InnerTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::InnerTypeSerializerState::Init__),
                name: name.unwrap_or("Inner"),
                is_root,
            })
        }
    }
    impl WithDeserializer for InnerType {
        type Deserializer = quick_xml_deserialize::InnerTypeDeserializer;
    }
    pub mod quick_xml_deserialize {
        use core::mem::replace;
        use xsd_parser_types::quick_xml::{
            BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
            RawByteStr, WithDeserializer,
        };
        #[derive(Debug)]
        pub struct InnerTypeDeserializer {
            a: Option<String>,
            state__: Box<InnerTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum InnerTypeDeserializerState {
            Init__,
            A(Option<<String as WithDeserializer>::Deserializer>),
            Done__,
            Unknown__,
        }
        impl InnerTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
                Ok(Self {
                    a: None,
                    state__: Box::new(InnerTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: InnerTypeDeserializerState,
            ) -> Result<(), Error> {
                use InnerTypeDeserializerState as S;
                match state {
                    S::A(Some(deserializer)) => self.store_a(deserializer.finish(helper)?)?,
                    _ => (),
                }
                Ok(())
            }
            fn store_a(&mut self, value: String) -> Result<(), Error> {
                if self.a.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"A")))?;
                }
                self.a = Some(value);
                Ok(())
            }
            fn handle_a<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, String>,
                fallback: &mut Option<InnerTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use InnerTypeDeserializerState as S;
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    fallback.get_or_insert(S::A(None));
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
                        self.store_a(data)?;
                        *self.state__ = S::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(S::A(Some(deserializer)));
                        *self.state__ = S::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                }
            }
        }
        impl<'de> Deserializer<'de, super::InnerType> for InnerTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::InnerType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::InnerType> {
                use InnerTypeDeserializerState as S;
                let mut event = event;
                let mut fallback = None;
                let mut allow_any_element = false;
                let (event, allow_any) = loop {
                    let state = replace(&mut *self.state__, S::Unknown__);
                    event = match (state, event) {
                        (S::Unknown__, _) => unreachable!(),
                        (S::A(Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_a(helper, output, &mut fallback)? {
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
                            *self.state__ = S::A(None);
                            event
                        }
                        (S::A(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_BAR),
                                b"A",
                                false,
                            )?;
                            match self.handle_a(helper, output, &mut fallback)? {
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
            fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::InnerType, Error> {
                let state = replace(&mut *self.state__, InnerTypeDeserializerState::Unknown__);
                self.finish_state(helper, state)?;
                Ok(super::InnerType {
                    a: helper.finish_element("A", self.a)?,
                })
            }
        }
    }
    pub mod quick_xml_serialize {
        use xsd_parser_types::quick_xml::{
            BytesEnd, BytesStart, Error, Event, SerializeHelper, Serializer, WithSerializer,
        };
        #[derive(Debug)]
        pub struct InnerTypeSerializer<'ser> {
            pub(super) value: &'ser super::InnerType,
            pub(super) state: Box<InnerTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum InnerTypeSerializerState<'ser> {
            Init__,
            A(<String as WithSerializer>::Serializer<'ser>),
            End__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> InnerTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        InnerTypeSerializerState::Init__ => {
                            *self.state = InnerTypeSerializerState::A(WithSerializer::serializer(
                                &self.value.a,
                                Some("A"),
                                false,
                            )?);
                            let bytes = BytesStart::new(self.name);
                            return Ok(Some(Event::Start(bytes)));
                        }
                        InnerTypeSerializerState::A(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = InnerTypeSerializerState::End__,
                        },
                        InnerTypeSerializerState::End__ => {
                            *self.state = InnerTypeSerializerState::Done__;
                            return Ok(Some(Event::End(BytesEnd::new(self.name))));
                        }
                        InnerTypeSerializerState::Done__ => return Ok(None),
                        InnerTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for InnerTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = InnerTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
    }
}
pub mod baz {
    use xsd_parser_types::quick_xml::{Error, WithDeserializer, WithSerializer};
    #[derive(Debug)]
    pub struct InnerType {
        pub b: String,
    }
    impl WithSerializer for InnerType {
        type Serializer<'x> = quick_xml_serialize::InnerTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::InnerTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::InnerTypeSerializerState::Init__),
                name: name.unwrap_or("Inner"),
                is_root,
            })
        }
    }
    impl WithDeserializer for InnerType {
        type Deserializer = quick_xml_deserialize::InnerTypeDeserializer;
    }
    pub mod quick_xml_deserialize {
        use core::mem::replace;
        use xsd_parser_types::quick_xml::{
            BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
            RawByteStr, WithDeserializer,
        };
        #[derive(Debug)]
        pub struct InnerTypeDeserializer {
            b: Option<String>,
            state__: Box<InnerTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum InnerTypeDeserializerState {
            Init__,
            B(Option<<String as WithDeserializer>::Deserializer>),
            Done__,
            Unknown__,
        }
        impl InnerTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
                Ok(Self {
                    b: None,
                    state__: Box::new(InnerTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: InnerTypeDeserializerState,
            ) -> Result<(), Error> {
                use InnerTypeDeserializerState as S;
                match state {
                    S::B(Some(deserializer)) => self.store_b(deserializer.finish(helper)?)?,
                    _ => (),
                }
                Ok(())
            }
            fn store_b(&mut self, value: String) -> Result<(), Error> {
                if self.b.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"B")))?;
                }
                self.b = Some(value);
                Ok(())
            }
            fn handle_b<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, String>,
                fallback: &mut Option<InnerTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use InnerTypeDeserializerState as S;
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    fallback.get_or_insert(S::B(None));
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
                        self.store_b(data)?;
                        *self.state__ = S::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(S::B(Some(deserializer)));
                        *self.state__ = S::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                }
            }
        }
        impl<'de> Deserializer<'de, super::InnerType> for InnerTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::InnerType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::InnerType> {
                use InnerTypeDeserializerState as S;
                let mut event = event;
                let mut fallback = None;
                let mut allow_any_element = false;
                let (event, allow_any) = loop {
                    let state = replace(&mut *self.state__, S::Unknown__);
                    event = match (state, event) {
                        (S::Unknown__, _) => unreachable!(),
                        (S::B(Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_b(helper, output, &mut fallback)? {
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
                            *self.state__ = S::B(None);
                            event
                        }
                        (S::B(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_BAZ),
                                b"B",
                                false,
                            )?;
                            match self.handle_b(helper, output, &mut fallback)? {
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
            fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::InnerType, Error> {
                let state = replace(&mut *self.state__, InnerTypeDeserializerState::Unknown__);
                self.finish_state(helper, state)?;
                Ok(super::InnerType {
                    b: helper.finish_element("B", self.b)?,
                })
            }
        }
    }
    pub mod quick_xml_serialize {
        use xsd_parser_types::quick_xml::{
            BytesEnd, BytesStart, Error, Event, SerializeHelper, Serializer, WithSerializer,
        };
        #[derive(Debug)]
        pub struct InnerTypeSerializer<'ser> {
            pub(super) value: &'ser super::InnerType,
            pub(super) state: Box<InnerTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum InnerTypeSerializerState<'ser> {
            Init__,
            B(<String as WithSerializer>::Serializer<'ser>),
            End__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> InnerTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        InnerTypeSerializerState::Init__ => {
                            *self.state = InnerTypeSerializerState::B(WithSerializer::serializer(
                                &self.value.b,
                                Some("B"),
                                false,
                            )?);
                            let bytes = BytesStart::new(self.name);
                            return Ok(Some(Event::Start(bytes)));
                        }
                        InnerTypeSerializerState::B(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = InnerTypeSerializerState::End__,
                        },
                        InnerTypeSerializerState::End__ => {
                            *self.state = InnerTypeSerializerState::Done__;
                            return Ok(Some(Event::End(BytesEnd::new(self.name))));
                        }
                        InnerTypeSerializerState::Done__ => return Ok(None),
                        InnerTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for InnerTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = InnerTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
    }
}
pub mod biz {
    use xsd_parser_types::quick_xml::{Error, WithDeserializer, WithSerializer};
    #[derive(Debug)]
    pub struct InnerType {
        pub c: String,
    }
    impl WithSerializer for InnerType {
        type Serializer<'x> = quick_xml_serialize::InnerTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::InnerTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::InnerTypeSerializerState::Init__),
                name: name.unwrap_or("Inner"),
                is_root,
            })
        }
    }
    impl WithDeserializer for InnerType {
        type Deserializer = quick_xml_deserialize::InnerTypeDeserializer;
    }
    pub mod quick_xml_deserialize {
        use core::mem::replace;
        use xsd_parser_types::quick_xml::{
            BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
            RawByteStr, WithDeserializer,
        };
        #[derive(Debug)]
        pub struct InnerTypeDeserializer {
            c: Option<String>,
            state__: Box<InnerTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum InnerTypeDeserializerState {
            Init__,
            C(Option<<String as WithDeserializer>::Deserializer>),
            Done__,
            Unknown__,
        }
        impl InnerTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
                Ok(Self {
                    c: None,
                    state__: Box::new(InnerTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: InnerTypeDeserializerState,
            ) -> Result<(), Error> {
                use InnerTypeDeserializerState as S;
                match state {
                    S::C(Some(deserializer)) => self.store_c(deserializer.finish(helper)?)?,
                    _ => (),
                }
                Ok(())
            }
            fn store_c(&mut self, value: String) -> Result<(), Error> {
                if self.c.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"C")))?;
                }
                self.c = Some(value);
                Ok(())
            }
            fn handle_c<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, String>,
                fallback: &mut Option<InnerTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use InnerTypeDeserializerState as S;
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    fallback.get_or_insert(S::C(None));
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
                        self.store_c(data)?;
                        *self.state__ = S::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(S::C(Some(deserializer)));
                        *self.state__ = S::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                }
            }
        }
        impl<'de> Deserializer<'de, super::InnerType> for InnerTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::InnerType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::InnerType> {
                use InnerTypeDeserializerState as S;
                let mut event = event;
                let mut fallback = None;
                let mut allow_any_element = false;
                let (event, allow_any) = loop {
                    let state = replace(&mut *self.state__, S::Unknown__);
                    event = match (state, event) {
                        (S::Unknown__, _) => unreachable!(),
                        (S::C(Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_c(helper, output, &mut fallback)? {
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
                            *self.state__ = S::C(None);
                            event
                        }
                        (S::C(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_BIZ),
                                b"C",
                                false,
                            )?;
                            match self.handle_c(helper, output, &mut fallback)? {
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
            fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::InnerType, Error> {
                let state = replace(&mut *self.state__, InnerTypeDeserializerState::Unknown__);
                self.finish_state(helper, state)?;
                Ok(super::InnerType {
                    c: helper.finish_element("C", self.c)?,
                })
            }
        }
    }
    pub mod quick_xml_serialize {
        use xsd_parser_types::quick_xml::{
            BytesEnd, BytesStart, Error, Event, SerializeHelper, Serializer, WithSerializer,
        };
        #[derive(Debug)]
        pub struct InnerTypeSerializer<'ser> {
            pub(super) value: &'ser super::InnerType,
            pub(super) state: Box<InnerTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum InnerTypeSerializerState<'ser> {
            Init__,
            C(<String as WithSerializer>::Serializer<'ser>),
            End__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> InnerTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        InnerTypeSerializerState::Init__ => {
                            *self.state = InnerTypeSerializerState::C(WithSerializer::serializer(
                                &self.value.c,
                                Some("C"),
                                false,
                            )?);
                            let bytes = BytesStart::new(self.name);
                            return Ok(Some(Event::Start(bytes)));
                        }
                        InnerTypeSerializerState::C(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = InnerTypeSerializerState::End__,
                        },
                        InnerTypeSerializerState::End__ => {
                            *self.state = InnerTypeSerializerState::Done__;
                            return Ok(Some(Event::End(BytesEnd::new(self.name))));
                        }
                        InnerTypeSerializerState::Done__ => return Ok(None),
                        InnerTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for InnerTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = InnerTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
    }
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct OuterTypeDeserializer {
        bar_inner: Option<super::bar::InnerType>,
        baz_inner: Option<super::baz::InnerType>,
        biz_inner: Option<super::biz::InnerType>,
        state__: Box<OuterTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum OuterTypeDeserializerState {
        Init__,
        BarInner(Option<<super::bar::InnerType as WithDeserializer>::Deserializer>),
        BazInner(Option<<super::baz::InnerType as WithDeserializer>::Deserializer>),
        BizInner(Option<<super::biz::InnerType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl OuterTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                bar_inner: None,
                baz_inner: None,
                biz_inner: None,
                state__: Box::new(OuterTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: OuterTypeDeserializerState,
        ) -> Result<(), Error> {
            use OuterTypeDeserializerState as S;
            match state {
                S::BarInner(Some(deserializer)) => {
                    self.store_bar_inner(deserializer.finish(helper)?)?
                }
                S::BazInner(Some(deserializer)) => {
                    self.store_baz_inner(deserializer.finish(helper)?)?
                }
                S::BizInner(Some(deserializer)) => {
                    self.store_biz_inner(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_bar_inner(&mut self, value: super::bar::InnerType) -> Result<(), Error> {
            if self.bar_inner.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Inner",
                )))?;
            }
            self.bar_inner = Some(value);
            Ok(())
        }
        fn store_baz_inner(&mut self, value: super::baz::InnerType) -> Result<(), Error> {
            if self.baz_inner.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Inner",
                )))?;
            }
            self.baz_inner = Some(value);
            Ok(())
        }
        fn store_biz_inner(&mut self, value: super::biz::InnerType) -> Result<(), Error> {
            if self.biz_inner.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Inner",
                )))?;
            }
            self.biz_inner = Some(value);
            Ok(())
        }
        fn handle_bar_inner<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::bar::InnerType>,
            fallback: &mut Option<OuterTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OuterTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::BarInner(None));
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
                    self.store_bar_inner(data)?;
                    *self.state__ = S::BazInner(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::BarInner(Some(deserializer)));
                    *self.state__ = S::BazInner(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_baz_inner<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::baz::InnerType>,
            fallback: &mut Option<OuterTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OuterTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::BazInner(None));
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
                    self.store_baz_inner(data)?;
                    *self.state__ = S::BizInner(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::BazInner(Some(deserializer)));
                    *self.state__ = S::BizInner(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_biz_inner<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::biz::InnerType>,
            fallback: &mut Option<OuterTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OuterTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::BizInner(None));
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
                    self.store_biz_inner(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::BizInner(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::OuterType> for OuterTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OuterType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OuterType> {
            use OuterTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::BarInner(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_bar_inner(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::BazInner(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_baz_inner(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::BizInner(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_biz_inner(helper, output, &mut fallback)? {
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
                        *self.state__ = S::BarInner(None);
                        event
                    }
                    (S::BarInner(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_BAR),
                            b"Inner",
                            false,
                        )?;
                        match self.handle_bar_inner(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::BazInner(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_BAZ),
                            b"Inner",
                            false,
                        )?;
                        match self.handle_baz_inner(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::BizInner(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_BIZ),
                            b"Inner",
                            false,
                        )?;
                        match self.handle_biz_inner(helper, output, &mut fallback)? {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::OuterType, Error> {
            let state = replace(&mut *self.state__, OuterTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::OuterType {
                bar_inner: helper.finish_element("Inner", self.bar_inner)?,
                baz_inner: helper.finish_element("Inner", self.baz_inner)?,
                biz_inner: helper.finish_element("Inner", self.biz_inner)?,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, Error, Event, SerializeHelper, Serializer, WithSerializer,
    };
    #[derive(Debug)]
    pub struct OuterTypeSerializer<'ser> {
        pub(super) value: &'ser super::OuterType,
        pub(super) state: Box<OuterTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum OuterTypeSerializerState<'ser> {
        Init__,
        BarInner(<super::bar::InnerType as WithSerializer>::Serializer<'ser>),
        BazInner(<super::baz::InnerType as WithSerializer>::Serializer<'ser>),
        BizInner(<super::biz::InnerType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> OuterTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    OuterTypeSerializerState::Init__ => {
                        *self.state =
                            OuterTypeSerializerState::BarInner(WithSerializer::serializer(
                                &self.value.bar_inner,
                                Some("Inner"),
                                false,
                            )?);
                        let bytes = BytesStart::new(self.name);
                        return Ok(Some(Event::Start(bytes)));
                    }
                    OuterTypeSerializerState::BarInner(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                OuterTypeSerializerState::BazInner(WithSerializer::serializer(
                                    &self.value.baz_inner,
                                    Some("Inner"),
                                    false,
                                )?)
                        }
                    },
                    OuterTypeSerializerState::BazInner(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                OuterTypeSerializerState::BizInner(WithSerializer::serializer(
                                    &self.value.biz_inner,
                                    Some("Inner"),
                                    false,
                                )?)
                        }
                    },
                    OuterTypeSerializerState::BizInner(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = OuterTypeSerializerState::End__,
                    },
                    OuterTypeSerializerState::End__ => {
                        *self.state = OuterTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    OuterTypeSerializerState::Done__ => return Ok(None),
                    OuterTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for OuterTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = OuterTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
