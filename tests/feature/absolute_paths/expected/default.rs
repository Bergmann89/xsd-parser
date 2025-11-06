use core::ops::Deref;
use xsd_parser::quick_xml::ValidateError;
pub type AsMut = TestType;
pub type AsRef = TestType;
pub type Box = TestType;
pub type Clone = TestType;
pub type Copy = TestType;
pub type Debug = TestType;
pub type Default = TestType;
pub type Drop = TestType;
pub type Eq = TestType;
pub type Fn = TestType;
pub type FnMut = TestType;
pub type FnOnce = TestType;
pub type From = TestType;
pub type FromIterator = TestType;
pub type Into = TestType;
pub type IntoIterator = TestType;
pub type Iterator = TestType;
pub type Option = TestType;
pub type Ord = TestType;
pub type PartialEq = TestType;
pub type PartialOrd = TestType;
pub type Result = TestType;
pub type Send = TestType;
pub type Sized = TestType;
pub type String = TestType;
pub type Sync = TestType;
#[derive(Debug)]
pub struct TestType(pub ::std::string::String);
impl TestType {
    pub fn new(inner: ::std::string::String) -> ::core::result::Result<Self, ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    pub fn into_inner(self) -> ::std::string::String {
        self.0
    }
    pub fn validate_value(
        value: &::std::string::String,
    ) -> ::core::result::Result<(), ValidateError> {
        if !value.is_empty() {
            return Err(ValidateError::MaxLength(0usize));
        }
        Ok(())
    }
}
impl ::core::convert::From<TestType> for ::std::string::String {
    fn from(value: TestType) -> ::std::string::String {
        value.0
    }
}
impl ::core::convert::TryFrom<::std::string::String> for TestType {
    type Error = ValidateError;
    fn try_from(value: ::std::string::String) -> ::core::result::Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for TestType {
    type Target = ::std::string::String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub type ToString = TestType;
pub type TryFrom = TestType;
pub type TryInto = TestType;
pub type Unpin = TestType;
pub type Vec = TestType;
#[derive(Debug, Default)]
pub struct EntitiesType(pub ::std::vec::Vec<::std::string::String>);
#[derive(Debug, Default)]
pub struct EntityType(pub ::std::vec::Vec<::std::string::String>);
pub type IdType = ::std::string::String;
pub type IdrefType = ::std::string::String;
#[derive(Debug, Default)]
pub struct IdrefsType(pub ::std::vec::Vec<::std::string::String>);
pub type NcNameType = ::std::string::String;
pub type NmtokenType = ::std::string::String;
#[derive(Debug, Default)]
pub struct NmtokensType(pub ::std::vec::Vec<::std::string::String>);
pub type NotationType = ::std::string::String;
pub type NameType = ::std::string::String;
pub type QNameType = ::std::string::String;
pub type AnySimpleType = ::std::string::String;
#[derive(Debug)]
pub struct AnyType;
pub type AnyUriType = ::std::string::String;
pub type Base64BinaryType = ::std::string::String;
pub type BooleanType = ::core::primitive::bool;
pub type ByteType = ::core::primitive::i8;
pub type DateType = ::std::string::String;
pub type DateTimeType = ::std::string::String;
pub type DecimalType = ::core::primitive::f64;
pub type DoubleType = ::core::primitive::f64;
pub type DurationType = ::std::string::String;
pub type FloatType = ::core::primitive::f32;
pub type GDayType = ::std::string::String;
pub type GMonthType = ::std::string::String;
pub type GMonthDayType = ::std::string::String;
pub type GYearType = ::std::string::String;
pub type GYearMonthType = ::std::string::String;
pub type HexBinaryType = ::std::string::String;
pub type IntType = ::core::primitive::i32;
pub type IntegerType = ::core::primitive::i32;
pub type LanguageType = ::std::string::String;
pub type LongType = ::core::primitive::i64;
pub type NegativeIntegerType = ::core::primitive::isize;
pub type NonNegativeIntegerType = ::core::primitive::usize;
pub type NonPositiveIntegerType = ::core::primitive::isize;
pub type NormalizedStringType = ::std::string::String;
pub type PositiveIntegerType = ::core::primitive::usize;
pub type ShortType = ::core::primitive::i16;
pub type StringType = ::std::string::String;
pub type TimeType = ::std::string::String;
pub type TokenType = ::std::string::String;
pub type UnsignedByteType = ::core::primitive::u8;
pub type UnsignedIntType = ::core::primitive::u32;
pub type UnsignedLongType = ::core::primitive::u64;
pub type UnsignedShortType = ::core::primitive::u16;
