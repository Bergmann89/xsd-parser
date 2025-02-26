pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub value: Option<String>,
    pub another_value: Option<String>,
    pub content: EnumType,
}
#[derive(Debug, Clone)]
pub enum EnumType {
    Off,
    On,
    Auto,
}
