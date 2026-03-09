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
pub type Item = ItemType;
#[derive(Debug)]
pub struct ItemType {
    pub metadata: Option<ItemMetadataType>,
    pub name: String,
}
impl WithSerializer for ItemType {
    type Serializer<'x> = quick_xml_serialize::ItemTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ItemTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ItemTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Item"),
            is_root,
        })
    }
}
impl WithDeserializer for ItemType {
    type Deserializer = quick_xml_deserialize::ItemTypeDeserializer;
}
#[derive(Debug)]
pub struct ItemMetadataType {
    pub label: Option<String>,
    pub description: Option<String>,
}
impl WithSerializer for ItemMetadataType {
    type Serializer<'x> = quick_xml_serialize::ItemMetadataTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::ItemMetadataTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ItemMetadataTypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for ItemMetadataType {
    type Deserializer = quick_xml_deserialize::ItemMetadataTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct ItemTypeDeserializer {
        metadata: Option<super::ItemMetadataType>,
        name: Option<String>,
        state__: Box<ItemTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ItemTypeDeserializerState {
        Init__,
        Metadata(Option<<super::ItemMetadataType as WithDeserializer>::Deserializer>),
        Name(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ItemTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                metadata: None,
                name: None,
                state__: Box::new(ItemTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ItemTypeDeserializerState,
        ) -> Result<(), Error> {
            use ItemTypeDeserializerState as S;
            match state {
                S::Metadata(Some(deserializer)) => {
                    self.store_metadata(deserializer.finish(helper)?)?
                }
                S::Name(Some(deserializer)) => self.store_name(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_metadata(&mut self, value: super::ItemMetadataType) -> Result<(), Error> {
            if self.metadata.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Metadata",
                )))?;
            }
            self.metadata = Some(value);
            Ok(())
        }
        fn store_name(&mut self, value: String) -> Result<(), Error> {
            if self.name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Name")))?;
            }
            self.name = Some(value);
            Ok(())
        }
        fn handle_metadata<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ItemMetadataType>,
            fallback: &mut Option<ItemTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ItemTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Name(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_metadata(data)?;
                    *self.state__ = S::Name(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Metadata(Some(deserializer)));
                    *self.state__ = S::Name(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_name<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ItemTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ItemTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::ItemType> for ItemTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ItemType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ItemType> {
            use ItemTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Metadata(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_metadata(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Metadata(None);
                        event
                    }
                    (S::Metadata(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::ItemMetadataType as WithDeserializer>::init(helper, event)?;
                        match self.handle_metadata(helper, output, &mut fallback)? {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ItemType, Error> {
            let state = replace(&mut *self.state__, ItemTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::ItemType {
                metadata: self.metadata,
                name: helper.finish_element("Name", self.name)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ItemMetadataTypeDeserializer {
        label: Option<String>,
        description: Option<String>,
        state__: Box<ItemMetadataTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ItemMetadataTypeDeserializerState {
        Init__,
        Label(Option<<String as WithDeserializer>::Deserializer>),
        Description(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ItemMetadataTypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ItemMetadataTypeDeserializerState,
        ) -> Result<(), Error> {
            use ItemMetadataTypeDeserializerState as S;
            match state {
                S::Label(Some(deserializer)) => self.store_label(deserializer.finish(helper)?)?,
                S::Description(Some(deserializer)) => {
                    self.store_description(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_label(&mut self, value: String) -> Result<(), Error> {
            if self.label.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Label",
                )))?;
            }
            self.label = Some(value);
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
        fn handle_label<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ItemMetadataTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ItemMetadataTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Description(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_label(data)?;
                    *self.state__ = S::Description(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Label(Some(deserializer)));
                    *self.state__ = S::Description(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_description<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ItemMetadataTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ItemMetadataTypeDeserializerState as S;
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
    impl Default for ItemMetadataTypeDeserializer {
        fn default() -> Self {
            Self {
                label: None,
                description: None,
                state__: Box::new(ItemMetadataTypeDeserializerState::Init__),
            }
        }
    }
    impl<'de> Deserializer<'de, super::ItemMetadataType> for ItemMetadataTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ItemMetadataType> {
            let deserializer = Self {
                label: None,
                description: None,
                state__: Box::new(ItemMetadataTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, ItemMetadataTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::ItemMetadataType> {
            use ItemMetadataTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Label(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_label(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Label(None);
                        event
                    }
                    (S::Label(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Label",
                            false,
                        )?;
                        match self.handle_label(helper, output, &mut fallback)? {
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
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::ItemMetadataType, Error> {
            let state = replace(
                &mut *self.state__,
                ItemMetadataTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ItemMetadataType {
                label: self.label,
                description: self.description,
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
    pub struct ItemTypeSerializer<'ser> {
        pub(super) value: &'ser super::ItemType,
        pub(super) state: Box<ItemTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ItemTypeSerializerState<'ser> {
        Init__,
        Metadata(
            IterSerializer<'ser, Option<&'ser super::ItemMetadataType>, super::ItemMetadataType>,
        ),
        Name(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ItemTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ItemTypeSerializerState::Init__ => {
                        *self.state = ItemTypeSerializerState::Metadata(IterSerializer::new(
                            self.value.metadata.as_ref(),
                            Some("Metadata"),
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
                    ItemTypeSerializerState::Metadata(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = ItemTypeSerializerState::Name(WithSerializer::serializer(
                                &self.value.name,
                                Some("tns:Name"),
                                false,
                            )?)
                        }
                    },
                    ItemTypeSerializerState::Name(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ItemTypeSerializerState::End__,
                    },
                    ItemTypeSerializerState::End__ => {
                        *self.state = ItemTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ItemTypeSerializerState::Done__ => return Ok(None),
                    ItemTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ItemTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ItemTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ItemMetadataTypeSerializer<'ser> {
        pub(super) value: &'ser super::ItemMetadataType,
        pub(super) state: Box<ItemMetadataTypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum ItemMetadataTypeSerializerState<'ser> {
        Init__,
        Label(IterSerializer<'ser, Option<&'ser String>, String>),
        Description(IterSerializer<'ser, Option<&'ser String>, String>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ItemMetadataTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ItemMetadataTypeSerializerState::Init__ => {
                        *self.state = ItemMetadataTypeSerializerState::Label(IterSerializer::new(
                            self.value.label.as_ref(),
                            Some("tns:Label"),
                            false,
                        ));
                    }
                    ItemMetadataTypeSerializerState::Label(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = ItemMetadataTypeSerializerState::Description(
                                    IterSerializer::new(
                                        self.value.description.as_ref(),
                                        Some("tns:Description"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    ItemMetadataTypeSerializerState::Description(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ItemMetadataTypeSerializerState::Done__,
                        }
                    }
                    ItemMetadataTypeSerializerState::Done__ => return Ok(None),
                    ItemMetadataTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ItemMetadataTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ItemMetadataTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
