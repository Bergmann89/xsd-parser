use core::ops::Deref;
use xsd_parser_types::quick_xml::ValidateError;
pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub a_list: ListType,
}
#[derive(Debug, Default)]
pub struct ListType(pub Vec<StringType>);
pub type StringType = String;
pub type ListOfMyStrings = ListOfMyStringsType;
#[derive(Debug)]
pub struct ListOfMyStringsType(pub Vec<MyStringType>);
impl ListOfMyStringsType {
    pub fn new(inner: Vec<MyStringType>) -> Result<Self, ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> Vec<MyStringType> {
        self.0
    }
    pub fn validate_value(value: &Vec<MyStringType>) -> Result<(), ValidateError> {
        if value.is_empty() {
            return Err(ValidateError::MinLength(1usize));
        }
        Ok(())
    }
}
impl From<ListOfMyStringsType> for Vec<MyStringType> {
    fn from(value: ListOfMyStringsType) -> Vec<MyStringType> {
        value.0
    }
}
impl TryFrom<Vec<MyStringType>> for ListOfMyStringsType {
    type Error = ValidateError;
    fn try_from(value: Vec<MyStringType>) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for ListOfMyStringsType {
    type Target = Vec<MyStringType>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub type MyStringType = String;
