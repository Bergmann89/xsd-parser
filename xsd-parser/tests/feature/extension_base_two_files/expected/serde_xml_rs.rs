pub mod other {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, Serialize)]
    pub struct BarType {
        #[serde(rename = "other:b")]
        pub b: i32,
        #[serde(rename = "other:c")]
        pub c: String,
    }
}
pub mod tns {
    use serde::{Deserialize, Serialize};
    pub type Foo = FooType;
    #[derive(Debug, Deserialize, Serialize)]
    pub struct FooType {
        #[serde(rename = "tns:a")]
        pub a: f32,
        #[serde(rename = "tns:b")]
        pub b: super::other::BarType,
    }
}
