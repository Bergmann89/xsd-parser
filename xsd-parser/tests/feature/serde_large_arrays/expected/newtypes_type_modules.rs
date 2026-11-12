pub use self::type_boolean_type::BooleanType;
pub use self::type_code_type::CodeType;
pub use self::type_list_of_rows_type::ListOfRowsType;
pub use self::type_matrix_type::MatrixType;
pub use self::type_message::Message;
pub use self::type_message_type::MessageType;
pub use self::type_row_type::RowType;
pub use self::type_string_type::StringType;
pub use self::type_tagged_type::TaggedType;
pub use self::type_union_type::UnionType;
pub mod type_boolean_type {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct BooleanType(pub bool);
}
pub mod type_code_type {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub enum CodeType {
        #[serde(rename = "EMPTY")]
        Empty,
    }
}
pub mod type_list_of_rows_type {
    use core::ops::Deref;
    use serde::{Deserialize, Serialize};
    use xsd_parser_types::quick_xml::ValidateError;
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct ListOfRowsType(pub Vec<super::type_row_type::RowType>);
    impl ListOfRowsType {
        pub fn new(inner: Vec<super::type_row_type::RowType>) -> Result<Self, ValidateError> {
            Self::validate_value(&inner)?;
            Ok(Self(inner))
        }
        #[must_use]
        pub fn into_inner(self) -> Vec<super::type_row_type::RowType> {
            self.0
        }
        pub fn validate_value(
            value: &Vec<super::type_row_type::RowType>,
        ) -> Result<(), ValidateError> {
            if value.is_empty() {
                return Err(ValidateError::MinLength(1usize));
            }
            Ok(())
        }
    }
    impl From<ListOfRowsType> for Vec<super::type_row_type::RowType> {
        fn from(value: ListOfRowsType) -> Vec<super::type_row_type::RowType> {
            value.0
        }
    }
    impl TryFrom<Vec<super::type_row_type::RowType>> for ListOfRowsType {
        type Error = ValidateError;
        fn try_from(value: Vec<super::type_row_type::RowType>) -> Result<Self, ValidateError> {
            Self::new(value)
        }
    }
    impl Deref for ListOfRowsType {
        type Target = Vec<super::type_row_type::RowType>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}
pub mod type_matrix_type {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct MatrixType(pub [super::type_row_type::RowType; 2usize]);
}
pub mod type_message {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Message(pub super::type_message_type::MessageType);
}
pub mod type_message_type {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct MessageType {
        #[serde(default, rename = "Row")]
        pub row: Option<super::type_row_type::RowType>,
        #[serde(default, rename = "Rows")]
        pub rows: Vec<super::type_row_type::RowType>,
        #[serde(rename = "Matrix")]
        pub matrix: super::type_matrix_type::MatrixType,
        #[serde(rename = "Boxed")]
        pub boxed: Box<super::type_row_type::RowType>,
        #[serde(
            with = "::serde_with::As::<[::std::boxed::Box<::serde_with::Same>; 64]>",
            rename = "BigBoxes"
        )]
        pub big_boxes: [Box<super::type_string_type::StringType>; 64usize],
        #[serde(rename = "List")]
        pub list: super::type_list_of_rows_type::ListOfRowsType,
        #[serde(rename = "Choice")]
        pub choice: super::type_union_type::UnionType,
        #[serde(rename = "Tagged")]
        pub tagged: super::type_tagged_type::TaggedType,
    }
}
pub mod type_row_type {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct RowType(
        #[serde(with = "::serde_with::As::<[::serde_with::Same; 64]>")] pub [String; 64usize],
    );
}
pub mod type_string_type {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct StringType(pub String);
}
pub mod type_tagged_type {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub enum TaggedType {
        CodeType(super::type_code_type::CodeType),
        RowType(super::type_row_type::RowType),
    }
}
pub mod type_union_type {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub enum UnionType {
        RowType(super::type_row_type::RowType),
        Boolean(super::type_boolean_type::BooleanType),
    }
}
