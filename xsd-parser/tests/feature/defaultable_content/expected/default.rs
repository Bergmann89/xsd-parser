pub type Simple = SimpleType;
#[derive(Debug)]
pub struct SimpleType {
    pub attrib_a: Option<xs::StringType>,
    pub attrib_b: Option<xs::StringType>,
    pub content: xs::StringType,
}
pub type Sequence = SequenceType;
#[derive(Debug)]
pub struct SequenceType {
    pub attrib_a: Option<xs::StringType>,
    pub attrib_b: Option<xs::StringType>,
    pub content: SequenceTypeContent,
}
#[derive(Debug)]
pub struct SequenceTypeContent {
    pub a: Option<xs::StringType>,
    pub b: Option<xs::StringType>,
    pub c: Option<xs::StringType>,
}
pub type NestedSeq = NestedSeqType;
#[derive(Debug)]
pub struct NestedSeqType {
    pub attrib_a: Option<xs::StringType>,
    pub attrib_b: Option<xs::StringType>,
    pub content: NestedSeqTypeContent,
}
#[derive(Debug)]
pub struct NestedSeqTypeContent {
    pub inner_choice: Option<NestedSeqInnerChoiceType>,
}
#[derive(Debug)]
pub struct NestedSeqInnerChoiceType {
    pub content: NestedSeqInnerChoiceTypeContent,
}
#[derive(Debug)]
pub enum NestedSeqInnerChoiceTypeContent {
    FinalSeq(Option<NestedSeqFinalSeqType>),
}
#[derive(Debug)]
pub struct NestedSeqFinalSeqType {
    pub content: NestedSeqFinalSeqTypeContent,
}
#[derive(Debug)]
pub struct NestedSeqFinalSeqTypeContent {
    pub a: Option<xs::StringType>,
    pub b: Option<xs::StringType>,
    pub c: Option<xs::StringType>,
}
pub mod xs {
    pub type StringType = String;
}
