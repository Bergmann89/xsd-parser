pub struct MyComplexType {
    pub element_a: Vec<String>,
    pub element_b: Vec<i32>,
    pub content_2: Option<MyComplexContent2Type>,
}
pub struct MyComplexContent2Type {
    pub content: MyComplexContent2TypeContent,
}
pub enum MyComplexContent2TypeContent {
    ElementC(bool),
    Content3(Option<MyComplexContent3Type>),
    Content4(MyComplexContent4Type),
}
pub struct MyComplexContent3Type {
    pub element_d: String,
    pub element_e: i32,
}
pub struct MyComplexContent4Type {
    pub element_f: String,
    pub element_g: Vec<i32>,
}
