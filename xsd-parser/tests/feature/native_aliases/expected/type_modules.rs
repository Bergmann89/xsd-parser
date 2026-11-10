pub mod ex {
    pub use self::type_crate_value_type::CrateValueType;
    pub use self::type_local_type::LocalType;
    pub use self::type_message::Message;
    pub use self::type_message_type::MessageType;
    pub use self::type_renamed_value::RenamedValue;
    pub use self::type_sibling_type::SiblingType;
    pub mod type_crate_value_type {
        pub type CrateValueType = crate::feature::native_aliases::support::Text;
    }
    pub mod type_local_type {
        pub type LocalType = super::super::native::Text;
    }
    pub mod type_message {
        pub type Message = super::type_message_type::MessageType;
    }
    pub mod type_message_type {
        #[derive(Debug)]
        pub struct MessageType {
            pub value: super::type_renamed_value::RenamedValue,
            pub sibling: super::type_sibling_type::SiblingType,
            pub local: super::type_local_type::LocalType,
            pub crate_value: super::type_crate_value_type::CrateValueType,
        }
    }
    pub mod type_renamed_value {
        ///First paragraph.
        ///
        ///Second paragraph.
        pub type RenamedValue = ::std::string::String;
    }
    pub mod type_sibling_type {
        pub type SiblingType = super::super::native::Text;
    }
}
