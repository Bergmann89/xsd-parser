pub struct FirstRestrictedType {
    pub a: StringType,
}
pub type StringType = String;
pub struct SecondRestrictedType {
    pub b: Vec<IntType>,
}
pub type IntType = i32;
pub struct ThirdRestrictedType {
    pub b: IntType,
}
