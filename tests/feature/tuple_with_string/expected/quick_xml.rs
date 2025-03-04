#[derive(Debug, Clone)]
pub struct Foo(pub String);
impl xsd_parser::quick_xml::SerializeBytes for Foo {
    fn serialize_bytes(
        &self,
    ) -> Result<Option<std::borrow::Cow<'_, str>>, xsd_parser::quick_xml::Error> {
        self.0.serialize_bytes()
    }
}
