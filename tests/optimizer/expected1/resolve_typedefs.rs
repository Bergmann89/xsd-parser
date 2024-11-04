pub struct MyComplexType {
    pub elenent_a: Vec<String>,
    pub element_b: Vec<i32>,
    pub content_2: MyComplexContent2Type,
}
pub struct MyComplexContent2Type {
    pub content: Option<MyComplexContent2TypeContent>,
}
pub enum MyComplexContent2TypeContent {
    ElementC(bool),
    Content3(MyComplexContent3Type),
    Content4(MyComplexContent4Type),
}
pub struct MyComplexContent3Type {
    pub elenent_d: String,
    pub element_e: i32,
}
pub struct MyComplexContent4Type {
    pub elenent_f: String,
    pub element_g: Vec<i32>,
}
