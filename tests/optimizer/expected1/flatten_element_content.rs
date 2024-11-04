pub struct MyComplexType {
    pub content: Vec<MyComplexTypeContent>,
}
pub enum MyComplexTypeContent {
    ElenentA(StringType),
    ElementB(IntType),
    ElementC(BooleanType),
    ElenentD(StringType),
    ElementE(IntType),
    ElenentF(StringType),
    ElementG(IntType),
}
pub type StringType = String;
pub type IntType = i32;
pub type BooleanType = bool;
