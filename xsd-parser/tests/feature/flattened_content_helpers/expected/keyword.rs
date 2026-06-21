pub type KeywordFoo = KeywordFooType;
#[derive(Debug)]
pub struct KeywordFooType {
    pub content: Vec<KeywordFooTypeContent>,
}
#[derive(Debug)]
pub enum KeywordFooTypeContent {
    Type(String),
    Match(i32),
}
impl KeywordFooType {
    #[inline]
    pub fn type_(&self) -> Option<&String> {
        self.content.iter().find_map(|x| match x {
            KeywordFooTypeContent::Type(v) => Option::Some(v),
            _ => Option::None,
        })
    }
    #[inline]
    pub fn type_mut(&mut self) -> Option<&mut String> {
        self.content.iter_mut().find_map(|x| match x {
            KeywordFooTypeContent::Type(v) => Option::Some(v),
            _ => Option::None,
        })
    }
    #[inline]
    pub fn match_(&self) -> Option<&i32> {
        self.content.iter().find_map(|x| match x {
            KeywordFooTypeContent::Match(v) => Option::Some(v),
            _ => Option::None,
        })
    }
    #[inline]
    pub fn match_mut(&mut self) -> Option<&mut i32> {
        self.content.iter_mut().find_map(|x| match x {
            KeywordFooTypeContent::Match(v) => Option::Some(v),
            _ => Option::None,
        })
    }
}
