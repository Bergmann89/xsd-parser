use core::{ops::Deref, str::from_utf8};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};
use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{
        DeserializeBytes, DeserializeHelper, ElementNamespace, Error, ErrorKind, SerializeBytes,
        SerializeHelper, ValidateError, WithDeserializer, WithDeserializerFromBytes,
        WithSerializeToBytes, WithSerializer,
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
pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub country: CountryCodeType,
}
impl WithSerializer for FooType {
    type Serializer<'x> = quick_xml_serialize::FooTypeSerializer<'x>;
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
        Ok(quick_xml_serialize::FooTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::FooTypeSerializerState::Init__),
            name: name.unwrap_or("tns:FooType"),
            namespace,
            is_root,
        })
    }
}
impl WithDeserializer for FooType {
    type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
}
#[derive(Debug)]
pub enum CountryCodeType {
    Alpha2(Alpha2Type),
    Alpha3(Alpha3Type),
    Germany,
    Usa,
}
impl SerializeBytes for CountryCodeType {
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        match self {
            Self::Alpha2(x) => x.serialize_bytes(helper),
            Self::Alpha3(x) => x.serialize_bytes(helper),
            Self::Germany => Ok(Some(Cow::Borrowed("Germany"))),
            Self::Usa => Ok(Some(Cow::Borrowed("USA"))),
        }
    }
}
impl WithSerializeToBytes for CountryCodeType {}
impl DeserializeBytes for CountryCodeType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"Germany" => Ok(Self::Germany),
            b"USA" => Ok(Self::Usa),
            x => {
                let mut errors = Vec::new();
                match Alpha2Type::deserialize_bytes(helper, x) {
                    Ok(value) => return Ok(Self::Alpha2(value)),
                    Err(error) => errors.push(Box::new(error)),
                }
                match Alpha3Type::deserialize_bytes(helper, x) {
                    Ok(value) => return Ok(Self::Alpha3(value)),
                    Err(error) => errors.push(Box::new(error)),
                }
                Err(Error::from(ErrorKind::InvalidUnion(errors.into())))
            }
        }
    }
}
impl WithDeserializerFromBytes for CountryCodeType {}
#[derive(Debug)]
pub struct Alpha2Type(pub String);
impl Alpha2Type {
    pub fn new(inner: String) -> Result<Self, ValidateError> {
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        static PATTERNS: LazyLock<[(&str, Regex); 1usize]> =
            LazyLock::new(|| [("[A-Z]{2}", Regex::new("^(?:[A-Z]{2})$").unwrap())]);
        if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
            return Err(ValidateError::Pattern(PATTERNS[0usize].0));
        }
        Ok(())
    }
}
impl From<Alpha2Type> for String {
    fn from(value: Alpha2Type) -> String {
        value.0
    }
}
impl TryFrom<String> for Alpha2Type {
    type Error = ValidateError;
    fn try_from(value: String) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for Alpha2Type {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl SerializeBytes for Alpha2Type {
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        self.0.serialize_bytes(helper)
    }
}
impl WithSerializeToBytes for Alpha2Type {}
impl DeserializeBytes for Alpha2Type {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let s = from_utf8(bytes).map_err(Error::from)?;
        Self::validate_str(s).map_err(|error| (bytes, error))?;
        let inner = String::deserialize_str(helper, s)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl WithDeserializerFromBytes for Alpha2Type {}
#[derive(Debug)]
pub struct Alpha3Type(pub String);
impl Alpha3Type {
    pub fn new(inner: String) -> Result<Self, ValidateError> {
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        static PATTERNS: LazyLock<[(&str, Regex); 1usize]> =
            LazyLock::new(|| [("[A-Z]{3}", Regex::new("^(?:[A-Z]{3})$").unwrap())]);
        if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
            return Err(ValidateError::Pattern(PATTERNS[0usize].0));
        }
        Ok(())
    }
}
impl From<Alpha3Type> for String {
    fn from(value: Alpha3Type) -> String {
        value.0
    }
}
impl TryFrom<String> for Alpha3Type {
    type Error = ValidateError;
    fn try_from(value: String) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for Alpha3Type {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl SerializeBytes for Alpha3Type {
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        self.0.serialize_bytes(helper)
    }
}
impl WithSerializeToBytes for Alpha3Type {}
impl DeserializeBytes for Alpha3Type {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let s = from_utf8(bytes).map_err(Error::from)?;
        Self::validate_str(s).map_err(|error| (bytes, error))?;
        let inner = String::deserialize_str(helper, s)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl WithDeserializerFromBytes for Alpha3Type {}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, Error, ErrorKind, Event,
    };
    #[derive(Debug)]
    pub struct FooTypeDeserializer {
        country: super::CountryCodeType,
        state__: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl FooTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut country: Option<super::CountryCodeType> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_attrib_local_name(attrib.key, &super::NS_TNS),
                    Some(b"country")
                ) {
                    helper.read_attrib(&mut country, b"country", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                country: country.ok_or_else(|| ErrorKind::MissingAttribute("country".into()))?,
                state__: Box::new(FooTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: FooTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::FooType> for FooTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooType> {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::FooType, Error> {
            let state = replace(&mut *self.state__, FooTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::FooType {
                country: self.country,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesStart, ElementNamespace, Error, Event, SerializeHelper, Serializer,
    };
    #[derive(Debug)]
    pub struct FooTypeSerializer<'ser> {
        pub(super) value: &'ser super::FooType,
        pub(super) state: Box<FooTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) namespace: ElementNamespace,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum FooTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FooTypeSerializerState::Init__ => {
                        *self.state = FooTypeSerializerState::Done__;
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
                        helper.write_attrib(&mut bytes, "country", &self.value.country)?;
                        helper.end_ns_scope();
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    FooTypeSerializerState::Done__ => return Ok(None),
                    FooTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for FooTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = FooTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
