pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub a_list: ListType,
}
impl FooType {
    #[must_use]
    pub fn default_a_list() -> ListType {
        ListType(Vec::new())
    }
}
#[derive(Debug, Clone, Default)]
pub struct ListType(pub Vec<StringType>);
pub type StringType = String;
