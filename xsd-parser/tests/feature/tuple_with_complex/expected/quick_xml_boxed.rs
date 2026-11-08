use core::fmt::Debug;
use xsd_parser_types::{misc::Namespace, quick_xml::WithDeserializer, AsAny};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
#[derive(Debug)]
pub struct Foo(pub FooType);
impl WithDeserializer for Foo {
    type Deserializer = quick_xml_deserialize::FooDeserializer;
}
#[derive(Debug)]
pub struct FooType {
    pub name: String,
    pub value: i32,
}
impl WithDeserializer for FooType {
    type Deserializer = Box<quick_xml_deserialize::FooTypeDeserializer>;
}
#[derive(Debug)]
pub struct Holder(pub HolderType);
impl WithDeserializer for Holder {
    type Deserializer = quick_xml_deserialize::HolderDeserializer;
}
#[derive(Debug)]
pub struct HolderType {
    pub head: Head,
}
impl WithDeserializer for HolderType {
    type Deserializer = Box<quick_xml_deserialize::HolderTypeDeserializer>;
}
#[derive(Debug)]
pub struct Head(pub Box<dyn HeadTrait>);
pub trait HeadTrait: Debug + AsAny {}
impl Head {
    pub fn new<T: HeadTrait + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
impl WithDeserializer for Head {
    type Deserializer = Box<quick_xml_deserialize::HeadDeserializer>;
}
#[derive(Debug)]
pub struct MemberType {
    pub info: String,
}
impl HeadTrait for MemberType {}
impl WithDeserializer for MemberType {
    type Deserializer = Box<quick_xml_deserialize::MemberTypeDeserializer>;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        QName, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct FooDeserializer(<super::FooType as WithDeserializer>::Deserializer);
    impl<'de> Deserializer<'de, super::Foo> for FooDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Foo> {
            let output = <super::FooType as WithDeserializer>::Deserializer::init(helper, event)?;
            Ok(Self::map_output(output))
        }
        fn next(
            self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Foo> {
            let output = self.0.next(helper, event)?;
            Ok(Self::map_output(output))
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::Foo, Error> {
            Ok(super::Foo(self.0.finish(helper)?))
        }
    }
    impl FooDeserializer {
        fn map_output<'de>(
            output: DeserializerOutput<'de, super::FooType>,
        ) -> DeserializerOutput<'de, super::Foo> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            DeserializerOutput {
                artifact: artifact.map(super::Foo, Self),
                event,
                allow_any,
            }
        }
    }
    #[derive(Debug)]
    pub struct FooTypeDeserializer {
        name: Option<String>,
        value: Option<i32>,
        state__: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Init__,
        Name(Option<<String as WithDeserializer>::Deserializer>),
        Value(Option<<i32 as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl FooTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Box::new(Self {
                name: None,
                value: None,
                state__: Box::new(FooTypeDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: FooTypeDeserializerState,
        ) -> Result<(), Error> {
            use FooTypeDeserializerState as S;
            match state {
                S::Name(Some(deserializer)) => self.store_name(deserializer.finish(helper)?)?,
                S::Value(Some(deserializer)) => self.store_value(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_name(&mut self, value: String) -> Result<(), Error> {
            if self.name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Name")))?;
            }
            self.name = Some(value);
            Ok(())
        }
        fn store_value(&mut self, value: i32) -> Result<(), Error> {
            if self.value.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Value",
                )))?;
            }
            self.value = Some(value);
            Ok(())
        }
        fn handle_name<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<FooTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FooTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Name(None));
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
                    self.store_name(data)?;
                    *self.state__ = S::Value(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Name(Some(deserializer)));
                    *self.state__ = S::Value(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_value<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<FooTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use FooTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Value(None));
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
                    self.store_value(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Value(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::FooType> for Box<FooTypeDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooType> {
            helper.init_deserializer_from_start_event(event, FooTypeDeserializer::from_bytes_start)
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
                    (S::Name(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_name(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Value(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_value(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Name(None);
                        event
                    }
                    (S::Name(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Name",
                            false,
                        )?;
                        match self.handle_name(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Value(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Value",
                            false,
                        )?;
                        match self.handle_value(helper, output, &mut fallback)? {
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
            let event = event.into_break();
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
                name: helper.finish_element("Name", self.name)?,
                value: helper.finish_element("Value", self.value)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct HolderDeserializer(<super::HolderType as WithDeserializer>::Deserializer);
    impl<'de> Deserializer<'de, super::Holder> for HolderDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Holder> {
            let output =
                <super::HolderType as WithDeserializer>::Deserializer::init(helper, event)?;
            Ok(Self::map_output(output))
        }
        fn next(
            self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Holder> {
            let output = self.0.next(helper, event)?;
            Ok(Self::map_output(output))
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::Holder, Error> {
            Ok(super::Holder(self.0.finish(helper)?))
        }
    }
    impl HolderDeserializer {
        fn map_output<'de>(
            output: DeserializerOutput<'de, super::HolderType>,
        ) -> DeserializerOutput<'de, super::Holder> {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            DeserializerOutput {
                artifact: artifact.map(super::Holder, Self),
                event,
                allow_any,
            }
        }
    }
    #[derive(Debug)]
    pub struct HolderTypeDeserializer {
        head: Option<super::Head>,
        state__: Box<HolderTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum HolderTypeDeserializerState {
        Init__,
        Head(Option<<super::Head as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl HolderTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Box::new(Self {
                head: None,
                state__: Box::new(HolderTypeDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: HolderTypeDeserializerState,
        ) -> Result<(), Error> {
            use HolderTypeDeserializerState as S;
            match state {
                S::Head(Some(deserializer)) => self.store_head(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_head(&mut self, value: super::Head) -> Result<(), Error> {
            if self.head.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Head")))?;
            }
            self.head = Some(value);
            Ok(())
        }
        fn handle_head<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::Head>,
            fallback: &mut Option<HolderTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HolderTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Head(None));
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
                    self.store_head(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Head(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::HolderType> for Box<HolderTypeDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HolderType> {
            helper
                .init_deserializer_from_start_event(event, HolderTypeDeserializer::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HolderType> {
            use HolderTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Head(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_head(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Head(None);
                        event
                    }
                    (S::Head(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = <super::Head as WithDeserializer>::init_with_name(
                            helper,
                            event,
                            Some(&super::NS_TNS),
                            b"Head",
                        )?;
                        match self.handle_head(helper, output, &mut fallback)? {
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
            let event = event.into_break();
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::HolderType, Error> {
            let state = replace(&mut *self.state__, HolderTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::HolderType {
                head: helper.finish_element("Head", self.head)?,
            })
        }
    }
    #[derive(Debug)]
    pub enum HeadDeserializer {
        Member(<super::MemberType as WithDeserializer>::Deserializer),
    }
    impl<'de> Deserializer<'de, super::Head> for Box<HeadDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Head> {
            let Some(type_name) = helper.get_dynamic_type_from_tag(&event) else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::None,
                    allow_any: false,
                });
            };
            let type_name = type_name.into_owned();
            if matches!(
                helper.resolve_element_local_name(QName(&type_name), &super::NS_TNS),
                Some(b"Member")
            ) {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = <super::MemberType as WithDeserializer>::init(helper, event)?;
                return Ok(DeserializerOutput {
                    artifact: artifact.map(
                        |x| super::Head(Box::new(x)),
                        |x| Box::new(HeadDeserializer::Member(x)),
                    ),
                    event,
                    allow_any,
                });
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::None,
                event: DeserializerEvent::Continue(event),
                allow_any: false,
            })
        }
        fn next(
            self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Head> {
            match *self {
                HeadDeserializer::Member(x) => {
                    let DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = x.next(helper, event)?;
                    Ok(DeserializerOutput {
                        artifact: artifact.map(
                            |x| super::Head(Box::new(x)),
                            |x| Box::new(HeadDeserializer::Member(x)),
                        ),
                        event,
                        allow_any,
                    })
                }
            }
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::Head, Error> {
            match *self {
                HeadDeserializer::Member(x) => Ok(super::Head(Box::new(x.finish(helper)?))),
            }
        }
    }
    #[derive(Debug)]
    pub struct MemberTypeDeserializer {
        info: Option<String>,
        state__: Box<MemberTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum MemberTypeDeserializerState {
        Init__,
        Info(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl MemberTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Box<Self>, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Box::new(Self {
                info: None,
                state__: Box::new(MemberTypeDeserializerState::Init__),
            }))
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: MemberTypeDeserializerState,
        ) -> Result<(), Error> {
            use MemberTypeDeserializerState as S;
            match state {
                S::Info(Some(deserializer)) => self.store_info(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_info(&mut self, value: String) -> Result<(), Error> {
            if self.info.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Info")))?;
            }
            self.info = Some(value);
            Ok(())
        }
        fn handle_info<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<MemberTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MemberTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Info(None));
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
                    self.store_info(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Info(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::MemberType> for Box<MemberTypeDeserializer> {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MemberType> {
            helper
                .init_deserializer_from_start_event(event, MemberTypeDeserializer::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MemberType> {
            use MemberTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Info(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_info(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Info(None);
                        event
                    }
                    (S::Info(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Info",
                            false,
                        )?;
                        match self.handle_info(helper, output, &mut fallback)? {
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
            let event = event.into_break();
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::MemberType, Error> {
            let state = replace(&mut *self.state__, MemberTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::MemberType {
                info: helper.finish_element("Info", self.info)?,
            })
        }
    }
}
