use serde::{Deserialize, Serialize};
pub type ComplexContent = ComplexContentType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ComplexContentType {
    #[serde(rename = "@lang")]
    pub lang: String,
    #[serde(rename = "Content")]
    pub content: String,
}
pub type SimpleContent = SimpleContentType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleContentType {
    #[serde(rename = "@lang")]
    pub lang: String,
    #[serde(default, rename = "$text")]
    pub content: String,
}
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct EntitiesType(pub Vec<String>);
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct EntityType(pub Vec<String>);
pub type IdType = String;
pub type IdrefType = String;
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct IdrefsType(pub Vec<String>);
pub type NcNameType = String;
pub type NmtokenType = String;
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NmtokensType(pub Vec<String>);
pub type NotationType = String;
pub type NameType = String;
pub type QNameType = String;
pub type AnySimpleType = String;
#[derive(Debug, Deserialize, Serialize)]
pub struct AnyType;
pub type AnyUriType = String;
pub type Base64BinaryType = String;
pub type BooleanType = bool;
pub type ByteType = i8;
pub type DateType = String;
pub type DateTimeType = String;
pub type DecimalType = f64;
pub type DoubleType = f64;
pub type DurationType = String;
pub type FloatType = f32;
pub type GDayType = String;
pub type GMonthType = String;
pub type GMonthDayType = String;
pub type GYearType = String;
pub type GYearMonthType = String;
pub type HexBinaryType = String;
pub type IntType = i32;
pub type IntegerType = i32;
pub type LanguageType = String;
pub type LongType = i64;
pub type NegativeIntegerType = isize;
pub type NonNegativeIntegerType = usize;
pub type NonPositiveIntegerType = isize;
pub type NormalizedStringType = String;
pub type PositiveIntegerType = usize;
pub type ShortType = i16;
pub type StringType = String;
pub type TimeType = String;
pub type TokenType = String;
pub type UnsignedByteType = u8;
pub type UnsignedIntType = u32;
pub type UnsignedLongType = u64;
pub type UnsignedShortType = u16;
