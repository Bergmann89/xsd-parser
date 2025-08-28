use xsd_parser::xml::Nillable;
pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub a: i32,
    pub b: Nillable<i32>,
    pub c: Option<i32>,
    pub d: Option<Nillable<i32>>,
}
pub type NillableFoo = Nillable<NillableFooInnerType>;
pub type NillableFooInnerType = FooType;
