pub type Outer = OuterType;
#[derive(Debug)]
pub enum OuterType {
    BarInner(bar::InnerType),
    BazInner(baz::InnerType),
    BizInner(biz::InnerType),
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
