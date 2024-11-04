pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub name: StringType,
}
pub type StringType = String;
