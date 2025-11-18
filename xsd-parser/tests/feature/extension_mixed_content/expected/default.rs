use xsd_parser_types::xml::{AnyAttributes, AnyElement, Mixed, Text};
pub type AttributeValue = AttributeValueType;
#[derive(Debug)]
pub struct AttributeValueType {
    pub base_attrib: String,
    pub data_type: String,
    pub any_attribute: AnyAttributes,
    pub text_before: Option<Text>,
    pub base_element: Mixed<String>,
    pub any: Vec<Mixed<AnyElement>>,
}
