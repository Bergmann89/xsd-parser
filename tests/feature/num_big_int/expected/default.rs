pub mod tns {
    use num::BigInt;
    pub type Foo = FooType;
    #[derive(Debug)]
    pub struct FooType {
        pub a_int: BigInt,
        pub b_int: BigInt,
    }
}
