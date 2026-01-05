use xsd_parser_types::misc::{Namespace, NamespacePrefix};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_UNNAMED_3: Namespace = Namespace::new_const(b"Test");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub mod bar {
    use xsd_parser_types::quick_xml::{Error, WithDeserializer, WithSerializer};
    pub type Inner = InnerType;
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
    pub type Outer = OuterType;
    #[derive(Debug)]
    pub struct OuterType {
        pub inner: InnerType,
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
                                Some(&super::super::NS_UNNAMED_3),
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
        #[derive(Debug)]
        pub struct OuterTypeDeserializer {
            inner: Option<super::InnerType>,
            state__: Box<OuterTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum OuterTypeDeserializerState {
            Init__,
            Inner(Option<<super::InnerType as WithDeserializer>::Deserializer>),
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
                    inner: None,
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
                    S::Inner(Some(deserializer)) => {
                        self.store_inner(deserializer.finish(helper)?)?
                    }
                    _ => (),
                }
                Ok(())
            }
            fn store_inner(&mut self, value: super::InnerType) -> Result<(), Error> {
                if self.inner.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"Inner",
                    )))?;
                }
                self.inner = Some(value);
                Ok(())
            }
            fn handle_inner<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, super::InnerType>,
                fallback: &mut Option<OuterTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use OuterTypeDeserializerState as S;
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    fallback.get_or_insert(S::Inner(None));
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
                        self.store_inner(data)?;
                        *self.state__ = S::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(S::Inner(Some(deserializer)));
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
                        (S::Inner(Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_inner(helper, output, &mut fallback)? {
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
                            *self.state__ = S::Inner(None);
                            event
                        }
                        (S::Inner(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_UNNAMED_3),
                                b"Inner",
                                false,
                            )?;
                            match self.handle_inner(helper, output, &mut fallback)? {
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
                    inner: helper.finish_element("Inner", self.inner)?,
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
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            helper.write_xmlns(&mut bytes, None, &super::super::NS_UNNAMED_3);
                            return Ok(Some(Event::Start(bytes)));
                        }
                        InnerTypeSerializerState::B(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = InnerTypeSerializerState::End__,
                        },
                        InnerTypeSerializerState::End__ => {
                            *self.state = InnerTypeSerializerState::Done__;
                            helper.end_ns_scope();
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
            Inner(<super::InnerType as WithSerializer>::Serializer<'ser>),
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
                                OuterTypeSerializerState::Inner(WithSerializer::serializer(
                                    &self.value.inner,
                                    Some("Inner"),
                                    false,
                                )?);
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            helper.write_xmlns(&mut bytes, None, &super::super::NS_UNNAMED_3);
                            return Ok(Some(Event::Start(bytes)));
                        }
                        OuterTypeSerializerState::Inner(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = OuterTypeSerializerState::End__,
                        },
                        OuterTypeSerializerState::End__ => {
                            *self.state = OuterTypeSerializerState::Done__;
                            helper.end_ns_scope();
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
}
pub mod foo {
    use xsd_parser_types::quick_xml::{Error, WithDeserializer, WithSerializer};
    pub type Inner = InnerType;
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
    pub type Outer = OuterType;
    #[derive(Debug)]
    pub struct OuterType {
        pub inner: InnerType,
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
                                Some(&super::super::NS_UNNAMED_3),
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
        #[derive(Debug)]
        pub struct OuterTypeDeserializer {
            inner: Option<super::InnerType>,
            state__: Box<OuterTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum OuterTypeDeserializerState {
            Init__,
            Inner(Option<<super::InnerType as WithDeserializer>::Deserializer>),
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
                    inner: None,
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
                    S::Inner(Some(deserializer)) => {
                        self.store_inner(deserializer.finish(helper)?)?
                    }
                    _ => (),
                }
                Ok(())
            }
            fn store_inner(&mut self, value: super::InnerType) -> Result<(), Error> {
                if self.inner.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"Inner",
                    )))?;
                }
                self.inner = Some(value);
                Ok(())
            }
            fn handle_inner<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, super::InnerType>,
                fallback: &mut Option<OuterTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use OuterTypeDeserializerState as S;
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    fallback.get_or_insert(S::Inner(None));
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
                        self.store_inner(data)?;
                        *self.state__ = S::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(S::Inner(Some(deserializer)));
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
                        (S::Inner(Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_inner(helper, output, &mut fallback)? {
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
                            *self.state__ = S::Inner(None);
                            event
                        }
                        (S::Inner(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_UNNAMED_3),
                                b"Inner",
                                false,
                            )?;
                            match self.handle_inner(helper, output, &mut fallback)? {
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
                    inner: helper.finish_element("Inner", self.inner)?,
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
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            helper.write_xmlns(&mut bytes, None, &super::super::NS_UNNAMED_3);
                            return Ok(Some(Event::Start(bytes)));
                        }
                        InnerTypeSerializerState::A(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = InnerTypeSerializerState::End__,
                        },
                        InnerTypeSerializerState::End__ => {
                            *self.state = InnerTypeSerializerState::Done__;
                            helper.end_ns_scope();
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
            Inner(<super::InnerType as WithSerializer>::Serializer<'ser>),
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
                                OuterTypeSerializerState::Inner(WithSerializer::serializer(
                                    &self.value.inner,
                                    Some("Inner"),
                                    false,
                                )?);
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            helper.write_xmlns(&mut bytes, None, &super::super::NS_UNNAMED_3);
                            return Ok(Some(Event::Start(bytes)));
                        }
                        OuterTypeSerializerState::Inner(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = OuterTypeSerializerState::End__,
                        },
                        OuterTypeSerializerState::End__ => {
                            *self.state = OuterTypeSerializerState::Done__;
                            helper.end_ns_scope();
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
}
pub mod xs {
    use std::borrow::Cow;
    use xsd_parser_types::quick_xml::{
        DeserializeBytes, DeserializeHelper, Error, SerializeBytes, SerializeHelper,
        WithDeserializer, WithSerializer,
    };
    #[derive(Debug, Default)]
    pub struct EntitiesType(pub Vec<String>);
    impl SerializeBytes for EntitiesType {
        fn serialize_bytes(
            &self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Cow<'_, str>>, Error> {
            if self.0.is_empty() {
                return Ok(None);
            }
            let mut data = String::new();
            for item in &self.0 {
                if let Some(bytes) = item.serialize_bytes(helper)? {
                    if !data.is_empty() {
                        data.push(' ');
                    }
                    data.push_str(&bytes);
                }
            }
            Ok(Some(Cow::Owned(data)))
        }
    }
    impl DeserializeBytes for EntitiesType {
        fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
            Ok(Self(
                bytes
                    .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                    .map(|bytes| String::deserialize_bytes(helper, bytes))
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }
    #[derive(Debug, Default)]
    pub struct EntityType(pub Vec<String>);
    impl SerializeBytes for EntityType {
        fn serialize_bytes(
            &self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Cow<'_, str>>, Error> {
            if self.0.is_empty() {
                return Ok(None);
            }
            let mut data = String::new();
            for item in &self.0 {
                if let Some(bytes) = item.serialize_bytes(helper)? {
                    if !data.is_empty() {
                        data.push(' ');
                    }
                    data.push_str(&bytes);
                }
            }
            Ok(Some(Cow::Owned(data)))
        }
    }
    impl DeserializeBytes for EntityType {
        fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
            Ok(Self(
                bytes
                    .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                    .map(|bytes| String::deserialize_bytes(helper, bytes))
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }
    pub type IdType = String;
    pub type IdrefType = String;
    #[derive(Debug, Default)]
    pub struct IdrefsType(pub Vec<String>);
    impl SerializeBytes for IdrefsType {
        fn serialize_bytes(
            &self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Cow<'_, str>>, Error> {
            if self.0.is_empty() {
                return Ok(None);
            }
            let mut data = String::new();
            for item in &self.0 {
                if let Some(bytes) = item.serialize_bytes(helper)? {
                    if !data.is_empty() {
                        data.push(' ');
                    }
                    data.push_str(&bytes);
                }
            }
            Ok(Some(Cow::Owned(data)))
        }
    }
    impl DeserializeBytes for IdrefsType {
        fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
            Ok(Self(
                bytes
                    .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                    .map(|bytes| String::deserialize_bytes(helper, bytes))
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }
    pub type NcNameType = String;
    pub type NmtokenType = String;
    #[derive(Debug, Default)]
    pub struct NmtokensType(pub Vec<String>);
    impl SerializeBytes for NmtokensType {
        fn serialize_bytes(
            &self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Cow<'_, str>>, Error> {
            if self.0.is_empty() {
                return Ok(None);
            }
            let mut data = String::new();
            for item in &self.0 {
                if let Some(bytes) = item.serialize_bytes(helper)? {
                    if !data.is_empty() {
                        data.push(' ');
                    }
                    data.push_str(&bytes);
                }
            }
            Ok(Some(Cow::Owned(data)))
        }
    }
    impl DeserializeBytes for NmtokensType {
        fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
            Ok(Self(
                bytes
                    .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                    .map(|bytes| String::deserialize_bytes(helper, bytes))
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }
    pub type NotationType = String;
    pub type NameType = String;
    pub type QNameType = String;
    pub type AnySimpleType = String;
    #[derive(Debug)]
    pub struct AnyType;
    impl WithSerializer for AnyType {
        type Serializer<'x> = quick_xml_serialize::AnyTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::AnyTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::AnyTypeSerializerState::Init__),
                name: name.unwrap_or("anyType"),
                is_root,
            })
        }
    }
    impl WithDeserializer for AnyType {
        type Deserializer = quick_xml_deserialize::AnyTypeDeserializer;
    }
    pub type AnyUriType = String;
    pub type Base64BinaryType = String;
    pub type BooleanType = bool;
    pub type ByteType = i8;
    pub type DateType = String;
    pub type DateTimeType = String;
    pub type DecimalType = f64;
    pub type DoubleType = f64;
    pub type DurationType = String;
    pub type FloatType = f32;
    pub type GDayType = String;
    pub type GMonthType = String;
    pub type GMonthDayType = String;
    pub type GYearType = String;
    pub type GYearMonthType = String;
    pub type HexBinaryType = String;
    pub type IntType = i32;
    pub type IntegerType = i32;
    pub type LanguageType = String;
    pub type LongType = i64;
    pub type NegativeIntegerType = isize;
    pub type NonNegativeIntegerType = usize;
    pub type NonPositiveIntegerType = isize;
    pub type NormalizedStringType = String;
    pub type PositiveIntegerType = usize;
    pub type ShortType = i16;
    pub type StringType = String;
    pub type TimeType = String;
    pub type TokenType = String;
    pub type UnsignedByteType = u8;
    pub type UnsignedIntType = u32;
    pub type UnsignedLongType = u64;
    pub type UnsignedShortType = u16;
    pub mod quick_xml_deserialize {
        use core::mem::replace;
        use xsd_parser_types::quick_xml::{
            BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult, Error, Event,
        };
        #[derive(Debug)]
        pub struct AnyTypeDeserializer {
            state__: Box<AnyTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum AnyTypeDeserializerState {
            Init__,
            Unknown__,
        }
        impl AnyTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                Ok(Self {
                    state__: Box::new(AnyTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: AnyTypeDeserializerState,
            ) -> Result<(), Error> {
                Ok(())
            }
        }
        impl<'de> Deserializer<'de, super::AnyType> for AnyTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::AnyType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::AnyType> {
                if let Event::End(_) = &event {
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(helper)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    })
                } else if matches!(&event, Event::Text(_) | Event::CData(_)) {
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event: DeserializerEvent::None,
                        allow_any: true,
                    })
                } else {
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event: DeserializerEvent::Break(event),
                        allow_any: true,
                    })
                }
            }
            fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::AnyType, Error> {
                let state = replace(&mut *self.state__, AnyTypeDeserializerState::Unknown__);
                self.finish_state(helper, state)?;
                Ok(super::AnyType {})
            }
        }
    }
    pub mod quick_xml_serialize {
        use xsd_parser_types::quick_xml::{BytesStart, Error, Event, SerializeHelper, Serializer};
        #[derive(Debug)]
        pub struct AnyTypeSerializer<'ser> {
            pub(super) value: &'ser super::AnyType,
            pub(super) state: Box<AnyTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum AnyTypeSerializerState<'ser> {
            Init__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> AnyTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        AnyTypeSerializerState::Init__ => {
                            *self.state = AnyTypeSerializerState::Done__;
                            let bytes = BytesStart::new(self.name);
                            return Ok(Some(Event::Empty(bytes)));
                        }
                        AnyTypeSerializerState::Done__ => return Ok(None),
                        AnyTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for AnyTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = AnyTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
    }
}
