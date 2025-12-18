use std::borrow::Cow;
use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{DeserializeBytes, DeserializeHelper, Error, SerializeBytes, SerializeHelper},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
#[derive(Debug)]
pub struct Foo(pub String);
impl SerializeBytes for Foo {
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        self.0.serialize_bytes(helper)
    }
}
impl DeserializeBytes for Foo {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self(String::deserialize_bytes(helper, bytes)?))
    }
}
