use xsd_parser::xml::{AnyAttributes, AnyElement, Mixed, Text};
pub type Foo = FooType;
pub struct FooType {
    pub anything: AnyType,
}
pub struct AnyType {
    pub any_attribute: AnyAttributes,
    pub text_before: Option<Text>,
    pub any: Vec<Mixed<AnyElement>>,
}
