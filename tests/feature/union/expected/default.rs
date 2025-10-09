use regex::Regex;
use std::sync::LazyLock;
use xsd_parser::quick_xml::ValidateError;
pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub union_: UnionType,
}
#[derive(Debug)]
pub enum UnionType {
    I32(i32),
    String(String),
}
impl UnionType {
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        static PATTERNS: LazyLock<[Regex; 1usize]> =
            LazyLock::new(|| [Regex::new("[a-z0-9]+").unwrap()]);
        for pattern in PATTERNS.iter() {
            if !pattern.is_match(s) {
                return Err(ValidateError::Pattern(pattern.as_str()));
            }
        }
        Ok(())
    }
}
