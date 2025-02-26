use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FooType(pub Vec<StringType>);
pub type StringType = String;
