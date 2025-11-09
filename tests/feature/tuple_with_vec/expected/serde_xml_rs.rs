use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FooType(pub Vec<StringType>);
pub type StringType = String;
