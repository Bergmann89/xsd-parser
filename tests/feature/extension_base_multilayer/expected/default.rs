pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub messages: FooTypeMessagesType,
}
#[derive(Debug, Clone)]
pub struct FooTypeMessagesType {
    pub aa: i32,
    pub bb: String,
    pub a: String,
}
