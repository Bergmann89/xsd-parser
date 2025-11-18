pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub messages: FooTypeMessagesType,
}
#[derive(Debug)]
pub struct FooTypeMessagesType {
    pub aa: i32,
    pub bb: String,
    pub a: String,
}
