use core::ops::Deref;
use regex::Regex;
use std::sync::LazyLock;
use xsd_parser_types::quick_xml::ValidateError;
pub type Value = DerivedType;
#[derive(Debug)]
pub struct DerivedType(pub String);
impl DerivedType {
    pub fn new(inner: String) -> Result<Self, ValidateError> {
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        {
            static PATTERNS: LazyLock<[(&str, Regex); 2usize]> = LazyLock::new(|| {
                [
                    ("[a-z]+", Regex::new("^(?:[a-z]+)$").unwrap()),
                    ("[0-9]+", Regex::new("^(?:[0-9]+)$").unwrap()),
                ]
            });
            if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
                return Err(ValidateError::Pattern(PATTERNS[0usize].0));
            }
        }
        {
            static PATTERNS: LazyLock<[(&str, Regex); 3usize]> = LazyLock::new(|| {
                [
                    ("[a-f]+", Regex::new("^(?:[a-f]+)$").unwrap()),
                    ("[0-4]+", Regex::new("^(?:[0-4]+)$").unwrap()),
                    ("[A-F]+", Regex::new("^(?:[A-F]+)$").unwrap()),
                ]
            });
            if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
                return Err(ValidateError::Pattern(PATTERNS[0usize].0));
            }
        }
        Ok(())
    }
}
impl From<DerivedType> for String {
    fn from(value: DerivedType) -> String {
        value.0
    }
}
impl TryFrom<String> for DerivedType {
    type Error = ValidateError;
    fn try_from(value: String) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for DerivedType {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub type Inline = InlineType;
#[derive(Debug)]
pub struct InlineType(pub String);
impl InlineType {
    pub fn new(inner: String) -> Result<Self, ValidateError> {
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        {
            static PATTERNS: LazyLock<[(&str, Regex); 2usize]> = LazyLock::new(|| {
                [
                    ("[a-z]+", Regex::new("^(?:[a-z]+)$").unwrap()),
                    ("[0-9]+", Regex::new("^(?:[0-9]+)$").unwrap()),
                ]
            });
            if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
                return Err(ValidateError::Pattern(PATTERNS[0usize].0));
            }
        }
        {
            static PATTERNS: LazyLock<[(&str, Regex); 1usize]> =
                LazyLock::new(|| [("[a-f]+", Regex::new("^(?:[a-f]+)$").unwrap())]);
            if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
                return Err(ValidateError::Pattern(PATTERNS[0usize].0));
            }
        }
        Ok(())
    }
}
impl From<InlineType> for String {
    fn from(value: InlineType) -> String {
        value.0
    }
}
impl TryFrom<String> for InlineType {
    type Error = ValidateError;
    fn try_from(value: String) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for InlineType {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub type Extended = ExtendedType;
#[derive(Debug)]
pub struct ExtendedType {
    pub content: BaseType,
}
#[derive(Debug)]
pub struct BaseType(pub String);
impl BaseType {
    pub fn new(inner: String) -> Result<Self, ValidateError> {
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        static PATTERNS: LazyLock<[(&str, Regex); 2usize]> = LazyLock::new(|| {
            [
                ("[a-z]+", Regex::new("^(?:[a-z]+)$").unwrap()),
                ("[0-9]+", Regex::new("^(?:[0-9]+)$").unwrap()),
            ]
        });
        if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
            return Err(ValidateError::Pattern(PATTERNS[0usize].0));
        }
        Ok(())
    }
}
impl From<BaseType> for String {
    fn from(value: BaseType) -> String {
        value.0
    }
}
impl TryFrom<String> for BaseType {
    type Error = ValidateError;
    fn try_from(value: String) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for BaseType {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub type Restricted = RestrictedType;
#[derive(Debug)]
pub struct RestrictedType {
    pub content: RestrictedContent1Type,
}
#[derive(Debug)]
pub struct RestrictedContent1Type(pub String);
impl RestrictedContent1Type {
    pub fn new(inner: String) -> Result<Self, ValidateError> {
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        {
            static PATTERNS: LazyLock<[(&str, Regex); 2usize]> = LazyLock::new(|| {
                [
                    ("[a-z]+", Regex::new("^(?:[a-z]+)$").unwrap()),
                    ("[0-9]+", Regex::new("^(?:[0-9]+)$").unwrap()),
                ]
            });
            if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
                return Err(ValidateError::Pattern(PATTERNS[0usize].0));
            }
        }
        {
            static PATTERNS: LazyLock<[(&str, Regex); 1usize]> =
                LazyLock::new(|| [("[a-f]+", Regex::new("^(?:[a-f]+)$").unwrap())]);
            if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
                return Err(ValidateError::Pattern(PATTERNS[0usize].0));
            }
        }
        Ok(())
    }
}
impl From<RestrictedContent1Type> for String {
    fn from(value: RestrictedContent1Type) -> String {
        value.0
    }
}
impl TryFrom<String> for RestrictedContent1Type {
    type Error = ValidateError;
    fn try_from(value: String) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for RestrictedContent1Type {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
