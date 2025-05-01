pub struct MyComplexType {
    pub element_a: Vec<StringType>,
    pub element_b: Vec<IntType>,
    pub content_2: MyComplexContent2Type,
}
pub type StringType = String;
pub type IntType = i32;
pub struct MyComplexContent2Type {
    pub content: MyComplexContent2TypeContent,
}
pub enum MyComplexContent2TypeContent {
    ElementC(BooleanType),
    Content3(Option<MyComplexContent3Type>),
    Content4(MyComplexContent4Type),
}
pub type BooleanType = bool;
pub struct MyComplexContent3Type {
    pub element_d: StringType,
    pub element_e: IntType,
}
pub struct MyComplexContent4Type {
    pub element_f: StringType,
    pub element_g: Vec<IntType>,
}
