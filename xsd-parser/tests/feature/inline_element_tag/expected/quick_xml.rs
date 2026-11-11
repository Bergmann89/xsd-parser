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
pub type Catalog = CatalogType;
#[derive(Debug)]
pub struct CatalogType {
    pub product: CatalogProductType,
}
impl WithSerializer for CatalogType {
    type Serializer<'x> = quick_xml_serialize::CatalogTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::CatalogTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::CatalogTypeSerializerState::Init__),
            name: name.unwrap_or("tns:CATALOG"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for CatalogType {
    type Deserializer = quick_xml_deserialize::CatalogTypeDeserializer;
}
#[derive(Debug)]
pub struct CatalogProductType {
    pub id: String,
}
impl WithSerializer for CatalogProductType {
    type Serializer<'x> = quick_xml_serialize::CatalogProductTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::CatalogProductTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::CatalogProductTypeSerializerState::Init__),
            name: name.unwrap_or("tns:PRODUCT"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for CatalogProductType {
    type Deserializer = quick_xml_deserialize::CatalogProductTypeDeserializer;
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
        product: Option<super::CatalogProductType>,
        state__: Box<CatalogTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CatalogTypeDeserializerState {
        Init__,
        Product(Option<<super::CatalogProductType as WithDeserializer>::Deserializer>),
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
                product: None,
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
                S::Product(Some(deserializer)) => {
                    self.store_product(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_product(&mut self, value: super::CatalogProductType) -> Result<(), Error> {
            if self.product.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PRODUCT",
                )))?;
            }
            self.product = Some(value);
            Ok(())
        }
        fn handle_product<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::CatalogProductType>,
            fallback: &mut Option<CatalogTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CatalogTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Product(None));
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
                    self.store_product(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Product(Some(deserializer)));
                    *self.state__ = S::Done__;
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
                    (S::Product(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_product(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Product(None);
                        event
                    }
                    (S::Product(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"PRODUCT",
                            false,
                        )?;
                        match self.handle_product(helper, output, &mut fallback)? {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::CatalogType, Error> {
            let state = replace(&mut *self.state__, CatalogTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::CatalogType {
                product: helper.finish_element("PRODUCT", self.product)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct CatalogProductTypeDeserializer {
        id: Option<String>,
        state__: Box<CatalogProductTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CatalogProductTypeDeserializerState {
        Init__,
        Id(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl CatalogProductTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                id: None,
                state__: Box::new(CatalogProductTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: CatalogProductTypeDeserializerState,
        ) -> Result<(), Error> {
            use CatalogProductTypeDeserializerState as S;
            match state {
                S::Id(Some(deserializer)) => self.store_id(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_id(&mut self, value: String) -> Result<(), Error> {
            if self.id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"ID")))?;
            }
            self.id = Some(value);
            Ok(())
        }
        fn handle_id<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<CatalogProductTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CatalogProductTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Id(None));
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
                    self.store_id(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Id(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::CatalogProductType> for CatalogProductTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CatalogProductType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CatalogProductType> {
            use CatalogProductTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Id(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_id(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Id(None);
                        event
                    }
                    (S::Id(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"ID",
                            false,
                        )?;
                        match self.handle_id(helper, output, &mut fallback)? {
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
        ) -> Result<super::CatalogProductType, Error> {
            let state = replace(
                &mut *self.state__,
                CatalogProductTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::CatalogProductType {
                id: helper.finish_element("ID", self.id)?,
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
    pub struct CatalogTypeSerializer<'ser> {
        pub(super) value: &'ser super::CatalogType,
        pub(super) state: Box<CatalogTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum CatalogTypeSerializerState<'ser> {
        Init__,
        Product(<super::CatalogProductType as WithSerializer>::Serializer<'ser>),
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
                        *self.state = CatalogTypeSerializerState::Product(
                            WithSerializer::serializer_with_namespace(
                                &self.value.product,
                                Some("tns:PRODUCT"),
                                ElementNamespace::Namespace(super::NS_TNS),
                                false,
                            )?,
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
                    CatalogTypeSerializerState::Product(x) => match x.next(helper).transpose()? {
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
    pub struct CatalogProductTypeSerializer<'ser> {
        pub(super) value: &'ser super::CatalogProductType,
        pub(super) state: Box<CatalogProductTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum CatalogProductTypeSerializerState<'ser> {
        Init__,
        Id(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CatalogProductTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CatalogProductTypeSerializerState::Init__ => {
                        *self.state = CatalogProductTypeSerializerState::Id(
                            WithSerializer::serializer_with_namespace(
                                &self.value.id,
                                Some("tns:ID"),
                                ElementNamespace::Namespace(super::NS_TNS),
                                false,
                            )?,
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
                    CatalogProductTypeSerializerState::Id(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CatalogProductTypeSerializerState::End__,
                    },
                    CatalogProductTypeSerializerState::End__ => {
                        *self.state = CatalogProductTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    CatalogProductTypeSerializerState::Done__ => return Ok(None),
                    CatalogProductTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for CatalogProductTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CatalogProductTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
