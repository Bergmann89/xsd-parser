#[doc = "SomeDetails doc string 1"]
#[doc = "SomeDetails doc string 2"]
#[doc = "This is a multi line comment with mixed content."]
#[doc = "<div>"]
#[doc = "    You could use for example HTML to apply some <b>formatting</b> to your documentation."]
#[doc = "    Of cause this must be supported by your language, but for rust docs this should work :)"]
#[doc = "</div>"]
pub type SomeDetails = SomeDetailsType;
#[derive(Debug)]
pub struct SomeDetailsType {
    #[doc = "Documentation for attributes is also possible."]
    pub test_attrib: Option<IntType>,
    #[doc = "Unique user identifier. Range 1000-99999"]
    pub id: IntType,
}
pub type IntType = i32;
