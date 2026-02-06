#[derive(Debug, Default)]
pub struct ENTITIESType(pub Vec<String>);
pub type ENTITYType = String;
pub type IDType = String;
pub type IDREFType = String;
#[derive(Debug, Default)]
pub struct IDREFSType(pub Vec<String>);
pub type NCNameType = String;
pub type NMTOKENType = String;
#[derive(Debug, Default)]
pub struct NMTOKENSType(pub Vec<String>);
pub type NOTATIONType = String;
pub type NameType = String;
pub type QNameType = String;
pub type anySimpleTypeType = String;
#[derive(Debug)]
pub struct anyTypeType;
pub type anyURIType = String;
pub type base64BinaryType = String;
pub type booleanType = bool;
pub type byteType = i8;
pub type dateType = String;
pub type dateTimeType = String;
pub type decimalType = f64;
pub type doubleType = f64;
pub type durationType = String;
pub type floatType = f32;
pub type gDayType = String;
pub type gMonthType = String;
pub type gMonthDayType = String;
pub type gYearType = String;
pub type gYearMonthType = String;
pub type hexBinaryType = String;
pub type intType = i32;
pub type integerType = i32;
pub type languageType = String;
pub type longType = i64;
pub type negativeIntegerType = isize;
pub type nonNegativeIntegerType = usize;
pub type nonPositiveIntegerType = isize;
pub type normalizedStringType = String;
pub type positiveIntegerType = usize;
pub type shortType = i16;
pub type stringType = String;
pub type timeType = String;
pub type tokenType = String;
pub type unsignedByteType = u8;
pub type unsignedIntType = u32;
pub type unsignedLongType = u64;
pub type unsignedShortType = u16;
#[derive(Debug)]
pub struct BaseType {
    pub my_group_1: Base_MyGroupType,
    pub my_group_2: BaseMyGroupType,
}
#[derive(Debug)]
pub struct Base_Type {
    pub my_group: Base_MyGroupType,
}
#[derive(Debug)]
pub struct Base_MyGroupType {
    pub name: Option<String>,
}
#[derive(Debug)]
pub struct BaseMyGroupType {
    pub name: Option<String>,
}
