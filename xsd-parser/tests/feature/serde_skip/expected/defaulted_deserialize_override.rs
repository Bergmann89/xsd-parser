use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct AliasesType {
    #[serde(
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty",
        rename = "@requiredValues"
    )]
    pub required_values: ValuesType,
    #[serde(deserialize_with = "crate::feature::serde_skip::deserialize_option")]
    #[serde(default)]
    #[serde(
        skip_serializing_if = "::core::option::Option::is_none",
        rename = "Optional"
    )]
    pub optional: OptionalType,
    #[serde(deserialize_with = "crate::feature::serde_skip::deserialize_vec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", rename = "Items")]
    pub items: ItemsType,
    #[serde(
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty",
        rename = "BoxedItems"
    )]
    pub boxed_items: Box<ItemsType>,
    #[serde(
        with = "::serde_with::As::<::core::option::Option<[::serde_with::Same; 64]>>",
        default,
        skip_serializing_if = "::core::option::Option::is_none",
        rename = "OptionalArray"
    )]
    pub optional_array: OptionalArrayType,
    #[serde(
        with = "::serde_with::As::<::std::vec::Vec<[::serde_with::Same; 64]>>",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty",
        rename = "ArrayItems"
    )]
    pub array_items: ArrayItemsType,
}
pub type ValuesType = Vec<StringType>;
pub type OptionalType = Option<String>;
pub type ItemsType = Vec<String>;
pub type OptionalArrayType = Option<ArrayType>;
pub type ArrayItemsType = Vec<ArrayType>;
pub type StringType = String;
pub type ArrayType = [String; 64usize];
