use num::BigInt;
pub mod tns {
    use num::BigInt;
    use serde::{Deserialize, Serialize};
    pub type Foo = FooType;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FooType {
        #[serde(default = "FooType::default_a_int", rename = "@a-int")]
        pub a_int: BigInt,
        #[serde(default = "FooType::default_b_int", rename = "@b-int")]
        pub b_int: BigInt,
    }
    impl FooType {
        #[must_use]
        pub fn default_a_int() -> BigInt {
            <num::BigInt as core::str::FromStr>::from_str("123").unwrap()
        }
        #[must_use]
        pub fn default_b_int() -> BigInt {
            <num::BigInt as core::str::FromStr>::from_str("456").unwrap()
        }
    }
}
