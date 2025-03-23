#[derive(Debug, Clone, Default)]
pub struct EntitiesType(pub Vec<String>);
#[derive(Debug, Clone, Default)]
pub struct EntityType(pub Vec<String>);
pub type IdType = String;
pub type IdrefType = String;
#[derive(Debug, Clone, Default)]
pub struct IdrefsType(pub Vec<String>);
pub type NcNameType = String;
pub type NmtokenType = String;
#[derive(Debug, Clone, Default)]
pub struct NmtokensType(pub Vec<String>);
pub type NotationType = String;
pub type NameType = String;
pub type QnameType = String;
#[derive(Debug, Clone)]
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
pub type GdayType = String;
pub type GmonthType = String;
pub type GmonthDayType = String;
pub type GyearType = String;
pub type GyearMonthType = String;
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
