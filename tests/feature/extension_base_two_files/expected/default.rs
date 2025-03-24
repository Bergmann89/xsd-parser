pub mod other {
    #[derive(Debug, Clone)]
    pub struct BarType {
        pub b: i32,
        pub c: String,
    }
}
pub mod tns {
    pub type Foo = FooType;
    #[derive(Debug, Clone)]
    pub struct FooType {
        pub a: f32,
        pub b: super::other::BarType,
    }
}
