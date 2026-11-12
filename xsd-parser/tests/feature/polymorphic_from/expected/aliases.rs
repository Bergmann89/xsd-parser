#[derive(Debug)]
pub enum ScalarHead {
    StandardNumber(u32),
    AliasedNumber(NativeNumberType),
    Boolean(bool),
}
impl From<bool> for ScalarHead {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
pub type NativeNumberType = u32;
#[derive(Debug)]
pub enum TextHead {
    StandardText(String),
    CustomText(String),
}
#[derive(Debug)]
pub enum PathHead {
    Bare(BareTextType),
    Local(LocalTextType),
    Crate(CrateTextType),
}
pub type BareTextType = native::Text;
pub type LocalTextType = native::Text;
pub type CrateTextType = crate::feature::polymorphic_from::support::Text;
#[derive(Debug)]
pub enum DistinctHead {
    Left(LeftTextType),
    Right(RightTextType),
}
pub type LeftTextType = left::Text;
pub type RightTextType = right::Text;
