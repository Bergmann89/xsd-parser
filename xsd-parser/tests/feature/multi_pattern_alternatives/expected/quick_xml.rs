use core::{ops::Deref, str::from_utf8};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};
use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{
        DeserializeBytes, DeserializeHelper, Error, SerializeBytes, SerializeHelper, ValidateError,
        WithDeserializerFromBytes, WithSerializeToBytes,
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
pub type OperatingMode = OperatingModeType;
#[derive(Debug)]
pub struct OperatingModeType(pub String);
impl OperatingModeType {
    pub fn new(inner: String) -> Result<Self, ValidateError> {
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        static PATTERNS: LazyLock<[(&str, Regex); 3usize]> = LazyLock::new(|| {
            [
                ("hybrid/auto", Regex::new("^(?:hybrid/auto)$").unwrap()),
                (
                    "heat pump only",
                    Regex::new("^(?:heat pump only)$").unwrap(),
                ),
                (
                    "electric heater only",
                    Regex::new("^(?:electric heater only)$").unwrap(),
                ),
            ]
        });
        if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
            return Err(ValidateError::Pattern(PATTERNS[0usize].0));
        }
        Ok(())
    }
}
impl From<OperatingModeType> for String {
    fn from(value: OperatingModeType) -> String {
        value.0
    }
}
impl TryFrom<String> for OperatingModeType {
    type Error = ValidateError;
    fn try_from(value: String) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for OperatingModeType {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl SerializeBytes for OperatingModeType {
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        self.0.serialize_bytes(helper)
    }
}
impl WithSerializeToBytes for OperatingModeType {}
impl DeserializeBytes for OperatingModeType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let s = from_utf8(bytes).map_err(Error::from)?;
        Self::validate_str(s).map_err(|error| (bytes, error))?;
        let inner = String::deserialize_str(helper, s)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl WithDeserializerFromBytes for OperatingModeType {}
