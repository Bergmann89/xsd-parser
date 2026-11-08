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
pub type SomeDetails = WithFirstDataType;
#[derive(Debug)]
pub struct WithFirstDataType {
    pub create_date: Option<String>,
    pub first_info: Option<DataType>,
}
impl WithSerializer for WithFirstDataType {
    type Serializer<'x> = quick_xml_serialize::WithFirstDataTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::WithFirstDataTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::WithFirstDataTypeSerializerState::Init__),
            name: name.unwrap_or("tns:withFirstDataType"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for WithFirstDataType {
    type Deserializer = quick_xml_deserialize::WithFirstDataTypeDeserializer;
}
#[derive(Debug)]
pub struct DataType {
    pub create_date: Option<String>,
}
impl WithSerializer for DataType {
    type Serializer<'x> = quick_xml_serialize::DataTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::DataTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DataTypeSerializerState::Init__),
            name: name.unwrap_or("tns:dataType"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for DataType {
    type Deserializer = quick_xml_deserialize::DataTypeDeserializer;
}
pub type Other = WithAttrType;
#[derive(Debug)]
pub struct WithAttrType {
    pub alt: Option<String>,
    pub content: String,
}
impl WithSerializer for WithAttrType {
    type Serializer<'x> = quick_xml_serialize::WithAttrTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::WithAttrTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::WithAttrTypeSerializerState::Init__),
            name: name.unwrap_or("tns:WithAttr"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for WithAttrType {
    type Deserializer = quick_xml_deserialize::WithAttrTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, ContentDeserializer, DeserializeHelper, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
        ErrorKind, Event, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct WithFirstDataTypeDeserializer {
        create_date: Option<String>,
        first_info: Option<super::DataType>,
        state__: Box<WithFirstDataTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum WithFirstDataTypeDeserializerState {
        Init__,
        CreateDate(Option<<String as WithDeserializer>::Deserializer>),
        FirstInfo(Option<<super::DataType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl WithFirstDataTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                create_date: None,
                first_info: None,
                state__: Box::new(WithFirstDataTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: WithFirstDataTypeDeserializerState,
        ) -> Result<(), Error> {
            use WithFirstDataTypeDeserializerState as S;
            match state {
                S::CreateDate(Some(deserializer)) => {
                    self.store_create_date(deserializer.finish(helper)?)?
                }
                S::FirstInfo(Some(deserializer)) => {
                    self.store_first_info(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_create_date(&mut self, value: String) -> Result<(), Error> {
            if self.create_date.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"createDate",
                )))?;
            }
            self.create_date = Some(value);
            Ok(())
        }
        fn store_first_info(&mut self, value: super::DataType) -> Result<(), Error> {
            if self.first_info.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"firstInfo",
                )))?;
            }
            self.first_info = Some(value);
            Ok(())
        }
        fn handle_create_date<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<WithFirstDataTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use WithFirstDataTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::FirstInfo(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_create_date(data)?;
                    *self.state__ = S::FirstInfo(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::CreateDate(Some(deserializer)));
                    *self.state__ = S::FirstInfo(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_first_info<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::DataType>,
            fallback: &mut Option<WithFirstDataTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use WithFirstDataTypeDeserializerState as S;
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
                    self.store_first_info(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::FirstInfo(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::WithFirstDataType> for WithFirstDataTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WithFirstDataType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WithFirstDataType> {
            use WithFirstDataTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::CreateDate(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_create_date(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::FirstInfo(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_first_info(helper, output, &mut fallback)? {
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
                        *self.state__ = S::CreateDate(None);
                        event
                    }
                    (S::CreateDate(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"createDate",
                            false,
                        )?;
                        match self.handle_create_date(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::FirstInfo(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"firstInfo",
                            false,
                        )?;
                        match self.handle_first_info(helper, output, &mut fallback)? {
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
        ) -> Result<super::WithFirstDataType, Error> {
            let state = replace(
                &mut *self.state__,
                WithFirstDataTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::WithFirstDataType {
                create_date: self.create_date,
                first_info: self.first_info,
            })
        }
    }
    #[derive(Debug)]
    pub struct DataTypeDeserializer {
        create_date: Option<String>,
        state__: Box<DataTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DataTypeDeserializerState {
        Init__,
        CreateDate(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DataTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                create_date: None,
                state__: Box::new(DataTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: DataTypeDeserializerState,
        ) -> Result<(), Error> {
            use DataTypeDeserializerState as S;
            match state {
                S::CreateDate(Some(deserializer)) => {
                    self.store_create_date(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_create_date(&mut self, value: String) -> Result<(), Error> {
            if self.create_date.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"createDate",
                )))?;
            }
            self.create_date = Some(value);
            Ok(())
        }
        fn handle_create_date<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DataTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DataTypeDeserializerState as S;
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
                    self.store_create_date(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::CreateDate(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DataType> for DataTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DataType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DataType> {
            use DataTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::CreateDate(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_create_date(helper, output, &mut fallback)? {
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
                        *self.state__ = S::CreateDate(None);
                        event
                    }
                    (S::CreateDate(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"createDate",
                            false,
                        )?;
                        match self.handle_create_date(helper, output, &mut fallback)? {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::DataType, Error> {
            let state = replace(&mut *self.state__, DataTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::DataType {
                create_date: self.create_date,
            })
        }
    }
    #[derive(Debug)]
    pub struct WithAttrTypeDeserializer {
        alt: Option<String>,
        content: Option<String>,
        state__: Box<WithAttrTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum WithAttrTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl WithAttrTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut alt: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_attrib_local_name(attrib.key, &super::NS_TNS),
                    Some(b"alt")
                ) {
                    helper.read_attrib(&mut alt, b"alt", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                alt: alt,
                content: None,
                state__: Box::new(WithAttrTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: WithAttrTypeDeserializerState,
        ) -> Result<(), Error> {
            if let WithAttrTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::WithAttrType> {
            use WithAttrTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::WithAttrType> for WithAttrTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WithAttrType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::WithAttrType> {
            use WithAttrTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::WithAttrType, Error> {
            let state = replace(&mut *self.state__, WithAttrTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::WithAttrType {
                alt: self.alt,
                content: helper.finish_content(self.content)?,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, ElementNamespace, Error, Event, IterSerializer, SerializeHelper,
        Serializer, WithSerializer,
    };
    #[derive(Debug)]
    pub struct WithFirstDataTypeSerializer<'ser> {
        pub(super) value: &'ser super::WithFirstDataType,
        pub(super) state: Box<WithFirstDataTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum WithFirstDataTypeSerializerState<'ser> {
        Init__,
        CreateDate(IterSerializer<'ser, Option<&'ser String>, String>),
        FirstInfo(IterSerializer<'ser, Option<&'ser super::DataType>, super::DataType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> WithFirstDataTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    WithFirstDataTypeSerializerState::Init__ => {
                        *self.state = WithFirstDataTypeSerializerState::CreateDate(
                            IterSerializer::new(
                                self.value.create_date.as_ref(),
                                Some("tns:createDate"),
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
                    WithFirstDataTypeSerializerState::CreateDate(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = WithFirstDataTypeSerializerState::FirstInfo(
                                    IterSerializer::new(
                                        self.value.first_info.as_ref(),
                                        Some("tns:firstInfo"),
                                        false,
                                    )
                                    .with_namespace(ElementNamespace::Namespace(super::NS_TNS)),
                                )
                            }
                        }
                    }
                    WithFirstDataTypeSerializerState::FirstInfo(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = WithFirstDataTypeSerializerState::End__,
                        }
                    }
                    WithFirstDataTypeSerializerState::End__ => {
                        *self.state = WithFirstDataTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    WithFirstDataTypeSerializerState::Done__ => return Ok(None),
                    WithFirstDataTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for WithFirstDataTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = WithFirstDataTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DataTypeSerializer<'ser> {
        pub(super) value: &'ser super::DataType,
        pub(super) state: Box<DataTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DataTypeSerializerState<'ser> {
        Init__,
        CreateDate(IterSerializer<'ser, Option<&'ser String>, String>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DataTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DataTypeSerializerState::Init__ => {
                        *self.state = DataTypeSerializerState::CreateDate(
                            IterSerializer::new(
                                self.value.create_date.as_ref(),
                                Some("tns:createDate"),
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
                    DataTypeSerializerState::CreateDate(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DataTypeSerializerState::End__,
                    },
                    DataTypeSerializerState::End__ => {
                        *self.state = DataTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DataTypeSerializerState::Done__ => return Ok(None),
                    DataTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for DataTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DataTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct WithAttrTypeSerializer<'ser> {
        pub(super) value: &'ser super::WithAttrType,
        pub(super) state: Box<WithAttrTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum WithAttrTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> WithAttrTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    WithAttrTypeSerializerState::Init__ => {
                        *self.state = WithAttrTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
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
                        helper.write_attrib_opt(&mut bytes, "alt", &self.value.alt)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    WithAttrTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = WithAttrTypeSerializerState::End__,
                        }
                    }
                    WithAttrTypeSerializerState::End__ => {
                        *self.state = WithAttrTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    WithAttrTypeSerializerState::Done__ => return Ok(None),
                    WithAttrTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for WithAttrTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = WithAttrTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
