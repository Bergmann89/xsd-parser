pub struct MyComplexType {
    pub elenent_a: Vec<MyComplexTypeElenentA>,
    pub element_b: Vec<MyComplexTypeElementB>,
    pub content_4: MyComplexContent4Type,
}
pub type MyComplexTypeElenentA = StringType;
pub type MyComplexTypeElementB = IntType;
pub struct MyComplexContent4Type {
    pub content: Option<MyComplexContent4TypeContent>,
}
pub enum MyComplexContent4TypeContent {
    ElementC(MyComplexTypeElementC),
    Content6(MyComplexContent6Type),
    Content9(MyComplexContent9Type),
}
pub type StringType = String;
pub type IntType = i32;
pub type MyComplexTypeElementC = BooleanType;
pub struct MyComplexContent6Type {
    pub elenent_d: MyComplexTypeElenentD,
    pub element_e: MyComplexTypeElementE,
}
pub struct MyComplexContent9Type {
    pub elenent_f: MyComplexTypeElenentF,
    pub element_g: Vec<MyComplexTypeElementG>,
}
pub type BooleanType = bool;
pub type MyComplexTypeElenentD = StringType;
pub type MyComplexTypeElementE = IntType;
pub type MyComplexTypeElenentF = StringType;
pub type MyComplexTypeElementG = IntType;
