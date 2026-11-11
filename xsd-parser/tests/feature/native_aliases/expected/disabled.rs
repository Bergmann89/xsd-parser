pub mod ex {
    pub type Message = MessageType;
    #[derive(Debug)]
    pub struct MessageType {
        pub value: ::std::string::String,
        pub sibling: super::native::Text,
        pub local: super::native::Text,
        pub crate_value: crate::feature::native_aliases::support::Text,
    }
}
