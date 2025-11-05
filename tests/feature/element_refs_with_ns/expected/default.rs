pub type Outer = OuterType;
#[derive(Debug)]
pub struct OuterType {
    pub bar_inner: bar::InnerType,
    pub baz_inner: baz::InnerType,
    pub biz_inner: biz::InnerType,
}
pub mod bar {
    #[derive(Debug)]
    pub struct InnerType {
        pub a: String,
    }
}
pub mod baz {
    #[derive(Debug)]
    pub struct InnerType {
        pub b: String,
    }
}
pub mod biz {
    #[derive(Debug)]
    pub struct InnerType {
        pub c: String,
    }
}
