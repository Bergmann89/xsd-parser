pub type Foo = FooType;
#[derive(Debug, Clone, Default)]
pub struct FooType(pub Vec<StringType>);
pub type StringType = String;
