pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub a_list: ListType,
}
#[derive(Debug, Clone, Default)]
pub struct ListType(pub Vec<StringType>);
pub type StringType = String;
