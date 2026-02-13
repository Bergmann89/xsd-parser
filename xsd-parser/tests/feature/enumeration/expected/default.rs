use regex::Regex;
use std::sync::LazyLock;
use xsd_parser_types::quick_xml::ValidateError;
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
        static PATTERNS: LazyLock<[(&str, Regex); 1usize]> =
            LazyLock::new(|| [("[A-Z]+", Regex::new("^(?:[A-Z]+)$").unwrap())]);
        for (pattern, regex) in PATTERNS.iter() {
            if !regex.is_match(s) {
                return Err(ValidateError::Pattern(pattern));
            }
        }
        Ok(())
    }
}
