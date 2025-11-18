use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct Foo(pub String);
