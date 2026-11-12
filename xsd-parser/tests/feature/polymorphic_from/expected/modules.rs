pub mod f {
    pub use self::type_foreign_leaf_type::ForeignLeafType;
    pub mod type_foreign_leaf_type {
        #[derive(Debug)]
        pub struct ForeignLeafType {
            pub id: super::super::xs::type_unsigned_int_type::UnsignedIntType,
            pub value: super::super::xs::type_int_type::IntType,
        }
    }
}
pub mod t {
    pub use self::type_base_type::BaseType;
    pub use self::type_empty_type::EmptyType;
    pub use self::type_head::Head;
    pub use self::type_leaf_type::LeafType;
    pub use self::type_middle_type::MiddleType;
    pub use self::type_mixed_head::MixedHead;
    pub use self::type_node_type::NodeType;
    pub use self::type_ordinary_type::OrdinaryType;
    pub use self::type_unique::Unique;
    pub mod type_base_type {
        #[derive(Debug)]
        pub enum BaseType {
            ForeignLeaf(super::super::f::type_foreign_leaf_type::ForeignLeafType),
            Leaf(super::type_leaf_type::LeafType),
            Node(super::type_node_type::NodeType),
        }
        impl From<super::super::f::type_foreign_leaf_type::ForeignLeafType> for BaseType {
            fn from(value: super::super::f::type_foreign_leaf_type::ForeignLeafType) -> Self {
                Self::ForeignLeaf(value)
            }
        }
        impl From<super::type_leaf_type::LeafType> for BaseType {
            fn from(value: super::type_leaf_type::LeafType) -> Self {
                Self::Leaf(value)
            }
        }
        impl From<super::type_node_type::NodeType> for BaseType {
            fn from(value: super::type_node_type::NodeType) -> Self {
                Self::Node(value)
            }
        }
    }
    pub mod type_empty_type {
        #[derive(Debug)]
        pub enum EmptyType {}
    }
    pub mod type_head {
        #[derive(Debug)]
        pub enum Head {
            First(String),
            Second(String),
            AliasFirst(String),
            AliasSecond(String),
            Number(i32),
        }
        impl From<i32> for Head {
            fn from(value: i32) -> Self {
                Self::Number(value)
            }
        }
    }
    pub mod type_leaf_type {
        #[derive(Debug)]
        pub struct LeafType {
            pub id: super::super::xs::type_unsigned_int_type::UnsignedIntType,
            pub label: super::super::xs::type_string_type::StringType,
        }
    }
    pub mod type_middle_type {
        #[derive(Debug)]
        pub enum MiddleType {
            Leaf(super::type_leaf_type::LeafType),
        }
        impl From<super::type_leaf_type::LeafType> for MiddleType {
            fn from(value: super::type_leaf_type::LeafType) -> Self {
                Self::Leaf(value)
            }
        }
    }
    pub mod type_mixed_head {
        #[derive(Debug)]
        pub enum MixedHead {
            Plain(String),
            Native(String),
        }
    }
    pub mod type_node_type {
        #[derive(Debug)]
        pub struct NodeType {
            pub id: super::super::xs::type_unsigned_int_type::UnsignedIntType,
            pub child: Option<Box<super::type_base_type::BaseType>>,
        }
    }
    pub mod type_ordinary_type {
        #[derive(Debug)]
        pub enum OrdinaryType {
            Text(super::super::xs::type_string_type::StringType),
            Number(super::super::xs::type_int_type::IntType),
        }
    }
    pub mod type_unique {
        #[derive(Debug)]
        pub enum Unique {
            Boolean(bool),
        }
        impl From<bool> for Unique {
            fn from(value: bool) -> Self {
                Self::Boolean(value)
            }
        }
    }
}
pub mod xs {
    pub use self::type_int_type::IntType;
    pub use self::type_string_type::StringType;
    pub use self::type_unsigned_int_type::UnsignedIntType;
    pub mod type_int_type {
        pub type IntType = i32;
    }
    pub mod type_string_type {
        pub type StringType = String;
    }
    pub mod type_unsigned_int_type {
        pub type UnsignedIntType = u32;
    }
}
