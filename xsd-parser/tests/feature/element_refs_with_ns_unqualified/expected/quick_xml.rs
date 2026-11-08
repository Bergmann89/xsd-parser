use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{ElementNamespace, Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_UNNAMED_5: Namespace = Namespace::new_const(b"Foo");
pub const NS_BAR: Namespace = Namespace::new_const(b"Bar");
pub const NS_BAZ: Namespace = Namespace::new_const(b"Baz");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const PREFIX_BAR: NamespacePrefix = NamespacePrefix::new_const(b"bar");
pub const PREFIX_BAZ: NamespacePrefix = NamespacePrefix::new_const(b"baz");
pub type Outer = OuterType;
#[derive(Debug)]
pub struct OuterType {
    pub inner: bar::InnerType,
    pub note: String,
    pub plain: String,
    pub typed: String,
}
impl WithSerializer for OuterType {
    type Serializer<'x> = quick_xml_serialize::OuterTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        WithSerializer::serializer_with_namespace(self, name, ElementNamespace::Unknown, is_root)
    }
    fn serializer_with_namespace<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        namespace: ElementNamespace,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::OuterTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::OuterTypeSerializerState::Init__),
            name: name.unwrap_or("Outer"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for OuterType {
    type Deserializer = quick_xml_deserialize::OuterTypeDeserializer;
}
pub mod bar {
    use xsd_parser_types::quick_xml::{ElementNamespace, Error, WithDeserializer, WithSerializer};
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
            Ok(quick_xml_serialize::InnerTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::InnerTypeSerializerState::Init__),
                name: name.unwrap_or("bar:Inner"),
                namespace,
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
            BytesEnd, BytesStart, ElementNamespace, Error, Event, SerializeHelper, Serializer,
            WithSerializer,
        };
        #[derive(Debug)]
        pub struct InnerTypeSerializer<'ser> {
            pub(super) value: &'ser super::InnerType,
            pub(super) state: Box<InnerTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) namespace: ElementNamespace,
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
                            *self.state = InnerTypeSerializerState::A(
                                WithSerializer::serializer_with_namespace(
                                    &self.value.a,
                                    Some("bar:A"),
                                    ElementNamespace::Namespace(super::super::NS_BAR),
                                    false,
                                )?,
                            );
                            let namespace = if self.is_root {
                                ElementNamespace::Namespace(super::super::NS_BAR)
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
                                    Some(&super::super::PREFIX_BAR),
                                    &super::super::NS_BAR,
                                );
                            }
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
        inner: Option<super::bar::InnerType>,
        note: Option<String>,
        plain: Option<String>,
        typed: Option<String>,
        state__: Box<OuterTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum OuterTypeDeserializerState {
        Init__,
        Inner(Option<<super::bar::InnerType as WithDeserializer>::Deserializer>),
        Note(Option<<String as WithDeserializer>::Deserializer>),
        Plain(Option<<String as WithDeserializer>::Deserializer>),
        Typed(Option<<String as WithDeserializer>::Deserializer>),
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
                note: None,
                plain: None,
                typed: None,
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
                S::Inner(Some(deserializer)) => self.store_inner(deserializer.finish(helper)?)?,
                S::Note(Some(deserializer)) => self.store_note(deserializer.finish(helper)?)?,
                S::Plain(Some(deserializer)) => self.store_plain(deserializer.finish(helper)?)?,
                S::Typed(Some(deserializer)) => self.store_typed(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_inner(&mut self, value: super::bar::InnerType) -> Result<(), Error> {
            if self.inner.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Inner",
                )))?;
            }
            self.inner = Some(value);
            Ok(())
        }
        fn store_note(&mut self, value: String) -> Result<(), Error> {
            if self.note.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Note")))?;
            }
            self.note = Some(value);
            Ok(())
        }
        fn store_plain(&mut self, value: String) -> Result<(), Error> {
            if self.plain.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Plain",
                )))?;
            }
            self.plain = Some(value);
            Ok(())
        }
        fn store_typed(&mut self, value: String) -> Result<(), Error> {
            if self.typed.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Typed",
                )))?;
            }
            self.typed = Some(value);
            Ok(())
        }
        fn handle_inner<'de>(
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
                    *self.state__ = S::Note(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Inner(Some(deserializer)));
                    *self.state__ = S::Note(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_note<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<OuterTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OuterTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Note(None));
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
                    self.store_note(data)?;
                    *self.state__ = S::Plain(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Note(Some(deserializer)));
                    *self.state__ = S::Plain(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_plain<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<OuterTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OuterTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Plain(None));
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
                    self.store_plain(data)?;
                    *self.state__ = S::Typed(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Plain(Some(deserializer)));
                    *self.state__ = S::Typed(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_typed<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<OuterTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OuterTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Typed(None));
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
                    self.store_typed(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Typed(Some(deserializer)));
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
                    (S::Note(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_note(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Plain(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_plain(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Typed(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_typed(helper, output, &mut fallback)? {
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
                            Some(&super::NS_BAR),
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
                    (S::Note(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_BAZ),
                            b"Note",
                            false,
                        )?;
                        match self.handle_note(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Plain(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"Plain", false)?;
                        match self.handle_plain(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Typed(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_5),
                            b"Typed",
                            false,
                        )?;
                        match self.handle_typed(helper, output, &mut fallback)? {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::OuterType, Error> {
            let state = replace(&mut *self.state__, OuterTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::OuterType {
                inner: helper.finish_element("Inner", self.inner)?,
                note: helper.finish_element("Note", self.note)?,
                plain: helper.finish_element("Plain", self.plain)?,
                typed: helper.finish_element("Typed", self.typed)?,
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
    pub struct OuterTypeSerializer<'ser> {
        pub(super) value: &'ser super::OuterType,
        pub(super) state: Box<OuterTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum OuterTypeSerializerState<'ser> {
        Init__,
        Inner(<super::bar::InnerType as WithSerializer>::Serializer<'ser>),
        Note(<String as WithSerializer>::Serializer<'ser>),
        Plain(<String as WithSerializer>::Serializer<'ser>),
        Typed(<String as WithSerializer>::Serializer<'ser>),
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
                        *self.state = OuterTypeSerializerState::Inner(
                            WithSerializer::serializer_with_namespace(
                                &self.value.inner,
                                Some("bar:Inner"),
                                ElementNamespace::Namespace(super::NS_BAR),
                                false,
                            )?,
                        );
                        let namespace = if self.is_root {
                            ElementNamespace::Namespace(super::NS_UNNAMED_5)
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
                                Some(&super::PREFIX_BAR),
                                &super::NS_BAR,
                            );
                            helper.write_xmlns_unless_default(
                                &mut bytes,
                                Some(&super::PREFIX_BAZ),
                                &super::NS_BAZ,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    OuterTypeSerializerState::Inner(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = OuterTypeSerializerState::Note(
                                WithSerializer::serializer_with_namespace(
                                    &self.value.note,
                                    Some("baz:Note"),
                                    ElementNamespace::Namespace(super::NS_BAZ),
                                    false,
                                )?,
                            )
                        }
                    },
                    OuterTypeSerializerState::Note(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = OuterTypeSerializerState::Plain(
                                WithSerializer::serializer_with_namespace(
                                    &self.value.plain,
                                    Some("Plain"),
                                    ElementNamespace::None,
                                    false,
                                )?,
                            )
                        }
                    },
                    OuterTypeSerializerState::Plain(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = OuterTypeSerializerState::Typed(
                                WithSerializer::serializer_with_namespace(
                                    &self.value.typed,
                                    Some("Typed"),
                                    ElementNamespace::None,
                                    false,
                                )?,
                            )
                        }
                    },
                    OuterTypeSerializerState::Typed(x) => match x.next(helper).transpose()? {
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
