use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{ElementNamespace, Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub type Option = OptionType;
#[derive(Debug)]
pub struct OptionType {
    pub value: ::core::option::Option<::std::string::String>,
}
impl WithSerializer for OptionType {
    type Serializer<'x> = quick_xml_serialize::OptionTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, Error> {
        WithSerializer::serializer_with_namespace(self, name, ElementNamespace::Unknown, is_root)
    }
    fn serializer_with_namespace<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        namespace: ElementNamespace,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::OptionTypeSerializer {
            value: self,
            state: ::std::boxed::Box::new(quick_xml_serialize::OptionTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Option"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for OptionType {
    type Deserializer = quick_xml_deserialize::OptionTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct OptionTypeDeserializer {
        value: ::core::option::Option<::std::string::String>,
        state__: ::std::boxed::Box<OptionTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum OptionTypeDeserializerState {
        Init__,
        Value(::core::option::Option<<::std::string::String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl OptionTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> ::core::result::Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                value: None,
                state__: ::std::boxed::Box::new(OptionTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: OptionTypeDeserializerState,
        ) -> ::core::result::Result<(), Error> {
            use OptionTypeDeserializerState as S;
            match state {
                S::Value(Some(deserializer)) => self.store_value(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_value(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), Error> {
            if self.value.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Value",
                )))?;
            }
            self.value = Some(value);
            Ok(())
        }
        fn handle_value<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut ::core::option::Option<OptionTypeDeserializerState>,
        ) -> ::core::result::Result<ElementHandlerOutput<'de>, Error> {
            use OptionTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::OptionType> for OptionTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OptionType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OptionType> {
            use OptionTypeDeserializerState as S;
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
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> ::core::result::Result<super::OptionType, Error> {
            let state = replace(&mut *self.state__, OptionTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::OptionType { value: self.value })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, ElementNamespace, Error, Event, IterSerializer, SerializeHelper,
        Serializer,
    };
    #[derive(Debug)]
    pub struct OptionTypeSerializer<'ser> {
        pub(super) value: &'ser super::OptionType,
        pub(super) state: ::std::boxed::Box<OptionTypeSerializerState<'ser>>,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum OptionTypeSerializerState<'ser> {
        Init__,
        Value(
            IterSerializer<
                'ser,
                ::core::option::Option<&'ser ::std::string::String>,
                ::std::string::String,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> OptionTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> ::core::result::Result<::core::option::Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    OptionTypeSerializerState::Init__ => {
                        *self.state = OptionTypeSerializerState::Value(
                            IterSerializer::new(
                                self.value.value.as_ref(),
                                Some("tns:Value"),
                                false,
                            )
                            .with_namespace(ElementNamespace::Namespace(super::NS_TNS)),
                        );
                        let namespace = if self.is_root {
                            ElementNamespace::Namespace(super::NS_TNS)
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
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    OptionTypeSerializerState::Value(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = OptionTypeSerializerState::End__,
                    },
                    OptionTypeSerializerState::End__ => {
                        *self.state = OptionTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    OptionTypeSerializerState::Done__ => return Ok(None),
                    OptionTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for OptionTypeSerializer<'ser> {
        fn next(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> ::core::option::Option<::core::result::Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = OptionTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
