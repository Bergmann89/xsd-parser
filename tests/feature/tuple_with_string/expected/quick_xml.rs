pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
use std::borrow::Cow;
use xsd_parser::{
    quick_xml::{Error, SerializeBytes},
    schema::Namespace,
};
#[derive(Debug, Clone)]
pub struct Foo(pub String);
impl SerializeBytes for Foo {
    fn serialize_bytes(&self) -> Result<Option<Cow<'_, str>>, Error> {
        self.0.serialize_bytes()
    }
}
