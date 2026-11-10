pub use self::type_enum_type::EnumType;
pub use self::type_foo::Foo;
pub use self::type_foo_type::FooType;
pub mod type_enum_type {
    use regex::Regex;
    use serde::{Deserialize, Serialize};
    use std::sync::LazyLock;
    use xsd_parser_types::quick_xml::ValidateError;
    #[derive(Debug, Deserialize, Serialize)]
    pub enum EnumType {
        #[serde(rename = "OFF")]
        Off,
        #[serde(rename = "ON")]
        On,
        #[serde(rename = "AUTO")]
        Auto,
    }
    impl EnumType {
        pub fn validate_str(s: &str) -> Result<(), ValidateError> {
            static PATTERNS: LazyLock<[(&str, Regex); 1usize]> =
                LazyLock::new(|| [("[A-Z]+", Regex::new("^(?:[A-Z]+)$").unwrap())]);
            if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
                return Err(ValidateError::Pattern(PATTERNS[0usize].0));
            }
            Ok(())
        }
    }
}
pub mod type_foo {
    pub type Foo = super::type_foo_type::FooType;
}
pub mod type_foo_type {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, Serialize)]
    pub struct FooType {
        #[serde(rename = "Enum")]
        pub enum_: super::type_enum_type::EnumType,
    }
}
