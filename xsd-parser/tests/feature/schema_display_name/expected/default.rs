pub mod example {
    pub type Foo = FooType;
    #[derive(Debug)]
    pub struct FooType {
        pub once: i32,
        pub optional: Option<i32>,
        pub once_specify: i32,
        pub twice_or_more: Vec<i32>,
    }
}
