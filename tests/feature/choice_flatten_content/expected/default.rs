pub type Foo = FooType;
#[derive(Debug)]
pub enum FooType {
    Once(i32),
    Optional(Option<i32>),
    OnceSpecify(i32),
    TwiceOrMore(Vec<i32>),
}
