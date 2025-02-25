pub struct MyComplexType {
    pub elenent_a: Vec<String>,
    pub element_b: Vec<i32>,
    pub content_5: MyComplexContent5Type,
}
pub struct MyComplexContent5Type {
    pub content: MyComplexContent5TypeContent,
}
pub enum MyComplexContent5TypeContent {
    ElementC(bool),
    Content7(Option<MyComplexContent7Type>),
    Content10(MyComplexContent10Type),
}
pub struct MyComplexContent7Type {
    pub elenent_d: String,
    pub element_e: i32,
}
pub struct MyComplexContent10Type {
    pub elenent_f: String,
    pub element_g: Vec<i32>,
}
