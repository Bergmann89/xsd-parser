use xsd_parser_types::misc::{Namespace, NamespacePrefix};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_EX: Namespace = Namespace::new_const(b"urn:example");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const PREFIX_EX: NamespacePrefix = NamespacePrefix::new_const(b"ex");
pub mod ex {
    use xsd_parser_types::quick_xml::{ElementNamespace, Error, WithDeserializer, WithSerializer};
    pub type Message = MessageType;
    #[derive(Debug)]
    pub struct MessageType {
        pub value: RenamedValue,
        pub sibling: SiblingType,
        pub local: LocalType,
        pub crate_value: CrateValueType,
    }
    impl WithSerializer for MessageType {
        type Serializer<'x> = quick_xml_serialize::MessageTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            WithSerializer::serializer_with_namespace(
                self,
                name,
                ElementNamespace::Unknown,
                is_root,
            )
        }
        fn serializer_with_namespace<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            namespace: ElementNamespace,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::MessageTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::MessageTypeSerializerState::Init__),
                name: name.unwrap_or("ex:Message"),
                namespace,
                is_root,
            })
        }
    }
    impl WithDeserializer for MessageType {
        type Deserializer = quick_xml_deserialize::MessageTypeDeserializer;
    }
    ///First paragraph.
    ///
    ///Second paragraph.
    pub type RenamedValue = ::std::string::String;
    pub type SiblingType = super::native::Text;
    pub type LocalType = super::native::Text;
    pub type CrateValueType = crate::feature::native_aliases::support::Text;
    pub mod quick_xml_deserialize {
        use core::mem::replace;
        use xsd_parser_types::quick_xml::{
            BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
            RawByteStr, WithDeserializer,
        };
        #[derive(Debug)]
        pub struct MessageTypeDeserializer {
            value: Option<super::RenamedValue>,
            sibling: Option<super::SiblingType>,
            local: Option<super::LocalType>,
            crate_value: Option<super::CrateValueType>,
            state__: Box<MessageTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum MessageTypeDeserializerState {
            Init__,
            Value(Option<<super::RenamedValue as WithDeserializer>::Deserializer>),
            Sibling(Option<<super::SiblingType as WithDeserializer>::Deserializer>),
            Local(Option<<super::LocalType as WithDeserializer>::Deserializer>),
            CrateValue(Option<<super::CrateValueType as WithDeserializer>::Deserializer>),
            Done__,
            Unknown__,
        }
        impl MessageTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
                Ok(Self {
                    value: None,
                    sibling: None,
                    local: None,
                    crate_value: None,
                    state__: Box::new(MessageTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: MessageTypeDeserializerState,
            ) -> Result<(), Error> {
                use MessageTypeDeserializerState as S;
                match state {
                    S::Value(Some(deserializer)) => {
                        self.store_value(deserializer.finish(helper)?)?
                    }
                    S::Sibling(Some(deserializer)) => {
                        self.store_sibling(deserializer.finish(helper)?)?
                    }
                    S::Local(Some(deserializer)) => {
                        self.store_local(deserializer.finish(helper)?)?
                    }
                    S::CrateValue(Some(deserializer)) => {
                        self.store_crate_value(deserializer.finish(helper)?)?
                    }
                    _ => (),
                }
                Ok(())
            }
            fn store_value(&mut self, value: super::RenamedValue) -> Result<(), Error> {
                if self.value.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"Value",
                    )))?;
                }
                self.value = Some(value);
                Ok(())
            }
            fn store_sibling(&mut self, value: super::SiblingType) -> Result<(), Error> {
                if self.sibling.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"Sibling",
                    )))?;
                }
                self.sibling = Some(value);
                Ok(())
            }
            fn store_local(&mut self, value: super::LocalType) -> Result<(), Error> {
                if self.local.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"Local",
                    )))?;
                }
                self.local = Some(value);
                Ok(())
            }
            fn store_crate_value(&mut self, value: super::CrateValueType) -> Result<(), Error> {
                if self.crate_value.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"CrateValue",
                    )))?;
                }
                self.crate_value = Some(value);
                Ok(())
            }
            fn handle_value<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, super::RenamedValue>,
                fallback: &mut Option<MessageTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use MessageTypeDeserializerState as S;
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
                        *self.state__ = S::Sibling(None);
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(S::Value(Some(deserializer)));
                        *self.state__ = S::Sibling(None);
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                }
            }
            fn handle_sibling<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, super::SiblingType>,
                fallback: &mut Option<MessageTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use MessageTypeDeserializerState as S;
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    fallback.get_or_insert(S::Sibling(None));
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
                        self.store_sibling(data)?;
                        *self.state__ = S::Local(None);
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(S::Sibling(Some(deserializer)));
                        *self.state__ = S::Local(None);
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                }
            }
            fn handle_local<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, super::LocalType>,
                fallback: &mut Option<MessageTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use MessageTypeDeserializerState as S;
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    fallback.get_or_insert(S::Local(None));
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
                        self.store_local(data)?;
                        *self.state__ = S::CrateValue(None);
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(S::Local(Some(deserializer)));
                        *self.state__ = S::CrateValue(None);
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                }
            }
            fn handle_crate_value<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, super::CrateValueType>,
                fallback: &mut Option<MessageTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use MessageTypeDeserializerState as S;
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    fallback.get_or_insert(S::CrateValue(None));
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
                        self.store_crate_value(data)?;
                        *self.state__ = S::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(S::CrateValue(Some(deserializer)));
                        *self.state__ = S::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                }
            }
        }
        impl<'de> Deserializer<'de, super::MessageType> for MessageTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::MessageType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::MessageType> {
                use MessageTypeDeserializerState as S;
                let mut event = event;
                let mut fallback = None;
                let mut allow_any_element = false;
                let (event, allow_any) = loop {
                    let state = replace(&mut *self.state__, S::Unknown__);
                    event = match (state, event) {
                        (S::Unknown__, _) => unreachable!(),
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
                        (S::Sibling(Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_sibling(helper, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        }
                        (S::Local(Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_local(helper, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        }
                        (S::CrateValue(Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_crate_value(helper, output, &mut fallback)? {
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
                            *self.state__ = S::Value(None);
                            event
                        }
                        (S::Value(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_EX),
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
                        (S::Sibling(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_EX),
                                b"Sibling",
                                false,
                            )?;
                            match self.handle_sibling(helper, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        }
                        (S::Local(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_EX),
                                b"Local",
                                false,
                            )?;
                            match self.handle_local(helper, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        }
                        (S::CrateValue(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_EX),
                                b"CrateValue",
                                false,
                            )?;
                            match self.handle_crate_value(helper, output, &mut fallback)? {
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
            fn finish(
                mut self,
                helper: &mut DeserializeHelper,
            ) -> Result<super::MessageType, Error> {
                let state = replace(&mut *self.state__, MessageTypeDeserializerState::Unknown__);
                self.finish_state(helper, state)?;
                Ok(super::MessageType {
                    value: helper.finish_element("Value", self.value)?,
                    sibling: helper.finish_element("Sibling", self.sibling)?,
                    local: helper.finish_element("Local", self.local)?,
                    crate_value: helper.finish_element("CrateValue", self.crate_value)?,
                })
            }
        }
    }
    pub mod quick_xml_serialize {
        use xsd_parser_types::quick_xml::{
            BytesEnd, BytesStart, ElementNamespace, Error, Event, SerializeHelper, Serializer,
            WithSerializer,
        };
        #[derive(Debug)]
        pub struct MessageTypeSerializer<'ser> {
            pub(super) value: &'ser super::MessageType,
            pub(super) state: Box<MessageTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) namespace: ElementNamespace,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum MessageTypeSerializerState<'ser> {
            Init__,
            Value(<super::RenamedValue as WithSerializer>::Serializer<'ser>),
            Sibling(<super::SiblingType as WithSerializer>::Serializer<'ser>),
            Local(<super::LocalType as WithSerializer>::Serializer<'ser>),
            CrateValue(<super::CrateValueType as WithSerializer>::Serializer<'ser>),
            End__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> MessageTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        MessageTypeSerializerState::Init__ => {
                            *self.state = MessageTypeSerializerState::Value(
                                WithSerializer::serializer_with_namespace(
                                    &self.value.value,
                                    Some("Value"),
                                    ElementNamespace::None,
                                    false,
                                )?,
                            );
                            let namespace = if self.is_root {
                                ElementNamespace::Namespace(super::super::NS_EX)
                            } else {
                                self.namespace.clone()
                            };
                            self.name = helper.element_name(self.name, &namespace);
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            helper.write_xmlns_for_element(&mut bytes, self.name, &namespace);
                            if self.is_root {
                                helper.write_default_namespace(&mut bytes, self.name, &namespace);
                            }
                            if self.is_root {
                                helper.write_xmlns_unless_default(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_EX),
                                    &super::super::NS_EX,
                                );
                            }
                            return Ok(Some(Event::Start(bytes)));
                        }
                        MessageTypeSerializerState::Value(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = MessageTypeSerializerState::Sibling(
                                    WithSerializer::serializer_with_namespace(
                                        &self.value.sibling,
                                        Some("Sibling"),
                                        ElementNamespace::None,
                                        false,
                                    )?,
                                )
                            }
                        },
                        MessageTypeSerializerState::Sibling(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => {
                                    *self.state = MessageTypeSerializerState::Local(
                                        WithSerializer::serializer_with_namespace(
                                            &self.value.local,
                                            Some("Local"),
                                            ElementNamespace::None,
                                            false,
                                        )?,
                                    )
                                }
                            }
                        }
                        MessageTypeSerializerState::Local(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = MessageTypeSerializerState::CrateValue(
                                    WithSerializer::serializer_with_namespace(
                                        &self.value.crate_value,
                                        Some("CrateValue"),
                                        ElementNamespace::None,
                                        false,
                                    )?,
                                )
                            }
                        },
                        MessageTypeSerializerState::CrateValue(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = MessageTypeSerializerState::End__,
                            }
                        }
                        MessageTypeSerializerState::End__ => {
                            *self.state = MessageTypeSerializerState::Done__;
                            helper.end_ns_scope();
                            return Ok(Some(Event::End(BytesEnd::new(self.name))));
                        }
                        MessageTypeSerializerState::Done__ => return Ok(None),
                        MessageTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for MessageTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = MessageTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
    }
}
