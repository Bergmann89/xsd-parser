pub struct MyComplexType {
    pub element_a: Vec<MyComplexTypeElementA>,
    pub element_b: Vec<MyComplexTypeElementB>,
    pub content_5: MyComplexContent5Type,
}
pub type MyComplexTypeElementA = StringType;
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
    pub element_d: MyComplexTypeElementD,
    pub element_e: MyComplexTypeElementE,
}
pub struct MyComplexContent10Type {
    pub element_f: MyComplexTypeElementF,
    pub element_g: Vec<MyComplexTypeElementG>,
}
pub type BooleanType = bool;
pub type MyComplexTypeElementD = StringType;
pub type MyComplexTypeElementE = IntType;
pub type MyComplexTypeElementF = StringType;
pub type MyComplexTypeElementG = IntType;
