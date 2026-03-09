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
pub type Collection = CollectionType;
#[derive(Debug)]
pub struct CollectionType {
    pub entry: [CollectionEntryType; 2usize],
    pub name: String,
}
impl WithSerializer for CollectionType {
    type Serializer<'x> = quick_xml_serialize::CollectionTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::CollectionTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::CollectionTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Collection"),
            is_root,
        })
    }
}
impl WithDeserializer for CollectionType {
    type Deserializer = quick_xml_deserialize::CollectionTypeDeserializer;
}
#[derive(Debug)]
pub struct CollectionEntryType {
    pub code: String,
}
impl WithSerializer for CollectionEntryType {
    type Serializer<'x> = quick_xml_serialize::CollectionEntryTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::CollectionEntryTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::CollectionEntryTypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for CollectionEntryType {
    type Deserializer = quick_xml_deserialize::CollectionEntryTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct CollectionTypeDeserializer {
        entry: Vec<super::CollectionEntryType>,
        name: Option<String>,
        state__: Box<CollectionTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CollectionTypeDeserializerState {
        Init__,
        Entry(Option<<super::CollectionEntryType as WithDeserializer>::Deserializer>),
        Name(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl CollectionTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                entry: Vec::new(),
                name: None,
                state__: Box::new(CollectionTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: CollectionTypeDeserializerState,
        ) -> Result<(), Error> {
            use CollectionTypeDeserializerState as S;
            match state {
                S::Entry(Some(deserializer)) => self.store_entry(deserializer.finish(helper)?)?,
                S::Name(Some(deserializer)) => self.store_name(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_entry(&mut self, value: super::CollectionEntryType) -> Result<(), Error> {
            self.entry.push(value);
            Ok(())
        }
        fn store_name(&mut self, value: String) -> Result<(), Error> {
            if self.name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Name")))?;
            }
            self.name = Some(value);
            Ok(())
        }
        fn handle_entry<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::CollectionEntryType>,
            fallback: &mut Option<CollectionTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CollectionTypeDeserializerState as S;
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
                if self.entry.len() < 2usize {
                    fallback.get_or_insert(S::Entry(None));
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
                    self.store_entry(data)?;
                    if self.entry.len() < 2usize {
                        *self.state__ = S::Entry(None);
                    } else {
                        *self.state__ = S::Name(None);
                    }
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Entry(Some(deserializer)));
                    if self.entry.len() < 1usize {
                        *self.state__ = S::Entry(None);
                    } else {
                        *self.state__ = S::Name(None);
                    }
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_name<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<CollectionTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CollectionTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::CollectionType> for CollectionTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CollectionType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CollectionType> {
            use CollectionTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Entry(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_entry(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Entry(None);
                        event
                    }
                    (S::Entry(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::CollectionEntryType as WithDeserializer>::init(helper, event)?;
                        match self.handle_entry(helper, output, &mut fallback)? {
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
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::CollectionType, Error> {
            let state = replace(
                &mut *self.state__,
                CollectionTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::CollectionType {
                entry: helper.finish_arr::<_, 2usize>(self.entry)?,
                name: helper.finish_element("Name", self.name)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct CollectionEntryTypeDeserializer {
        code: Option<String>,
        state__: Box<CollectionEntryTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CollectionEntryTypeDeserializerState {
        Init__,
        Code(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl CollectionEntryTypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: CollectionEntryTypeDeserializerState,
        ) -> Result<(), Error> {
            use CollectionEntryTypeDeserializerState as S;
            match state {
                S::Code(Some(deserializer)) => self.store_code(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_code(&mut self, value: String) -> Result<(), Error> {
            if self.code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Code")))?;
            }
            self.code = Some(value);
            Ok(())
        }
        fn handle_code<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<CollectionEntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CollectionEntryTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Code(None));
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
                    self.store_code(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Code(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::CollectionEntryType> for CollectionEntryTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CollectionEntryType> {
            let deserializer = Self {
                code: None,
                state__: Box::new(CollectionEntryTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, CollectionEntryTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::CollectionEntryType> {
            use CollectionEntryTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Code(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_code(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Code(None);
                        event
                    }
                    (S::Code(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Code",
                            false,
                        )?;
                        match self.handle_code(helper, output, &mut fallback)? {
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
        ) -> Result<super::CollectionEntryType, Error> {
            let state = replace(
                &mut *self.state__,
                CollectionEntryTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::CollectionEntryType {
                code: helper.finish_element("Code", self.code)?,
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
    pub struct CollectionTypeSerializer<'ser> {
        pub(super) value: &'ser super::CollectionType,
        pub(super) state: Box<CollectionTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum CollectionTypeSerializerState<'ser> {
        Init__,
        Entry(IterSerializer<'ser, &'ser [super::CollectionEntryType], super::CollectionEntryType>),
        Name(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CollectionTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CollectionTypeSerializerState::Init__ => {
                        *self.state = CollectionTypeSerializerState::Entry(IterSerializer::new(
                            &self.value.entry[..],
                            Some("Entry"),
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
                    CollectionTypeSerializerState::Entry(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                CollectionTypeSerializerState::Name(WithSerializer::serializer(
                                    &self.value.name,
                                    Some("tns:Name"),
                                    false,
                                )?)
                        }
                    },
                    CollectionTypeSerializerState::Name(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CollectionTypeSerializerState::End__,
                    },
                    CollectionTypeSerializerState::End__ => {
                        *self.state = CollectionTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    CollectionTypeSerializerState::Done__ => return Ok(None),
                    CollectionTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for CollectionTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CollectionTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct CollectionEntryTypeSerializer<'ser> {
        pub(super) value: &'ser super::CollectionEntryType,
        pub(super) state: Box<CollectionEntryTypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum CollectionEntryTypeSerializerState<'ser> {
        Init__,
        Code(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CollectionEntryTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CollectionEntryTypeSerializerState::Init__ => {
                        *self.state = CollectionEntryTypeSerializerState::Code(
                            WithSerializer::serializer(&self.value.code, Some("tns:Code"), false)?,
                        );
                    }
                    CollectionEntryTypeSerializerState::Code(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CollectionEntryTypeSerializerState::Done__,
                        }
                    }
                    CollectionEntryTypeSerializerState::Done__ => return Ok(None),
                    CollectionEntryTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for CollectionEntryTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CollectionEntryTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
