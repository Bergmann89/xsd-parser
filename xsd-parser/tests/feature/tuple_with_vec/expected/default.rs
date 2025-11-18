pub type Foo = FooType;
#[derive(Debug, Default)]
pub struct FooType(pub Vec<StringType>);
pub type StringType = String;
