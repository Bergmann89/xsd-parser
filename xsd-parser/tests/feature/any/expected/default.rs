use xsd_parser_types::xml::{AnyAttributes, AnyElement};
pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub any_attribute: AnyAttributes,
    pub name: String,
    pub any_0: Vec<AnyElement>,
    pub choice: Vec<ChoiceType>,
    pub any_1: Vec<AnyElement>,
}
#[derive(Debug)]
pub struct ChoiceType {
    pub any_attribute: AnyAttributes,
    pub content: ChoiceTypeContent,
}
#[derive(Debug)]
pub enum ChoiceTypeContent {
    Name(String),
    Any(AnyElement),
}
