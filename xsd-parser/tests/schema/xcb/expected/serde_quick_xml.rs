use serde::{Deserialize, Serialize};
pub type Xcb = XcbType;
#[derive(Debug, Deserialize, Serialize)]
pub struct XcbType {
    #[serde(rename = "@header")]
    pub header: String,
    #[serde(default, rename = "@extension-xname")]
    pub extension_xname: Option<String>,
    #[serde(default, rename = "@extension-name")]
    pub extension_name: Option<String>,
    #[serde(
        default = "XcbType::default_extension_multiword",
        rename = "@extension-multiword"
    )]
    pub extension_multiword: bool,
    #[serde(default, rename = "@major-version")]
    pub major_version: Option<i32>,
    #[serde(default, rename = "@minor-version")]
    pub minor_version: Option<i32>,
    #[serde(default, rename = "$value")]
    pub content: Vec<XcbTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum XcbTypeContent {
    #[serde(rename = "request")]
    Request(RequestType),
    #[serde(rename = "event")]
    Event(EventType),
    #[serde(rename = "eventcopy")]
    Eventcopy(PacketStructCopyType),
    #[serde(rename = "error")]
    Error(PacketStructType),
    #[serde(rename = "errorcopy")]
    Errorcopy(PacketStructCopyType),
    #[serde(rename = "struct")]
    Struct(StructType),
    #[serde(rename = "union")]
    Union(StructType),
    #[serde(rename = "xidtype")]
    Xidtype(XidtypeType),
    #[serde(rename = "xidunion")]
    Xidunion(XidunionType),
    #[serde(rename = "enum")]
    Enum(EnumType),
    #[serde(rename = "typedef")]
    Typedef(TypedefType),
    #[serde(rename = "import")]
    Import(String),
}
impl XcbType {
    #[must_use]
    pub fn default_extension_multiword() -> bool {
        false
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RequestType {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@opcode")]
    pub opcode: i32,
    #[serde(default, rename = "@combine-adjacent")]
    pub combine_adjacent: Option<bool>,
    #[serde(default, rename = "$value")]
    pub content: Vec<RequestTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum RequestTypeContent {
    #[serde(rename = "pad")]
    Pad(PadType),
    #[serde(rename = "field")]
    Field(VarType),
    #[serde(rename = "list")]
    List(ListType),
    #[serde(rename = "fd")]
    Fd(AnyType),
    #[serde(rename = "exprfield")]
    Exprfield(ExprfieldType),
    #[serde(rename = "valueparam")]
    Valueparam(ValueparamType),
    #[serde(rename = "switch")]
    Switch(SwitchexprType),
    #[serde(rename = "reply")]
    Reply(RequestReplyType),
    #[serde(rename = "doc")]
    Doc(DocType),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct EventType {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@number")]
    pub number: i32,
    #[serde(default, rename = "@no-sequence-number")]
    pub no_sequence_number: Option<bool>,
    #[serde(default, rename = "@xge")]
    pub xge: Option<bool>,
    #[serde(default, rename = "$value")]
    pub content: Vec<EventTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum EventTypeContent {
    #[serde(rename = "pad")]
    Pad(PadType),
    #[serde(rename = "field")]
    Field(VarType),
    #[serde(rename = "list")]
    List(ListType),
    #[serde(rename = "fd")]
    Fd(AnyType),
    #[serde(rename = "doc")]
    Doc(DocType),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PacketStructCopyType {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@number")]
    pub number: i32,
    #[serde(rename = "@ref")]
    pub ref_: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PacketStructType {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@number")]
    pub number: i32,
    #[serde(default, rename = "$value")]
    pub content: Vec<PacketStructTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum PacketStructTypeContent {
    #[serde(rename = "pad")]
    Pad(PadType),
    #[serde(rename = "field")]
    Field(VarType),
    #[serde(rename = "list")]
    List(ListType),
    #[serde(rename = "fd")]
    Fd(AnyType),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct StructType {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value")]
    pub content: Vec<StructTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum StructTypeContent {
    #[serde(rename = "pad")]
    Pad(PadType),
    #[serde(rename = "field")]
    Field(VarType),
    #[serde(rename = "list")]
    List(ListType),
    #[serde(rename = "fd")]
    Fd(AnyType),
    #[serde(rename = "switch")]
    Switch(SwitchexprType),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct XidtypeType {
    #[serde(rename = "@name")]
    pub name: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct XidunionType {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(default, rename = "type")]
    pub type_: Vec<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct EnumType {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value")]
    pub content: Vec<EnumTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct EnumTypeContent {
    #[serde(rename = "item")]
    pub item: EnumItemType,
    #[serde(default, rename = "doc")]
    pub doc: Option<DocType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TypedefType {
    #[serde(rename = "@oldname")]
    pub oldname: String,
    #[serde(rename = "@newname")]
    pub newname: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PadType {
    #[serde(default, rename = "@bytes")]
    pub bytes: Option<i32>,
    #[serde(default, rename = "@align")]
    pub align: Option<i32>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct VarType {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub type_: String,
    #[serde(default, rename = "@enum")]
    pub enum_: Option<String>,
    #[serde(default, rename = "@altenum")]
    pub altenum: Option<String>,
    #[serde(default, rename = "@mask")]
    pub mask: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ListType {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub type_: String,
    #[serde(default, rename = "@enum")]
    pub enum_: Option<String>,
    #[serde(default, rename = "@altenum")]
    pub altenum: Option<String>,
    #[serde(default, rename = "@mask")]
    pub mask: Option<String>,
    #[serde(default, rename = "$value")]
    pub content: Option<ListTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum ListTypeContent {
    #[serde(rename = "op")]
    Op(OpType),
    #[serde(rename = "unop")]
    Unop(UnopType),
    #[serde(rename = "fieldref")]
    Fieldref(String),
    #[serde(rename = "enumref")]
    Enumref(EnumrefType),
    #[serde(rename = "popcount")]
    Popcount(PopcountType),
    #[serde(rename = "sumof")]
    Sumof(SumofType),
    #[serde(rename = "value")]
    Value(DecOrHexIntegerType),
    #[serde(rename = "bit")]
    Bit(i32),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct AnyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ExprfieldType {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub type_: String,
    #[serde(default, rename = "@enum")]
    pub enum_: Option<String>,
    #[serde(default, rename = "@altenum")]
    pub altenum: Option<String>,
    #[serde(default, rename = "@mask")]
    pub mask: Option<String>,
    #[serde(rename = "$value")]
    pub content: ExprfieldTypeContent,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum ExprfieldTypeContent {
    #[serde(rename = "op")]
    Op(OpType),
    #[serde(rename = "unop")]
    Unop(UnopType),
    #[serde(rename = "fieldref")]
    Fieldref(String),
    #[serde(rename = "enumref")]
    Enumref(EnumrefType),
    #[serde(rename = "popcount")]
    Popcount(PopcountType),
    #[serde(rename = "sumof")]
    Sumof(SumofType),
    #[serde(rename = "value")]
    Value(DecOrHexIntegerType),
    #[serde(rename = "bit")]
    Bit(i32),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ValueparamType {
    #[serde(rename = "@value-mask-type")]
    pub value_mask_type: String,
    #[serde(rename = "@value-mask-name")]
    pub value_mask_name: String,
    #[serde(rename = "@value-list-name")]
    pub value_list_name: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SwitchexprType {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value")]
    pub content: Vec<SwitchexprTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum SwitchexprTypeContent {
    #[serde(rename = "op")]
    Op(OpType),
    #[serde(rename = "unop")]
    Unop(UnopType),
    #[serde(rename = "fieldref")]
    Fieldref(String),
    #[serde(rename = "enumref")]
    Enumref(EnumrefType),
    #[serde(rename = "popcount")]
    Popcount(PopcountType),
    #[serde(rename = "sumof")]
    Sumof(SumofType),
    #[serde(rename = "value")]
    Value(DecOrHexIntegerType),
    #[serde(rename = "bit")]
    Bit(i32),
    #[serde(rename = "bitcase")]
    Bitcase(CaseexprType),
    #[serde(rename = "pad")]
    Pad(PadType),
    #[serde(rename = "field")]
    Field(VarType),
    #[serde(rename = "list")]
    List(ListType),
    #[serde(rename = "fd")]
    Fd(AnyType),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RequestReplyType {
    #[serde(rename = "$value")]
    pub content: Vec<RequestReplyTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum RequestReplyTypeContent {
    #[serde(rename = "pad")]
    Pad(PadType),
    #[serde(rename = "field")]
    Field(VarType),
    #[serde(rename = "list")]
    List(ListType),
    #[serde(rename = "fd")]
    Fd(AnyType),
    #[serde(rename = "valueparam")]
    Valueparam(ValueparamType),
    #[serde(rename = "switch")]
    Switch(SwitchexprType),
    #[serde(rename = "doc")]
    Doc(DocType),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct DocType {
    #[serde(default, rename = "$value")]
    pub content: Vec<DocTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum DocTypeContent {
    #[serde(rename = "brief")]
    Brief(String),
    #[serde(rename = "description")]
    Description(String),
    #[serde(rename = "example")]
    Example(String),
    #[serde(rename = "field")]
    Field(FieldType),
    #[serde(rename = "error")]
    Error(ErrorType),
    #[serde(rename = "see")]
    See(SeeType),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct EnumItemType {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value")]
    pub content: EnumItemTypeContent,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum EnumItemTypeContent {
    #[serde(rename = "value")]
    Value(DecOrHexIntegerType),
    #[serde(rename = "bit")]
    Bit(i32),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct OpType {
    #[serde(rename = "@op")]
    pub op: String,
    #[serde(rename = "$value")]
    pub content: [OpTypeContent; 2usize],
}
#[derive(Debug, Deserialize, Serialize)]
pub enum OpTypeContent {
    #[serde(rename = "op")]
    Op(Box<OpType>),
    #[serde(rename = "unop")]
    Unop(UnopType),
    #[serde(rename = "fieldref")]
    Fieldref(String),
    #[serde(rename = "enumref")]
    Enumref(EnumrefType),
    #[serde(rename = "popcount")]
    Popcount(PopcountType),
    #[serde(rename = "sumof")]
    Sumof(SumofType),
    #[serde(rename = "value")]
    Value(DecOrHexIntegerType),
    #[serde(rename = "bit")]
    Bit(i32),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UnopType {
    #[serde(rename = "@op")]
    pub op: String,
    #[serde(rename = "$value")]
    pub content: UnopTypeContent,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum UnopTypeContent {
    #[serde(rename = "op")]
    Op(Box<OpType>),
    #[serde(rename = "unop")]
    Unop(Box<UnopType>),
    #[serde(rename = "fieldref")]
    Fieldref(String),
    #[serde(rename = "enumref")]
    Enumref(EnumrefType),
    #[serde(rename = "popcount")]
    Popcount(PopcountType),
    #[serde(rename = "sumof")]
    Sumof(SumofType),
    #[serde(rename = "value")]
    Value(DecOrHexIntegerType),
    #[serde(rename = "bit")]
    Bit(i32),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct EnumrefType {
    #[serde(rename = "@ref")]
    pub ref_: String,
    #[serde(default, rename = "$text")]
    pub content: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum PopcountType {
    #[serde(rename = "op")]
    Op(Box<OpType>),
    #[serde(rename = "unop")]
    Unop(Box<UnopType>),
    #[serde(rename = "fieldref")]
    Fieldref(String),
    #[serde(rename = "enumref")]
    Enumref(EnumrefType),
    #[serde(rename = "popcount")]
    Popcount(Box<PopcountType>),
    #[serde(rename = "sumof")]
    Sumof(SumofType),
    #[serde(rename = "value")]
    Value(DecOrHexIntegerType),
    #[serde(rename = "bit")]
    Bit(i32),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SumofType {
    #[serde(rename = "@ref")]
    pub ref_: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum DecOrHexIntegerType {
    I32(i32),
    String(String),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CaseexprType {
    #[serde(default, rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "$value")]
    pub content: Vec<CaseexprTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum CaseexprTypeContent {
    #[serde(rename = "op")]
    Op(OpType),
    #[serde(rename = "unop")]
    Unop(UnopType),
    #[serde(rename = "fieldref")]
    Fieldref(String),
    #[serde(rename = "enumref")]
    Enumref(EnumrefType),
    #[serde(rename = "popcount")]
    Popcount(PopcountType),
    #[serde(rename = "sumof")]
    Sumof(SumofType),
    #[serde(rename = "value")]
    Value(DecOrHexIntegerType),
    #[serde(rename = "bit")]
    Bit(i32),
    #[serde(rename = "pad")]
    Pad(PadType),
    #[serde(rename = "field")]
    Field(VarType),
    #[serde(rename = "list")]
    List(ListType),
    #[serde(rename = "fd")]
    Fd(AnyType),
    #[serde(rename = "switch")]
    Switch(SwitchexprType),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct FieldType {
    #[serde(default, rename = "@name")]
    pub name: Option<String>,
    #[serde(default, rename = "$text")]
    pub content: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorType {
    #[serde(default, rename = "@type")]
    pub type_: Option<String>,
    #[serde(default, rename = "$text")]
    pub content: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SeeType {
    #[serde(default, rename = "@name")]
    pub name: Option<String>,
    #[serde(default, rename = "@type")]
    pub type_: Option<String>,
}
