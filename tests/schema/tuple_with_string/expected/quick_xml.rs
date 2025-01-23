#[derive(Debug, Clone)]
pub struct Foo(pub String);
impl xsd_parser::quick_xml::SerializeBytes for Foo {
    fn serialize_bytes(
        &self,
    ) -> Result<Option<std::borrow::Cow<'_, str>>, xsd_parser::quick_xml::Error> {
        self.0.serialize_bytes()
    }
}
impl xsd_parser::quick_xml::DeserializeBytes for Foo {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        Ok(Self(String::deserialize_bytes(reader, bytes)?))
    }
}
