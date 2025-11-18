pub enum MyUnionType {
    Language(LanguageType),
    String(StringType),
}
pub type LanguageType = String;
pub type StringType = String;
