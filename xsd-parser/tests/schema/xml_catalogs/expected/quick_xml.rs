use xsd_parser_types::misc::{Namespace, NamespacePrefix};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_ER: Namespace = Namespace::new_const(b"urn:oasis:names:tc:entity:xmlns:xml:catalog");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_ER: NamespacePrefix = NamespacePrefix::new_const(b"er");
pub mod er {
    use std::borrow::Cow;
    use xsd_parser_types::quick_xml::{
        DeserializeBytes, DeserializeHelper, Error, ErrorKind, RawByteStr, SerializeBytes,
        SerializeHelper, WithDeserializer, WithSerializer,
    };
    #[derive(Debug)]
    pub struct CatalogType {
        pub id: Option<String>,
        pub prefer: Option<SystemOrPublicType>,
        pub content: Vec<CatalogTypeContent>,
    }
    #[derive(Debug)]
    pub enum CatalogTypeContent {
        Public(PublicType),
        System(SystemType),
        Uri(UriType),
        RewriteSystem(RewriteSystemType),
        RewriteUri(RewriteUriType),
        UriSuffix(UriSuffixType),
        SystemSuffix(SystemSuffixType),
        DelegatePublic(DelegatePublicType),
        DelegateSystem(DelegateSystemType),
        DelegateUri(DelegateUriType),
        NextCatalog(NextCatalogType),
        Group(GroupType),
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
                name: name.unwrap_or("er:catalog"),
                is_root,
            })
        }
    }
    impl WithSerializer for CatalogTypeContent {
        type Serializer<'x> = quick_xml_serialize::CatalogTypeContentSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            let _name = name;
            let _is_root = is_root;
            Ok(quick_xml_serialize::CatalogTypeContentSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::CatalogTypeContentSerializerState::Init__),
            })
        }
    }
    impl WithDeserializer for CatalogType {
        type Deserializer = quick_xml_deserialize::CatalogTypeDeserializer;
    }
    impl WithDeserializer for CatalogTypeContent {
        type Deserializer = quick_xml_deserialize::CatalogTypeContentDeserializer;
    }
    pub type Catalog = CatalogType;
    #[derive(Debug)]
    pub struct DelegatePublicType {
        pub public_id_start_string: String,
        pub catalog: String,
        pub id: Option<String>,
    }
    impl WithSerializer for DelegatePublicType {
        type Serializer<'x> = quick_xml_serialize::DelegatePublicTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::DelegatePublicTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::DelegatePublicTypeSerializerState::Init__),
                name: name.unwrap_or("er:delegatePublic"),
                is_root,
            })
        }
    }
    impl WithDeserializer for DelegatePublicType {
        type Deserializer = quick_xml_deserialize::DelegatePublicTypeDeserializer;
    }
    pub type DelegatePublic = DelegatePublicType;
    #[derive(Debug)]
    pub struct DelegateSystemType {
        pub system_id_start_string: String,
        pub catalog: String,
        pub id: Option<String>,
    }
    impl WithSerializer for DelegateSystemType {
        type Serializer<'x> = quick_xml_serialize::DelegateSystemTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::DelegateSystemTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::DelegateSystemTypeSerializerState::Init__),
                name: name.unwrap_or("er:delegateSystem"),
                is_root,
            })
        }
    }
    impl WithDeserializer for DelegateSystemType {
        type Deserializer = quick_xml_deserialize::DelegateSystemTypeDeserializer;
    }
    pub type DelegateSystem = DelegateSystemType;
    #[derive(Debug)]
    pub struct DelegateUriType {
        pub uri_start_string: String,
        pub catalog: String,
        pub id: Option<String>,
    }
    impl WithSerializer for DelegateUriType {
        type Serializer<'x> = quick_xml_serialize::DelegateUriTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::DelegateUriTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::DelegateUriTypeSerializerState::Init__),
                name: name.unwrap_or("er:delegateURI"),
                is_root,
            })
        }
    }
    impl WithDeserializer for DelegateUriType {
        type Deserializer = quick_xml_deserialize::DelegateUriTypeDeserializer;
    }
    pub type DelegateUri = DelegateUriType;
    #[derive(Debug)]
    pub struct GroupType {
        pub prefer: Option<SystemOrPublicType>,
        pub id: Option<String>,
        pub content: Vec<GroupTypeContent>,
    }
    #[derive(Debug)]
    pub enum GroupTypeContent {
        Public(PublicType),
        System(SystemType),
        Uri(UriType),
        RewriteSystem(RewriteSystemType),
        RewriteUri(RewriteUriType),
        UriSuffix(UriSuffixType),
        SystemSuffix(SystemSuffixType),
        DelegatePublic(DelegatePublicType),
        DelegateSystem(DelegateSystemType),
        DelegateUri(DelegateUriType),
        NextCatalog(NextCatalogType),
    }
    impl WithSerializer for GroupType {
        type Serializer<'x> = quick_xml_serialize::GroupTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::GroupTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::GroupTypeSerializerState::Init__),
                name: name.unwrap_or("er:group"),
                is_root,
            })
        }
    }
    impl WithSerializer for GroupTypeContent {
        type Serializer<'x> = quick_xml_serialize::GroupTypeContentSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            let _name = name;
            let _is_root = is_root;
            Ok(quick_xml_serialize::GroupTypeContentSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::GroupTypeContentSerializerState::Init__),
            })
        }
    }
    impl WithDeserializer for GroupType {
        type Deserializer = quick_xml_deserialize::GroupTypeDeserializer;
    }
    impl WithDeserializer for GroupTypeContent {
        type Deserializer = quick_xml_deserialize::GroupTypeContentDeserializer;
    }
    pub type Group = GroupType;
    #[derive(Debug)]
    pub struct NextCatalogType {
        pub catalog: String,
        pub id: Option<String>,
    }
    impl WithSerializer for NextCatalogType {
        type Serializer<'x> = quick_xml_serialize::NextCatalogTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::NextCatalogTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::NextCatalogTypeSerializerState::Init__),
                name: name.unwrap_or("er:nextCatalog"),
                is_root,
            })
        }
    }
    impl WithDeserializer for NextCatalogType {
        type Deserializer = quick_xml_deserialize::NextCatalogTypeDeserializer;
    }
    pub type NextCatalog = NextCatalogType;
    pub type PartialPublicIdentifierType = String;
    pub type PubIdCharsType = String;
    #[derive(Debug)]
    pub struct PublicType {
        pub public_id: String,
        pub uri: String,
        pub id: Option<String>,
    }
    impl WithSerializer for PublicType {
        type Serializer<'x> = quick_xml_serialize::PublicTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::PublicTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::PublicTypeSerializerState::Init__),
                name: name.unwrap_or("er:public"),
                is_root,
            })
        }
    }
    impl WithDeserializer for PublicType {
        type Deserializer = quick_xml_deserialize::PublicTypeDeserializer;
    }
    pub type Public = PublicType;
    pub type PublicIdentifierType = String;
    #[derive(Debug)]
    pub struct RewriteSystemType {
        pub system_id_start_string: String,
        pub rewrite_prefix: String,
        pub id: Option<String>,
    }
    impl WithSerializer for RewriteSystemType {
        type Serializer<'x> = quick_xml_serialize::RewriteSystemTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::RewriteSystemTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::RewriteSystemTypeSerializerState::Init__),
                name: name.unwrap_or("er:rewriteSystem"),
                is_root,
            })
        }
    }
    impl WithDeserializer for RewriteSystemType {
        type Deserializer = quick_xml_deserialize::RewriteSystemTypeDeserializer;
    }
    pub type RewriteSystem = RewriteSystemType;
    #[derive(Debug)]
    pub struct RewriteUriType {
        pub uri_start_string: String,
        pub rewrite_prefix: String,
        pub id: Option<String>,
    }
    impl WithSerializer for RewriteUriType {
        type Serializer<'x> = quick_xml_serialize::RewriteUriTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::RewriteUriTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::RewriteUriTypeSerializerState::Init__),
                name: name.unwrap_or("er:rewriteURI"),
                is_root,
            })
        }
    }
    impl WithDeserializer for RewriteUriType {
        type Deserializer = quick_xml_deserialize::RewriteUriTypeDeserializer;
    }
    pub type RewriteUri = RewriteUriType;
    #[derive(Debug)]
    pub struct SystemType {
        pub system_id: String,
        pub uri: String,
        pub id: Option<String>,
    }
    impl WithSerializer for SystemType {
        type Serializer<'x> = quick_xml_serialize::SystemTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::SystemTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::SystemTypeSerializerState::Init__),
                name: name.unwrap_or("er:system"),
                is_root,
            })
        }
    }
    impl WithDeserializer for SystemType {
        type Deserializer = quick_xml_deserialize::SystemTypeDeserializer;
    }
    pub type System = SystemType;
    #[derive(Debug)]
    pub enum SystemOrPublicType {
        System,
        Public,
    }
    impl SerializeBytes for SystemOrPublicType {
        fn serialize_bytes(
            &self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Cow<'_, str>>, Error> {
            match self {
                Self::System => Ok(Some(Cow::Borrowed("system"))),
                Self::Public => Ok(Some(Cow::Borrowed("public"))),
            }
        }
    }
    impl DeserializeBytes for SystemOrPublicType {
        fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
            match bytes {
                b"system" => Ok(Self::System),
                b"public" => Ok(Self::Public),
                x => Err(Error::from(ErrorKind::UnknownOrInvalidValue(
                    RawByteStr::from_slice(x),
                ))),
            }
        }
    }
    #[derive(Debug)]
    pub struct SystemSuffixType {
        pub system_id_suffix: String,
        pub uri: String,
        pub id: Option<String>,
    }
    impl WithSerializer for SystemSuffixType {
        type Serializer<'x> = quick_xml_serialize::SystemSuffixTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::SystemSuffixTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::SystemSuffixTypeSerializerState::Init__),
                name: name.unwrap_or("er:systemSuffix"),
                is_root,
            })
        }
    }
    impl WithDeserializer for SystemSuffixType {
        type Deserializer = quick_xml_deserialize::SystemSuffixTypeDeserializer;
    }
    pub type SystemSuffix = SystemSuffixType;
    #[derive(Debug)]
    pub struct UriType {
        pub name: String,
        pub uri: String,
        pub id: Option<String>,
    }
    impl WithSerializer for UriType {
        type Serializer<'x> = quick_xml_serialize::UriTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::UriTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::UriTypeSerializerState::Init__),
                name: name.unwrap_or("er:uri"),
                is_root,
            })
        }
    }
    impl WithDeserializer for UriType {
        type Deserializer = quick_xml_deserialize::UriTypeDeserializer;
    }
    pub type Uri = UriType;
    #[derive(Debug)]
    pub struct UriSuffixType {
        pub uri_suffix: String,
        pub uri: String,
        pub id: Option<String>,
    }
    impl WithSerializer for UriSuffixType {
        type Serializer<'x> = quick_xml_serialize::UriSuffixTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::UriSuffixTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::UriSuffixTypeSerializerState::Init__),
                name: name.unwrap_or("er:uriSuffix"),
                is_root,
            })
        }
    }
    impl WithDeserializer for UriSuffixType {
        type Deserializer = quick_xml_deserialize::UriSuffixTypeDeserializer;
    }
    pub type UriSuffix = UriSuffixType;
    pub mod quick_xml_deserialize {
        use core::mem::replace;
        use xsd_parser_types::quick_xml::{
            BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
            RawByteStr, WithDeserializer,
        };
        #[derive(Debug)]
        pub struct CatalogTypeDeserializer {
            id: Option<String>,
            prefer: Option<super::SystemOrPublicType>,
            content: Vec<super::CatalogTypeContent>,
            state__: Box<CatalogTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum CatalogTypeDeserializerState {
            Init__,
            Next__,
            Content__(<super::CatalogTypeContent as WithDeserializer>::Deserializer),
            Unknown__,
        }
        impl CatalogTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                let mut id: Option<String> = None;
                let mut prefer: Option<super::SystemOrPublicType> = None;
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"id")
                    ) {
                        helper.read_attrib(&mut id, b"id", &attrib.value)?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"prefer")
                    ) {
                        helper.read_attrib(&mut prefer, b"prefer", &attrib.value)?;
                    }
                }
                Ok(Self {
                    id: id,
                    prefer: prefer,
                    content: Vec::new(),
                    state__: Box::new(CatalogTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: CatalogTypeDeserializerState,
            ) -> Result<(), Error> {
                if let CatalogTypeDeserializerState::Content__(deserializer) = state {
                    self.store_content(deserializer.finish(helper)?)?;
                }
                Ok(())
            }
            fn store_content(&mut self, value: super::CatalogTypeContent) -> Result<(), Error> {
                self.content.push(value);
                Ok(())
            }
            fn handle_content<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, super::CatalogTypeContent>,
                fallback: &mut Option<CatalogTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use CatalogTypeDeserializerState as S;
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
                            let output = <super::CatalogTypeContent as WithDeserializer>::init(
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
            ) -> Result<super::CatalogType, Error> {
                let state = replace(&mut *self.state__, CatalogTypeDeserializerState::Unknown__);
                self.finish_state(helper, state)?;
                Ok(super::CatalogType {
                    id: self.id,
                    prefer: self.prefer,
                    content: helper.finish_vec(1usize, None, self.content)?,
                })
            }
        }
        #[derive(Debug)]
        pub struct CatalogTypeContentDeserializer {
            state__: Box<CatalogTypeContentDeserializerState>,
        }
        #[derive(Debug)]
        pub enum CatalogTypeContentDeserializerState {
            Init__,
            Public(
                Option<super::PublicType>,
                Option<<super::PublicType as WithDeserializer>::Deserializer>,
                Option<<super::PublicType as WithDeserializer>::Deserializer>,
            ),
            System(
                Option<super::SystemType>,
                Option<<super::SystemType as WithDeserializer>::Deserializer>,
                Option<<super::SystemType as WithDeserializer>::Deserializer>,
            ),
            Uri(
                Option<super::UriType>,
                Option<<super::UriType as WithDeserializer>::Deserializer>,
                Option<<super::UriType as WithDeserializer>::Deserializer>,
            ),
            RewriteSystem(
                Option<super::RewriteSystemType>,
                Option<<super::RewriteSystemType as WithDeserializer>::Deserializer>,
                Option<<super::RewriteSystemType as WithDeserializer>::Deserializer>,
            ),
            RewriteUri(
                Option<super::RewriteUriType>,
                Option<<super::RewriteUriType as WithDeserializer>::Deserializer>,
                Option<<super::RewriteUriType as WithDeserializer>::Deserializer>,
            ),
            UriSuffix(
                Option<super::UriSuffixType>,
                Option<<super::UriSuffixType as WithDeserializer>::Deserializer>,
                Option<<super::UriSuffixType as WithDeserializer>::Deserializer>,
            ),
            SystemSuffix(
                Option<super::SystemSuffixType>,
                Option<<super::SystemSuffixType as WithDeserializer>::Deserializer>,
                Option<<super::SystemSuffixType as WithDeserializer>::Deserializer>,
            ),
            DelegatePublic(
                Option<super::DelegatePublicType>,
                Option<<super::DelegatePublicType as WithDeserializer>::Deserializer>,
                Option<<super::DelegatePublicType as WithDeserializer>::Deserializer>,
            ),
            DelegateSystem(
                Option<super::DelegateSystemType>,
                Option<<super::DelegateSystemType as WithDeserializer>::Deserializer>,
                Option<<super::DelegateSystemType as WithDeserializer>::Deserializer>,
            ),
            DelegateUri(
                Option<super::DelegateUriType>,
                Option<<super::DelegateUriType as WithDeserializer>::Deserializer>,
                Option<<super::DelegateUriType as WithDeserializer>::Deserializer>,
            ),
            NextCatalog(
                Option<super::NextCatalogType>,
                Option<<super::NextCatalogType as WithDeserializer>::Deserializer>,
                Option<<super::NextCatalogType as WithDeserializer>::Deserializer>,
            ),
            Group(
                Option<super::GroupType>,
                Option<<super::GroupType as WithDeserializer>::Deserializer>,
                Option<<super::GroupType as WithDeserializer>::Deserializer>,
            ),
            Done__(super::CatalogTypeContent),
            Unknown__,
        }
        impl CatalogTypeContentDeserializer {
            fn find_suitable<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                if let Event::Start(x) | Event::Empty(x) = &event {
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"public")
                    ) {
                        let output = <super::PublicType as WithDeserializer>::init(helper, event)?;
                        return self.handle_public(helper, Default::default(), None, output);
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"system")
                    ) {
                        let output = <super::SystemType as WithDeserializer>::init(helper, event)?;
                        return self.handle_system(helper, Default::default(), None, output);
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"uri")
                    ) {
                        let output = <super::UriType as WithDeserializer>::init(helper, event)?;
                        return self.handle_uri(helper, Default::default(), None, output);
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"rewriteSystem")
                    ) {
                        let output =
                            <super::RewriteSystemType as WithDeserializer>::init(helper, event)?;
                        return self.handle_rewrite_system(
                            helper,
                            Default::default(),
                            None,
                            output,
                        );
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"rewriteURI")
                    ) {
                        let output =
                            <super::RewriteUriType as WithDeserializer>::init(helper, event)?;
                        return self.handle_rewrite_uri(helper, Default::default(), None, output);
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"uriSuffix")
                    ) {
                        let output =
                            <super::UriSuffixType as WithDeserializer>::init(helper, event)?;
                        return self.handle_uri_suffix(helper, Default::default(), None, output);
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"systemSuffix")
                    ) {
                        let output =
                            <super::SystemSuffixType as WithDeserializer>::init(helper, event)?;
                        return self.handle_system_suffix(helper, Default::default(), None, output);
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"delegatePublic")
                    ) {
                        let output =
                            <super::DelegatePublicType as WithDeserializer>::init(helper, event)?;
                        return self.handle_delegate_public(
                            helper,
                            Default::default(),
                            None,
                            output,
                        );
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"delegateSystem")
                    ) {
                        let output =
                            <super::DelegateSystemType as WithDeserializer>::init(helper, event)?;
                        return self.handle_delegate_system(
                            helper,
                            Default::default(),
                            None,
                            output,
                        );
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"delegateURI")
                    ) {
                        let output =
                            <super::DelegateUriType as WithDeserializer>::init(helper, event)?;
                        return self.handle_delegate_uri(helper, Default::default(), None, output);
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"nextCatalog")
                    ) {
                        let output =
                            <super::NextCatalogType as WithDeserializer>::init(helper, event)?;
                        return self.handle_next_catalog(helper, Default::default(), None, output);
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"group")
                    ) {
                        let output = <super::GroupType as WithDeserializer>::init(helper, event)?;
                        return self.handle_group(helper, Default::default(), None, output);
                    }
                }
                *self.state__ = CatalogTypeContentDeserializerState::Init__;
                Ok(ElementHandlerOutput::return_to_parent(event, true))
            }
            fn finish_state(
                helper: &mut DeserializeHelper,
                state: CatalogTypeContentDeserializerState,
            ) -> Result<super::CatalogTypeContent, Error> {
                use CatalogTypeContentDeserializerState as S;
                match state {
                    S::Init__ => Err(ErrorKind::MissingContent.into()),
                    S::Public(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_public(&mut values, value)?;
                        }
                        Ok(super::CatalogTypeContent::Public(
                            helper.finish_element("public", values)?,
                        ))
                    }
                    S::System(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_system(&mut values, value)?;
                        }
                        Ok(super::CatalogTypeContent::System(
                            helper.finish_element("system", values)?,
                        ))
                    }
                    S::Uri(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_uri(&mut values, value)?;
                        }
                        Ok(super::CatalogTypeContent::Uri(
                            helper.finish_element("uri", values)?,
                        ))
                    }
                    S::RewriteSystem(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_rewrite_system(&mut values, value)?;
                        }
                        Ok(super::CatalogTypeContent::RewriteSystem(
                            helper.finish_element("rewriteSystem", values)?,
                        ))
                    }
                    S::RewriteUri(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_rewrite_uri(&mut values, value)?;
                        }
                        Ok(super::CatalogTypeContent::RewriteUri(
                            helper.finish_element("rewriteURI", values)?,
                        ))
                    }
                    S::UriSuffix(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_uri_suffix(&mut values, value)?;
                        }
                        Ok(super::CatalogTypeContent::UriSuffix(
                            helper.finish_element("uriSuffix", values)?,
                        ))
                    }
                    S::SystemSuffix(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_system_suffix(&mut values, value)?;
                        }
                        Ok(super::CatalogTypeContent::SystemSuffix(
                            helper.finish_element("systemSuffix", values)?,
                        ))
                    }
                    S::DelegatePublic(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_delegate_public(&mut values, value)?;
                        }
                        Ok(super::CatalogTypeContent::DelegatePublic(
                            helper.finish_element("delegatePublic", values)?,
                        ))
                    }
                    S::DelegateSystem(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_delegate_system(&mut values, value)?;
                        }
                        Ok(super::CatalogTypeContent::DelegateSystem(
                            helper.finish_element("delegateSystem", values)?,
                        ))
                    }
                    S::DelegateUri(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_delegate_uri(&mut values, value)?;
                        }
                        Ok(super::CatalogTypeContent::DelegateUri(
                            helper.finish_element("delegateURI", values)?,
                        ))
                    }
                    S::NextCatalog(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_next_catalog(&mut values, value)?;
                        }
                        Ok(super::CatalogTypeContent::NextCatalog(
                            helper.finish_element("nextCatalog", values)?,
                        ))
                    }
                    S::Group(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_group(&mut values, value)?;
                        }
                        Ok(super::CatalogTypeContent::Group(
                            helper.finish_element("group", values)?,
                        ))
                    }
                    S::Done__(data) => Ok(data),
                    _ => unreachable!(),
                }
            }
            fn store_public(
                values: &mut Option<super::PublicType>,
                value: super::PublicType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"public",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_system(
                values: &mut Option<super::SystemType>,
                value: super::SystemType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"system",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_uri(
                values: &mut Option<super::UriType>,
                value: super::UriType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"uri")))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_rewrite_system(
                values: &mut Option<super::RewriteSystemType>,
                value: super::RewriteSystemType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"rewriteSystem",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_rewrite_uri(
                values: &mut Option<super::RewriteUriType>,
                value: super::RewriteUriType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"rewriteURI",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_uri_suffix(
                values: &mut Option<super::UriSuffixType>,
                value: super::UriSuffixType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"uriSuffix",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_system_suffix(
                values: &mut Option<super::SystemSuffixType>,
                value: super::SystemSuffixType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"systemSuffix",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_delegate_public(
                values: &mut Option<super::DelegatePublicType>,
                value: super::DelegatePublicType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"delegatePublic",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_delegate_system(
                values: &mut Option<super::DelegateSystemType>,
                value: super::DelegateSystemType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"delegateSystem",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_delegate_uri(
                values: &mut Option<super::DelegateUriType>,
                value: super::DelegateUriType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"delegateURI",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_next_catalog(
                values: &mut Option<super::NextCatalogType>,
                value: super::NextCatalogType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"nextCatalog",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_group(
                values: &mut Option<super::GroupType>,
                value: super::GroupType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"group",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn handle_public<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::PublicType>,
                fallback: Option<<super::PublicType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::PublicType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use CatalogTypeContentDeserializerState as S;
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
                    Self::store_public(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_public(&mut values, data)?;
                        let data = Self::finish_state(helper, S::Public(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::Public(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_system<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::SystemType>,
                fallback: Option<<super::SystemType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::SystemType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use CatalogTypeContentDeserializerState as S;
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
                    Self::store_system(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_system(&mut values, data)?;
                        let data = Self::finish_state(helper, S::System(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::System(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_uri<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::UriType>,
                fallback: Option<<super::UriType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::UriType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use CatalogTypeContentDeserializerState as S;
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
                    Self::store_uri(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_uri(&mut values, data)?;
                        let data = Self::finish_state(helper, S::Uri(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::Uri(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_rewrite_system<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::RewriteSystemType>,
                fallback: Option<<super::RewriteSystemType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::RewriteSystemType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use CatalogTypeContentDeserializerState as S;
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
                    Self::store_rewrite_system(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_rewrite_system(&mut values, data)?;
                        let data =
                            Self::finish_state(helper, S::RewriteSystem(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::RewriteSystem(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_rewrite_uri<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::RewriteUriType>,
                fallback: Option<<super::RewriteUriType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::RewriteUriType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use CatalogTypeContentDeserializerState as S;
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
                    Self::store_rewrite_uri(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_rewrite_uri(&mut values, data)?;
                        let data = Self::finish_state(helper, S::RewriteUri(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::RewriteUri(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_uri_suffix<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::UriSuffixType>,
                fallback: Option<<super::UriSuffixType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::UriSuffixType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use CatalogTypeContentDeserializerState as S;
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
                    Self::store_uri_suffix(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_uri_suffix(&mut values, data)?;
                        let data = Self::finish_state(helper, S::UriSuffix(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::UriSuffix(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_system_suffix<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::SystemSuffixType>,
                fallback: Option<<super::SystemSuffixType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::SystemSuffixType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use CatalogTypeContentDeserializerState as S;
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
                    Self::store_system_suffix(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_system_suffix(&mut values, data)?;
                        let data = Self::finish_state(helper, S::SystemSuffix(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::SystemSuffix(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_delegate_public<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::DelegatePublicType>,
                fallback: Option<<super::DelegatePublicType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::DelegatePublicType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use CatalogTypeContentDeserializerState as S;
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
                    Self::store_delegate_public(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_delegate_public(&mut values, data)?;
                        let data =
                            Self::finish_state(helper, S::DelegatePublic(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::DelegatePublic(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_delegate_system<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::DelegateSystemType>,
                fallback: Option<<super::DelegateSystemType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::DelegateSystemType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use CatalogTypeContentDeserializerState as S;
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
                    Self::store_delegate_system(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_delegate_system(&mut values, data)?;
                        let data =
                            Self::finish_state(helper, S::DelegateSystem(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::DelegateSystem(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_delegate_uri<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::DelegateUriType>,
                fallback: Option<<super::DelegateUriType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::DelegateUriType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use CatalogTypeContentDeserializerState as S;
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
                    Self::store_delegate_uri(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_delegate_uri(&mut values, data)?;
                        let data = Self::finish_state(helper, S::DelegateUri(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::DelegateUri(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_next_catalog<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::NextCatalogType>,
                fallback: Option<<super::NextCatalogType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::NextCatalogType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use CatalogTypeContentDeserializerState as S;
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
                    Self::store_next_catalog(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_next_catalog(&mut values, data)?;
                        let data = Self::finish_state(helper, S::NextCatalog(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::NextCatalog(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_group<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::GroupType>,
                fallback: Option<<super::GroupType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::GroupType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use CatalogTypeContentDeserializerState as S;
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
                    Self::store_group(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_group(&mut values, data)?;
                        let data = Self::finish_state(helper, S::Group(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::Group(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
        }
        impl<'de> Deserializer<'de, super::CatalogTypeContent> for CatalogTypeContentDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::CatalogTypeContent> {
                let deserializer = Self {
                    state__: Box::new(CatalogTypeContentDeserializerState::Init__),
                };
                let mut output = deserializer.next(helper, event)?;
                output.artifact = match output.artifact {
                    DeserializerArtifact::Deserializer(x)
                        if matches!(&*x.state__, CatalogTypeContentDeserializerState::Init__) =>
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
            ) -> DeserializerResult<'de, super::CatalogTypeContent> {
                use CatalogTypeContentDeserializerState as S;
                let mut event = event;
                let (event, allow_any) = loop {
                    let state = replace(&mut *self.state__, S::Unknown__);
                    event = match (state, event) {
                        (S::Unknown__, _) => unreachable!(),
                        (S::Public(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_public(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::System(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_system(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::Uri(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_uri(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::RewriteSystem(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_rewrite_system(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::RewriteUri(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_rewrite_uri(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::UriSuffix(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_uri_suffix(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::SystemSuffix(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_system_suffix(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::DelegatePublic(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_delegate_public(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::DelegateSystem(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_delegate_system(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::DelegateUri(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_delegate_uri(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::NextCatalog(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_next_catalog(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::Group(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_group(helper, values, fallback, output)? {
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
                            S::Public(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"public",
                                false,
                            )?;
                            match self.handle_public(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::System(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"system",
                                false,
                            )?;
                            match self.handle_system(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::Uri(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"uri",
                                false,
                            )?;
                            match self.handle_uri(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::RewriteSystem(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"rewriteSystem",
                                false,
                            )?;
                            match self.handle_rewrite_system(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::RewriteUri(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"rewriteURI",
                                false,
                            )?;
                            match self.handle_rewrite_uri(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::UriSuffix(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"uriSuffix",
                                false,
                            )?;
                            match self.handle_uri_suffix(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::SystemSuffix(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"systemSuffix",
                                false,
                            )?;
                            match self.handle_system_suffix(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::DelegatePublic(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"delegatePublic",
                                false,
                            )?;
                            match self.handle_delegate_public(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::DelegateSystem(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"delegateSystem",
                                false,
                            )?;
                            match self.handle_delegate_system(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::DelegateUri(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"delegateURI",
                                false,
                            )?;
                            match self.handle_delegate_uri(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::NextCatalog(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"nextCatalog",
                                false,
                            )?;
                            match self.handle_next_catalog(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::Group(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"group",
                                true,
                            )?;
                            match self.handle_group(helper, values, fallback, output)? {
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
            ) -> Result<super::CatalogTypeContent, Error> {
                Self::finish_state(helper, *self.state__)
            }
        }
        #[derive(Debug)]
        pub struct DelegatePublicTypeDeserializer {
            public_id_start_string: String,
            catalog: String,
            id: Option<String>,
            state__: Box<DelegatePublicTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum DelegatePublicTypeDeserializerState {
            Init__,
            Unknown__,
        }
        impl DelegatePublicTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                let mut public_id_start_string: Option<String> = None;
                let mut catalog: Option<String> = None;
                let mut id: Option<String> = None;
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"publicIdStartString")
                    ) {
                        helper.read_attrib(
                            &mut public_id_start_string,
                            b"publicIdStartString",
                            &attrib.value,
                        )?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"catalog")
                    ) {
                        helper.read_attrib(&mut catalog, b"catalog", &attrib.value)?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"id")
                    ) {
                        helper.read_attrib(&mut id, b"id", &attrib.value)?;
                    }
                }
                Ok(Self {
                    public_id_start_string: public_id_start_string
                        .ok_or_else(|| ErrorKind::MissingAttribute("publicIdStartString".into()))?,
                    catalog: catalog
                        .ok_or_else(|| ErrorKind::MissingAttribute("catalog".into()))?,
                    id: id,
                    state__: Box::new(DelegatePublicTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: DelegatePublicTypeDeserializerState,
            ) -> Result<(), Error> {
                Ok(())
            }
        }
        impl<'de> Deserializer<'de, super::DelegatePublicType> for DelegatePublicTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::DelegatePublicType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::DelegatePublicType> {
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
            fn finish(
                mut self,
                helper: &mut DeserializeHelper,
            ) -> Result<super::DelegatePublicType, Error> {
                let state = replace(
                    &mut *self.state__,
                    DelegatePublicTypeDeserializerState::Unknown__,
                );
                self.finish_state(helper, state)?;
                Ok(super::DelegatePublicType {
                    public_id_start_string: self.public_id_start_string,
                    catalog: self.catalog,
                    id: self.id,
                })
            }
        }
        #[derive(Debug)]
        pub struct DelegateSystemTypeDeserializer {
            system_id_start_string: String,
            catalog: String,
            id: Option<String>,
            state__: Box<DelegateSystemTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum DelegateSystemTypeDeserializerState {
            Init__,
            Unknown__,
        }
        impl DelegateSystemTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                let mut system_id_start_string: Option<String> = None;
                let mut catalog: Option<String> = None;
                let mut id: Option<String> = None;
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"systemIdStartString")
                    ) {
                        helper.read_attrib(
                            &mut system_id_start_string,
                            b"systemIdStartString",
                            &attrib.value,
                        )?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"catalog")
                    ) {
                        helper.read_attrib(&mut catalog, b"catalog", &attrib.value)?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"id")
                    ) {
                        helper.read_attrib(&mut id, b"id", &attrib.value)?;
                    }
                }
                Ok(Self {
                    system_id_start_string: system_id_start_string
                        .ok_or_else(|| ErrorKind::MissingAttribute("systemIdStartString".into()))?,
                    catalog: catalog
                        .ok_or_else(|| ErrorKind::MissingAttribute("catalog".into()))?,
                    id: id,
                    state__: Box::new(DelegateSystemTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: DelegateSystemTypeDeserializerState,
            ) -> Result<(), Error> {
                Ok(())
            }
        }
        impl<'de> Deserializer<'de, super::DelegateSystemType> for DelegateSystemTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::DelegateSystemType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::DelegateSystemType> {
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
            fn finish(
                mut self,
                helper: &mut DeserializeHelper,
            ) -> Result<super::DelegateSystemType, Error> {
                let state = replace(
                    &mut *self.state__,
                    DelegateSystemTypeDeserializerState::Unknown__,
                );
                self.finish_state(helper, state)?;
                Ok(super::DelegateSystemType {
                    system_id_start_string: self.system_id_start_string,
                    catalog: self.catalog,
                    id: self.id,
                })
            }
        }
        #[derive(Debug)]
        pub struct DelegateUriTypeDeserializer {
            uri_start_string: String,
            catalog: String,
            id: Option<String>,
            state__: Box<DelegateUriTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum DelegateUriTypeDeserializerState {
            Init__,
            Unknown__,
        }
        impl DelegateUriTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                let mut uri_start_string: Option<String> = None;
                let mut catalog: Option<String> = None;
                let mut id: Option<String> = None;
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"uriStartString")
                    ) {
                        helper.read_attrib(
                            &mut uri_start_string,
                            b"uriStartString",
                            &attrib.value,
                        )?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"catalog")
                    ) {
                        helper.read_attrib(&mut catalog, b"catalog", &attrib.value)?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"id")
                    ) {
                        helper.read_attrib(&mut id, b"id", &attrib.value)?;
                    }
                }
                Ok(Self {
                    uri_start_string: uri_start_string
                        .ok_or_else(|| ErrorKind::MissingAttribute("uriStartString".into()))?,
                    catalog: catalog
                        .ok_or_else(|| ErrorKind::MissingAttribute("catalog".into()))?,
                    id: id,
                    state__: Box::new(DelegateUriTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: DelegateUriTypeDeserializerState,
            ) -> Result<(), Error> {
                Ok(())
            }
        }
        impl<'de> Deserializer<'de, super::DelegateUriType> for DelegateUriTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::DelegateUriType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::DelegateUriType> {
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
            fn finish(
                mut self,
                helper: &mut DeserializeHelper,
            ) -> Result<super::DelegateUriType, Error> {
                let state = replace(
                    &mut *self.state__,
                    DelegateUriTypeDeserializerState::Unknown__,
                );
                self.finish_state(helper, state)?;
                Ok(super::DelegateUriType {
                    uri_start_string: self.uri_start_string,
                    catalog: self.catalog,
                    id: self.id,
                })
            }
        }
        #[derive(Debug)]
        pub struct GroupTypeDeserializer {
            prefer: Option<super::SystemOrPublicType>,
            id: Option<String>,
            content: Vec<super::GroupTypeContent>,
            state__: Box<GroupTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum GroupTypeDeserializerState {
            Init__,
            Next__,
            Content__(<super::GroupTypeContent as WithDeserializer>::Deserializer),
            Unknown__,
        }
        impl GroupTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                let mut prefer: Option<super::SystemOrPublicType> = None;
                let mut id: Option<String> = None;
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"prefer")
                    ) {
                        helper.read_attrib(&mut prefer, b"prefer", &attrib.value)?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"id")
                    ) {
                        helper.read_attrib(&mut id, b"id", &attrib.value)?;
                    }
                }
                Ok(Self {
                    prefer: prefer,
                    id: id,
                    content: Vec::new(),
                    state__: Box::new(GroupTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: GroupTypeDeserializerState,
            ) -> Result<(), Error> {
                if let GroupTypeDeserializerState::Content__(deserializer) = state {
                    self.store_content(deserializer.finish(helper)?)?;
                }
                Ok(())
            }
            fn store_content(&mut self, value: super::GroupTypeContent) -> Result<(), Error> {
                self.content.push(value);
                Ok(())
            }
            fn handle_content<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, super::GroupTypeContent>,
                fallback: &mut Option<GroupTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use GroupTypeDeserializerState as S;
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
        impl<'de> Deserializer<'de, super::GroupType> for GroupTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::GroupType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::GroupType> {
                use GroupTypeDeserializerState as S;
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
                            let output =
                                <super::GroupTypeContent as WithDeserializer>::init(helper, event)?;
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
            fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::GroupType, Error> {
                let state = replace(&mut *self.state__, GroupTypeDeserializerState::Unknown__);
                self.finish_state(helper, state)?;
                Ok(super::GroupType {
                    prefer: self.prefer,
                    id: self.id,
                    content: helper.finish_vec(1usize, None, self.content)?,
                })
            }
        }
        #[derive(Debug)]
        pub struct GroupTypeContentDeserializer {
            state__: Box<GroupTypeContentDeserializerState>,
        }
        #[derive(Debug)]
        pub enum GroupTypeContentDeserializerState {
            Init__,
            Public(
                Option<super::PublicType>,
                Option<<super::PublicType as WithDeserializer>::Deserializer>,
                Option<<super::PublicType as WithDeserializer>::Deserializer>,
            ),
            System(
                Option<super::SystemType>,
                Option<<super::SystemType as WithDeserializer>::Deserializer>,
                Option<<super::SystemType as WithDeserializer>::Deserializer>,
            ),
            Uri(
                Option<super::UriType>,
                Option<<super::UriType as WithDeserializer>::Deserializer>,
                Option<<super::UriType as WithDeserializer>::Deserializer>,
            ),
            RewriteSystem(
                Option<super::RewriteSystemType>,
                Option<<super::RewriteSystemType as WithDeserializer>::Deserializer>,
                Option<<super::RewriteSystemType as WithDeserializer>::Deserializer>,
            ),
            RewriteUri(
                Option<super::RewriteUriType>,
                Option<<super::RewriteUriType as WithDeserializer>::Deserializer>,
                Option<<super::RewriteUriType as WithDeserializer>::Deserializer>,
            ),
            UriSuffix(
                Option<super::UriSuffixType>,
                Option<<super::UriSuffixType as WithDeserializer>::Deserializer>,
                Option<<super::UriSuffixType as WithDeserializer>::Deserializer>,
            ),
            SystemSuffix(
                Option<super::SystemSuffixType>,
                Option<<super::SystemSuffixType as WithDeserializer>::Deserializer>,
                Option<<super::SystemSuffixType as WithDeserializer>::Deserializer>,
            ),
            DelegatePublic(
                Option<super::DelegatePublicType>,
                Option<<super::DelegatePublicType as WithDeserializer>::Deserializer>,
                Option<<super::DelegatePublicType as WithDeserializer>::Deserializer>,
            ),
            DelegateSystem(
                Option<super::DelegateSystemType>,
                Option<<super::DelegateSystemType as WithDeserializer>::Deserializer>,
                Option<<super::DelegateSystemType as WithDeserializer>::Deserializer>,
            ),
            DelegateUri(
                Option<super::DelegateUriType>,
                Option<<super::DelegateUriType as WithDeserializer>::Deserializer>,
                Option<<super::DelegateUriType as WithDeserializer>::Deserializer>,
            ),
            NextCatalog(
                Option<super::NextCatalogType>,
                Option<<super::NextCatalogType as WithDeserializer>::Deserializer>,
                Option<<super::NextCatalogType as WithDeserializer>::Deserializer>,
            ),
            Done__(super::GroupTypeContent),
            Unknown__,
        }
        impl GroupTypeContentDeserializer {
            fn find_suitable<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                if let Event::Start(x) | Event::Empty(x) = &event {
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"public")
                    ) {
                        let output = <super::PublicType as WithDeserializer>::init(helper, event)?;
                        return self.handle_public(helper, Default::default(), None, output);
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"system")
                    ) {
                        let output = <super::SystemType as WithDeserializer>::init(helper, event)?;
                        return self.handle_system(helper, Default::default(), None, output);
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"uri")
                    ) {
                        let output = <super::UriType as WithDeserializer>::init(helper, event)?;
                        return self.handle_uri(helper, Default::default(), None, output);
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"rewriteSystem")
                    ) {
                        let output =
                            <super::RewriteSystemType as WithDeserializer>::init(helper, event)?;
                        return self.handle_rewrite_system(
                            helper,
                            Default::default(),
                            None,
                            output,
                        );
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"rewriteURI")
                    ) {
                        let output =
                            <super::RewriteUriType as WithDeserializer>::init(helper, event)?;
                        return self.handle_rewrite_uri(helper, Default::default(), None, output);
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"uriSuffix")
                    ) {
                        let output =
                            <super::UriSuffixType as WithDeserializer>::init(helper, event)?;
                        return self.handle_uri_suffix(helper, Default::default(), None, output);
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"systemSuffix")
                    ) {
                        let output =
                            <super::SystemSuffixType as WithDeserializer>::init(helper, event)?;
                        return self.handle_system_suffix(helper, Default::default(), None, output);
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"delegatePublic")
                    ) {
                        let output =
                            <super::DelegatePublicType as WithDeserializer>::init(helper, event)?;
                        return self.handle_delegate_public(
                            helper,
                            Default::default(),
                            None,
                            output,
                        );
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"delegateSystem")
                    ) {
                        let output =
                            <super::DelegateSystemType as WithDeserializer>::init(helper, event)?;
                        return self.handle_delegate_system(
                            helper,
                            Default::default(),
                            None,
                            output,
                        );
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"delegateURI")
                    ) {
                        let output =
                            <super::DelegateUriType as WithDeserializer>::init(helper, event)?;
                        return self.handle_delegate_uri(helper, Default::default(), None, output);
                    }
                    if matches!(
                        helper.resolve_local_name(x.name(), &super::super::NS_ER),
                        Some(b"nextCatalog")
                    ) {
                        let output =
                            <super::NextCatalogType as WithDeserializer>::init(helper, event)?;
                        return self.handle_next_catalog(helper, Default::default(), None, output);
                    }
                }
                *self.state__ = GroupTypeContentDeserializerState::Init__;
                Ok(ElementHandlerOutput::return_to_parent(event, true))
            }
            fn finish_state(
                helper: &mut DeserializeHelper,
                state: GroupTypeContentDeserializerState,
            ) -> Result<super::GroupTypeContent, Error> {
                use GroupTypeContentDeserializerState as S;
                match state {
                    S::Init__ => Err(ErrorKind::MissingContent.into()),
                    S::Public(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_public(&mut values, value)?;
                        }
                        Ok(super::GroupTypeContent::Public(
                            helper.finish_element("public", values)?,
                        ))
                    }
                    S::System(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_system(&mut values, value)?;
                        }
                        Ok(super::GroupTypeContent::System(
                            helper.finish_element("system", values)?,
                        ))
                    }
                    S::Uri(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_uri(&mut values, value)?;
                        }
                        Ok(super::GroupTypeContent::Uri(
                            helper.finish_element("uri", values)?,
                        ))
                    }
                    S::RewriteSystem(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_rewrite_system(&mut values, value)?;
                        }
                        Ok(super::GroupTypeContent::RewriteSystem(
                            helper.finish_element("rewriteSystem", values)?,
                        ))
                    }
                    S::RewriteUri(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_rewrite_uri(&mut values, value)?;
                        }
                        Ok(super::GroupTypeContent::RewriteUri(
                            helper.finish_element("rewriteURI", values)?,
                        ))
                    }
                    S::UriSuffix(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_uri_suffix(&mut values, value)?;
                        }
                        Ok(super::GroupTypeContent::UriSuffix(
                            helper.finish_element("uriSuffix", values)?,
                        ))
                    }
                    S::SystemSuffix(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_system_suffix(&mut values, value)?;
                        }
                        Ok(super::GroupTypeContent::SystemSuffix(
                            helper.finish_element("systemSuffix", values)?,
                        ))
                    }
                    S::DelegatePublic(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_delegate_public(&mut values, value)?;
                        }
                        Ok(super::GroupTypeContent::DelegatePublic(
                            helper.finish_element("delegatePublic", values)?,
                        ))
                    }
                    S::DelegateSystem(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_delegate_system(&mut values, value)?;
                        }
                        Ok(super::GroupTypeContent::DelegateSystem(
                            helper.finish_element("delegateSystem", values)?,
                        ))
                    }
                    S::DelegateUri(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_delegate_uri(&mut values, value)?;
                        }
                        Ok(super::GroupTypeContent::DelegateUri(
                            helper.finish_element("delegateURI", values)?,
                        ))
                    }
                    S::NextCatalog(mut values, None, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(helper)?;
                            Self::store_next_catalog(&mut values, value)?;
                        }
                        Ok(super::GroupTypeContent::NextCatalog(
                            helper.finish_element("nextCatalog", values)?,
                        ))
                    }
                    S::Done__(data) => Ok(data),
                    _ => unreachable!(),
                }
            }
            fn store_public(
                values: &mut Option<super::PublicType>,
                value: super::PublicType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"public",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_system(
                values: &mut Option<super::SystemType>,
                value: super::SystemType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"system",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_uri(
                values: &mut Option<super::UriType>,
                value: super::UriType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"uri")))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_rewrite_system(
                values: &mut Option<super::RewriteSystemType>,
                value: super::RewriteSystemType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"rewriteSystem",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_rewrite_uri(
                values: &mut Option<super::RewriteUriType>,
                value: super::RewriteUriType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"rewriteURI",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_uri_suffix(
                values: &mut Option<super::UriSuffixType>,
                value: super::UriSuffixType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"uriSuffix",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_system_suffix(
                values: &mut Option<super::SystemSuffixType>,
                value: super::SystemSuffixType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"systemSuffix",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_delegate_public(
                values: &mut Option<super::DelegatePublicType>,
                value: super::DelegatePublicType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"delegatePublic",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_delegate_system(
                values: &mut Option<super::DelegateSystemType>,
                value: super::DelegateSystemType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"delegateSystem",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_delegate_uri(
                values: &mut Option<super::DelegateUriType>,
                value: super::DelegateUriType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"delegateURI",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_next_catalog(
                values: &mut Option<super::NextCatalogType>,
                value: super::NextCatalogType,
            ) -> Result<(), Error> {
                if values.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"nextCatalog",
                    )))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn handle_public<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::PublicType>,
                fallback: Option<<super::PublicType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::PublicType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use GroupTypeContentDeserializerState as S;
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
                    Self::store_public(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_public(&mut values, data)?;
                        let data = Self::finish_state(helper, S::Public(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::Public(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_system<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::SystemType>,
                fallback: Option<<super::SystemType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::SystemType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use GroupTypeContentDeserializerState as S;
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
                    Self::store_system(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_system(&mut values, data)?;
                        let data = Self::finish_state(helper, S::System(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::System(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_uri<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::UriType>,
                fallback: Option<<super::UriType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::UriType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use GroupTypeContentDeserializerState as S;
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
                    Self::store_uri(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_uri(&mut values, data)?;
                        let data = Self::finish_state(helper, S::Uri(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::Uri(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_rewrite_system<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::RewriteSystemType>,
                fallback: Option<<super::RewriteSystemType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::RewriteSystemType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use GroupTypeContentDeserializerState as S;
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
                    Self::store_rewrite_system(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_rewrite_system(&mut values, data)?;
                        let data =
                            Self::finish_state(helper, S::RewriteSystem(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::RewriteSystem(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_rewrite_uri<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::RewriteUriType>,
                fallback: Option<<super::RewriteUriType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::RewriteUriType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use GroupTypeContentDeserializerState as S;
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
                    Self::store_rewrite_uri(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_rewrite_uri(&mut values, data)?;
                        let data = Self::finish_state(helper, S::RewriteUri(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::RewriteUri(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_uri_suffix<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::UriSuffixType>,
                fallback: Option<<super::UriSuffixType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::UriSuffixType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use GroupTypeContentDeserializerState as S;
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
                    Self::store_uri_suffix(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_uri_suffix(&mut values, data)?;
                        let data = Self::finish_state(helper, S::UriSuffix(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::UriSuffix(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_system_suffix<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::SystemSuffixType>,
                fallback: Option<<super::SystemSuffixType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::SystemSuffixType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use GroupTypeContentDeserializerState as S;
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
                    Self::store_system_suffix(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_system_suffix(&mut values, data)?;
                        let data = Self::finish_state(helper, S::SystemSuffix(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::SystemSuffix(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_delegate_public<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::DelegatePublicType>,
                fallback: Option<<super::DelegatePublicType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::DelegatePublicType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use GroupTypeContentDeserializerState as S;
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
                    Self::store_delegate_public(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_delegate_public(&mut values, data)?;
                        let data =
                            Self::finish_state(helper, S::DelegatePublic(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::DelegatePublic(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_delegate_system<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::DelegateSystemType>,
                fallback: Option<<super::DelegateSystemType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::DelegateSystemType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use GroupTypeContentDeserializerState as S;
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
                    Self::store_delegate_system(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_delegate_system(&mut values, data)?;
                        let data =
                            Self::finish_state(helper, S::DelegateSystem(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::DelegateSystem(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_delegate_uri<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::DelegateUriType>,
                fallback: Option<<super::DelegateUriType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::DelegateUriType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use GroupTypeContentDeserializerState as S;
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
                    Self::store_delegate_uri(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_delegate_uri(&mut values, data)?;
                        let data = Self::finish_state(helper, S::DelegateUri(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::DelegateUri(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
            fn handle_next_catalog<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                mut values: Option<super::NextCatalogType>,
                fallback: Option<<super::NextCatalogType as WithDeserializer>::Deserializer>,
                output: DeserializerOutput<'de, super::NextCatalogType>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use GroupTypeContentDeserializerState as S;
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
                    Self::store_next_catalog(&mut values, data)?;
                }
                match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        Self::store_next_catalog(&mut values, data)?;
                        let data = Self::finish_state(helper, S::NextCatalog(values, None, None))?;
                        *self.state__ = S::Done__(data);
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::NextCatalog(values, None, Some(deserializer));
                        Ok(ElementHandlerOutput::break_(event, allow_any))
                    }
                }
            }
        }
        impl<'de> Deserializer<'de, super::GroupTypeContent> for GroupTypeContentDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::GroupTypeContent> {
                let deserializer = Self {
                    state__: Box::new(GroupTypeContentDeserializerState::Init__),
                };
                let mut output = deserializer.next(helper, event)?;
                output.artifact = match output.artifact {
                    DeserializerArtifact::Deserializer(x)
                        if matches!(&*x.state__, GroupTypeContentDeserializerState::Init__) =>
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
            ) -> DeserializerResult<'de, super::GroupTypeContent> {
                use GroupTypeContentDeserializerState as S;
                let mut event = event;
                let (event, allow_any) = loop {
                    let state = replace(&mut *self.state__, S::Unknown__);
                    event = match (state, event) {
                        (S::Unknown__, _) => unreachable!(),
                        (S::Public(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_public(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::System(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_system(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::Uri(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_uri(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::RewriteSystem(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_rewrite_system(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::RewriteUri(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_rewrite_uri(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::UriSuffix(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_uri_suffix(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::SystemSuffix(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_system_suffix(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::DelegatePublic(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_delegate_public(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::DelegateSystem(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_delegate_system(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::DelegateUri(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_delegate_uri(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (S::NextCatalog(values, fallback, Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_next_catalog(helper, values, fallback, output)? {
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
                            S::Public(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"public",
                                false,
                            )?;
                            match self.handle_public(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::System(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"system",
                                false,
                            )?;
                            match self.handle_system(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::Uri(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"uri",
                                false,
                            )?;
                            match self.handle_uri(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::RewriteSystem(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"rewriteSystem",
                                false,
                            )?;
                            match self.handle_rewrite_system(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::RewriteUri(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"rewriteURI",
                                false,
                            )?;
                            match self.handle_rewrite_uri(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::UriSuffix(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"uriSuffix",
                                false,
                            )?;
                            match self.handle_uri_suffix(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::SystemSuffix(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"systemSuffix",
                                false,
                            )?;
                            match self.handle_system_suffix(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::DelegatePublic(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"delegatePublic",
                                false,
                            )?;
                            match self.handle_delegate_public(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::DelegateSystem(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"delegateSystem",
                                false,
                            )?;
                            match self.handle_delegate_system(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::DelegateUri(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"delegateURI",
                                false,
                            )?;
                            match self.handle_delegate_uri(helper, values, fallback, output)? {
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                                ElementHandlerOutput::Continue { event, .. } => event,
                            }
                        }
                        (
                            S::NextCatalog(values, fallback, None),
                            event @ (Event::Start(_) | Event::Empty(_)),
                        ) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_ER),
                                b"nextCatalog",
                                false,
                            )?;
                            match self.handle_next_catalog(helper, values, fallback, output)? {
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
            ) -> Result<super::GroupTypeContent, Error> {
                Self::finish_state(helper, *self.state__)
            }
        }
        #[derive(Debug)]
        pub struct NextCatalogTypeDeserializer {
            catalog: String,
            id: Option<String>,
            state__: Box<NextCatalogTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum NextCatalogTypeDeserializerState {
            Init__,
            Unknown__,
        }
        impl NextCatalogTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                let mut catalog: Option<String> = None;
                let mut id: Option<String> = None;
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"catalog")
                    ) {
                        helper.read_attrib(&mut catalog, b"catalog", &attrib.value)?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"id")
                    ) {
                        helper.read_attrib(&mut id, b"id", &attrib.value)?;
                    }
                }
                Ok(Self {
                    catalog: catalog
                        .ok_or_else(|| ErrorKind::MissingAttribute("catalog".into()))?,
                    id: id,
                    state__: Box::new(NextCatalogTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: NextCatalogTypeDeserializerState,
            ) -> Result<(), Error> {
                Ok(())
            }
        }
        impl<'de> Deserializer<'de, super::NextCatalogType> for NextCatalogTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::NextCatalogType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::NextCatalogType> {
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
            fn finish(
                mut self,
                helper: &mut DeserializeHelper,
            ) -> Result<super::NextCatalogType, Error> {
                let state = replace(
                    &mut *self.state__,
                    NextCatalogTypeDeserializerState::Unknown__,
                );
                self.finish_state(helper, state)?;
                Ok(super::NextCatalogType {
                    catalog: self.catalog,
                    id: self.id,
                })
            }
        }
        #[derive(Debug)]
        pub struct PublicTypeDeserializer {
            public_id: String,
            uri: String,
            id: Option<String>,
            state__: Box<PublicTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum PublicTypeDeserializerState {
            Init__,
            Unknown__,
        }
        impl PublicTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                let mut public_id: Option<String> = None;
                let mut uri: Option<String> = None;
                let mut id: Option<String> = None;
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"publicId")
                    ) {
                        helper.read_attrib(&mut public_id, b"publicId", &attrib.value)?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"uri")
                    ) {
                        helper.read_attrib(&mut uri, b"uri", &attrib.value)?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"id")
                    ) {
                        helper.read_attrib(&mut id, b"id", &attrib.value)?;
                    }
                }
                Ok(Self {
                    public_id: public_id
                        .ok_or_else(|| ErrorKind::MissingAttribute("publicId".into()))?,
                    uri: uri.ok_or_else(|| ErrorKind::MissingAttribute("uri".into()))?,
                    id: id,
                    state__: Box::new(PublicTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: PublicTypeDeserializerState,
            ) -> Result<(), Error> {
                Ok(())
            }
        }
        impl<'de> Deserializer<'de, super::PublicType> for PublicTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::PublicType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::PublicType> {
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
            fn finish(
                mut self,
                helper: &mut DeserializeHelper,
            ) -> Result<super::PublicType, Error> {
                let state = replace(&mut *self.state__, PublicTypeDeserializerState::Unknown__);
                self.finish_state(helper, state)?;
                Ok(super::PublicType {
                    public_id: self.public_id,
                    uri: self.uri,
                    id: self.id,
                })
            }
        }
        #[derive(Debug)]
        pub struct RewriteSystemTypeDeserializer {
            system_id_start_string: String,
            rewrite_prefix: String,
            id: Option<String>,
            state__: Box<RewriteSystemTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum RewriteSystemTypeDeserializerState {
            Init__,
            Unknown__,
        }
        impl RewriteSystemTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                let mut system_id_start_string: Option<String> = None;
                let mut rewrite_prefix: Option<String> = None;
                let mut id: Option<String> = None;
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"systemIdStartString")
                    ) {
                        helper.read_attrib(
                            &mut system_id_start_string,
                            b"systemIdStartString",
                            &attrib.value,
                        )?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"rewritePrefix")
                    ) {
                        helper.read_attrib(&mut rewrite_prefix, b"rewritePrefix", &attrib.value)?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"id")
                    ) {
                        helper.read_attrib(&mut id, b"id", &attrib.value)?;
                    }
                }
                Ok(Self {
                    system_id_start_string: system_id_start_string
                        .ok_or_else(|| ErrorKind::MissingAttribute("systemIdStartString".into()))?,
                    rewrite_prefix: rewrite_prefix
                        .ok_or_else(|| ErrorKind::MissingAttribute("rewritePrefix".into()))?,
                    id: id,
                    state__: Box::new(RewriteSystemTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: RewriteSystemTypeDeserializerState,
            ) -> Result<(), Error> {
                Ok(())
            }
        }
        impl<'de> Deserializer<'de, super::RewriteSystemType> for RewriteSystemTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::RewriteSystemType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::RewriteSystemType> {
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
            fn finish(
                mut self,
                helper: &mut DeserializeHelper,
            ) -> Result<super::RewriteSystemType, Error> {
                let state = replace(
                    &mut *self.state__,
                    RewriteSystemTypeDeserializerState::Unknown__,
                );
                self.finish_state(helper, state)?;
                Ok(super::RewriteSystemType {
                    system_id_start_string: self.system_id_start_string,
                    rewrite_prefix: self.rewrite_prefix,
                    id: self.id,
                })
            }
        }
        #[derive(Debug)]
        pub struct RewriteUriTypeDeserializer {
            uri_start_string: String,
            rewrite_prefix: String,
            id: Option<String>,
            state__: Box<RewriteUriTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum RewriteUriTypeDeserializerState {
            Init__,
            Unknown__,
        }
        impl RewriteUriTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                let mut uri_start_string: Option<String> = None;
                let mut rewrite_prefix: Option<String> = None;
                let mut id: Option<String> = None;
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"uriStartString")
                    ) {
                        helper.read_attrib(
                            &mut uri_start_string,
                            b"uriStartString",
                            &attrib.value,
                        )?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"rewritePrefix")
                    ) {
                        helper.read_attrib(&mut rewrite_prefix, b"rewritePrefix", &attrib.value)?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"id")
                    ) {
                        helper.read_attrib(&mut id, b"id", &attrib.value)?;
                    }
                }
                Ok(Self {
                    uri_start_string: uri_start_string
                        .ok_or_else(|| ErrorKind::MissingAttribute("uriStartString".into()))?,
                    rewrite_prefix: rewrite_prefix
                        .ok_or_else(|| ErrorKind::MissingAttribute("rewritePrefix".into()))?,
                    id: id,
                    state__: Box::new(RewriteUriTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: RewriteUriTypeDeserializerState,
            ) -> Result<(), Error> {
                Ok(())
            }
        }
        impl<'de> Deserializer<'de, super::RewriteUriType> for RewriteUriTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::RewriteUriType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::RewriteUriType> {
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
            fn finish(
                mut self,
                helper: &mut DeserializeHelper,
            ) -> Result<super::RewriteUriType, Error> {
                let state = replace(
                    &mut *self.state__,
                    RewriteUriTypeDeserializerState::Unknown__,
                );
                self.finish_state(helper, state)?;
                Ok(super::RewriteUriType {
                    uri_start_string: self.uri_start_string,
                    rewrite_prefix: self.rewrite_prefix,
                    id: self.id,
                })
            }
        }
        #[derive(Debug)]
        pub struct SystemTypeDeserializer {
            system_id: String,
            uri: String,
            id: Option<String>,
            state__: Box<SystemTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum SystemTypeDeserializerState {
            Init__,
            Unknown__,
        }
        impl SystemTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                let mut system_id: Option<String> = None;
                let mut uri: Option<String> = None;
                let mut id: Option<String> = None;
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"systemId")
                    ) {
                        helper.read_attrib(&mut system_id, b"systemId", &attrib.value)?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"uri")
                    ) {
                        helper.read_attrib(&mut uri, b"uri", &attrib.value)?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"id")
                    ) {
                        helper.read_attrib(&mut id, b"id", &attrib.value)?;
                    }
                }
                Ok(Self {
                    system_id: system_id
                        .ok_or_else(|| ErrorKind::MissingAttribute("systemId".into()))?,
                    uri: uri.ok_or_else(|| ErrorKind::MissingAttribute("uri".into()))?,
                    id: id,
                    state__: Box::new(SystemTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: SystemTypeDeserializerState,
            ) -> Result<(), Error> {
                Ok(())
            }
        }
        impl<'de> Deserializer<'de, super::SystemType> for SystemTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::SystemType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::SystemType> {
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
            fn finish(
                mut self,
                helper: &mut DeserializeHelper,
            ) -> Result<super::SystemType, Error> {
                let state = replace(&mut *self.state__, SystemTypeDeserializerState::Unknown__);
                self.finish_state(helper, state)?;
                Ok(super::SystemType {
                    system_id: self.system_id,
                    uri: self.uri,
                    id: self.id,
                })
            }
        }
        #[derive(Debug)]
        pub struct SystemSuffixTypeDeserializer {
            system_id_suffix: String,
            uri: String,
            id: Option<String>,
            state__: Box<SystemSuffixTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum SystemSuffixTypeDeserializerState {
            Init__,
            Unknown__,
        }
        impl SystemSuffixTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                let mut system_id_suffix: Option<String> = None;
                let mut uri: Option<String> = None;
                let mut id: Option<String> = None;
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"systemIdSuffix")
                    ) {
                        helper.read_attrib(
                            &mut system_id_suffix,
                            b"systemIdSuffix",
                            &attrib.value,
                        )?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"uri")
                    ) {
                        helper.read_attrib(&mut uri, b"uri", &attrib.value)?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"id")
                    ) {
                        helper.read_attrib(&mut id, b"id", &attrib.value)?;
                    }
                }
                Ok(Self {
                    system_id_suffix: system_id_suffix
                        .ok_or_else(|| ErrorKind::MissingAttribute("systemIdSuffix".into()))?,
                    uri: uri.ok_or_else(|| ErrorKind::MissingAttribute("uri".into()))?,
                    id: id,
                    state__: Box::new(SystemSuffixTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: SystemSuffixTypeDeserializerState,
            ) -> Result<(), Error> {
                Ok(())
            }
        }
        impl<'de> Deserializer<'de, super::SystemSuffixType> for SystemSuffixTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::SystemSuffixType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::SystemSuffixType> {
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
            fn finish(
                mut self,
                helper: &mut DeserializeHelper,
            ) -> Result<super::SystemSuffixType, Error> {
                let state = replace(
                    &mut *self.state__,
                    SystemSuffixTypeDeserializerState::Unknown__,
                );
                self.finish_state(helper, state)?;
                Ok(super::SystemSuffixType {
                    system_id_suffix: self.system_id_suffix,
                    uri: self.uri,
                    id: self.id,
                })
            }
        }
        #[derive(Debug)]
        pub struct UriTypeDeserializer {
            name: String,
            uri: String,
            id: Option<String>,
            state__: Box<UriTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum UriTypeDeserializerState {
            Init__,
            Unknown__,
        }
        impl UriTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                let mut name: Option<String> = None;
                let mut uri: Option<String> = None;
                let mut id: Option<String> = None;
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"name")
                    ) {
                        helper.read_attrib(&mut name, b"name", &attrib.value)?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"uri")
                    ) {
                        helper.read_attrib(&mut uri, b"uri", &attrib.value)?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"id")
                    ) {
                        helper.read_attrib(&mut id, b"id", &attrib.value)?;
                    }
                }
                Ok(Self {
                    name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                    uri: uri.ok_or_else(|| ErrorKind::MissingAttribute("uri".into()))?,
                    id: id,
                    state__: Box::new(UriTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: UriTypeDeserializerState,
            ) -> Result<(), Error> {
                Ok(())
            }
        }
        impl<'de> Deserializer<'de, super::UriType> for UriTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::UriType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::UriType> {
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
            fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::UriType, Error> {
                let state = replace(&mut *self.state__, UriTypeDeserializerState::Unknown__);
                self.finish_state(helper, state)?;
                Ok(super::UriType {
                    name: self.name,
                    uri: self.uri,
                    id: self.id,
                })
            }
        }
        #[derive(Debug)]
        pub struct UriSuffixTypeDeserializer {
            uri_suffix: String,
            uri: String,
            id: Option<String>,
            state__: Box<UriSuffixTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum UriSuffixTypeDeserializerState {
            Init__,
            Unknown__,
        }
        impl UriSuffixTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                let mut uri_suffix: Option<String> = None;
                let mut uri: Option<String> = None;
                let mut id: Option<String> = None;
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"uriSuffix")
                    ) {
                        helper.read_attrib(&mut uri_suffix, b"uriSuffix", &attrib.value)?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"uri")
                    ) {
                        helper.read_attrib(&mut uri, b"uri", &attrib.value)?;
                    } else if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_ER),
                        Some(b"id")
                    ) {
                        helper.read_attrib(&mut id, b"id", &attrib.value)?;
                    }
                }
                Ok(Self {
                    uri_suffix: uri_suffix
                        .ok_or_else(|| ErrorKind::MissingAttribute("uriSuffix".into()))?,
                    uri: uri.ok_or_else(|| ErrorKind::MissingAttribute("uri".into()))?,
                    id: id,
                    state__: Box::new(UriSuffixTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: UriSuffixTypeDeserializerState,
            ) -> Result<(), Error> {
                Ok(())
            }
        }
        impl<'de> Deserializer<'de, super::UriSuffixType> for UriSuffixTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::UriSuffixType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::UriSuffixType> {
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
            fn finish(
                mut self,
                helper: &mut DeserializeHelper,
            ) -> Result<super::UriSuffixType, Error> {
                let state = replace(
                    &mut *self.state__,
                    UriSuffixTypeDeserializerState::Unknown__,
                );
                self.finish_state(helper, state)?;
                Ok(super::UriSuffixType {
                    uri_suffix: self.uri_suffix,
                    uri: self.uri,
                    id: self.id,
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
            Content__(
                IterSerializer<'ser, &'ser [super::CatalogTypeContent], super::CatalogTypeContent>,
            ),
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
                            *self.state = CatalogTypeSerializerState::Content__(
                                IterSerializer::new(&self.value.content[..], None, false),
                            );
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            if self.is_root {
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_ER),
                                    &super::super::NS_ER,
                                );
                            }
                            helper.write_attrib_opt(&mut bytes, "id", &self.value.id)?;
                            helper.write_attrib_opt(&mut bytes, "prefer", &self.value.prefer)?;
                            return Ok(Some(Event::Start(bytes)));
                        }
                        CatalogTypeSerializerState::Content__(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = CatalogTypeSerializerState::End__,
                            }
                        }
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
        pub struct CatalogTypeContentSerializer<'ser> {
            pub(super) value: &'ser super::CatalogTypeContent,
            pub(super) state: Box<CatalogTypeContentSerializerState<'ser>>,
        }
        #[derive(Debug)]
        pub(super) enum CatalogTypeContentSerializerState<'ser> {
            Init__,
            Public(<super::PublicType as WithSerializer>::Serializer<'ser>),
            System(<super::SystemType as WithSerializer>::Serializer<'ser>),
            Uri(<super::UriType as WithSerializer>::Serializer<'ser>),
            RewriteSystem(<super::RewriteSystemType as WithSerializer>::Serializer<'ser>),
            RewriteUri(<super::RewriteUriType as WithSerializer>::Serializer<'ser>),
            UriSuffix(<super::UriSuffixType as WithSerializer>::Serializer<'ser>),
            SystemSuffix(<super::SystemSuffixType as WithSerializer>::Serializer<'ser>),
            DelegatePublic(<super::DelegatePublicType as WithSerializer>::Serializer<'ser>),
            DelegateSystem(<super::DelegateSystemType as WithSerializer>::Serializer<'ser>),
            DelegateUri(<super::DelegateUriType as WithSerializer>::Serializer<'ser>),
            NextCatalog(<super::NextCatalogType as WithSerializer>::Serializer<'ser>),
            Group(<super::GroupType as WithSerializer>::Serializer<'ser>),
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> CatalogTypeContentSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        CatalogTypeContentSerializerState::Init__ => match self.value {
                            super::CatalogTypeContent::Public(x) => {
                                *self.state = CatalogTypeContentSerializerState::Public(
                                    WithSerializer::serializer(x, Some("er:public"), false)?,
                                )
                            }
                            super::CatalogTypeContent::System(x) => {
                                *self.state = CatalogTypeContentSerializerState::System(
                                    WithSerializer::serializer(x, Some("er:system"), false)?,
                                )
                            }
                            super::CatalogTypeContent::Uri(x) => {
                                *self.state = CatalogTypeContentSerializerState::Uri(
                                    WithSerializer::serializer(x, Some("er:uri"), false)?,
                                )
                            }
                            super::CatalogTypeContent::RewriteSystem(x) => {
                                *self.state = CatalogTypeContentSerializerState::RewriteSystem(
                                    WithSerializer::serializer(x, Some("er:rewriteSystem"), false)?,
                                )
                            }
                            super::CatalogTypeContent::RewriteUri(x) => {
                                *self.state = CatalogTypeContentSerializerState::RewriteUri(
                                    WithSerializer::serializer(x, Some("er:rewriteURI"), false)?,
                                )
                            }
                            super::CatalogTypeContent::UriSuffix(x) => {
                                *self.state = CatalogTypeContentSerializerState::UriSuffix(
                                    WithSerializer::serializer(x, Some("er:uriSuffix"), false)?,
                                )
                            }
                            super::CatalogTypeContent::SystemSuffix(x) => {
                                *self.state = CatalogTypeContentSerializerState::SystemSuffix(
                                    WithSerializer::serializer(x, Some("er:systemSuffix"), false)?,
                                )
                            }
                            super::CatalogTypeContent::DelegatePublic(x) => {
                                *self.state = CatalogTypeContentSerializerState::DelegatePublic(
                                    WithSerializer::serializer(
                                        x,
                                        Some("er:delegatePublic"),
                                        false,
                                    )?,
                                )
                            }
                            super::CatalogTypeContent::DelegateSystem(x) => {
                                *self.state = CatalogTypeContentSerializerState::DelegateSystem(
                                    WithSerializer::serializer(
                                        x,
                                        Some("er:delegateSystem"),
                                        false,
                                    )?,
                                )
                            }
                            super::CatalogTypeContent::DelegateUri(x) => {
                                *self.state = CatalogTypeContentSerializerState::DelegateUri(
                                    WithSerializer::serializer(x, Some("er:delegateURI"), false)?,
                                )
                            }
                            super::CatalogTypeContent::NextCatalog(x) => {
                                *self.state = CatalogTypeContentSerializerState::NextCatalog(
                                    WithSerializer::serializer(x, Some("er:nextCatalog"), false)?,
                                )
                            }
                            super::CatalogTypeContent::Group(x) => {
                                *self.state = CatalogTypeContentSerializerState::Group(
                                    WithSerializer::serializer(x, Some("er:group"), false)?,
                                )
                            }
                        },
                        CatalogTypeContentSerializerState::Public(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = CatalogTypeContentSerializerState::Done__,
                            }
                        }
                        CatalogTypeContentSerializerState::System(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = CatalogTypeContentSerializerState::Done__,
                            }
                        }
                        CatalogTypeContentSerializerState::Uri(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = CatalogTypeContentSerializerState::Done__,
                            }
                        }
                        CatalogTypeContentSerializerState::RewriteSystem(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = CatalogTypeContentSerializerState::Done__,
                            }
                        }
                        CatalogTypeContentSerializerState::RewriteUri(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = CatalogTypeContentSerializerState::Done__,
                            }
                        }
                        CatalogTypeContentSerializerState::UriSuffix(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = CatalogTypeContentSerializerState::Done__,
                            }
                        }
                        CatalogTypeContentSerializerState::SystemSuffix(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = CatalogTypeContentSerializerState::Done__,
                            }
                        }
                        CatalogTypeContentSerializerState::DelegatePublic(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = CatalogTypeContentSerializerState::Done__,
                            }
                        }
                        CatalogTypeContentSerializerState::DelegateSystem(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = CatalogTypeContentSerializerState::Done__,
                            }
                        }
                        CatalogTypeContentSerializerState::DelegateUri(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = CatalogTypeContentSerializerState::Done__,
                            }
                        }
                        CatalogTypeContentSerializerState::NextCatalog(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = CatalogTypeContentSerializerState::Done__,
                            }
                        }
                        CatalogTypeContentSerializerState::Group(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = CatalogTypeContentSerializerState::Done__,
                            }
                        }
                        CatalogTypeContentSerializerState::Done__ => return Ok(None),
                        CatalogTypeContentSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for CatalogTypeContentSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = CatalogTypeContentSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct DelegatePublicTypeSerializer<'ser> {
            pub(super) value: &'ser super::DelegatePublicType,
            pub(super) state: Box<DelegatePublicTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum DelegatePublicTypeSerializerState<'ser> {
            Init__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> DelegatePublicTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        DelegatePublicTypeSerializerState::Init__ => {
                            *self.state = DelegatePublicTypeSerializerState::Done__;
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            if self.is_root {
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_ER),
                                    &super::super::NS_ER,
                                );
                            }
                            helper.write_attrib(
                                &mut bytes,
                                "publicIdStartString",
                                &self.value.public_id_start_string,
                            )?;
                            helper.write_attrib(&mut bytes, "catalog", &self.value.catalog)?;
                            helper.write_attrib_opt(&mut bytes, "id", &self.value.id)?;
                            helper.end_ns_scope();
                            return Ok(Some(Event::Empty(bytes)));
                        }
                        DelegatePublicTypeSerializerState::Done__ => return Ok(None),
                        DelegatePublicTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for DelegatePublicTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = DelegatePublicTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct DelegateSystemTypeSerializer<'ser> {
            pub(super) value: &'ser super::DelegateSystemType,
            pub(super) state: Box<DelegateSystemTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum DelegateSystemTypeSerializerState<'ser> {
            Init__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> DelegateSystemTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        DelegateSystemTypeSerializerState::Init__ => {
                            *self.state = DelegateSystemTypeSerializerState::Done__;
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            if self.is_root {
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_ER),
                                    &super::super::NS_ER,
                                );
                            }
                            helper.write_attrib(
                                &mut bytes,
                                "systemIdStartString",
                                &self.value.system_id_start_string,
                            )?;
                            helper.write_attrib(&mut bytes, "catalog", &self.value.catalog)?;
                            helper.write_attrib_opt(&mut bytes, "id", &self.value.id)?;
                            helper.end_ns_scope();
                            return Ok(Some(Event::Empty(bytes)));
                        }
                        DelegateSystemTypeSerializerState::Done__ => return Ok(None),
                        DelegateSystemTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for DelegateSystemTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = DelegateSystemTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct DelegateUriTypeSerializer<'ser> {
            pub(super) value: &'ser super::DelegateUriType,
            pub(super) state: Box<DelegateUriTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum DelegateUriTypeSerializerState<'ser> {
            Init__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> DelegateUriTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        DelegateUriTypeSerializerState::Init__ => {
                            *self.state = DelegateUriTypeSerializerState::Done__;
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            if self.is_root {
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_ER),
                                    &super::super::NS_ER,
                                );
                            }
                            helper.write_attrib(
                                &mut bytes,
                                "uriStartString",
                                &self.value.uri_start_string,
                            )?;
                            helper.write_attrib(&mut bytes, "catalog", &self.value.catalog)?;
                            helper.write_attrib_opt(&mut bytes, "id", &self.value.id)?;
                            helper.end_ns_scope();
                            return Ok(Some(Event::Empty(bytes)));
                        }
                        DelegateUriTypeSerializerState::Done__ => return Ok(None),
                        DelegateUriTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for DelegateUriTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = DelegateUriTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct GroupTypeSerializer<'ser> {
            pub(super) value: &'ser super::GroupType,
            pub(super) state: Box<GroupTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum GroupTypeSerializerState<'ser> {
            Init__,
            Content__(
                IterSerializer<'ser, &'ser [super::GroupTypeContent], super::GroupTypeContent>,
            ),
            End__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> GroupTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        GroupTypeSerializerState::Init__ => {
                            *self.state = GroupTypeSerializerState::Content__(IterSerializer::new(
                                &self.value.content[..],
                                None,
                                false,
                            ));
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            if self.is_root {
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_ER),
                                    &super::super::NS_ER,
                                );
                            }
                            helper.write_attrib_opt(&mut bytes, "prefer", &self.value.prefer)?;
                            helper.write_attrib_opt(&mut bytes, "id", &self.value.id)?;
                            return Ok(Some(Event::Start(bytes)));
                        }
                        GroupTypeSerializerState::Content__(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = GroupTypeSerializerState::End__,
                            }
                        }
                        GroupTypeSerializerState::End__ => {
                            *self.state = GroupTypeSerializerState::Done__;
                            helper.end_ns_scope();
                            return Ok(Some(Event::End(BytesEnd::new(self.name))));
                        }
                        GroupTypeSerializerState::Done__ => return Ok(None),
                        GroupTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for GroupTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = GroupTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct GroupTypeContentSerializer<'ser> {
            pub(super) value: &'ser super::GroupTypeContent,
            pub(super) state: Box<GroupTypeContentSerializerState<'ser>>,
        }
        #[derive(Debug)]
        pub(super) enum GroupTypeContentSerializerState<'ser> {
            Init__,
            Public(<super::PublicType as WithSerializer>::Serializer<'ser>),
            System(<super::SystemType as WithSerializer>::Serializer<'ser>),
            Uri(<super::UriType as WithSerializer>::Serializer<'ser>),
            RewriteSystem(<super::RewriteSystemType as WithSerializer>::Serializer<'ser>),
            RewriteUri(<super::RewriteUriType as WithSerializer>::Serializer<'ser>),
            UriSuffix(<super::UriSuffixType as WithSerializer>::Serializer<'ser>),
            SystemSuffix(<super::SystemSuffixType as WithSerializer>::Serializer<'ser>),
            DelegatePublic(<super::DelegatePublicType as WithSerializer>::Serializer<'ser>),
            DelegateSystem(<super::DelegateSystemType as WithSerializer>::Serializer<'ser>),
            DelegateUri(<super::DelegateUriType as WithSerializer>::Serializer<'ser>),
            NextCatalog(<super::NextCatalogType as WithSerializer>::Serializer<'ser>),
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> GroupTypeContentSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        GroupTypeContentSerializerState::Init__ => match self.value {
                            super::GroupTypeContent::Public(x) => {
                                *self.state = GroupTypeContentSerializerState::Public(
                                    WithSerializer::serializer(x, Some("er:public"), false)?,
                                )
                            }
                            super::GroupTypeContent::System(x) => {
                                *self.state = GroupTypeContentSerializerState::System(
                                    WithSerializer::serializer(x, Some("er:system"), false)?,
                                )
                            }
                            super::GroupTypeContent::Uri(x) => {
                                *self.state = GroupTypeContentSerializerState::Uri(
                                    WithSerializer::serializer(x, Some("er:uri"), false)?,
                                )
                            }
                            super::GroupTypeContent::RewriteSystem(x) => {
                                *self.state = GroupTypeContentSerializerState::RewriteSystem(
                                    WithSerializer::serializer(x, Some("er:rewriteSystem"), false)?,
                                )
                            }
                            super::GroupTypeContent::RewriteUri(x) => {
                                *self.state = GroupTypeContentSerializerState::RewriteUri(
                                    WithSerializer::serializer(x, Some("er:rewriteURI"), false)?,
                                )
                            }
                            super::GroupTypeContent::UriSuffix(x) => {
                                *self.state = GroupTypeContentSerializerState::UriSuffix(
                                    WithSerializer::serializer(x, Some("er:uriSuffix"), false)?,
                                )
                            }
                            super::GroupTypeContent::SystemSuffix(x) => {
                                *self.state = GroupTypeContentSerializerState::SystemSuffix(
                                    WithSerializer::serializer(x, Some("er:systemSuffix"), false)?,
                                )
                            }
                            super::GroupTypeContent::DelegatePublic(x) => {
                                *self.state = GroupTypeContentSerializerState::DelegatePublic(
                                    WithSerializer::serializer(
                                        x,
                                        Some("er:delegatePublic"),
                                        false,
                                    )?,
                                )
                            }
                            super::GroupTypeContent::DelegateSystem(x) => {
                                *self.state = GroupTypeContentSerializerState::DelegateSystem(
                                    WithSerializer::serializer(
                                        x,
                                        Some("er:delegateSystem"),
                                        false,
                                    )?,
                                )
                            }
                            super::GroupTypeContent::DelegateUri(x) => {
                                *self.state = GroupTypeContentSerializerState::DelegateUri(
                                    WithSerializer::serializer(x, Some("er:delegateURI"), false)?,
                                )
                            }
                            super::GroupTypeContent::NextCatalog(x) => {
                                *self.state = GroupTypeContentSerializerState::NextCatalog(
                                    WithSerializer::serializer(x, Some("er:nextCatalog"), false)?,
                                )
                            }
                        },
                        GroupTypeContentSerializerState::Public(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = GroupTypeContentSerializerState::Done__,
                            }
                        }
                        GroupTypeContentSerializerState::System(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = GroupTypeContentSerializerState::Done__,
                            }
                        }
                        GroupTypeContentSerializerState::Uri(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = GroupTypeContentSerializerState::Done__,
                            }
                        }
                        GroupTypeContentSerializerState::RewriteSystem(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = GroupTypeContentSerializerState::Done__,
                            }
                        }
                        GroupTypeContentSerializerState::RewriteUri(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = GroupTypeContentSerializerState::Done__,
                            }
                        }
                        GroupTypeContentSerializerState::UriSuffix(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = GroupTypeContentSerializerState::Done__,
                            }
                        }
                        GroupTypeContentSerializerState::SystemSuffix(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = GroupTypeContentSerializerState::Done__,
                            }
                        }
                        GroupTypeContentSerializerState::DelegatePublic(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = GroupTypeContentSerializerState::Done__,
                            }
                        }
                        GroupTypeContentSerializerState::DelegateSystem(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = GroupTypeContentSerializerState::Done__,
                            }
                        }
                        GroupTypeContentSerializerState::DelegateUri(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = GroupTypeContentSerializerState::Done__,
                            }
                        }
                        GroupTypeContentSerializerState::NextCatalog(x) => {
                            match x.next(helper).transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = GroupTypeContentSerializerState::Done__,
                            }
                        }
                        GroupTypeContentSerializerState::Done__ => return Ok(None),
                        GroupTypeContentSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for GroupTypeContentSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = GroupTypeContentSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct NextCatalogTypeSerializer<'ser> {
            pub(super) value: &'ser super::NextCatalogType,
            pub(super) state: Box<NextCatalogTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum NextCatalogTypeSerializerState<'ser> {
            Init__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> NextCatalogTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        NextCatalogTypeSerializerState::Init__ => {
                            *self.state = NextCatalogTypeSerializerState::Done__;
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            if self.is_root {
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_ER),
                                    &super::super::NS_ER,
                                );
                            }
                            helper.write_attrib(&mut bytes, "catalog", &self.value.catalog)?;
                            helper.write_attrib_opt(&mut bytes, "id", &self.value.id)?;
                            helper.end_ns_scope();
                            return Ok(Some(Event::Empty(bytes)));
                        }
                        NextCatalogTypeSerializerState::Done__ => return Ok(None),
                        NextCatalogTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for NextCatalogTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = NextCatalogTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct PublicTypeSerializer<'ser> {
            pub(super) value: &'ser super::PublicType,
            pub(super) state: Box<PublicTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum PublicTypeSerializerState<'ser> {
            Init__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> PublicTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        PublicTypeSerializerState::Init__ => {
                            *self.state = PublicTypeSerializerState::Done__;
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            if self.is_root {
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_ER),
                                    &super::super::NS_ER,
                                );
                            }
                            helper.write_attrib(&mut bytes, "publicId", &self.value.public_id)?;
                            helper.write_attrib(&mut bytes, "uri", &self.value.uri)?;
                            helper.write_attrib_opt(&mut bytes, "id", &self.value.id)?;
                            helper.end_ns_scope();
                            return Ok(Some(Event::Empty(bytes)));
                        }
                        PublicTypeSerializerState::Done__ => return Ok(None),
                        PublicTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for PublicTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = PublicTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct RewriteSystemTypeSerializer<'ser> {
            pub(super) value: &'ser super::RewriteSystemType,
            pub(super) state: Box<RewriteSystemTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum RewriteSystemTypeSerializerState<'ser> {
            Init__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> RewriteSystemTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        RewriteSystemTypeSerializerState::Init__ => {
                            *self.state = RewriteSystemTypeSerializerState::Done__;
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            if self.is_root {
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_ER),
                                    &super::super::NS_ER,
                                );
                            }
                            helper.write_attrib(
                                &mut bytes,
                                "systemIdStartString",
                                &self.value.system_id_start_string,
                            )?;
                            helper.write_attrib(
                                &mut bytes,
                                "rewritePrefix",
                                &self.value.rewrite_prefix,
                            )?;
                            helper.write_attrib_opt(&mut bytes, "id", &self.value.id)?;
                            helper.end_ns_scope();
                            return Ok(Some(Event::Empty(bytes)));
                        }
                        RewriteSystemTypeSerializerState::Done__ => return Ok(None),
                        RewriteSystemTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for RewriteSystemTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = RewriteSystemTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct RewriteUriTypeSerializer<'ser> {
            pub(super) value: &'ser super::RewriteUriType,
            pub(super) state: Box<RewriteUriTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum RewriteUriTypeSerializerState<'ser> {
            Init__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> RewriteUriTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        RewriteUriTypeSerializerState::Init__ => {
                            *self.state = RewriteUriTypeSerializerState::Done__;
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            if self.is_root {
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_ER),
                                    &super::super::NS_ER,
                                );
                            }
                            helper.write_attrib(
                                &mut bytes,
                                "uriStartString",
                                &self.value.uri_start_string,
                            )?;
                            helper.write_attrib(
                                &mut bytes,
                                "rewritePrefix",
                                &self.value.rewrite_prefix,
                            )?;
                            helper.write_attrib_opt(&mut bytes, "id", &self.value.id)?;
                            helper.end_ns_scope();
                            return Ok(Some(Event::Empty(bytes)));
                        }
                        RewriteUriTypeSerializerState::Done__ => return Ok(None),
                        RewriteUriTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for RewriteUriTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = RewriteUriTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct SystemTypeSerializer<'ser> {
            pub(super) value: &'ser super::SystemType,
            pub(super) state: Box<SystemTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum SystemTypeSerializerState<'ser> {
            Init__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> SystemTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        SystemTypeSerializerState::Init__ => {
                            *self.state = SystemTypeSerializerState::Done__;
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            if self.is_root {
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_ER),
                                    &super::super::NS_ER,
                                );
                            }
                            helper.write_attrib(&mut bytes, "systemId", &self.value.system_id)?;
                            helper.write_attrib(&mut bytes, "uri", &self.value.uri)?;
                            helper.write_attrib_opt(&mut bytes, "id", &self.value.id)?;
                            helper.end_ns_scope();
                            return Ok(Some(Event::Empty(bytes)));
                        }
                        SystemTypeSerializerState::Done__ => return Ok(None),
                        SystemTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for SystemTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = SystemTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct SystemSuffixTypeSerializer<'ser> {
            pub(super) value: &'ser super::SystemSuffixType,
            pub(super) state: Box<SystemSuffixTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum SystemSuffixTypeSerializerState<'ser> {
            Init__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> SystemSuffixTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        SystemSuffixTypeSerializerState::Init__ => {
                            *self.state = SystemSuffixTypeSerializerState::Done__;
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            if self.is_root {
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_ER),
                                    &super::super::NS_ER,
                                );
                            }
                            helper.write_attrib(
                                &mut bytes,
                                "systemIdSuffix",
                                &self.value.system_id_suffix,
                            )?;
                            helper.write_attrib(&mut bytes, "uri", &self.value.uri)?;
                            helper.write_attrib_opt(&mut bytes, "id", &self.value.id)?;
                            helper.end_ns_scope();
                            return Ok(Some(Event::Empty(bytes)));
                        }
                        SystemSuffixTypeSerializerState::Done__ => return Ok(None),
                        SystemSuffixTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for SystemSuffixTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = SystemSuffixTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct UriTypeSerializer<'ser> {
            pub(super) value: &'ser super::UriType,
            pub(super) state: Box<UriTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum UriTypeSerializerState<'ser> {
            Init__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> UriTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        UriTypeSerializerState::Init__ => {
                            *self.state = UriTypeSerializerState::Done__;
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            if self.is_root {
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_ER),
                                    &super::super::NS_ER,
                                );
                            }
                            helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                            helper.write_attrib(&mut bytes, "uri", &self.value.uri)?;
                            helper.write_attrib_opt(&mut bytes, "id", &self.value.id)?;
                            helper.end_ns_scope();
                            return Ok(Some(Event::Empty(bytes)));
                        }
                        UriTypeSerializerState::Done__ => return Ok(None),
                        UriTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for UriTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = UriTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct UriSuffixTypeSerializer<'ser> {
            pub(super) value: &'ser super::UriSuffixType,
            pub(super) state: Box<UriSuffixTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum UriSuffixTypeSerializerState<'ser> {
            Init__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> UriSuffixTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        UriSuffixTypeSerializerState::Init__ => {
                            *self.state = UriSuffixTypeSerializerState::Done__;
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            if self.is_root {
                                helper.write_xmlns(
                                    &mut bytes,
                                    Some(&super::super::PREFIX_ER),
                                    &super::super::NS_ER,
                                );
                            }
                            helper.write_attrib(&mut bytes, "uriSuffix", &self.value.uri_suffix)?;
                            helper.write_attrib(&mut bytes, "uri", &self.value.uri)?;
                            helper.write_attrib_opt(&mut bytes, "id", &self.value.id)?;
                            helper.end_ns_scope();
                            return Ok(Some(Event::Empty(bytes)));
                        }
                        UriSuffixTypeSerializerState::Done__ => return Ok(None),
                        UriSuffixTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for UriSuffixTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = UriSuffixTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
    }
}
pub mod xs {
    use std::borrow::Cow;
    use xsd_parser_types::quick_xml::{
        DeserializeBytes, DeserializeHelper, Error, SerializeBytes, SerializeHelper,
    };
    #[derive(Debug, Default)]
    pub struct EntitiesType(pub Vec<String>);
    impl SerializeBytes for EntitiesType {
        fn serialize_bytes(
            &self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Cow<'_, str>>, Error> {
            if self.0.is_empty() {
                return Ok(None);
            }
            let mut data = String::new();
            for item in &self.0 {
                if let Some(bytes) = item.serialize_bytes(helper)? {
                    if !data.is_empty() {
                        data.push(' ');
                    }
                    data.push_str(&bytes);
                }
            }
            Ok(Some(Cow::Owned(data)))
        }
    }
    impl DeserializeBytes for EntitiesType {
        fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
            Ok(Self(helper.deserialize_list(bytes)?))
        }
    }
    pub type EntityType = EntitiesType;
    pub type IdType = String;
    pub type IdrefType = String;
    pub type IdrefsType = EntitiesType;
    pub type NcNameType = String;
    pub type NmtokenType = String;
    pub type NmtokensType = EntitiesType;
    pub type NotationType = String;
    pub type NameType = String;
    pub type QNameType = String;
    pub type AnySimpleType = String;
    pub type AnyUriType = String;
    pub type Base64BinaryType = String;
    pub type BooleanType = bool;
    pub type ByteType = i8;
    pub type DateType = String;
    pub type DateTimeType = String;
    pub type DecimalType = f64;
    pub type DoubleType = f64;
    pub type DurationType = String;
    pub type FloatType = f32;
    pub type GDayType = String;
    pub type GMonthType = String;
    pub type GMonthDayType = String;
    pub type GYearType = String;
    pub type GYearMonthType = String;
    pub type HexBinaryType = String;
    pub type IntType = i32;
    pub type IntegerType = i32;
    pub type LanguageType = String;
    pub type LongType = i64;
    pub type NegativeIntegerType = isize;
    pub type NonNegativeIntegerType = usize;
    pub type NonPositiveIntegerType = isize;
    pub type NormalizedStringType = String;
    pub type PositiveIntegerType = usize;
    pub type ShortType = i16;
    pub type StringType = String;
    pub type TimeType = String;
    pub type TokenType = String;
    pub type UnsignedByteType = u8;
    pub type UnsignedIntType = u32;
    pub type UnsignedLongType = u64;
    pub type UnsignedShortType = u16;
}
