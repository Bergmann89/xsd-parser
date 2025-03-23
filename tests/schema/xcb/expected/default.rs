pub type Xcb = XcbType;
#[derive(Debug, Clone)]
pub struct XcbType {
    pub header: String,
    pub extension_xname: Option<String>,
    pub extension_name: Option<String>,
    pub extension_multiword: bool,
    pub major_version: Option<i32>,
    pub minor_version: Option<i32>,
    pub content: Vec<XcbTypeContent>,
}
#[derive(Debug, Clone)]
pub enum XcbTypeContent {
    Request(RequestType),
    Event(EventType),
    Eventcopy(PacketStructCopyType),
    Error(PacketStructType),
    Errorcopy(PacketStructCopyType),
    Struct(StructType),
    Union(StructType),
    Xidtype(XidtypeType),
    Xidunion(XidunionType),
    Enum(EnumType),
    Typedef(TypedefType),
    Import(String),
}
impl XcbType {
    #[must_use]
    pub fn default_extension_multiword() -> bool {
        false
    }
}
#[derive(Debug, Clone)]
pub struct RequestType {
    pub name: String,
    pub opcode: i32,
    pub combine_adjacent: Option<bool>,
    pub content: Vec<RequestTypeContent>,
}
#[derive(Debug, Clone)]
pub enum RequestTypeContent {
    Pad(PadType),
    Field(VarType),
    List(ListType),
    Fd(AnyType),
    Exprfield(ExprfieldType),
    Valueparam(ValueparamType),
    Switch(SwitchexprType),
    Reply(RequestReplyType),
    Doc(DocType),
}
#[derive(Debug, Clone)]
pub struct EventType {
    pub name: String,
    pub number: i32,
    pub no_sequence_number: Option<bool>,
    pub xge: Option<bool>,
    pub content: Vec<EventTypeContent>,
}
#[derive(Debug, Clone)]
pub enum EventTypeContent {
    Pad(PadType),
    Field(VarType),
    List(ListType),
    Fd(AnyType),
    Doc(DocType),
}
#[derive(Debug, Clone)]
pub struct PacketStructCopyType {
    pub name: String,
    pub number: i32,
    pub ref_: String,
}
#[derive(Debug, Clone)]
pub struct PacketStructType {
    pub name: String,
    pub number: i32,
    pub content: Vec<PacketStructTypeContent>,
}
#[derive(Debug, Clone)]
pub enum PacketStructTypeContent {
    Pad(PadType),
    Field(VarType),
    List(ListType),
    Fd(AnyType),
}
#[derive(Debug, Clone)]
pub struct StructType {
    pub name: String,
    pub content: Vec<StructTypeContent>,
}
#[derive(Debug, Clone)]
pub enum StructTypeContent {
    Pad(PadType),
    Field(VarType),
    List(ListType),
    Fd(AnyType),
    Switch(SwitchexprType),
}
#[derive(Debug, Clone)]
pub struct XidtypeType {
    pub name: String,
}
#[derive(Debug, Clone)]
pub struct XidunionType {
    pub name: String,
    pub type_: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct EnumType {
    pub name: String,
    pub content: Vec<EnumTypeContent>,
}
#[derive(Debug, Clone)]
pub struct EnumTypeContent {
    pub item: EnumItemType,
    pub doc: Option<DocType>,
}
#[derive(Debug, Clone)]
pub struct TypedefType {
    pub oldname: String,
    pub newname: String,
}
#[derive(Debug, Clone)]
pub struct PadType {
    pub bytes: Option<i32>,
    pub align: Option<i32>,
}
#[derive(Debug, Clone)]
pub struct VarType {
    pub name: String,
    pub type_: String,
    pub enum_: Option<String>,
    pub altenum: Option<String>,
    pub mask: Option<String>,
}
#[derive(Debug, Clone)]
pub struct ListType {
    pub name: String,
    pub type_: String,
    pub enum_: Option<String>,
    pub altenum: Option<String>,
    pub mask: Option<String>,
    pub content: Option<ListTypeContent>,
}
#[derive(Debug, Clone)]
pub enum ListTypeContent {
    Op(OpType),
    Unop(UnopType),
    Fieldref(String),
    Enumref(EnumrefType),
    Popcount(PopcountType),
    Sumof(SumofType),
    Value(DecOrHexIntegerType),
    Bit(i32),
}
#[derive(Debug, Clone)]
pub struct AnyType;
#[derive(Debug, Clone)]
pub struct ExprfieldType {
    pub name: String,
    pub type_: String,
    pub enum_: Option<String>,
    pub altenum: Option<String>,
    pub mask: Option<String>,
    pub content: ExprfieldTypeContent,
}
#[derive(Debug, Clone)]
pub enum ExprfieldTypeContent {
    Op(OpType),
    Unop(UnopType),
    Fieldref(String),
    Enumref(EnumrefType),
    Popcount(PopcountType),
    Sumof(SumofType),
    Value(DecOrHexIntegerType),
    Bit(i32),
}
#[derive(Debug, Clone)]
pub struct ValueparamType {
    pub value_mask_type: String,
    pub value_mask_name: String,
    pub value_list_name: String,
}
#[derive(Debug, Clone)]
pub struct SwitchexprType {
    pub name: String,
    pub content: Vec<SwitchexprTypeContent>,
}
#[derive(Debug, Clone)]
pub enum SwitchexprTypeContent {
    Op(OpType),
    Unop(UnopType),
    Fieldref(String),
    Enumref(EnumrefType),
    Popcount(PopcountType),
    Sumof(SumofType),
    Value(DecOrHexIntegerType),
    Bit(i32),
    Bitcase(CaseexprType),
    Pad(PadType),
    Field(VarType),
    List(ListType),
    Fd(AnyType),
}
#[derive(Debug, Clone)]
pub struct RequestReplyType {
    pub content: Vec<RequestReplyTypeContent>,
}
#[derive(Debug, Clone)]
pub enum RequestReplyTypeContent {
    Pad(PadType),
    Field(VarType),
    List(ListType),
    Fd(AnyType),
    Valueparam(ValueparamType),
    Switch(SwitchexprType),
    Doc(DocType),
}
#[derive(Debug, Clone)]
pub struct DocType {
    pub content: Vec<DocTypeContent>,
}
#[derive(Debug, Clone)]
pub enum DocTypeContent {
    Brief(String),
    Description(String),
    Example(String),
    Field(FieldType),
    Error(PacketStructType),
    See(SeeType),
}
#[derive(Debug, Clone)]
pub struct EnumItemType {
    pub name: String,
    pub content: EnumItemTypeContent,
}
#[derive(Debug, Clone)]
pub enum EnumItemTypeContent {
    Value(DecOrHexIntegerType),
    Bit(i32),
}
#[derive(Debug, Clone)]
pub struct OpType {
    pub op: String,
    pub content: [OpTypeContent; 8usize],
}
#[derive(Debug, Clone)]
pub enum OpTypeContent {
    Op(Box<OpType>),
    Unop(Box<UnopType>),
    Fieldref(String),
    Enumref(EnumrefType),
    Popcount(Box<PopcountType>),
    Sumof(SumofType),
    Value(DecOrHexIntegerType),
    Bit(i32),
}
#[derive(Debug, Clone)]
pub struct UnopType {
    pub op: String,
    pub content: [UnopTypeContent; 8usize],
}
#[derive(Debug, Clone)]
pub enum UnopTypeContent {
    Op(Box<OpType>),
    Unop(Box<UnopType>),
    Fieldref(String),
    Enumref(EnumrefType),
    Popcount(Box<PopcountType>),
    Sumof(SumofType),
    Value(DecOrHexIntegerType),
    Bit(i32),
}
#[derive(Debug, Clone)]
pub struct EnumrefType {
    pub ref_: String,
    pub content: String,
}
#[derive(Debug, Clone)]
pub enum PopcountType {
    Op(Box<OpType>),
    Unop(Box<UnopType>),
    Fieldref(String),
    Enumref(EnumrefType),
    Popcount(Box<PopcountType>),
    Sumof(SumofType),
    Value(DecOrHexIntegerType),
    Bit(i32),
}
#[derive(Debug, Clone)]
pub struct SumofType {
    pub ref_: String,
}
#[derive(Debug, Clone)]
pub enum DecOrHexIntegerType {
    I32(i32),
    String(String),
}
#[derive(Debug, Clone)]
pub struct CaseexprType {
    pub name: Option<String>,
    pub content: Vec<CaseexprTypeContent>,
}
#[derive(Debug, Clone)]
pub enum CaseexprTypeContent {
    Op(OpType),
    Unop(UnopType),
    Fieldref(String),
    Enumref(EnumrefType),
    Popcount(PopcountType),
    Sumof(SumofType),
    Value(DecOrHexIntegerType),
    Bit(i32),
    Pad(PadType),
    Field(VarType),
    List(ListType),
    Fd(AnyType),
    Switch(SwitchexprType),
}
#[derive(Debug, Clone)]
pub struct FieldType {
    pub name: Option<String>,
    pub content: String,
}
#[derive(Debug, Clone)]
pub struct SeeType {
    pub name: Option<String>,
    pub type_: Option<String>,
}
