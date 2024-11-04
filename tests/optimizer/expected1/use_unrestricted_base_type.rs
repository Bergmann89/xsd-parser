pub type FirstRestrictedType = BaseType;
pub struct BaseType {
    pub a: Vec<StringType>,
    pub b: Vec<IntType>,
}
pub type StringType = String;
pub type IntType = i32;
pub type SecondRestrictedType = BaseType;
pub type ThirdRestrictedType = BaseType;
