pub struct MyComplexType {
    pub elenent_a: Vec<String>,
    pub element_b: Vec<i32>,
    pub content_4: MyComplexContent4Type,
}
pub struct MyComplexContent4Type {
    pub content: Option<MyComplexContent4TypeContent>,
}
pub enum MyComplexContent4TypeContent {
    ElementC(bool),
    Content6(MyComplexContent6Type),
    Content9(MyComplexContent9Type),
}
pub struct MyComplexContent6Type {
    pub elenent_d: String,
    pub element_e: i32,
}
pub struct MyComplexContent9Type {
    pub elenent_f: String,
    pub element_g: Vec<i32>,
}
