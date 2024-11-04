pub struct MyComplexType {
    pub elenent_a: Vec<StringType>,
    pub element_b: Vec<IntType>,
    pub content_2: MyComplexContent2Type,
}
pub type StringType = String;
pub type IntType = i32;
pub struct MyComplexContent2Type {
    pub content: Option<MyComplexContent2TypeContent>,
}
pub enum MyComplexContent2TypeContent {
    ElementC(BooleanType),
    Content3(MyComplexContent3Type),
    Content4(MyComplexContent4Type),
}
pub type BooleanType = bool;
pub struct MyComplexContent3Type {
    pub elenent_d: StringType,
    pub element_e: IntType,
}
pub struct MyComplexContent4Type {
    pub elenent_f: StringType,
    pub element_g: Vec<IntType>,
}
