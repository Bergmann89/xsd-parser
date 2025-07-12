use xsd_parser::xml::Text;
pub struct MixedChoiceListType {
    pub content: Vec<MixedChoiceListTypeContent>,
}
pub enum MixedChoiceListTypeContent {
    Fuu(i32),
    Bar(String),
    Text(Text),
}
pub struct MixedSequenceType {
    pub text_before: Option<Text>,
    pub fuu: i32,
    pub text_after_fuu: Option<Text>,
    pub bar: String,
    pub text_after_bar: Option<Text>,
}
