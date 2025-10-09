use regex::Regex;
use std::sync::LazyLock;
use xsd_parser::quick_xml::ValidateError;
pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub enum_: EnumType,
}
#[derive(Debug)]
pub enum EnumType {
    Off,
    On,
    Auto,
}
impl EnumType {
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        static PATTERNS: LazyLock<[Regex; 1usize]> =
            LazyLock::new(|| [Regex::new("[A-Z]+").unwrap()]);
        for pattern in PATTERNS.iter() {
            if !pattern.is_match(s) {
                return Err(ValidateError::Pattern(pattern.as_str()));
            }
        }
        Ok(())
    }
}
