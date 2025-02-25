pub struct MyComplexType {
    pub elenent_a: Vec<MyComplexTypeElenentA>,
    pub element_b: Vec<MyComplexTypeElementB>,
    pub content_5: MyComplexContent5Type,
}
pub type MyComplexTypeElenentA = StringType;
pub type MyComplexTypeElementB = IntType;
pub struct MyComplexContent5Type {
    pub content: MyComplexContent5TypeContent,
}
pub enum MyComplexContent5TypeContent {
    ElementC(MyComplexTypeElementC),
    Content7(Option<MyComplexContent7Type>),
    Content10(MyComplexContent10Type),
}
pub type StringType = String;
pub type IntType = i32;
pub type MyComplexTypeElementC = BooleanType;
pub struct MyComplexContent7Type {
    pub elenent_d: MyComplexTypeElenentD,
    pub element_e: MyComplexTypeElementE,
}
pub struct MyComplexContent10Type {
    pub elenent_f: MyComplexTypeElenentF,
    pub element_g: Vec<MyComplexTypeElementG>,
}
pub type BooleanType = bool;
pub type MyComplexTypeElenentD = StringType;
pub type MyComplexTypeElementE = IntType;
pub type MyComplexTypeElenentF = StringType;
pub type MyComplexTypeElementG = IntType;
