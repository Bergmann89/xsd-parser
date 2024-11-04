pub type FirstType = SecondType;
pub struct SecondType {
    pub a: StringType,
    pub b: Option<StringType>,
    pub c: Vec<StringType>,
}
pub type StringType = String;
