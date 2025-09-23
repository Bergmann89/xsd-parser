use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Serialize, Deserialize)]
pub struct FooType {
    #[serde(default = "FooType::default_a_list", rename = "@a-list")]
    pub a_list: ListType,
}
impl FooType {
    #[must_use]
    pub fn default_a_list() -> ListType {
        ListType(Vec::new())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListType(pub Vec<StringType>);
pub type StringType = String;
