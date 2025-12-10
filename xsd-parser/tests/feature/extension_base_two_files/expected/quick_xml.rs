use xsd_parser_types::misc::{Namespace, NamespacePrefix};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const NS_OTHER: Namespace = Namespace::new_const(b"http://other.example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub const PREFIX_OTHER: NamespacePrefix = NamespacePrefix::new_const(b"other");
pub mod other {
    use xsd_parser_types::quick_xml::{Error, WithDeserializer, WithSerializer};
    #[derive(Debug)]
    pub struct BarType {
        pub b: i32,
        pub c: String,
    }
    impl WithSerializer for BarType {
        type Serializer<'x> = quick_xml_serialize::BarTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::BarTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::BarTypeSerializerState::Init__),
                name: name.unwrap_or("other:BarType"),
                is_root,
            })
        }
    }
    impl WithDeserializer for BarType {
        type Deserializer = quick_xml_deserialize::BarTypeDeserializer;
    }
    pub mod quick_xml_deserialize {
        use core::mem::replace;
        use xsd_parser_types::quick_xml::{
            BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
            RawByteStr, WithDeserializer,
        };
        #[derive(Debug)]
        pub struct BarTypeDeserializer {
            b: Option<i32>,
            c: Option<String>,
            state__: Box<BarTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum BarTypeDeserializerState {
            Init__,
            B(Option<<i32 as WithDeserializer>::Deserializer>),
            C(Option<<String as WithDeserializer>::Deserializer>),
            Done__,
            Unknown__,
        }
        impl BarTypeDeserializer {
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
                    c: None,
                    state__: Box::new(BarTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: BarTypeDeserializerState,
            ) -> Result<(), Error> {
                use BarTypeDeserializerState as S;
                match state {
                    S::B(Some(deserializer)) => self.store_b(deserializer.finish(helper)?)?,
                    S::C(Some(deserializer)) => self.store_c(deserializer.finish(helper)?)?,
                    _ => (),
                }
                Ok(())
            }
            fn store_b(&mut self, value: i32) -> Result<(), Error> {
                if self.b.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"b")))?;
                }
                self.b = Some(value);
                Ok(())
            }
            fn store_c(&mut self, value: String) -> Result<(), Error> {
                if self.c.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"c")))?;
                }
                self.c = Some(value);
                Ok(())
            }
            fn handle_b<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, i32>,
                fallback: &mut Option<BarTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    fallback.get_or_insert(BarTypeDeserializerState::B(None));
                    if matches!(&fallback, Some(BarTypeDeserializerState::Init__)) {
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
                        *self.state__ = BarTypeDeserializerState::C(None);
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(BarTypeDeserializerState::B(Some(deserializer)));
                        *self.state__ = BarTypeDeserializerState::C(None);
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                }
            }
            fn handle_c<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, String>,
                fallback: &mut Option<BarTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    fallback.get_or_insert(BarTypeDeserializerState::C(None));
                    if matches!(&fallback, Some(BarTypeDeserializerState::Init__)) {
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
                        *self.state__ = BarTypeDeserializerState::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(BarTypeDeserializerState::C(Some(deserializer)));
                        *self.state__ = BarTypeDeserializerState::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                }
            }
        }
        impl<'de> Deserializer<'de, super::BarType> for BarTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::BarType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::BarType> {
                use BarTypeDeserializerState as S;
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
                            *self.state__ = BarTypeDeserializerState::B(None);
                            event
                        }
                        (S::B(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_OTHER),
                                b"b",
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
                        (S::C(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_OTHER),
                                b"c",
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
            fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::BarType, Error> {
                let state = replace(&mut *self.state__, BarTypeDeserializerState::Unknown__);
                self.finish_state(helper, state)?;
                Ok(super::BarType {
                    b: helper.finish_element("b", self.b)?,
                    c: helper.finish_element("c", self.c)?,
                })
            }
        }
    }
    pub mod quick_xml_serialize {
        use xsd_parser_types::quick_xml::{
            BytesEnd, BytesStart, Error, Event, SerializeHelper, Serializer, WithSerializer,
        };
        #[derive(Debug)]
        pub struct BarTypeSerializer<'ser> {
            pub(super) value: &'ser super::BarType,
            pub(super) state: Box<BarTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum BarTypeSerializerState<'ser> {
            Init__,
            B(<i32 as WithSerializer>::Serializer<'ser>),
            C(<String as WithSerializer>::Serializer<'ser>),
            End__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> BarTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        BarTypeSerializerState::Init__ => {
                            *self.state = BarTypeSerializerState::B(WithSerializer::serializer(
                                &self.value.b,
                                Some("other:b"),
                                false,
                            )?);
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            if self.is_root {
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_OTHER),
                                    &super::super::NS_OTHER,
                                );
                            }
                            return Ok(Some(Event::Start(bytes)));
                        }
                        BarTypeSerializerState::B(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = BarTypeSerializerState::C(WithSerializer::serializer(
                                    &self.value.c,
                                    Some("other:c"),
                                    false,
                                )?)
                            }
                        },
                        BarTypeSerializerState::C(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = BarTypeSerializerState::End__,
                        },
                        BarTypeSerializerState::End__ => {
                            *self.state = BarTypeSerializerState::Done__;
                            helper.end_ns_scope();
                            return Ok(Some(Event::End(BytesEnd::new(self.name))));
                        }
                        BarTypeSerializerState::Done__ => return Ok(None),
                        BarTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for BarTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = BarTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
    }
}
pub mod tns {
    use xsd_parser_types::quick_xml::{Error, WithDeserializer, WithSerializer};
    pub type Foo = FooType;
    #[derive(Debug)]
    pub struct FooType {
        pub a: f32,
        pub b: super::other::BarType,
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
            a: Option<f32>,
            b: Option<super::super::other::BarType>,
            state__: Box<FooTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum FooTypeDeserializerState {
            Init__,
            A(Option<<f32 as WithDeserializer>::Deserializer>),
            B(Option<<super::super::other::BarType as WithDeserializer>::Deserializer>),
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
                    a: None,
                    b: None,
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
                    S::A(Some(deserializer)) => self.store_a(deserializer.finish(helper)?)?,
                    S::B(Some(deserializer)) => self.store_b(deserializer.finish(helper)?)?,
                    _ => (),
                }
                Ok(())
            }
            fn store_a(&mut self, value: f32) -> Result<(), Error> {
                if self.a.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"a")))?;
                }
                self.a = Some(value);
                Ok(())
            }
            fn store_b(&mut self, value: super::super::other::BarType) -> Result<(), Error> {
                if self.b.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"b")))?;
                }
                self.b = Some(value);
                Ok(())
            }
            fn handle_a<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, f32>,
                fallback: &mut Option<FooTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    fallback.get_or_insert(FooTypeDeserializerState::A(None));
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
                        self.store_a(data)?;
                        *self.state__ = FooTypeDeserializerState::B(None);
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(FooTypeDeserializerState::A(Some(deserializer)));
                        *self.state__ = FooTypeDeserializerState::B(None);
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                }
            }
            fn handle_b<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, super::super::other::BarType>,
                fallback: &mut Option<FooTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    fallback.get_or_insert(FooTypeDeserializerState::B(None));
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
                        self.store_b(data)?;
                        *self.state__ = FooTypeDeserializerState::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(FooTypeDeserializerState::B(Some(deserializer)));
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
                            *self.state__ = FooTypeDeserializerState::A(None);
                            event
                        }
                        (S::A(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_TNS),
                                b"a",
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
                        (S::B(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_TNS),
                                b"b",
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
            fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::FooType, Error> {
                let state = replace(&mut *self.state__, FooTypeDeserializerState::Unknown__);
                self.finish_state(helper, state)?;
                Ok(super::FooType {
                    a: helper.finish_element("a", self.a)?,
                    b: helper.finish_element("b", self.b)?,
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
            A(<f32 as WithSerializer>::Serializer<'ser>),
            B(<super::super::other::BarType as WithSerializer>::Serializer<'ser>),
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
                            *self.state = FooTypeSerializerState::A(WithSerializer::serializer(
                                &self.value.a,
                                Some("tns:a"),
                                false,
                            )?);
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            if self.is_root {
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_TNS),
                                    &super::super::NS_TNS,
                                );
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_OTHER),
                                    &super::super::NS_OTHER,
                                );
                            }
                            return Ok(Some(Event::Start(bytes)));
                        }
                        FooTypeSerializerState::A(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = FooTypeSerializerState::B(WithSerializer::serializer(
                                    &self.value.b,
                                    Some("tns:b"),
                                    false,
                                )?)
                            }
                        },
                        FooTypeSerializerState::B(x) => match x.next(helper).transpose()? {
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
}
