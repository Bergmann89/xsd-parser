use core::num::{NonZeroIsize, NonZeroUsize};
pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub a_int: NonZeroUsize,
    pub b_int: NonZeroIsize,
}
