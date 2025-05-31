use xsd_parser::xml::{AnyAttributes, AnyElement};
pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub any_attributes: AnyAttributes,
    pub name: String,
    pub choice: ChoiceType,
    pub any_elements: Vec<AnyElement>,
}
#[derive(Debug)]
pub struct ChoiceType {
    pub any_attributes: AnyAttributes,
    pub content: ChoiceTypeContent,
}
#[derive(Debug)]
pub enum ChoiceTypeContent {
    Name(String),
    AnyElement(AnyElement),
}
