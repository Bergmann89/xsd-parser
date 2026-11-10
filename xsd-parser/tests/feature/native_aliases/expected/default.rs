pub mod ex {
    pub type Message = MessageType;
    #[derive(Debug)]
    pub struct MessageType {
        pub value: RenamedValue,
        pub sibling: SiblingType,
        pub local: LocalType,
        pub crate_value: CrateValueType,
    }
    ///First paragraph.
    ///
    ///Second paragraph.
    pub type RenamedValue = ::std::string::String;
    pub type SiblingType = super::native::Text;
    pub type LocalType = super::native::Text;
    pub type CrateValueType = crate::feature::native_aliases::support::Text;
}
