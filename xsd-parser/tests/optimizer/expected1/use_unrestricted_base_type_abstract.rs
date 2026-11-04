pub type FirstRestrictedType = BaseTypeDyn;
pub struct BaseTypeDyn {
    pub a: Vec<String>,
    pub b: Vec<i32>,
}
pub type SecondRestrictedType = BaseTypeDyn;
pub type ThirdRestrictedType = BaseTypeDyn;
