use num::BigInt;
pub mod tns {
    pub type Foo = FooType;
    #[derive(Debug)]
    pub struct FooType {
        pub a_int: super::BigInt,
        pub b_int: super::BigInt,
    }
}
