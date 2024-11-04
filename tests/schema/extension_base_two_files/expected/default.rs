pub mod xs {
    use super::*;
    pub type FloatType = f32;
    pub type IntType = i32;
    pub type StringType = String;
}
pub mod tns {
    use super::*;
    pub type Foo = FooType;
    #[derive(Debug, Clone)]
    pub struct FooType {
        pub a: xs::FloatType,
        pub b: other::BarType,
    }
}
pub mod other {
    use super::*;
    #[derive(Debug, Clone)]
    pub struct BarType {
        pub b: xs::IntType,
        pub c: xs::StringType,
    }
}
