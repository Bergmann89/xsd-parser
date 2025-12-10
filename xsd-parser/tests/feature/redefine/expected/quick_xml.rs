use std::borrow::Cow;
use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{
        DeserializeBytes, DeserializeHelper, Error, ErrorKind, RawByteStr, SerializeBytes,
        SerializeHelper,
    },
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub type Foo = EnumType;
#[derive(Debug)]
pub enum EnumType {
    _1,
    _2,
    _3,
}
impl SerializeBytes for EnumType {
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        match self {
            Self::_1 => Ok(Some(Cow::Borrowed("1"))),
            Self::_2 => Ok(Some(Cow::Borrowed("2"))),
            Self::_3 => Ok(Some(Cow::Borrowed("3"))),
        }
    }
}
impl DeserializeBytes for EnumType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"1" => Ok(Self::_1),
            b"2" => Ok(Self::_2),
            b"3" => Ok(Self::_3),
            x => Err(Error::from(ErrorKind::UnknownOrInvalidValue(
                RawByteStr::from_slice(x),
            ))),
        }
    }
}
