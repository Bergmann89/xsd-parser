pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub value: Option<String>,
    pub another_value: Option<String>,
    pub content: EnumType,
}
#[derive(Debug)]
pub enum EnumType {
    Off,
    On,
    Auto,
}
