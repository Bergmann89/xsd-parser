pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub a_list: ListType,
}
#[derive(Debug, Default)]
pub struct ListType(pub Vec<StringType>);
pub type StringType = String;
