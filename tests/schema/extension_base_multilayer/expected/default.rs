pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub messages: FooTypeMessages,
}
#[derive(Debug, Clone)]
pub struct FooTypeMessages {
    pub aa: IntType,
    pub bb: StringType,
    pub a: StringType,
}
pub type IntType = i32;
pub type StringType = String;
