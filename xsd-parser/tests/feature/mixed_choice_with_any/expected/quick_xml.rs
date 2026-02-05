use xsd_parser_types::misc::{Namespace, NamespacePrefix};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub mod tns {
    use xsd_parser_types::{
        quick_xml::{Error, WithDeserializer, WithSerializer},
        xml::Text,
    };
    pub type Sdl = RootType;
    #[derive(Debug)]
    pub struct RootType {
        pub container: ContainerType,
    }
    impl WithSerializer for RootType {
        type Serializer<'x> = quick_xml_serialize::RootTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::RootTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::RootTypeSerializerState::Init__),
                name: name.unwrap_or("tns:RootType"),
                is_root,
            })
        }
    }
    impl WithDeserializer for RootType {
        type Deserializer = quick_xml_deserialize::RootTypeDeserializer;
    }
    #[derive(Debug)]
    pub struct ContainerType {
        pub content: Vec<ContainerTypeContent>,
    }
    #[derive(Debug)]
    pub enum ContainerTypeContent {
        Known(KnownType),
        Text(Text),
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
                name: name.unwrap_or("tns:ContainerType"),
                is_root,
            })
        }
    }
    impl WithSerializer for ContainerTypeContent {
        type Serializer<'x> = quick_xml_serialize::ContainerTypeContentSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            let _name = name;
            let _is_root = is_root;
            Ok(quick_xml_serialize::ContainerTypeContentSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::ContainerTypeContentSerializerState::Init__),
            })
        }
    }
    impl WithDeserializer for ContainerType {
        type Deserializer = quick_xml_deserialize::ContainerTypeDeserializer;
    }
    impl WithDeserializer for ContainerTypeContent {
        type Deserializer = quick_xml_deserialize::ContainerTypeContentDeserializer;
    }
    #[derive(Debug)]
    pub struct KnownType {
        pub name: Option<String>,
    }
    impl WithSerializer for KnownType {
        type Serializer<'x> = quick_xml_serialize::KnownTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::KnownTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::KnownTypeSerializerState::Init__),
                name: name.unwrap_or("tns:KnownType"),
                is_root,
            })
        }
    }
    impl WithDeserializer for KnownType {
        type Deserializer = quick_xml_deserialize::KnownTypeDeserializer;
    }
    pub mod quick_xml_deserialize {
        use core::mem::replace;
        use xsd_parser_types::{
            quick_xml::{
                BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact,
                DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput,
                Error, ErrorKind, Event, RawByteStr, WithDeserializer,
            },
            xml::Text,
        };
        #[derive(Debug)]
        pub struct RootTypeDeserializer {
            container: Option<super::ContainerType>,
            state__: Box<RootTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum RootTypeDeserializerState {
            Init__,
            Container(Option<<super::ContainerType as WithDeserializer>::Deserializer>),
            Done__,
            Unknown__,
        }
        impl RootTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
                Ok(Self {
                    container: None,
                    state__: Box::new(RootTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: RootTypeDeserializerState,
            ) -> Result<(), Error> {
                use RootTypeDeserializerState as S;
                match state {
                    S::Container(Some(deserializer)) => {
                        self.store_container(deserializer.finish(helper)?)?
                    }
                    _ => (),
                }
                Ok(())
            }
            fn store_container(&mut self, value: super::ContainerType) -> Result<(), Error> {
                if self.container.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"Container",
                    )))?;
                }
                self.container = Some(value);
                Ok(())
            }
            fn handle_container<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, super::ContainerType>,
                fallback: &mut Option<RootTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use RootTypeDeserializerState as S;
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    fallback.get_or_insert(S::Container(None));
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
                        self.store_container(data)?;
                        *self.state__ = S::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(S::Container(Some(deserializer)));
                        *self.state__ = S::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                }
            }
        }
        impl<'de> Deserializer<'de, super::RootType> for RootTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::RootType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::RootType> {
                use RootTypeDeserializerState as S;
                let mut event = event;
                let mut fallback = None;
                let mut allow_any_element = false;
                let (event, allow_any) = loop {
                    let state = replace(&mut *self.state__, S::Unknown__);
                    event = match (state, event) {
                        (S::Unknown__, _) => unreachable!(),
                        (S::Container(Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_container(helper, output, &mut fallback)? {
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
                            *self.state__ = S::Container(None);
                            event
                        }
                        (S::Container(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_TNS),
                                b"Container",
                                true,
                            )?;
                            match self.handle_container(helper, output, &mut fallback)? {
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
            fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::RootType, Error> {
                let state = replace(&mut *self.state__, RootTypeDeserializerState::Unknown__);
                self.finish_state(helper, state)?;
                Ok(super::RootType {
                    container: helper.finish_element("Container", self.container)?,
                })
            }
        }
        #[derive(Debug)]
        pub struct ContainerTypeDeserializer {
            content: Vec<super::ContainerTypeContent>,
            state__: Box<ContainerTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum ContainerTypeDeserializerState {
            Init__,
            Next__,
            Content__(<super::ContainerTypeContent as WithDeserializer>::Deserializer),
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
                    content: Vec::new(),
                    state__: Box::new(ContainerTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: ContainerTypeDeserializerState,
            ) -> Result<(), Error> {
                if let ContainerTypeDeserializerState::Content__(deserializer) = state {
                    self.store_content(deserializer.finish(helper)?)?;
                }
                Ok(())
            }
            fn store_content(&mut self, value: super::ContainerTypeContent) -> Result<(), Error> {
                self.content.push(value);
                Ok(())
            }
            fn handle_content<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, super::ContainerTypeContent>,
                fallback: &mut Option<ContainerTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use ContainerTypeDeserializerState as S;
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    *self.state__ = fallback.take().unwrap_or(S::Next__);
                    return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
                }
                if let Some(fallback) = fallback.take() {
                    self.finish_state(helper, fallback)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        self.store_content(data)?;
                        *self.state__ = S::Next__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *fallback = Some(S::Content__(deserializer));
                        *self.state__ = S::Next__;
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
                let (event, allow_any) = loop {
                    let state = replace(&mut *self.state__, S::Unknown__);
                    event = match (state, event) {
                        (S::Unknown__, _) => unreachable!(),
                        (S::Content__(deserializer), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_content(helper, output, &mut fallback)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (_, Event::End(_)) => {
                            return Ok(DeserializerOutput {
                                artifact: DeserializerArtifact::Data(self.finish(helper)?),
                                event: DeserializerEvent::None,
                                allow_any: false,
                            });
                        }
                        (state @ (S::Init__ | S::Next__), event) => {
                            fallback.get_or_insert(state);
                            let output = <super::ContainerTypeContent as WithDeserializer>::init(
                                helper, event,
                            )?;
                            match self.handle_content(helper, output, &mut fallback)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                    }
                };
                if let Some(fallback) = fallback {
                    *self.state__ = fallback;
                }
                let artifact = DeserializerArtifact::Deserializer(self);
                Ok(DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                })
            }
            fn finish(
                mut self,
                helper: &mut DeserializeHelper,
            ) -> Result<super::ContainerType, Error> {
                let state = replace(
                    &mut *self.state__,
                    ContainerTypeDeserializerState::Unknown__,
                );
                self.finish_state(helper, state)?;
                Ok(super::ContainerType {
                    content: helper.finish_vec(0usize, None, self.content)?,
                })
            }
        }
        #[derive(Debug)]
        pub struct ContainerTypeContentDeserializer {
            state__: Box<ContainerTypeContentDeserializerState>,
        }
        #[derive(Debug)]
        pub enum ContainerTypeContentDeserializerState {
            Init__,
            Known(
                Option<super::KnownType>,
                Option<<super::KnownType as WithDeserializer>::Deserializer>,
                Option<<super::KnownType as WithDeserializer>::Deserializer>,
            ),
            Text(
                Option<Text>,
                Option<<Text as WithDeserializer>::Deserializer>,
                Option<<Text as WithDeserializer>::Deserializer>,
            ),
            Done__(super::ContainerTypeContent),
            Unknown__,
        }
        impl ContainerTypeContentDeserializer {
            fn find_suitable<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                let mut event = event;
                if let Event::Start(x) | Event::Empty(x) = &event {
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_TNS),
                        Some(b"Known")
                    ) {
                        let output = <super::KnownType as WithDeserializer>::init(helper, event)?;
                        return self.handle_known(helper, Default::default(), None, output);
                    }
                }
                event = {
                    let output = <Text as WithDeserializer>::init(helper, event)?;
                    match self.handle_text(helper, Default::default(), None, output)? {
                        ElementHandlerOutput::Continue { event, .. } => event,
                        output => {
                            return Ok(output);
                        }
                    }
                };
                *self.state__ = ContainerTypeContentDeserializerState::Init__;
                Ok(ElementHandlerOutput::return_to_parent(event, true))
            }
            fn finish_state(
                helper: &mut DeserializeHelper,
                state: ContainerTypeContentDeserializerState,
            ) -> Result<super::ContainerTypeContent, Error> {
                use ContainerTypeContentDeserializerState as S;
                match state {
                    S::Init__ => Err(ErrorKind::MissingContent.into()),
                    S::Known(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_known(&mut values, value)?;
                        }
                        Ok(super::ContainerTypeContent::Known(
                            helper.finish_element("Known", values)?,
                        ))
                    }
                    S::Text(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_text(&mut values, value)?;
                        }
                        Ok(super::ContainerTypeContent::Text(
                            helper.finish_element("text", values)?,
                        ))
                    }
                    S::Done__(data) => Ok(data),
                    _ => unreachable!(),
                }
            }
            fn store_known(
                values: &mut Option<super::KnownType>,
                value: super::KnownType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"Known",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_text(values: &mut Option<Text>, value: Text) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"text")))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn handle_known<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::KnownType>,
                fallback: Option<<super::KnownType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::KnownType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use ContainerTypeContentDeserializerState as S;
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
                    Self::store_known(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_known(&mut values, data)?;
                        let data = Self::finish_state(helper, S::Known(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::Known(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_text<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<Text>,
                fallback: Option<<Text as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, Text>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use ContainerTypeContentDeserializerState as S;
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
                    Self::store_text(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_text(&mut values, data)?;
                        let data = Self::finish_state(helper, S::Text(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::Text(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
        }
        impl<'de> Deserializer<'de, super::ContainerTypeContent> for ContainerTypeContentDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::ContainerTypeContent> {
                let deserializer = Self {
                    state__: Box::new(ContainerTypeContentDeserializerState::Init__),
                };
                let mut output = deserializer.next(helper, event)?;
                output.artifact = match output.artifact {
                    DeserializerArtifact::Deserializer(x)
                        if matches!(&*x.state__, ContainerTypeContentDeserializerState::Init__) =>
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
            ) -> DeserializerResult<'de, super::ContainerTypeContent> {
                use ContainerTypeContentDeserializerState as S;
                let mut event = event;
                let (event, allow_any) = loop {
                    let state = replace(&mut *self.state__, S::Unknown__);
                    event = match (state, event) {
                        (S::Unknown__, _) => unreachable!(),
                        (S::Known(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_known(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::Text(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_text(helper, values, fallback, output)? {
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
                            S::Known(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_TNS),
                                b"Known",
                                false,
                            )?;
                            match self.handle_known(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::Text(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = <Text as WithDeserializer>::init(helper, event)?;
                            match self.handle_text(helper, values, fallback, output)? {
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
            ) -> Result<super::ContainerTypeContent, Error> {
                Self::finish_state(helper, *self.state__)
            }
        }
        #[derive(Debug)]
        pub struct KnownTypeDeserializer {
            name: Option<String>,
            state__: Box<KnownTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum KnownTypeDeserializerState {
            Init__,
            Unknown__,
        }
        impl KnownTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                let mut name: Option<String> = None;
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_TNS),
                        Some(b"name")
                    ) {
                        helper.read_attrib(&mut name, b"name", &attrib.value)?;
                    } else {
                        helper.raise_unexpected_attrib_checked(&attrib)?;
                    }
                }
                Ok(Self {
                    name: name,
                    state__: Box::new(KnownTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: KnownTypeDeserializerState,
            ) -> Result<(), Error> {
                Ok(())
            }
        }
        impl<'de> Deserializer<'de, super::KnownType> for KnownTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::KnownType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::KnownType> {
                if let Event::End(_) = &event {
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(helper)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    })
                } else {
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event: DeserializerEvent::Break(event),
                        allow_any: false,
                    })
                }
            }
            fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::KnownType, Error> {
                let state = replace(&mut *self.state__, KnownTypeDeserializerState::Unknown__);
                self.finish_state(helper, state)?;
                Ok(super::KnownType { name: self.name })
            }
        }
    }
    pub mod quick_xml_serialize {
        use xsd_parser_types::{
            quick_xml::{
                BytesEnd, BytesStart, Error, Event, IterSerializer, SerializeHelper, Serializer,
                WithSerializer,
            },
            xml::Text,
        };
        #[derive(Debug)]
        pub struct RootTypeSerializer<'ser> {
            pub(super) value: &'ser super::RootType,
            pub(super) state: Box<RootTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum RootTypeSerializerState<'ser> {
            Init__,
            Container(<super::ContainerType as WithSerializer>::Serializer<'ser>),
            End__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> RootTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        RootTypeSerializerState::Init__ => {
                            *self.state =
                                RootTypeSerializerState::Container(WithSerializer::serializer(
                                    &self.value.container,
                                    Some("tns:Container"),
                                    false,
                                )?);
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            if self.is_root {
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_XSI),
                                    &super::super::NS_XSI,
                                );
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_TNS),
                                    &super::super::NS_TNS,
                                );
                            }
                            return Ok(Some(Event::Start(bytes)));
                        }
                        RootTypeSerializerState::Container(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = RootTypeSerializerState::End__,
                            }
                        }
                        RootTypeSerializerState::End__ => {
                            *self.state = RootTypeSerializerState::Done__;
                            helper.end_ns_scope();
                            return Ok(Some(Event::End(BytesEnd::new(self.name))));
                        }
                        RootTypeSerializerState::Done__ => return Ok(None),
                        RootTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for RootTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = RootTypeSerializerState::Done__;
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
            Content__(
                IterSerializer<
                    'ser,
                    &'ser [super::ContainerTypeContent],
                    super::ContainerTypeContent,
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
                            *self.state = ContainerTypeSerializerState::Content__(
                                IterSerializer::new(&self.value.content[..], None, false),
                            );
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            if self.is_root {
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_XSI),
                                    &super::super::NS_XSI,
                                );
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_TNS),
                                    &super::super::NS_TNS,
                                );
                            }
                            return Ok(Some(Event::Start(bytes)));
                        }
                        ContainerTypeSerializerState::Content__(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = ContainerTypeSerializerState::End__,
                            }
                        }
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
        pub struct ContainerTypeContentSerializer<'ser> {
            pub(super) value: &'ser super::ContainerTypeContent,
            pub(super) state: Box<ContainerTypeContentSerializerState<'ser>>,
        }
        #[derive(Debug)]
        pub(super) enum ContainerTypeContentSerializerState<'ser> {
            Init__,
            Known(<super::KnownType as WithSerializer>::Serializer<'ser>),
            Text(<Text as WithSerializer>::Serializer<'ser>),
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> ContainerTypeContentSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        ContainerTypeContentSerializerState::Init__ => match self.value {
                            super::ContainerTypeContent::Known(x) => {
                                *self.state = ContainerTypeContentSerializerState::Known(
                                    WithSerializer::serializer(x, Some("tns:Known"), false)?,
                                )
                            }
                            super::ContainerTypeContent::Text(x) => {
                                *self.state = ContainerTypeContentSerializerState::Text(
                                    WithSerializer::serializer(x, Some("text"), false)?,
                                )
                            }
                        },
                        ContainerTypeContentSerializerState::Known(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = ContainerTypeContentSerializerState::Done__,
                            }
                        }
                        ContainerTypeContentSerializerState::Text(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = ContainerTypeContentSerializerState::Done__,
                            }
                        }
                        ContainerTypeContentSerializerState::Done__ => return Ok(None),
                        ContainerTypeContentSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for ContainerTypeContentSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = ContainerTypeContentSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct KnownTypeSerializer<'ser> {
            pub(super) value: &'ser super::KnownType,
            pub(super) state: Box<KnownTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum KnownTypeSerializerState<'ser> {
            Init__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> KnownTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        KnownTypeSerializerState::Init__ => {
                            *self.state = KnownTypeSerializerState::Done__;
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            if self.is_root {
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_XSI),
                                    &super::super::NS_XSI,
                                );
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_TNS),
                                    &super::super::NS_TNS,
                                );
                            }
                            helper.write_attrib_opt(&mut bytes, "name", &self.value.name)?;
                            helper.end_ns_scope();
                            return Ok(Some(Event::Empty(bytes)));
                        }
                        KnownTypeSerializerState::Done__ => return Ok(None),
                        KnownTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for KnownTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = KnownTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
    }
}
