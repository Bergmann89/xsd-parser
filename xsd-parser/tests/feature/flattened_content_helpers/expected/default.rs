pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub content: Vec<FooTypeContent>,
}
#[derive(Debug)]
pub enum FooTypeContent {
    Bar(String),
    Baz(i32),
}
impl FooType {
    #[inline]
    pub fn bar(&self) -> Option<&String> {
        self.content.iter().find_map(|x| match x {
            FooTypeContent::Bar(v) => Option::Some(v),
            _ => Option::None,
        })
    }
    #[inline]
    pub fn bar_mut(&mut self) -> Option<&mut String> {
        self.content.iter_mut().find_map(|x| match x {
            FooTypeContent::Bar(v) => Option::Some(v),
            _ => Option::None,
        })
    }
    #[inline]
    pub fn baz(&self) -> Option<&i32> {
        self.content.iter().find_map(|x| match x {
            FooTypeContent::Baz(v) => Option::Some(v),
            _ => Option::None,
        })
    }
    #[inline]
    pub fn baz_mut(&mut self) -> Option<&mut i32> {
        self.content.iter_mut().find_map(|x| match x {
            FooTypeContent::Baz(v) => Option::Some(v),
            _ => Option::None,
        })
    }
}
