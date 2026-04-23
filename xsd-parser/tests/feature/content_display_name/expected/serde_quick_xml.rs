use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Deserialize, Serialize)]
pub struct FooType {
    #[serde(default, rename = "@content")]
    pub content: Option<String>,
    #[serde(default, rename = "$text")]
    pub text_value: String,
}
