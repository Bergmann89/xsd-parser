use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{Error, WithDeserializer, WithSerializer},
    xml::{Mixed, Text},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub type NormalElement = NormalType;
#[derive(Debug)]
pub struct NormalType {
    pub group: NormalGroupType,
    pub baz: String,
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
pub struct NormalGroupType {
    pub fuu: i32,
    pub bar: String,
}
impl WithSerializer for NormalGroupType {
    type Serializer<'x> = quick_xml_serialize::NormalGroupTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::NormalGroupTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::NormalGroupTypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for NormalGroupType {
    type Deserializer = quick_xml_deserialize::NormalGroupTypeDeserializer;
}
pub type MixedElement = MixedType;
#[derive(Debug)]
pub struct MixedType {
    pub text_before: Option<Text>,
    pub group: MixedGroupType,
    pub baz: Mixed<String>,
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
pub struct MixedGroupType {
    pub fuu: Mixed<i32>,
    pub bar: Mixed<String>,
}
impl WithSerializer for MixedGroupType {
    type Serializer<'x> = quick_xml_serialize::MixedGroupTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::MixedGroupTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::MixedGroupTypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for MixedGroupType {
    type Deserializer = quick_xml_deserialize::MixedGroupTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::{
        quick_xml::{
            BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
            RawByteStr, WithDeserializer,
        },
        xml::{Mixed, Text},
    };
    #[derive(Debug)]
    pub struct NormalTypeDeserializer {
        group: Option<super::NormalGroupType>,
        baz: Option<String>,
        state__: Box<NormalTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum NormalTypeDeserializerState {
        Init__,
        Group(Option<<super::NormalGroupType as WithDeserializer>::Deserializer>),
        Baz(Option<<String as WithDeserializer>::Deserializer>),
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
                group: None,
                baz: None,
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
            if self.group.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Group",
                )))?;
            }
            self.group = Some(value);
            Ok(())
        }
        fn store_baz(&mut self, value: String) -> Result<(), Error> {
            if self.baz.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Baz")))?;
            }
            self.baz = Some(value);
            Ok(())
        }
        fn handle_group<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::NormalGroupType>,
            fallback: &mut Option<NormalTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use NormalTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Group(None));
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
                    self.store_group(data)?;
                    *self.state__ = S::Baz(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Group(Some(deserializer)));
                    *self.state__ = S::Baz(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_baz<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<NormalTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use NormalTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Baz(None));
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
                    self.store_baz(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Baz(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
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
                        *self.state__ = S::Group(None);
                        event
                    }
                    (S::Group(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::NormalGroupType as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::NormalType, Error> {
            let state = replace(&mut *self.state__, NormalTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::NormalType {
                group: helper.finish_element("Group", self.group)?,
                baz: helper.finish_element("Baz", self.baz)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct NormalGroupTypeDeserializer {
        fuu: Option<i32>,
        bar: Option<String>,
        state__: Box<NormalGroupTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum NormalGroupTypeDeserializerState {
        Init__,
        Fuu(Option<<i32 as WithDeserializer>::Deserializer>),
        Bar(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl NormalGroupTypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: NormalGroupTypeDeserializerState,
        ) -> Result<(), Error> {
            use NormalGroupTypeDeserializerState as S;
            match state {
                S::Fuu(Some(deserializer)) => self.store_fuu(deserializer.finish(helper)?)?,
                S::Bar(Some(deserializer)) => self.store_bar(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_fuu(&mut self, value: i32) -> Result<(), Error> {
            if self.fuu.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Fuu")))?;
            }
            self.fuu = Some(value);
            Ok(())
        }
        fn store_bar(&mut self, value: String) -> Result<(), Error> {
            if self.bar.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Bar")))?;
            }
            self.bar = Some(value);
            Ok(())
        }
        fn handle_fuu<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<NormalGroupTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use NormalGroupTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Fuu(None));
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
                    self.store_fuu(data)?;
                    *self.state__ = S::Bar(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Fuu(Some(deserializer)));
                    *self.state__ = S::Bar(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_bar<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<NormalGroupTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use NormalGroupTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Bar(None));
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
                    self.store_bar(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Bar(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::NormalGroupType> for NormalGroupTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NormalGroupType> {
            let deserializer = Self {
                fuu: None,
                bar: None,
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
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Fuu(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fuu(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Bar(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_bar(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Fuu(None);
                        event
                    }
                    (S::Fuu(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Fuu",
                            false,
                        )?;
                        match self.handle_fuu(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Bar(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Bar",
                            false,
                        )?;
                        match self.handle_bar(helper, output, &mut fallback)? {
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
        ) -> Result<super::NormalGroupType, Error> {
            let state = replace(
                &mut *self.state__,
                NormalGroupTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::NormalGroupType {
                fuu: helper.finish_element("Fuu", self.fuu)?,
                bar: helper.finish_element("Bar", self.bar)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct MixedTypeDeserializer {
        text_before: Option<Text>,
        group: Option<super::MixedGroupType>,
        baz: Option<Mixed<String>>,
        state__: Box<MixedTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum MixedTypeDeserializerState {
        Init__,
        TextBefore(Option<<Text as WithDeserializer>::Deserializer>),
        Group(Option<<super::MixedGroupType as WithDeserializer>::Deserializer>),
        Baz(Option<<Mixed<String> as WithDeserializer>::Deserializer>),
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
                text_before: None,
                group: None,
                baz: None,
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
                S::TextBefore(Some(deserializer)) => {
                    self.store_text_before(deserializer.finish(helper)?)?
                }
                S::Group(Some(deserializer)) => self.store_group(deserializer.finish(helper)?)?,
                S::Baz(Some(deserializer)) => self.store_baz(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_text_before(&mut self, value: Text) -> Result<(), Error> {
            if self.text_before.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"text_before",
                )))?;
            }
            self.text_before = Some(value);
            Ok(())
        }
        fn store_group(&mut self, value: super::MixedGroupType) -> Result<(), Error> {
            if self.group.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Group",
                )))?;
            }
            self.group = Some(value);
            Ok(())
        }
        fn store_baz(&mut self, value: Mixed<String>) -> Result<(), Error> {
            if self.baz.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Baz")))?;
            }
            self.baz = Some(value);
            Ok(())
        }
        fn handle_text_before<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, Text>,
            fallback: &mut Option<MixedTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::TextBefore(None));
                *self.state__ = S::Group(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_before(data)?;
                    *self.state__ = S::Group(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::TextBefore(Some(deserializer)));
                    *self.state__ = S::Group(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_group<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::MixedGroupType>,
            fallback: &mut Option<MixedTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Group(None));
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
                    self.store_group(data)?;
                    *self.state__ = S::Baz(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Group(Some(deserializer)));
                    *self.state__ = S::Baz(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_baz<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, Mixed<String>>,
            fallback: &mut Option<MixedTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Baz(None));
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
                    self.store_baz(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Baz(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
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
                    (S::TextBefore(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_text_before(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
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
                        *self.state__ = S::TextBefore(None);
                        event
                    }
                    (S::TextBefore(None), event) => {
                        let output = <Text as WithDeserializer>::init(helper, event)?;
                        match self.handle_text_before(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Group(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::MixedGroupType as WithDeserializer>::init(helper, event)?;
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
                        *self.state__ = S::Done__;
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
                text_before: self.text_before,
                group: helper.finish_element("Group", self.group)?,
                baz: helper.finish_element("Baz", self.baz)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct MixedGroupTypeDeserializer {
        fuu: Option<Mixed<i32>>,
        bar: Option<Mixed<String>>,
        state__: Box<MixedGroupTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum MixedGroupTypeDeserializerState {
        Init__,
        Fuu(Option<<Mixed<i32> as WithDeserializer>::Deserializer>),
        Bar(Option<<Mixed<String> as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl MixedGroupTypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: MixedGroupTypeDeserializerState,
        ) -> Result<(), Error> {
            use MixedGroupTypeDeserializerState as S;
            match state {
                S::Fuu(Some(deserializer)) => self.store_fuu(deserializer.finish(helper)?)?,
                S::Bar(Some(deserializer)) => self.store_bar(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_fuu(&mut self, value: Mixed<i32>) -> Result<(), Error> {
            if self.fuu.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Fuu")))?;
            }
            self.fuu = Some(value);
            Ok(())
        }
        fn store_bar(&mut self, value: Mixed<String>) -> Result<(), Error> {
            if self.bar.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Bar")))?;
            }
            self.bar = Some(value);
            Ok(())
        }
        fn handle_fuu<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, Mixed<i32>>,
            fallback: &mut Option<MixedGroupTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedGroupTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Fuu(None));
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
                    self.store_fuu(data)?;
                    *self.state__ = S::Bar(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Fuu(Some(deserializer)));
                    *self.state__ = S::Bar(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_bar<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, Mixed<String>>,
            fallback: &mut Option<MixedGroupTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedGroupTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Bar(None));
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
                    self.store_bar(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Bar(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::MixedGroupType> for MixedGroupTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedGroupType> {
            let deserializer = Self {
                fuu: None,
                bar: None,
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
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Fuu(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fuu(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Bar(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_bar(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Fuu(None);
                        event
                    }
                    (S::Fuu(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Fuu",
                            false,
                        )?;
                        match self.handle_fuu(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Bar(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Bar",
                            false,
                        )?;
                        match self.handle_bar(helper, output, &mut fallback)? {
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
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::MixedGroupType, Error> {
            let state = replace(
                &mut *self.state__,
                MixedGroupTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::MixedGroupType {
                fuu: helper.finish_element("Fuu", self.fuu)?,
                bar: helper.finish_element("Bar", self.bar)?,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::{
        quick_xml::{
            BytesEnd, BytesStart, Error, Event, IterSerializer, SerializeHelper, Serializer,
            WithSerializer,
        },
        xml::{Mixed, Text},
    };
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
        Group(<super::NormalGroupType as WithSerializer>::Serializer<'ser>),
        Baz(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> NormalTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    NormalTypeSerializerState::Init__ => {
                        *self.state = NormalTypeSerializerState::Group(WithSerializer::serializer(
                            &self.value.group,
                            Some("Group"),
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
                    NormalTypeSerializerState::Group(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                NormalTypeSerializerState::Baz(WithSerializer::serializer(
                                    &self.value.baz,
                                    Some("tns:Baz"),
                                    false,
                                )?)
                        }
                    },
                    NormalTypeSerializerState::Baz(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = NormalTypeSerializerState::End__,
                    },
                    NormalTypeSerializerState::End__ => {
                        *self.state = NormalTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    NormalTypeSerializerState::Done__ => return Ok(None),
                    NormalTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for NormalTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
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
    }
    #[derive(Debug)]
    pub(super) enum NormalGroupTypeSerializerState<'ser> {
        Init__,
        Fuu(<i32 as WithSerializer>::Serializer<'ser>),
        Bar(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> NormalGroupTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    NormalGroupTypeSerializerState::Init__ => {
                        *self.state = NormalGroupTypeSerializerState::Fuu(
                            WithSerializer::serializer(&self.value.fuu, Some("tns:Fuu"), false)?,
                        );
                    }
                    NormalGroupTypeSerializerState::Fuu(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                NormalGroupTypeSerializerState::Bar(WithSerializer::serializer(
                                    &self.value.bar,
                                    Some("tns:Bar"),
                                    false,
                                )?)
                        }
                    },
                    NormalGroupTypeSerializerState::Bar(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = NormalGroupTypeSerializerState::Done__,
                    },
                    NormalGroupTypeSerializerState::Done__ => return Ok(None),
                    NormalGroupTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for NormalGroupTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = NormalGroupTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
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
        TextBefore(IterSerializer<'ser, Option<&'ser Text>, Text>),
        Group(<super::MixedGroupType as WithSerializer>::Serializer<'ser>),
        Baz(<Mixed<String> as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> MixedTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MixedTypeSerializerState::Init__ => {
                        *self.state = MixedTypeSerializerState::TextBefore(IterSerializer::new(
                            self.value.text_before.as_ref(),
                            Some(""),
                            false,
                        ));
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
                    MixedTypeSerializerState::TextBefore(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                MixedTypeSerializerState::Group(WithSerializer::serializer(
                                    &self.value.group,
                                    Some("Group"),
                                    false,
                                )?)
                        }
                    },
                    MixedTypeSerializerState::Group(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = MixedTypeSerializerState::Baz(WithSerializer::serializer(
                                &self.value.baz,
                                Some("tns:Baz"),
                                false,
                            )?)
                        }
                    },
                    MixedTypeSerializerState::Baz(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = MixedTypeSerializerState::End__,
                    },
                    MixedTypeSerializerState::End__ => {
                        *self.state = MixedTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    MixedTypeSerializerState::Done__ => return Ok(None),
                    MixedTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for MixedTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
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
    }
    #[derive(Debug)]
    pub(super) enum MixedGroupTypeSerializerState<'ser> {
        Init__,
        Fuu(<Mixed<i32> as WithSerializer>::Serializer<'ser>),
        Bar(<Mixed<String> as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> MixedGroupTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MixedGroupTypeSerializerState::Init__ => {
                        *self.state = MixedGroupTypeSerializerState::Fuu(
                            WithSerializer::serializer(&self.value.fuu, Some("tns:Fuu"), false)?,
                        );
                    }
                    MixedGroupTypeSerializerState::Fuu(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                MixedGroupTypeSerializerState::Bar(WithSerializer::serializer(
                                    &self.value.bar,
                                    Some("tns:Bar"),
                                    false,
                                )?)
                        }
                    },
                    MixedGroupTypeSerializerState::Bar(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = MixedGroupTypeSerializerState::Done__,
                    },
                    MixedGroupTypeSerializerState::Done__ => return Ok(None),
                    MixedGroupTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for MixedGroupTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = MixedGroupTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
