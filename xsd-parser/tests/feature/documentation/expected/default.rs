///SomeDetails doc string 1
///SomeDetails doc string 2
///This is a multi line comment with mixed content.
///<div>
///    You could use for example HTML to apply some <b>formatting</b> to your documentation.
///    Of cause this must be supported by your language, but for rust docs this should work :)
///</div>
pub type SomeDetails = SomeDetailsType;
#[derive(Debug)]
pub struct SomeDetailsType {
    ///Documentation for attributes is also possible.
    pub test_attrib: Option<IntType>,
    ///Unique user identifier. Range 1000-99999
    pub id: IntType,
}
pub type IntType = i32;
