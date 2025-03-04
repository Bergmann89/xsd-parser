pub type Foo = FooType;
#[derive(Debug, Clone, Default)]
pub struct FooType(pub Vec<StringType>);
impl xsd_parser::quick_xml::SerializeBytes for FooType {
    fn serialize_bytes(
        &self,
    ) -> Result<Option<std::borrow::Cow<'_, str>>, xsd_parser::quick_xml::Error> {
        if self.0.is_empty() {
            return Ok(None);
        }
        let mut data = String::new();
        for item in &self.0 {
            if let Some(bytes) = item.serialize_bytes()? {
                if !data.is_empty() {
                    data.push(' ');
                }
                data.push_str(&bytes);
            }
        }
        Ok(Some(std::borrow::Cow::Owned(data)))
    }
}
pub type StringType = String;
