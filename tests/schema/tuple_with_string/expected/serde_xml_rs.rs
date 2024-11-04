use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Foo(pub FooType);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooType(pub String);
