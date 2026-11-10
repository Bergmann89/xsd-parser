use xsd_parser_types::misc::{Namespace, NamespacePrefix};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_TNS: Namespace = Namespace::new_const(b"urn:imports");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub mod tns {
    pub use self::type_choice_type::ChoiceType;
    pub use self::type_label_type::LabelType;
    pub use self::type_limit_type::LimitType;
    pub use self::type_mode_type::ModeType;
    pub use self::type_root::Root;
    pub use self::type_root_type::RootType;
    pub use self::type_words_type::WordsType;
    pub mod type_choice_type {
        use std::borrow::Cow;
        use xsd_parser_types::quick_xml::{
            DeserializeBytes, DeserializeHelper, Error, ErrorKind, SerializeBytes, SerializeHelper,
            WithDeserializerFromBytes, WithSerializeToBytes,
        };
        #[derive(Debug)]
        pub enum ChoiceType {
            I32(i32),
            String(String),
        }
        impl SerializeBytes for ChoiceType {
            fn serialize_bytes(
                &self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Cow<'_, str>>, Error> {
                match self {
                    Self::I32(x) => x.serialize_bytes(helper),
                    Self::String(x) => x.serialize_bytes(helper),
                }
            }
        }
        impl WithSerializeToBytes for ChoiceType {}
        impl DeserializeBytes for ChoiceType {
            fn deserialize_bytes(
                helper: &mut DeserializeHelper,
                bytes: &[u8],
            ) -> Result<Self, Error> {
                let mut errors = Vec::new();
                match i32::deserialize_bytes(helper, bytes) {
                    Ok(value) => return Ok(Self::I32(value)),
                    Err(error) => errors.push(Box::new(error)),
                }
                match String::deserialize_bytes(helper, bytes) {
                    Ok(value) => return Ok(Self::String(value)),
                    Err(error) => errors.push(Box::new(error)),
                }
                Err(Error::from(ErrorKind::InvalidUnion(errors.into())))
            }
        }
        impl WithDeserializerFromBytes for ChoiceType {}
    }
    pub mod type_label_type {
        use core::ops::Deref;
        use std::borrow::Cow;
        use xsd_parser_types::quick_xml::{
            DeserializeBytes, DeserializeHelper, Error, SerializeBytes, SerializeHelper,
            ValidateError, WithDeserializerFromBytes, WithSerializeToBytes,
        };
        #[derive(Debug)]
        pub struct LabelType(pub String);
        impl LabelType {
            pub fn new(inner: String) -> Result<Self, ValidateError> {
                Self::validate_value(&inner)?;
                Ok(Self(inner))
            }
            #[must_use]
            pub fn into_inner(self) -> String {
                self.0
            }
            pub fn validate_value(value: &String) -> Result<(), ValidateError> {
                if value.is_empty() {
                    return Err(ValidateError::MinLength(1usize));
                }
                Ok(())
            }
        }
        impl From<LabelType> for String {
            fn from(value: LabelType) -> String {
                value.0
            }
        }
        impl TryFrom<String> for LabelType {
            type Error = ValidateError;
            fn try_from(value: String) -> Result<Self, ValidateError> {
                Self::new(value)
            }
        }
        impl Deref for LabelType {
            type Target = String;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl SerializeBytes for LabelType {
            fn serialize_bytes(
                &self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Cow<'_, str>>, Error> {
                self.0.serialize_bytes(helper)
            }
        }
        impl WithSerializeToBytes for LabelType {}
        impl DeserializeBytes for LabelType {
            fn deserialize_bytes(
                helper: &mut DeserializeHelper,
                bytes: &[u8],
            ) -> Result<Self, Error> {
                let inner = String::deserialize_bytes(helper, bytes)?;
                Ok(Self::new(inner).map_err(|error| (bytes, error))?)
            }
        }
        impl WithDeserializerFromBytes for LabelType {}
    }
    pub mod type_limit_type {
        use std::borrow::Cow;
        use xsd_parser_types::quick_xml::{
            DeserializeBytes, DeserializeHelper, Error, SerializeBytes, SerializeHelper,
            WithDeserializerFromBytes, WithSerializeToBytes,
        };
        #[derive(Debug)]
        pub enum LimitType {
            U32(u32),
            Unbounded,
        }
        impl SerializeBytes for LimitType {
            fn serialize_bytes(
                &self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Cow<'_, str>>, Error> {
                match self {
                    Self::U32(x) => x.serialize_bytes(helper),
                    Self::Unbounded => Ok(Some(Cow::Borrowed("unbounded"))),
                }
            }
        }
        impl WithSerializeToBytes for LimitType {}
        impl DeserializeBytes for LimitType {
            fn deserialize_bytes(
                helper: &mut DeserializeHelper,
                bytes: &[u8],
            ) -> Result<Self, Error> {
                match bytes {
                    b"unbounded" => Ok(Self::Unbounded),
                    x => Ok(Self::U32(u32::deserialize_bytes(helper, x)?)),
                }
            }
        }
        impl WithDeserializerFromBytes for LimitType {}
    }
    pub mod type_mode_type {
        use std::borrow::Cow;
        use xsd_parser_types::quick_xml::{
            DeserializeBytes, DeserializeHelper, Error, ErrorKind, RawByteStr, SerializeBytes,
            SerializeHelper, WithDeserializerFromBytes, WithSerializeToBytes,
        };
        #[derive(Debug)]
        pub enum ModeType {
            On,
            Off,
        }
        impl SerializeBytes for ModeType {
            fn serialize_bytes(
                &self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Cow<'_, str>>, Error> {
                match self {
                    Self::On => Ok(Some(Cow::Borrowed("on"))),
                    Self::Off => Ok(Some(Cow::Borrowed("off"))),
                }
            }
        }
        impl WithSerializeToBytes for ModeType {}
        impl DeserializeBytes for ModeType {
            fn deserialize_bytes(
                helper: &mut DeserializeHelper,
                bytes: &[u8],
            ) -> Result<Self, Error> {
                match bytes {
                    b"on" => Ok(Self::On),
                    b"off" => Ok(Self::Off),
                    x => Err(Error::from(ErrorKind::UnknownOrInvalidValue(
                        RawByteStr::from_slice(x),
                    ))),
                }
            }
        }
        impl WithDeserializerFromBytes for ModeType {}
    }
    pub mod type_root {
        pub type Root = super::type_root_type::RootType;
    }
    pub mod type_root_type {
        use xsd_parser_types::quick_xml::{
            ElementNamespace, Error, WithDeserializer, WithSerializer,
        };
        #[derive(Debug)]
        pub struct RootType {
            pub limit: super::type_limit_type::LimitType,
            pub choice: super::type_choice_type::ChoiceType,
            pub label: super::type_label_type::LabelType,
            pub words: super::type_words_type::WordsType,
            pub mode: super::type_mode_type::ModeType,
        }
        impl WithSerializer for RootType {
            type Serializer<'x> = quick_xml_serialize::RootTypeSerializer<'x>;
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
                Ok(quick_xml_serialize::RootTypeSerializer {
                    value: self,
                    state: Box::new(quick_xml_serialize::RootTypeSerializerState::Init__),
                    name: name.unwrap_or("tns:Root"),
                    namespace,
                    is_root,
                })
            }
        }
        impl WithDeserializer for RootType {
            type Deserializer = quick_xml_deserialize::RootTypeDeserializer;
        }
        mod quick_xml_serialize {
            use xsd_parser_types::quick_xml::{
                BytesStart, ElementNamespace, Error, Event, SerializeHelper, Serializer,
            };
            #[derive(Debug)]
            pub struct RootTypeSerializer<'ser> {
                pub(super) value: &'ser super::RootType,
                pub(super) state: Box<RootTypeSerializerState<'ser>>,
                pub(super) name: &'ser str,
                pub(super) namespace: ElementNamespace,
                pub(super) is_root: bool,
            }
            #[derive(Debug)]
            pub(super) enum RootTypeSerializerState<'ser> {
                Init__,
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
                                *self.state = RootTypeSerializerState::Done__;
                                let namespace = if self.is_root {
                                    ElementNamespace::Namespace(super::super::super::NS_TNS)
                                } else {
                                    self.namespace.clone()
                                };
                                self.name = helper.element_name(self.name, &namespace);
                                let mut bytes = BytesStart::new(self.name);
                                helper.begin_ns_scope();
                                helper.write_xmlns_for_element(&mut bytes, self.name, &namespace);
                                if self.is_root {
                                    helper
                                        .write_default_namespace(&mut bytes, self.name, &namespace);
                                }
                                if self.is_root {
                                    helper.write_xmlns_unless_default(
                                        &mut bytes,
                                        Some(&super::super::super::PREFIX_TNS),
                                        &super::super::super::NS_TNS,
                                    );
                                }
                                helper.write_attrib(&mut bytes, "limit", &self.value.limit)?;
                                helper.write_attrib(&mut bytes, "choice", &self.value.choice)?;
                                helper.write_attrib(&mut bytes, "label", &self.value.label)?;
                                helper.write_attrib(&mut bytes, "words", &self.value.words)?;
                                helper.write_attrib(&mut bytes, "mode", &self.value.mode)?;
                                helper.end_ns_scope();
                                return Ok(Some(Event::Empty(bytes)));
                            }
                            RootTypeSerializerState::Done__ => return Ok(None),
                            RootTypeSerializerState::Phantom__(_) => unreachable!(),
                        }
                    }
                }
            }
            impl<'ser> Serializer<'ser> for RootTypeSerializer<'ser> {
                fn next(
                    &mut self,
                    helper: &mut SerializeHelper,
                ) -> Option<Result<Event<'ser>, Error>> {
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
        }
        mod quick_xml_deserialize {
            use core::mem::replace;
            use xsd_parser_types::quick_xml::{
                BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact,
                DeserializerEvent, DeserializerOutput, DeserializerResult, Error, ErrorKind, Event,
            };
            #[derive(Debug)]
            pub struct RootTypeDeserializer {
                limit: super::super::type_limit_type::LimitType,
                choice: super::super::type_choice_type::ChoiceType,
                label: super::super::type_label_type::LabelType,
                words: super::super::type_words_type::WordsType,
                mode: super::super::type_mode_type::ModeType,
                state__: Box<RootTypeDeserializerState>,
            }
            #[derive(Debug)]
            enum RootTypeDeserializerState {
                Init__,
                Unknown__,
            }
            impl RootTypeDeserializer {
                fn from_bytes_start(
                    helper: &mut DeserializeHelper,
                    bytes_start: &BytesStart<'_>,
                ) -> Result<Self, Error> {
                    let mut limit: Option<super::super::type_limit_type::LimitType> = None;
                    let mut choice: Option<super::super::type_choice_type::ChoiceType> = None;
                    let mut label: Option<super::super::type_label_type::LabelType> = None;
                    let mut words: Option<super::super::type_words_type::WordsType> = None;
                    let mut mode: Option<super::super::type_mode_type::ModeType> = None;
                    for attrib in helper.filter_xmlns_attributes(bytes_start) {
                        let attrib = attrib?;
                        if matches!(
                            helper.resolve_attrib_local_name(
                                attrib.key,
                                &super::super::super::NS_TNS
                            ),
                            Some(b"limit")
                        ) {
                            helper.read_attrib(&mut limit, b"limit", &attrib.value)?;
                        } else if matches!(
                            helper.resolve_attrib_local_name(
                                attrib.key,
                                &super::super::super::NS_TNS
                            ),
                            Some(b"choice")
                        ) {
                            helper.read_attrib(&mut choice, b"choice", &attrib.value)?;
                        } else if matches!(
                            helper.resolve_attrib_local_name(
                                attrib.key,
                                &super::super::super::NS_TNS
                            ),
                            Some(b"label")
                        ) {
                            helper.read_attrib(&mut label, b"label", &attrib.value)?;
                        } else if matches!(
                            helper.resolve_attrib_local_name(
                                attrib.key,
                                &super::super::super::NS_TNS
                            ),
                            Some(b"words")
                        ) {
                            helper.read_attrib(&mut words, b"words", &attrib.value)?;
                        } else if matches!(
                            helper.resolve_attrib_local_name(
                                attrib.key,
                                &super::super::super::NS_TNS
                            ),
                            Some(b"mode")
                        ) {
                            helper.read_attrib(&mut mode, b"mode", &attrib.value)?;
                        } else {
                            helper.raise_unexpected_attrib_checked(&attrib)?;
                        }
                    }
                    Ok(Self {
                        limit: limit.ok_or_else(|| ErrorKind::MissingAttribute("limit".into()))?,
                        choice: choice
                            .ok_or_else(|| ErrorKind::MissingAttribute("choice".into()))?,
                        label: label.ok_or_else(|| ErrorKind::MissingAttribute("label".into()))?,
                        words: words.ok_or_else(|| ErrorKind::MissingAttribute("words".into()))?,
                        mode: mode.ok_or_else(|| ErrorKind::MissingAttribute("mode".into()))?,
                        state__: Box::new(RootTypeDeserializerState::Init__),
                    })
                }
                fn finish_state(
                    &mut self,
                    helper: &mut DeserializeHelper,
                    state: RootTypeDeserializerState,
                ) -> Result<(), Error> {
                    Ok(())
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
                ) -> Result<super::RootType, Error> {
                    let state = replace(&mut *self.state__, RootTypeDeserializerState::Unknown__);
                    self.finish_state(helper, state)?;
                    Ok(super::RootType {
                        limit: self.limit,
                        choice: self.choice,
                        label: self.label,
                        words: self.words,
                        mode: self.mode,
                    })
                }
            }
        }
    }
    pub mod type_words_type {
        use std::borrow::Cow;
        use xsd_parser_types::quick_xml::{
            DeserializeBytes, DeserializeHelper, Error, SerializeBytes, SerializeHelper,
            WithDeserializerFromBytes, WithSerializeToBytes,
        };
        #[derive(Debug, Default)]
        pub struct WordsType(pub Vec<super::super::xs::type_string_type::StringType>);
        impl SerializeBytes for WordsType {
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
        impl WithSerializeToBytes for WordsType {}
        impl DeserializeBytes for WordsType {
            fn deserialize_bytes(
                helper: &mut DeserializeHelper,
                bytes: &[u8],
            ) -> Result<Self, Error> {
                Ok(Self(helper.deserialize_list(bytes)?))
            }
        }
        impl WithDeserializerFromBytes for WordsType {}
    }
}
pub mod xs {
    pub use self::type_string_type::StringType;
    pub mod type_string_type {
        pub type StringType = String;
    }
}
