pub mod bar {
    pub use self::type_inner_type::InnerType;
    pub use self::type_outer::Outer;
    pub use self::type_outer_type::OuterType;
    pub mod type_inner_type {
        #[derive(Debug)]
        pub struct InnerType {
            pub b: String,
        }
    }
    pub mod type_outer {
        pub type Outer = super::type_outer_type::OuterType;
    }
    pub mod type_outer_type {
        #[derive(Debug)]
        pub struct OuterType {
            pub inner: super::type_inner_type::InnerType,
        }
    }
}
pub mod foo {
    pub use self::type_inner_type::InnerType;
    pub use self::type_outer::Outer;
    pub use self::type_outer_type::OuterType;
    pub mod type_inner_type {
        #[derive(Debug)]
        pub struct InnerType {
            pub a: String,
        }
    }
    pub mod type_outer {
        pub type Outer = super::type_outer_type::OuterType;
    }
    pub mod type_outer_type {
        #[derive(Debug)]
        pub struct OuterType {
            pub inner: super::type_inner_type::InnerType,
        }
    }
}
