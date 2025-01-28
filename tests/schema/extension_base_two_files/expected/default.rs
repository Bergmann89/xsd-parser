pub mod tns {
    use super::*;
    pub type Foo = FooType;
    #[derive(Debug, Clone)]
    pub struct FooType {
        pub a: f32,
        pub b: other::BarType,
    }
}
pub mod other {
    use super::*;
    #[derive(Debug, Clone)]
    pub struct BarType {
        pub b: i32,
        pub c: String,
    }
}
