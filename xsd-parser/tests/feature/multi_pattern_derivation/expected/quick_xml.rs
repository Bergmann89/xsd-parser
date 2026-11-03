use core::{ops::Deref, str::from_utf8};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};
use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{
        DeserializeBytes, DeserializeHelper, Error, SerializeBytes, SerializeHelper, ValidateError,
        WithDeserializer, WithDeserializerFromBytes, WithSerializeToBytes, WithSerializer,
    },
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub type Value = DerivedType;
#[derive(Debug)]
pub struct DerivedType(pub String);
impl DerivedType {
    pub fn new(inner: String) -> Result<Self, ValidateError> {
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        {
            static PATTERNS: LazyLock<[(&str, Regex); 2usize]> = LazyLock::new(|| {
                [
                    ("[a-z]+", Regex::new("^(?:[a-z]+)$").unwrap()),
                    ("[0-9]+", Regex::new("^(?:[0-9]+)$").unwrap()),
                ]
            });
            if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
                return Err(ValidateError::Pattern(PATTERNS[0usize].0));
            }
        }
        {
            static PATTERNS: LazyLock<[(&str, Regex); 3usize]> = LazyLock::new(|| {
                [
                    ("[a-f]+", Regex::new("^(?:[a-f]+)$").unwrap()),
                    ("[0-4]+", Regex::new("^(?:[0-4]+)$").unwrap()),
                    ("[A-F]+", Regex::new("^(?:[A-F]+)$").unwrap()),
                ]
            });
            if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
                return Err(ValidateError::Pattern(PATTERNS[0usize].0));
            }
        }
        Ok(())
    }
}
impl From<DerivedType> for String {
    fn from(value: DerivedType) -> String {
        value.0
    }
}
impl TryFrom<String> for DerivedType {
    type Error = ValidateError;
    fn try_from(value: String) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for DerivedType {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl SerializeBytes for DerivedType {
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        self.0.serialize_bytes(helper)
    }
}
impl WithSerializeToBytes for DerivedType {}
impl DeserializeBytes for DerivedType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let s = from_utf8(bytes).map_err(Error::from)?;
        Self::validate_str(s).map_err(|error| (bytes, error))?;
        let inner = String::deserialize_str(helper, s)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl WithDeserializerFromBytes for DerivedType {}
pub type Inline = InlineType;
#[derive(Debug)]
pub struct InlineType(pub String);
impl InlineType {
    pub fn new(inner: String) -> Result<Self, ValidateError> {
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        {
            static PATTERNS: LazyLock<[(&str, Regex); 2usize]> = LazyLock::new(|| {
                [
                    ("[a-z]+", Regex::new("^(?:[a-z]+)$").unwrap()),
                    ("[0-9]+", Regex::new("^(?:[0-9]+)$").unwrap()),
                ]
            });
            if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
                return Err(ValidateError::Pattern(PATTERNS[0usize].0));
            }
        }
        {
            static PATTERNS: LazyLock<[(&str, Regex); 1usize]> =
                LazyLock::new(|| [("[a-f]+", Regex::new("^(?:[a-f]+)$").unwrap())]);
            if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
                return Err(ValidateError::Pattern(PATTERNS[0usize].0));
            }
        }
        Ok(())
    }
}
impl From<InlineType> for String {
    fn from(value: InlineType) -> String {
        value.0
    }
}
impl TryFrom<String> for InlineType {
    type Error = ValidateError;
    fn try_from(value: String) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for InlineType {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl SerializeBytes for InlineType {
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        self.0.serialize_bytes(helper)
    }
}
impl WithSerializeToBytes for InlineType {}
impl DeserializeBytes for InlineType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let s = from_utf8(bytes).map_err(Error::from)?;
        Self::validate_str(s).map_err(|error| (bytes, error))?;
        let inner = String::deserialize_str(helper, s)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl WithDeserializerFromBytes for InlineType {}
pub type Extended = ExtendedType;
#[derive(Debug)]
pub struct ExtendedType {
    pub content: BaseType,
}
impl WithSerializer for ExtendedType {
    type Serializer<'x> = quick_xml_serialize::ExtendedTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ExtendedTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ExtendedTypeSerializerState::Init__),
            name: name.unwrap_or("tns:ExtendedType"),
            is_root,
        })
    }
}
impl WithDeserializer for ExtendedType {
    type Deserializer = quick_xml_deserialize::ExtendedTypeDeserializer;
}
#[derive(Debug)]
pub struct BaseType(pub String);
impl BaseType {
    pub fn new(inner: String) -> Result<Self, ValidateError> {
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        static PATTERNS: LazyLock<[(&str, Regex); 2usize]> = LazyLock::new(|| {
            [
                ("[a-z]+", Regex::new("^(?:[a-z]+)$").unwrap()),
                ("[0-9]+", Regex::new("^(?:[0-9]+)$").unwrap()),
            ]
        });
        if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
            return Err(ValidateError::Pattern(PATTERNS[0usize].0));
        }
        Ok(())
    }
}
impl From<BaseType> for String {
    fn from(value: BaseType) -> String {
        value.0
    }
}
impl TryFrom<String> for BaseType {
    type Error = ValidateError;
    fn try_from(value: String) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for BaseType {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl SerializeBytes for BaseType {
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        self.0.serialize_bytes(helper)
    }
}
impl WithSerializeToBytes for BaseType {}
impl DeserializeBytes for BaseType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let s = from_utf8(bytes).map_err(Error::from)?;
        Self::validate_str(s).map_err(|error| (bytes, error))?;
        let inner = String::deserialize_str(helper, s)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl WithDeserializerFromBytes for BaseType {}
pub type Restricted = RestrictedType;
#[derive(Debug)]
pub struct RestrictedType {
    pub content: RestrictedContent1Type,
}
impl WithSerializer for RestrictedType {
    type Serializer<'x> = quick_xml_serialize::RestrictedTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::RestrictedTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::RestrictedTypeSerializerState::Init__),
            name: name.unwrap_or("tns:RestrictedType"),
            is_root,
        })
    }
}
impl WithDeserializer for RestrictedType {
    type Deserializer = quick_xml_deserialize::RestrictedTypeDeserializer;
}
#[derive(Debug)]
pub struct RestrictedContent1Type(pub String);
impl RestrictedContent1Type {
    pub fn new(inner: String) -> Result<Self, ValidateError> {
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        {
            static PATTERNS: LazyLock<[(&str, Regex); 2usize]> = LazyLock::new(|| {
                [
                    ("[a-z]+", Regex::new("^(?:[a-z]+)$").unwrap()),
                    ("[0-9]+", Regex::new("^(?:[0-9]+)$").unwrap()),
                ]
            });
            if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
                return Err(ValidateError::Pattern(PATTERNS[0usize].0));
            }
        }
        {
            static PATTERNS: LazyLock<[(&str, Regex); 1usize]> =
                LazyLock::new(|| [("[a-f]+", Regex::new("^(?:[a-f]+)$").unwrap())]);
            if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
                return Err(ValidateError::Pattern(PATTERNS[0usize].0));
            }
        }
        Ok(())
    }
}
impl From<RestrictedContent1Type> for String {
    fn from(value: RestrictedContent1Type) -> String {
        value.0
    }
}
impl TryFrom<String> for RestrictedContent1Type {
    type Error = ValidateError;
    fn try_from(value: String) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for RestrictedContent1Type {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl SerializeBytes for RestrictedContent1Type {
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        self.0.serialize_bytes(helper)
    }
}
impl WithSerializeToBytes for RestrictedContent1Type {}
impl DeserializeBytes for RestrictedContent1Type {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let s = from_utf8(bytes).map_err(Error::from)?;
        Self::validate_str(s).map_err(|error| (bytes, error))?;
        let inner = String::deserialize_str(helper, s)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl WithDeserializerFromBytes for RestrictedContent1Type {}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, ContentDeserializer, DeserializeHelper, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, Error, ErrorKind, Event,
        WithDeserializer,
    };
    #[derive(Debug)]
    pub struct ExtendedTypeDeserializer {
        content: Option<super::BaseType>,
        state__: Box<ExtendedTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ExtendedTypeDeserializerState {
        Init__,
        Content__(<super::BaseType as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ExtendedTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: None,
                state__: Box::new(ExtendedTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ExtendedTypeDeserializerState,
        ) -> Result<(), Error> {
            if let ExtendedTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::BaseType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::BaseType>,
        ) -> DeserializerResult<'de, super::ExtendedType> {
            use ExtendedTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::ExtendedType> for ExtendedTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExtendedType> {
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
        ) -> DeserializerResult<'de, super::ExtendedType> {
            use ExtendedTypeDeserializerState as S;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ExtendedType, Error> {
            let state = replace(&mut *self.state__, ExtendedTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::ExtendedType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct RestrictedTypeDeserializer {
        content: Option<super::RestrictedContent1Type>,
        state__: Box<RestrictedTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RestrictedTypeDeserializerState {
        Init__,
        Content__(<super::RestrictedContent1Type as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl RestrictedTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: None,
                state__: Box::new(RestrictedTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: RestrictedTypeDeserializerState,
        ) -> Result<(), Error> {
            if let RestrictedTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::RestrictedContent1Type) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::RestrictedContent1Type>,
        ) -> DeserializerResult<'de, super::RestrictedType> {
            use RestrictedTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::RestrictedType> for RestrictedTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RestrictedType> {
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
        ) -> DeserializerResult<'de, super::RestrictedType> {
            use RestrictedTypeDeserializerState as S;
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
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::RestrictedType, Error> {
            let state = replace(
                &mut *self.state__,
                RestrictedTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::RestrictedType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, Error, Event, SerializeHelper, Serializer, WithSerializer,
    };
    #[derive(Debug)]
    pub struct ExtendedTypeSerializer<'ser> {
        pub(super) value: &'ser super::ExtendedType,
        pub(super) state: Box<ExtendedTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ExtendedTypeSerializerState<'ser> {
        Init__,
        Content__(<super::BaseType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ExtendedTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ExtendedTypeSerializerState::Init__ => {
                        *self.state = ExtendedTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
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
                    ExtendedTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ExtendedTypeSerializerState::End__,
                        }
                    }
                    ExtendedTypeSerializerState::End__ => {
                        *self.state = ExtendedTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ExtendedTypeSerializerState::Done__ => return Ok(None),
                    ExtendedTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ExtendedTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ExtendedTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct RestrictedTypeSerializer<'ser> {
        pub(super) value: &'ser super::RestrictedType,
        pub(super) state: Box<RestrictedTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum RestrictedTypeSerializerState<'ser> {
        Init__,
        Content__(<super::RestrictedContent1Type as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RestrictedTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    RestrictedTypeSerializerState::Init__ => {
                        *self.state = RestrictedTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
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
                    RestrictedTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RestrictedTypeSerializerState::End__,
                        }
                    }
                    RestrictedTypeSerializerState::End__ => {
                        *self.state = RestrictedTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    RestrictedTypeSerializerState::Done__ => return Ok(None),
                    RestrictedTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for RestrictedTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = RestrictedTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
