pub mod ex {
    use serde::{Deserialize, Serialize};
    pub type Message = MessageType;
    #[derive(Debug, Deserialize, Serialize)]
    pub struct MessageType {
        #[serde(rename = "Value")]
        pub value: RenamedValue,
        #[serde(rename = "Sibling")]
        pub sibling: SiblingType,
        #[serde(rename = "Local")]
        pub local: LocalType,
        #[serde(rename = "CrateValue")]
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
