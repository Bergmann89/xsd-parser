pub struct MyComplexType {
    pub content: MyComplexTypeContent,
}
pub enum MyComplexTypeContent {
    ElementA(Vec<String>),
    ElementB(Vec<i32>),
    ElementC(bool),
    ElementD(Option<String>),
    ElementE(Option<i32>),
    ElementF(String),
    ElementG(Vec<i32>),
}
