use std::borrow::Cow;
use xsd_parser_types::{
    misc::Namespace,
    quick_xml::{DeserializeBytes, DeserializeHelper, Error, SerializeBytes},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
#[derive(Debug)]
pub struct Foo(pub i32);
impl SerializeBytes for Foo {
    fn serialize_bytes(&self) -> Result<Option<Cow<'_, str>>, Error> {
        self.0.serialize_bytes()
    }
}
impl DeserializeBytes for Foo {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self(i32::deserialize_bytes(helper, bytes)?))
    }
}
