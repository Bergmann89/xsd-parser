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
pub type Catalog = CatalogType;
#[derive(Debug)]
pub struct CatalogType {
    pub title: String,
    pub entry: Vec<CatalogEntryType>,
}
impl WithSerializer for CatalogType {
    type Serializer<'x> = quick_xml_serialize::CatalogTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::CatalogTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::CatalogTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Catalog"),
            is_root,
        })
    }
}
impl WithDeserializer for CatalogType {
    type Deserializer = quick_xml_deserialize::CatalogTypeDeserializer;
}
#[derive(Debug)]
pub struct CatalogEntryType {
    pub key: String,
    pub value: Option<String>,
}
impl WithSerializer for CatalogEntryType {
    type Serializer<'x> = quick_xml_serialize::CatalogEntryTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::CatalogEntryTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::CatalogEntryTypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for CatalogEntryType {
    type Deserializer = quick_xml_deserialize::CatalogEntryTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct CatalogTypeDeserializer {
        title: Option<String>,
        entry: Vec<super::CatalogEntryType>,
        state__: Box<CatalogTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CatalogTypeDeserializerState {
        Init__,
        Title(Option<<String as WithDeserializer>::Deserializer>),
        Entry(Option<<super::CatalogEntryType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl CatalogTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                title: None,
                entry: Vec::new(),
                state__: Box::new(CatalogTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: CatalogTypeDeserializerState,
        ) -> Result<(), Error> {
            use CatalogTypeDeserializerState as S;
            match state {
                S::Title(Some(deserializer)) => self.store_title(deserializer.finish(helper)?)?,
                S::Entry(Some(deserializer)) => self.store_entry(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_title(&mut self, value: String) -> Result<(), Error> {
            if self.title.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Title",
                )))?;
            }
            self.title = Some(value);
            Ok(())
        }
        fn store_entry(&mut self, value: super::CatalogEntryType) -> Result<(), Error> {
            self.entry.push(value);
            Ok(())
        }
        fn handle_title<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<CatalogTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CatalogTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Title(None));
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
                    self.store_title(data)?;
                    *self.state__ = S::Entry(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Title(Some(deserializer)));
                    *self.state__ = S::Entry(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_entry<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::CatalogEntryType>,
            fallback: &mut Option<CatalogTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CatalogTypeDeserializerState as S;
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
                if self.entry.len() < 1usize {
                    fallback.get_or_insert(S::Entry(None));
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else {
                    *self.state__ = S::Done__;
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
                    *self.state__ = S::Entry(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Entry(Some(deserializer)));
                    *self.state__ = S::Entry(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::CatalogType> for CatalogTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CatalogType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CatalogType> {
            use CatalogTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Title(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_title(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
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
                        *self.state__ = S::Title(None);
                        event
                    }
                    (S::Title(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Title",
                            false,
                        )?;
                        match self.handle_title(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Entry(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::CatalogEntryType as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::CatalogType, Error> {
            let state = replace(&mut *self.state__, CatalogTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::CatalogType {
                title: helper.finish_element("Title", self.title)?,
                entry: helper.finish_vec(1usize, None, self.entry)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct CatalogEntryTypeDeserializer {
        key: Option<String>,
        value: Option<String>,
        state__: Box<CatalogEntryTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CatalogEntryTypeDeserializerState {
        Init__,
        Key(Option<<String as WithDeserializer>::Deserializer>),
        Value(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl CatalogEntryTypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: CatalogEntryTypeDeserializerState,
        ) -> Result<(), Error> {
            use CatalogEntryTypeDeserializerState as S;
            match state {
                S::Key(Some(deserializer)) => self.store_key(deserializer.finish(helper)?)?,
                S::Value(Some(deserializer)) => self.store_value(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_key(&mut self, value: String) -> Result<(), Error> {
            if self.key.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Key")))?;
            }
            self.key = Some(value);
            Ok(())
        }
        fn store_value(&mut self, value: String) -> Result<(), Error> {
            if self.value.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Value",
                )))?;
            }
            self.value = Some(value);
            Ok(())
        }
        fn handle_key<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<CatalogEntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CatalogEntryTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Key(None));
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
                    self.store_key(data)?;
                    *self.state__ = S::Value(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Key(Some(deserializer)));
                    *self.state__ = S::Value(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_value<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<CatalogEntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CatalogEntryTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::CatalogEntryType> for CatalogEntryTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CatalogEntryType> {
            let deserializer = Self {
                key: None,
                value: None,
                state__: Box::new(CatalogEntryTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, CatalogEntryTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::CatalogEntryType> {
            use CatalogEntryTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Key(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_key(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Key(None);
                        event
                    }
                    (S::Key(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Key",
                            false,
                        )?;
                        match self.handle_key(helper, output, &mut fallback)? {
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
        ) -> Result<super::CatalogEntryType, Error> {
            let state = replace(
                &mut *self.state__,
                CatalogEntryTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::CatalogEntryType {
                key: helper.finish_element("Key", self.key)?,
                value: self.value,
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
    pub struct CatalogTypeSerializer<'ser> {
        pub(super) value: &'ser super::CatalogType,
        pub(super) state: Box<CatalogTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum CatalogTypeSerializerState<'ser> {
        Init__,
        Title(<String as WithSerializer>::Serializer<'ser>),
        Entry(IterSerializer<'ser, &'ser [super::CatalogEntryType], super::CatalogEntryType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CatalogTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CatalogTypeSerializerState::Init__ => {
                        *self.state =
                            CatalogTypeSerializerState::Title(WithSerializer::serializer(
                                &self.value.title,
                                Some("tns:Title"),
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
                    CatalogTypeSerializerState::Title(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = CatalogTypeSerializerState::Entry(IterSerializer::new(
                                &self.value.entry[..],
                                Some("Entry"),
                                false,
                            ))
                        }
                    },
                    CatalogTypeSerializerState::Entry(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CatalogTypeSerializerState::End__,
                    },
                    CatalogTypeSerializerState::End__ => {
                        *self.state = CatalogTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    CatalogTypeSerializerState::Done__ => return Ok(None),
                    CatalogTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for CatalogTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CatalogTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct CatalogEntryTypeSerializer<'ser> {
        pub(super) value: &'ser super::CatalogEntryType,
        pub(super) state: Box<CatalogEntryTypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum CatalogEntryTypeSerializerState<'ser> {
        Init__,
        Key(<String as WithSerializer>::Serializer<'ser>),
        Value(IterSerializer<'ser, Option<&'ser String>, String>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CatalogEntryTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CatalogEntryTypeSerializerState::Init__ => {
                        *self.state = CatalogEntryTypeSerializerState::Key(
                            WithSerializer::serializer(&self.value.key, Some("tns:Key"), false)?,
                        );
                    }
                    CatalogEntryTypeSerializerState::Key(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                CatalogEntryTypeSerializerState::Value(IterSerializer::new(
                                    self.value.value.as_ref(),
                                    Some("tns:Value"),
                                    false,
                                ))
                        }
                    },
                    CatalogEntryTypeSerializerState::Value(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CatalogEntryTypeSerializerState::Done__,
                        }
                    }
                    CatalogEntryTypeSerializerState::Done__ => return Ok(None),
                    CatalogEntryTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for CatalogEntryTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CatalogEntryTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
