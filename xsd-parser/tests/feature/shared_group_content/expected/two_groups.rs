use xsd_parser_types::xml::{Base64String, HexString};
#[derive(Debug, Default)]
pub struct EntitiesType(pub Vec<String>);
pub type EntityType = String;
pub type IdType = String;
pub type IdrefType = String;
pub type IdrefsType = EntitiesType;
pub type NcNameType = String;
pub type NmtokenType = String;
pub type NmtokensType = EntitiesType;
pub type NotationType = String;
pub type NameType = String;
pub type QNameType = String;
pub type AnySimpleType = String;
pub type AnyUriType = String;
pub type Base64BinaryType = Base64String;
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
pub type HexBinaryType = HexString;
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
#[derive(Debug)]
pub struct Content0Type;
#[derive(Debug)]
pub struct AlphaType {
    pub alpha: Option<String>,
    pub content: AlphaContent1Type,
}
#[derive(Debug)]
pub struct BetaType {
    pub beta: Option<String>,
    pub content: BetaContent2Type,
}
#[derive(Debug)]
pub enum AlphaContent1Type {
    NestedBeta(Box<NestedBetaType>),
    LeafA(String),
}
pub type AlphaGroupAType = AlphaContent1Type;
#[derive(Debug)]
pub enum BetaContent2Type {
    NestedAlpha(Box<NestedAlphaType>),
    LeafB(String),
}
pub type BetaGroupBType = BetaContent2Type;
pub type NestedAlpha = NestedAlphaType;
#[derive(Debug)]
pub struct NestedAlphaType {
    pub nested_alpha: Option<String>,
    pub content: AlphaContent1Type,
}
pub type NestedAlphaContent4Type = AlphaContent1Type;
pub type NestedAlphaGroupAType = AlphaContent1Type;
pub type NestedBeta = NestedBetaType;
#[derive(Debug)]
pub struct NestedBetaType {
    pub nested_beta: Option<String>,
    pub content: BetaContent2Type,
}
pub type NestedBetaContent3Type = BetaContent2Type;
pub type NestedBetaGroupBType = BetaContent2Type;
