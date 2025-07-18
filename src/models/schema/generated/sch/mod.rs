pub mod quick_xml_deserialize;
use crate::quick_xml::{
    DeserializeBytes, DeserializeReader, Error, ErrorKind, RawByteStr, WithDeserializer,
};
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Schema {
    pub id: Option<String>,
    pub icon: Option<String>,
    pub see: Option<String>,
    pub fpi: Option<String>,
    pub lang: Option<String>,
    pub space: Option<super::xml::Space>,
    pub schema_version: Option<String>,
    pub default_phase: Option<String>,
    pub query_binding: Option<String>,
    pub content: Vec<SchemaContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SchemaContent {
    Include(Include),
    Title(Title),
    Ns(Ns),
    P(P),
    Let(Let),
    Phase(Phase),
    Pattern(Pattern),
    Diagnostics(Diagnostics),
    Properties(Properties),
}
impl WithDeserializer for Schema {
    type Deserializer = Box<quick_xml_deserialize::SchemaDeserializer>;
}
impl WithDeserializer for SchemaContent {
    type Deserializer = Box<quick_xml_deserialize::SchemaContentDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Include {
    pub href: String,
}
impl WithDeserializer for Include {
    type Deserializer = Box<quick_xml_deserialize::IncludeDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Title {
    pub dir: Vec<Dir>,
}
impl WithDeserializer for Title {
    type Deserializer = Box<quick_xml_deserialize::TitleDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ns {
    pub uri: String,
    pub prefix: String,
}
impl WithDeserializer for Ns {
    type Deserializer = Box<quick_xml_deserialize::NsDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct P {
    pub id: Option<String>,
    pub class: Option<String>,
    pub icon: Option<String>,
    pub content: Vec<PContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PContent {
    Dir(Dir),
    Emph(Emph),
    Span(Span),
}
impl WithDeserializer for P {
    type Deserializer = Box<quick_xml_deserialize::PDeserializer>;
}
impl WithDeserializer for PContent {
    type Deserializer = Box<quick_xml_deserialize::PContentDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Let {
    pub name: String,
    pub value: Option<String>,
}
impl WithDeserializer for Let {
    type Deserializer = Box<quick_xml_deserialize::LetDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Phase {
    pub id: String,
    pub icon: Option<String>,
    pub see: Option<String>,
    pub fpi: Option<String>,
    pub lang: Option<String>,
    pub space: Option<super::xml::Space>,
    pub content: Vec<PhaseContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PhaseContent {
    Include(Include),
    P(P),
    Let(Let),
    Active(Active),
}
impl WithDeserializer for Phase {
    type Deserializer = Box<quick_xml_deserialize::PhaseDeserializer>;
}
impl WithDeserializer for PhaseContent {
    type Deserializer = Box<quick_xml_deserialize::PhaseContentDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pattern {
    pub documents: Option<String>,
    pub icon: Option<String>,
    pub see: Option<String>,
    pub fpi: Option<String>,
    pub lang: Option<String>,
    pub space: Option<super::xml::Space>,
    pub abstract_: Option<PatternAbstractType>,
    pub id: Option<String>,
    pub is_a: Option<String>,
    pub content: Vec<PatternContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PatternContent {
    Include(Include),
    Title(Title),
    P(P),
    Param(Param),
    Let(Let),
    Rule(Rule),
}
impl WithDeserializer for Pattern {
    type Deserializer = Box<quick_xml_deserialize::PatternDeserializer>;
}
impl WithDeserializer for PatternContent {
    type Deserializer = Box<quick_xml_deserialize::PatternContentDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Diagnostics {
    pub content: Vec<DiagnosticsContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DiagnosticsContent {
    Include(Include),
    Diagnostic(Diagnostic),
}
impl WithDeserializer for Diagnostics {
    type Deserializer = Box<quick_xml_deserialize::DiagnosticsDeserializer>;
}
impl WithDeserializer for DiagnosticsContent {
    type Deserializer = Box<quick_xml_deserialize::DiagnosticsContentDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Properties {
    pub property: Vec<Property>,
}
impl WithDeserializer for Properties {
    type Deserializer = Box<quick_xml_deserialize::PropertiesDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Dir {
    pub value: Option<DirValueType>,
}
impl WithDeserializer for Dir {
    type Deserializer = Box<quick_xml_deserialize::DirDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Emph;
impl WithDeserializer for Emph {
    type Deserializer = Box<quick_xml_deserialize::EmphDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Span {
    pub class: String,
}
impl WithDeserializer for Span {
    type Deserializer = Box<quick_xml_deserialize::SpanDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Active {
    pub pattern: String,
    pub content: Vec<ActiveContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ActiveContent {
    Dir(Dir),
    Emph(Emph),
    Span(Span),
}
impl WithDeserializer for Active {
    type Deserializer = Box<quick_xml_deserialize::ActiveDeserializer>;
}
impl WithDeserializer for ActiveContent {
    type Deserializer = Box<quick_xml_deserialize::ActiveContentDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PatternAbstractType {
    True,
    False,
}
impl DeserializeBytes for PatternAbstractType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"true" => Ok(Self::True),
            b"false" => Ok(Self::False),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Param {
    pub name: String,
    pub value: String,
}
impl WithDeserializer for Param {
    type Deserializer = Box<quick_xml_deserialize::ParamDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Rule {
    pub flag: Option<String>,
    pub icon: Option<String>,
    pub see: Option<String>,
    pub fpi: Option<String>,
    pub lang: Option<String>,
    pub space: Option<super::xml::Space>,
    pub role: Option<String>,
    pub subject: Option<String>,
    pub abstract_: Option<PatternAbstractType>,
    pub id: Option<String>,
    pub context: Option<String>,
    pub content: Vec<RuleContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RuleContent {
    Include(Include),
    Let(Let),
    Assert(Assert),
    Report(Report),
    Extends(Extends),
}
impl WithDeserializer for Rule {
    type Deserializer = Box<quick_xml_deserialize::RuleDeserializer>;
}
impl WithDeserializer for RuleContent {
    type Deserializer = Box<quick_xml_deserialize::RuleContentDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Diagnostic {
    pub id: String,
    pub icon: Option<String>,
    pub see: Option<String>,
    pub fpi: Option<String>,
    pub lang: Option<String>,
    pub space: Option<super::xml::Space>,
    pub content: Vec<DiagnosticContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DiagnosticContent {
    ValueOf(ValueOf),
    Emph(Emph),
    Dir(Dir),
    Span(Span),
}
impl WithDeserializer for Diagnostic {
    type Deserializer = Box<quick_xml_deserialize::DiagnosticDeserializer>;
}
impl WithDeserializer for DiagnosticContent {
    type Deserializer = Box<quick_xml_deserialize::DiagnosticContentDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Property {
    pub id: String,
    pub role: Option<String>,
    pub scheme: Option<String>,
    pub content: Vec<PropertyContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PropertyContent {
    Name(Name),
    ValueOf(ValueOf),
    Emph(Emph),
    Dir(Dir),
    Span(Span),
}
impl WithDeserializer for Property {
    type Deserializer = Box<quick_xml_deserialize::PropertyDeserializer>;
}
impl WithDeserializer for PropertyContent {
    type Deserializer = Box<quick_xml_deserialize::PropertyContentDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DirValueType {
    Ltr,
    Rtl,
}
impl DeserializeBytes for DirValueType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"ltr" => Ok(Self::Ltr),
            b"rtl" => Ok(Self::Rtl),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Assert {
    pub test: String,
    pub flag: Option<String>,
    pub id: Option<String>,
    pub diagnostics: Option<super::xs::EntitiesType>,
    pub properties: Option<super::xs::EntitiesType>,
    pub icon: Option<String>,
    pub see: Option<String>,
    pub fpi: Option<String>,
    pub lang: Option<String>,
    pub space: Option<super::xml::Space>,
    pub role: Option<String>,
    pub subject: Option<String>,
    pub content: Vec<AssertContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AssertContent {
    Name(Name),
    ValueOf(ValueOf),
    Emph(Emph),
    Dir(Dir),
    Span(Span),
}
impl WithDeserializer for Assert {
    type Deserializer = Box<quick_xml_deserialize::AssertDeserializer>;
}
impl WithDeserializer for AssertContent {
    type Deserializer = Box<quick_xml_deserialize::AssertContentDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Report {
    pub test: String,
    pub flag: Option<String>,
    pub id: Option<String>,
    pub diagnostics: Option<super::xs::EntitiesType>,
    pub properties: Option<super::xs::EntitiesType>,
    pub icon: Option<String>,
    pub see: Option<String>,
    pub fpi: Option<String>,
    pub lang: Option<String>,
    pub space: Option<super::xml::Space>,
    pub role: Option<String>,
    pub subject: Option<String>,
    pub content: Vec<ReportContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ReportContent {
    Name(Name),
    ValueOf(ValueOf),
    Emph(Emph),
    Dir(Dir),
    Span(Span),
}
impl WithDeserializer for Report {
    type Deserializer = Box<quick_xml_deserialize::ReportDeserializer>;
}
impl WithDeserializer for ReportContent {
    type Deserializer = Box<quick_xml_deserialize::ReportContentDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Extends {
    pub rule: Option<String>,
    pub href: Option<String>,
}
impl WithDeserializer for Extends {
    type Deserializer = Box<quick_xml_deserialize::ExtendsDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValueOf {
    pub select: String,
}
impl WithDeserializer for ValueOf {
    type Deserializer = Box<quick_xml_deserialize::ValueOfDeserializer>;
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Name {
    pub path: Option<String>,
}
impl WithDeserializer for Name {
    type Deserializer = Box<quick_xml_deserialize::NameDeserializer>;
}
