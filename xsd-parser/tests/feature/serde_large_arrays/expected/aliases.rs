use core::ops::Deref;
use serde::{Deserialize, Serialize};
use xsd_parser_types::quick_xml::ValidateError;
pub type Message = MessageType;
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageType {
    #[serde(with = "::serde_with::As::<::core::option::Option<[::serde_with::Same; 64]>>")]
    #[serde(default, rename = "Row")]
    pub row: Option<RowType>,
    #[serde(with = "::serde_with::As::<::std::vec::Vec<[::serde_with::Same; 64]>>")]
    #[serde(default, rename = "Rows")]
    pub rows: Vec<RowType>,
    #[serde(with = "::serde_with::As::<[[::serde_with::Same; 64]; 2]>")]
    #[serde(rename = "Matrix")]
    pub matrix: MatrixType,
    #[serde(with = "::serde_with::As::<::std::boxed::Box<[::serde_with::Same; 64]>>")]
    #[serde(rename = "Boxed")]
    pub boxed: Box<RowType>,
    #[serde(with = "::serde_with::As::<[::std::boxed::Box<::serde_with::Same>; 64]>")]
    #[serde(rename = "BigBoxes")]
    pub big_boxes: [Box<StringType>; 64usize],
    #[serde(rename = "List")]
    pub list: ListOfRowsType,
    #[serde(rename = "Choice")]
    pub choice: UnionType,
    #[serde(rename = "Tagged")]
    pub tagged: TaggedType,
}
pub type RowType = [String; 64usize];
pub type MatrixType = [RowType; 2usize];
pub type StringType = String;
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ListOfRowsType(
    #[serde(with = "::serde_with::As::<::std::vec::Vec<[::serde_with::Same; 64]>>")]
    pub  Vec<RowType>,
);
impl ListOfRowsType {
    pub fn new(inner: Vec<RowType>) -> Result<Self, ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> Vec<RowType> {
        self.0
    }
    pub fn validate_value(value: &Vec<RowType>) -> Result<(), ValidateError> {
        if value.is_empty() {
            return Err(ValidateError::MinLength(1usize));
        }
        Ok(())
    }
}
impl From<ListOfRowsType> for Vec<RowType> {
    fn from(value: ListOfRowsType) -> Vec<RowType> {
        value.0
    }
}
impl TryFrom<Vec<RowType>> for ListOfRowsType {
    type Error = ValidateError;
    fn try_from(value: Vec<RowType>) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for ListOfRowsType {
    type Target = Vec<RowType>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum UnionType {
    RowType(#[serde(with = "::serde_with::As::<[::serde_with::Same; 64]>")] RowType),
    Boolean(BooleanType),
}
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum TaggedType {
    CodeType(CodeType),
    RowType(#[serde(with = "::serde_with::As::<[::serde_with::Same; 64]>")] RowType),
}
pub type BooleanType = bool;
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum CodeType {
    #[serde(rename = "EMPTY")]
    Empty,
}
