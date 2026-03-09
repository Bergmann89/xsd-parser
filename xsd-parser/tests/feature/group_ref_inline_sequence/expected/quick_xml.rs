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
pub type Widget = WidgetType;
#[derive(Debug)]
pub struct WidgetType {
    pub identity: WidgetIdentityType,
    pub name: String,
    pub description: Option<String>,
}
impl WithSerializer for WidgetType {
    type Serializer<'x> = quick_xml_serialize::WidgetTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::WidgetTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::WidgetTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Widget"),
            is_root,
        })
    }
}
impl WithDeserializer for WidgetType {
    type Deserializer = quick_xml_deserialize::WidgetTypeDeserializer;
}
#[derive(Debug)]
pub struct WidgetIdentityType {
    pub identifier: String,
    pub tag: Vec<String>,
}
impl WithSerializer for WidgetIdentityType {
    type Serializer<'x> = quick_xml_serialize::WidgetIdentityTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::WidgetIdentityTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::WidgetIdentityTypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for WidgetIdentityType {
    type Deserializer = quick_xml_deserialize::WidgetIdentityTypeDeserializer;
}
pub type Container = ContainerType;
#[derive(Debug)]
pub struct ContainerType {
    pub identity: Vec<ContainerIdentityType>,
    pub name: String,
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
            name: name.unwrap_or("tns:Container"),
            is_root,
        })
    }
}
impl WithDeserializer for ContainerType {
    type Deserializer = quick_xml_deserialize::ContainerTypeDeserializer;
}
#[derive(Debug)]
pub struct ContainerIdentityType {
    pub identifier: String,
    pub tag: Vec<String>,
}
impl WithSerializer for ContainerIdentityType {
    type Serializer<'x> = quick_xml_serialize::ContainerIdentityTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::ContainerIdentityTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ContainerIdentityTypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for ContainerIdentityType {
    type Deserializer = quick_xml_deserialize::ContainerIdentityTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct WidgetTypeDeserializer {
        identity: Option<super::WidgetIdentityType>,
        name: Option<String>,
        description: Option<String>,
        state__: Box<WidgetTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum WidgetTypeDeserializerState {
        Init__,
        Identity(Option<<super::WidgetIdentityType as WithDeserializer>::Deserializer>),
        Name(Option<<String as WithDeserializer>::Deserializer>),
        Description(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl WidgetTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                identity: None,
                name: None,
                description: None,
                state__: Box::new(WidgetTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: WidgetTypeDeserializerState,
        ) -> Result<(), Error> {
            use WidgetTypeDeserializerState as S;
            match state {
                S::Identity(Some(deserializer)) => {
                    self.store_identity(deserializer.finish(helper)?)?
                }
                S::Name(Some(deserializer)) => self.store_name(deserializer.finish(helper)?)?,
                S::Description(Some(deserializer)) => {
                    self.store_description(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_identity(&mut self, value: super::WidgetIdentityType) -> Result<(), Error> {
            if self.identity.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Identity",
                )))?;
            }
            self.identity = Some(value);
            Ok(())
        }
        fn store_name(&mut self, value: String) -> Result<(), Error> {
            if self.name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Name")))?;
            }
            self.name = Some(value);
            Ok(())
        }
        fn store_description(&mut self, value: String) -> Result<(), Error> {
            if self.description.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Description",
                )))?;
            }
            self.description = Some(value);
            Ok(())
        }
        fn handle_identity<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::WidgetIdentityType>,
            fallback: &mut Option<WidgetTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use WidgetTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Identity(None));
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
                    self.store_identity(data)?;
                    *self.state__ = S::Name(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Identity(Some(deserializer)));
                    *self.state__ = S::Name(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_name<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<WidgetTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use WidgetTypeDeserializerState as S;
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
                    *self.state__ = S::Description(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Name(Some(deserializer)));
                    *self.state__ = S::Description(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_description<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<WidgetTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use WidgetTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_description(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Description(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::WidgetType> for WidgetTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WidgetType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WidgetType> {
            use WidgetTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Identity(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_identity(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
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
                    (S::Description(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_description(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Identity(None);
                        event
                    }
                    (S::Identity(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::WidgetIdentityType as WithDeserializer>::init(helper, event)?;
                        match self.handle_identity(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
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
                    (S::Description(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Description",
                            false,
                        )?;
                        match self.handle_description(helper, output, &mut fallback)? {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::WidgetType, Error> {
            let state = replace(&mut *self.state__, WidgetTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::WidgetType {
                identity: helper.finish_element("Identity", self.identity)?,
                name: helper.finish_element("Name", self.name)?,
                description: self.description,
            })
        }
    }
    #[derive(Debug)]
    pub struct WidgetIdentityTypeDeserializer {
        identifier: Option<String>,
        tag: Vec<String>,
        state__: Box<WidgetIdentityTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum WidgetIdentityTypeDeserializerState {
        Init__,
        Identifier(Option<<String as WithDeserializer>::Deserializer>),
        Tag(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl WidgetIdentityTypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: WidgetIdentityTypeDeserializerState,
        ) -> Result<(), Error> {
            use WidgetIdentityTypeDeserializerState as S;
            match state {
                S::Identifier(Some(deserializer)) => {
                    self.store_identifier(deserializer.finish(helper)?)?
                }
                S::Tag(Some(deserializer)) => self.store_tag(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_identifier(&mut self, value: String) -> Result<(), Error> {
            if self.identifier.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Identifier",
                )))?;
            }
            self.identifier = Some(value);
            Ok(())
        }
        fn store_tag(&mut self, value: String) -> Result<(), Error> {
            self.tag.push(value);
            Ok(())
        }
        fn handle_identifier<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<WidgetIdentityTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use WidgetIdentityTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Identifier(None));
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
                    self.store_identifier(data)?;
                    *self.state__ = S::Tag(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Identifier(Some(deserializer)));
                    *self.state__ = S::Tag(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_tag<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<WidgetIdentityTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use WidgetIdentityTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_tag(data)?;
                    *self.state__ = S::Tag(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Tag(Some(deserializer)));
                    *self.state__ = S::Tag(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::WidgetIdentityType> for WidgetIdentityTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WidgetIdentityType> {
            let deserializer = Self {
                identifier: None,
                tag: Vec::new(),
                state__: Box::new(WidgetIdentityTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, WidgetIdentityTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::WidgetIdentityType> {
            use WidgetIdentityTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Identifier(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_identifier(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Tag(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_tag(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Identifier(None);
                        event
                    }
                    (S::Identifier(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Identifier",
                            false,
                        )?;
                        match self.handle_identifier(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Tag(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Tag",
                            false,
                        )?;
                        match self.handle_tag(helper, output, &mut fallback)? {
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
        ) -> Result<super::WidgetIdentityType, Error> {
            let state = replace(
                &mut *self.state__,
                WidgetIdentityTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::WidgetIdentityType {
                identifier: helper.finish_element("Identifier", self.identifier)?,
                tag: self.tag,
            })
        }
    }
    #[derive(Debug)]
    pub struct ContainerTypeDeserializer {
        identity: Vec<super::ContainerIdentityType>,
        name: Option<String>,
        state__: Box<ContainerTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ContainerTypeDeserializerState {
        Init__,
        Identity(Option<<super::ContainerIdentityType as WithDeserializer>::Deserializer>),
        Name(Option<<String as WithDeserializer>::Deserializer>),
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
                identity: Vec::new(),
                name: None,
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
                S::Identity(Some(deserializer)) => {
                    self.store_identity(deserializer.finish(helper)?)?
                }
                S::Name(Some(deserializer)) => self.store_name(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_identity(&mut self, value: super::ContainerIdentityType) -> Result<(), Error> {
            self.identity.push(value);
            Ok(())
        }
        fn store_name(&mut self, value: String) -> Result<(), Error> {
            if self.name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Name")))?;
            }
            self.name = Some(value);
            Ok(())
        }
        fn handle_identity<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ContainerIdentityType>,
            fallback: &mut Option<ContainerTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ContainerTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
                if let Some(fallback) = fallback.take() {
                    self.finish_state(helper, fallback)?;
                }
                if self.identity.len() < 1usize {
                    fallback.get_or_insert(S::Identity(None));
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else {
                    *self.state__ = S::Name(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_identity(data)?;
                    *self.state__ = S::Identity(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Identity(Some(deserializer)));
                    *self.state__ = S::Identity(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_name<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ContainerTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ContainerTypeDeserializerState as S;
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
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Name(Some(deserializer)));
                    *self.state__ = S::Done__;
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
                    (S::Identity(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_identity(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
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
                        *self.state__ = S::Identity(None);
                        event
                    }
                    (S::Identity(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = <super::ContainerIdentityType as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_identity(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
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
                identity: helper.finish_vec(1usize, None, self.identity)?,
                name: helper.finish_element("Name", self.name)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ContainerIdentityTypeDeserializer {
        identifier: Option<String>,
        tag: Vec<String>,
        state__: Box<ContainerIdentityTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ContainerIdentityTypeDeserializerState {
        Init__,
        Identifier(Option<<String as WithDeserializer>::Deserializer>),
        Tag(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ContainerIdentityTypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ContainerIdentityTypeDeserializerState,
        ) -> Result<(), Error> {
            use ContainerIdentityTypeDeserializerState as S;
            match state {
                S::Identifier(Some(deserializer)) => {
                    self.store_identifier(deserializer.finish(helper)?)?
                }
                S::Tag(Some(deserializer)) => self.store_tag(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_identifier(&mut self, value: String) -> Result<(), Error> {
            if self.identifier.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Identifier",
                )))?;
            }
            self.identifier = Some(value);
            Ok(())
        }
        fn store_tag(&mut self, value: String) -> Result<(), Error> {
            self.tag.push(value);
            Ok(())
        }
        fn handle_identifier<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ContainerIdentityTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ContainerIdentityTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Identifier(None));
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
                    self.store_identifier(data)?;
                    *self.state__ = S::Tag(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Identifier(Some(deserializer)));
                    *self.state__ = S::Tag(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_tag<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ContainerIdentityTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ContainerIdentityTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_tag(data)?;
                    *self.state__ = S::Tag(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Tag(Some(deserializer)));
                    *self.state__ = S::Tag(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ContainerIdentityType> for ContainerIdentityTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ContainerIdentityType> {
            let deserializer = Self {
                identifier: None,
                tag: Vec::new(),
                state__: Box::new(ContainerIdentityTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, ContainerIdentityTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::ContainerIdentityType> {
            use ContainerIdentityTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Identifier(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_identifier(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Tag(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_tag(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Identifier(None);
                        event
                    }
                    (S::Identifier(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Identifier",
                            false,
                        )?;
                        match self.handle_identifier(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Tag(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Tag",
                            false,
                        )?;
                        match self.handle_tag(helper, output, &mut fallback)? {
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
        ) -> Result<super::ContainerIdentityType, Error> {
            let state = replace(
                &mut *self.state__,
                ContainerIdentityTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ContainerIdentityType {
                identifier: helper.finish_element("Identifier", self.identifier)?,
                tag: self.tag,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, Error, Event, IterSerializer, SerializeHelper, Serializer,
        WithSerializer,
    };
    #[derive(Debug)]
    pub struct WidgetTypeSerializer<'ser> {
        pub(super) value: &'ser super::WidgetType,
        pub(super) state: Box<WidgetTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum WidgetTypeSerializerState<'ser> {
        Init__,
        Identity(<super::WidgetIdentityType as WithSerializer>::Serializer<'ser>),
        Name(<String as WithSerializer>::Serializer<'ser>),
        Description(IterSerializer<'ser, Option<&'ser String>, String>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> WidgetTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    WidgetTypeSerializerState::Init__ => {
                        *self.state =
                            WidgetTypeSerializerState::Identity(WithSerializer::serializer(
                                &self.value.identity,
                                Some("Identity"),
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
                    WidgetTypeSerializerState::Identity(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                WidgetTypeSerializerState::Name(WithSerializer::serializer(
                                    &self.value.name,
                                    Some("tns:Name"),
                                    false,
                                )?)
                        }
                    },
                    WidgetTypeSerializerState::Name(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                WidgetTypeSerializerState::Description(IterSerializer::new(
                                    self.value.description.as_ref(),
                                    Some("tns:Description"),
                                    false,
                                ))
                        }
                    },
                    WidgetTypeSerializerState::Description(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = WidgetTypeSerializerState::End__,
                        }
                    }
                    WidgetTypeSerializerState::End__ => {
                        *self.state = WidgetTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    WidgetTypeSerializerState::Done__ => return Ok(None),
                    WidgetTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for WidgetTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = WidgetTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct WidgetIdentityTypeSerializer<'ser> {
        pub(super) value: &'ser super::WidgetIdentityType,
        pub(super) state: Box<WidgetIdentityTypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum WidgetIdentityTypeSerializerState<'ser> {
        Init__,
        Identifier(<String as WithSerializer>::Serializer<'ser>),
        Tag(IterSerializer<'ser, &'ser [String], String>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> WidgetIdentityTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    WidgetIdentityTypeSerializerState::Init__ => {
                        *self.state = WidgetIdentityTypeSerializerState::Identifier(
                            WithSerializer::serializer(
                                &self.value.identifier,
                                Some("tns:Identifier"),
                                false,
                            )?,
                        );
                    }
                    WidgetIdentityTypeSerializerState::Identifier(x) => match x
                        .next(helper)
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = WidgetIdentityTypeSerializerState::Tag(
                                IterSerializer::new(&self.value.tag[..], Some("tns:Tag"), false),
                            )
                        }
                    },
                    WidgetIdentityTypeSerializerState::Tag(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = WidgetIdentityTypeSerializerState::Done__,
                        }
                    }
                    WidgetIdentityTypeSerializerState::Done__ => return Ok(None),
                    WidgetIdentityTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for WidgetIdentityTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = WidgetIdentityTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
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
        Identity(
            IterSerializer<
                'ser,
                &'ser [super::ContainerIdentityType],
                super::ContainerIdentityType,
            >,
        ),
        Name(<String as WithSerializer>::Serializer<'ser>),
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
                        *self.state = ContainerTypeSerializerState::Identity(IterSerializer::new(
                            &self.value.identity[..],
                            Some("Identity"),
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
                    ContainerTypeSerializerState::Identity(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    ContainerTypeSerializerState::Name(WithSerializer::serializer(
                                        &self.value.name,
                                        Some("tns:Name"),
                                        false,
                                    )?)
                            }
                        }
                    }
                    ContainerTypeSerializerState::Name(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ContainerTypeSerializerState::End__,
                    },
                    ContainerTypeSerializerState::End__ => {
                        *self.state = ContainerTypeSerializerState::Done__;
                        helper.end_ns_scope();
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
    pub struct ContainerIdentityTypeSerializer<'ser> {
        pub(super) value: &'ser super::ContainerIdentityType,
        pub(super) state: Box<ContainerIdentityTypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum ContainerIdentityTypeSerializerState<'ser> {
        Init__,
        Identifier(<String as WithSerializer>::Serializer<'ser>),
        Tag(IterSerializer<'ser, &'ser [String], String>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ContainerIdentityTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ContainerIdentityTypeSerializerState::Init__ => {
                        *self.state = ContainerIdentityTypeSerializerState::Identifier(
                            WithSerializer::serializer(
                                &self.value.identifier,
                                Some("tns:Identifier"),
                                false,
                            )?,
                        );
                    }
                    ContainerIdentityTypeSerializerState::Identifier(x) => match x
                        .next(helper)
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = ContainerIdentityTypeSerializerState::Tag(
                                IterSerializer::new(&self.value.tag[..], Some("tns:Tag"), false),
                            )
                        }
                    },
                    ContainerIdentityTypeSerializerState::Tag(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ContainerIdentityTypeSerializerState::Done__,
                        }
                    }
                    ContainerIdentityTypeSerializerState::Done__ => return Ok(None),
                    ContainerIdentityTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ContainerIdentityTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ContainerIdentityTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
