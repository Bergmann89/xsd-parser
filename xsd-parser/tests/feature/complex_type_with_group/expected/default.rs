pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub outer_1: FooOuterType,
    pub outer_2: FooOuterType,
}
#[derive(Debug)]
pub enum FooOuterType {
    Bar(String),
    Baz(i32),
    Inner(FooInnerType),
}
#[derive(Debug)]
pub struct FooInnerType {
    pub fizz: String,
    pub buzz: i32,
}
