pub struct MyComplexType {
    pub element_a: Vec<MyComplexTypeElementA>,
    pub element_b: Vec<MyComplexTypeElementB>,
    pub content_2: MyComplexContent2Type,
}
pub type MyComplexTypeElementA = StringType;
pub type MyComplexTypeElementB = IntType;
pub struct MyComplexContent2Type {
    pub content: MyComplexContent2TypeContent,
}
pub enum MyComplexContent2TypeContent {
    ElementC(MyComplexTypeElementC),
    Content3(Option<MyComplexContent3Type>),
    Content4(MyComplexContent4Type),
}
pub type StringType = String;
pub type IntType = i32;
pub type MyComplexTypeElementC = BooleanType;
pub struct MyComplexContent3Type {
    pub element_d: MyComplexTypeElementD,
    pub element_e: MyComplexTypeElementE,
}
pub struct MyComplexContent4Type {
    pub element_f: MyComplexTypeElementF,
    pub element_g: Vec<MyComplexTypeElementG>,
}
pub type BooleanType = bool;
pub type MyComplexTypeElementD = StringType;
pub type MyComplexTypeElementE = IntType;
pub type MyComplexTypeElementF = StringType;
pub type MyComplexTypeElementG = IntType;
