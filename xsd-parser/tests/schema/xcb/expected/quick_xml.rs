use std::borrow::Cow;
use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{
        DeserializeBytes, DeserializeHelper, Error, ErrorKind, SerializeBytes, SerializeHelper,
        WithDeserializer, WithSerializer,
    },
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub type Xcb = XcbType;
#[derive(Debug)]
pub struct XcbType {
    pub header: String,
    pub extension_xname: Option<String>,
    pub extension_name: Option<String>,
    pub extension_multiword: bool,
    pub major_version: Option<i32>,
    pub minor_version: Option<i32>,
    pub content: Vec<XcbTypeContent>,
}
#[derive(Debug)]
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
impl WithSerializer for XcbType {
    type Serializer<'x> = quick_xml_serialize::XcbTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::XcbTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::XcbTypeSerializerState::Init__),
            name: name.unwrap_or("xcb"),
            is_root,
        })
    }
}
impl WithSerializer for XcbTypeContent {
    type Serializer<'x> = quick_xml_serialize::XcbTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::XcbTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::XcbTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for XcbType {
    type Deserializer = quick_xml_deserialize::XcbTypeDeserializer;
}
impl WithDeserializer for XcbTypeContent {
    type Deserializer = quick_xml_deserialize::XcbTypeContentDeserializer;
}
#[derive(Debug)]
pub struct RequestType {
    pub name: String,
    pub opcode: i32,
    pub combine_adjacent: Option<bool>,
    pub content: Vec<RequestTypeContent>,
}
#[derive(Debug)]
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
impl WithSerializer for RequestType {
    type Serializer<'x> = quick_xml_serialize::RequestTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::RequestTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::RequestTypeSerializerState::Init__),
            name: name.unwrap_or("Request"),
            is_root,
        })
    }
}
impl WithSerializer for RequestTypeContent {
    type Serializer<'x> = quick_xml_serialize::RequestTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::RequestTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::RequestTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for RequestType {
    type Deserializer = quick_xml_deserialize::RequestTypeDeserializer;
}
impl WithDeserializer for RequestTypeContent {
    type Deserializer = quick_xml_deserialize::RequestTypeContentDeserializer;
}
#[derive(Debug)]
pub struct EventType {
    pub name: String,
    pub number: i32,
    pub no_sequence_number: Option<bool>,
    pub xge: Option<bool>,
    pub content: Vec<EventTypeContent>,
}
#[derive(Debug)]
pub enum EventTypeContent {
    Pad(PadType),
    Field(VarType),
    List(ListType),
    Fd(AnyType),
    Doc(DocType),
}
impl WithSerializer for EventType {
    type Serializer<'x> = quick_xml_serialize::EventTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::EventTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::EventTypeSerializerState::Init__),
            name: name.unwrap_or("Event"),
            is_root,
        })
    }
}
impl WithSerializer for EventTypeContent {
    type Serializer<'x> = quick_xml_serialize::EventTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::EventTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::EventTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for EventType {
    type Deserializer = quick_xml_deserialize::EventTypeDeserializer;
}
impl WithDeserializer for EventTypeContent {
    type Deserializer = quick_xml_deserialize::EventTypeContentDeserializer;
}
#[derive(Debug)]
pub struct PacketStructCopyType {
    pub name: String,
    pub number: i32,
    pub ref_: String,
}
impl WithSerializer for PacketStructCopyType {
    type Serializer<'x> = quick_xml_serialize::PacketStructCopyTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::PacketStructCopyTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::PacketStructCopyTypeSerializerState::Init__),
            name: name.unwrap_or("packet-struct-copy"),
            is_root,
        })
    }
}
impl WithDeserializer for PacketStructCopyType {
    type Deserializer = quick_xml_deserialize::PacketStructCopyTypeDeserializer;
}
#[derive(Debug)]
pub struct PacketStructType {
    pub name: String,
    pub number: i32,
    pub content: Vec<PacketStructTypeContent>,
}
#[derive(Debug)]
pub enum PacketStructTypeContent {
    Pad(PadType),
    Field(VarType),
    List(ListType),
    Fd(AnyType),
}
impl WithSerializer for PacketStructType {
    type Serializer<'x> = quick_xml_serialize::PacketStructTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::PacketStructTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::PacketStructTypeSerializerState::Init__),
            name: name.unwrap_or("packet-struct"),
            is_root,
        })
    }
}
impl WithSerializer for PacketStructTypeContent {
    type Serializer<'x> = quick_xml_serialize::PacketStructTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::PacketStructTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::PacketStructTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for PacketStructType {
    type Deserializer = quick_xml_deserialize::PacketStructTypeDeserializer;
}
impl WithDeserializer for PacketStructTypeContent {
    type Deserializer = quick_xml_deserialize::PacketStructTypeContentDeserializer;
}
#[derive(Debug)]
pub struct StructType {
    pub name: String,
    pub content: Vec<StructTypeContent>,
}
#[derive(Debug)]
pub enum StructTypeContent {
    Pad(PadType),
    Field(VarType),
    List(ListType),
    Fd(AnyType),
    Switch(SwitchexprType),
}
impl WithSerializer for StructType {
    type Serializer<'x> = quick_xml_serialize::StructTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::StructTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::StructTypeSerializerState::Init__),
            name: name.unwrap_or("struct"),
            is_root,
        })
    }
}
impl WithSerializer for StructTypeContent {
    type Serializer<'x> = quick_xml_serialize::StructTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::StructTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::StructTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for StructType {
    type Deserializer = quick_xml_deserialize::StructTypeDeserializer;
}
impl WithDeserializer for StructTypeContent {
    type Deserializer = quick_xml_deserialize::StructTypeContentDeserializer;
}
#[derive(Debug)]
pub struct XidtypeType {
    pub name: String,
}
impl WithSerializer for XidtypeType {
    type Serializer<'x> = quick_xml_serialize::XidtypeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::XidtypeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::XidtypeTypeSerializerState::Init__),
            name: name.unwrap_or("Xidtype"),
            is_root,
        })
    }
}
impl WithDeserializer for XidtypeType {
    type Deserializer = quick_xml_deserialize::XidtypeTypeDeserializer;
}
#[derive(Debug)]
pub struct XidunionType {
    pub name: String,
    pub type_: Vec<String>,
}
impl WithSerializer for XidunionType {
    type Serializer<'x> = quick_xml_serialize::XidunionTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::XidunionTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::XidunionTypeSerializerState::Init__),
            name: name.unwrap_or("Xidunion"),
            is_root,
        })
    }
}
impl WithDeserializer for XidunionType {
    type Deserializer = quick_xml_deserialize::XidunionTypeDeserializer;
}
#[derive(Debug)]
pub struct EnumType {
    pub name: String,
    pub content: Vec<EnumTypeContent>,
}
#[derive(Debug)]
pub struct EnumTypeContent {
    pub item: EnumItemType,
    pub doc: Option<DocType>,
}
impl WithSerializer for EnumType {
    type Serializer<'x> = quick_xml_serialize::EnumTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::EnumTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::EnumTypeSerializerState::Init__),
            name: name.unwrap_or("Enum"),
            is_root,
        })
    }
}
impl WithSerializer for EnumTypeContent {
    type Serializer<'x> = quick_xml_serialize::EnumTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::EnumTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::EnumTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for EnumType {
    type Deserializer = quick_xml_deserialize::EnumTypeDeserializer;
}
impl WithDeserializer for EnumTypeContent {
    type Deserializer = quick_xml_deserialize::EnumTypeContentDeserializer;
}
#[derive(Debug)]
pub struct TypedefType {
    pub oldname: String,
    pub newname: String,
}
impl WithSerializer for TypedefType {
    type Serializer<'x> = quick_xml_serialize::TypedefTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TypedefTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TypedefTypeSerializerState::Init__),
            name: name.unwrap_or("Typedef"),
            is_root,
        })
    }
}
impl WithDeserializer for TypedefType {
    type Deserializer = quick_xml_deserialize::TypedefTypeDeserializer;
}
#[derive(Debug)]
pub struct PadType {
    pub bytes: Option<i32>,
    pub align: Option<i32>,
}
impl WithSerializer for PadType {
    type Serializer<'x> = quick_xml_serialize::PadTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::PadTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::PadTypeSerializerState::Init__),
            name: name.unwrap_or("pad"),
            is_root,
        })
    }
}
impl WithDeserializer for PadType {
    type Deserializer = quick_xml_deserialize::PadTypeDeserializer;
}
#[derive(Debug)]
pub struct VarType {
    pub name: String,
    pub type_: String,
    pub enum_: Option<String>,
    pub altenum: Option<String>,
    pub mask: Option<String>,
}
impl WithSerializer for VarType {
    type Serializer<'x> = quick_xml_serialize::VarTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::VarTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::VarTypeSerializerState::Init__),
            name: name.unwrap_or("var"),
            is_root,
        })
    }
}
impl WithDeserializer for VarType {
    type Deserializer = quick_xml_deserialize::VarTypeDeserializer;
}
#[derive(Debug)]
pub struct ListType {
    pub name: String,
    pub type_: String,
    pub enum_: Option<String>,
    pub altenum: Option<String>,
    pub mask: Option<String>,
    pub content: Option<ListTypeContent>,
}
#[derive(Debug)]
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
impl WithSerializer for ListType {
    type Serializer<'x> = quick_xml_serialize::ListTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ListTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ListTypeSerializerState::Init__),
            name: name.unwrap_or("list"),
            is_root,
        })
    }
}
impl WithSerializer for ListTypeContent {
    type Serializer<'x> = quick_xml_serialize::ListTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::ListTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ListTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for ListType {
    type Deserializer = quick_xml_deserialize::ListTypeDeserializer;
}
impl WithDeserializer for ListTypeContent {
    type Deserializer = quick_xml_deserialize::ListTypeContentDeserializer;
}
#[derive(Debug)]
pub struct AnyType;
impl WithSerializer for AnyType {
    type Serializer<'x> = quick_xml_serialize::AnyTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::AnyTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::AnyTypeSerializerState::Init__),
            name: name.unwrap_or("anyType"),
            is_root,
        })
    }
}
impl WithDeserializer for AnyType {
    type Deserializer = quick_xml_deserialize::AnyTypeDeserializer;
}
#[derive(Debug)]
pub struct ExprfieldType {
    pub name: String,
    pub type_: String,
    pub enum_: Option<String>,
    pub altenum: Option<String>,
    pub mask: Option<String>,
    pub content: ExprfieldTypeContent,
}
#[derive(Debug)]
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
impl WithSerializer for ExprfieldType {
    type Serializer<'x> = quick_xml_serialize::ExprfieldTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ExprfieldTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ExprfieldTypeSerializerState::Init__),
            name: name.unwrap_or("exprfield"),
            is_root,
        })
    }
}
impl WithSerializer for ExprfieldTypeContent {
    type Serializer<'x> = quick_xml_serialize::ExprfieldTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::ExprfieldTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ExprfieldTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for ExprfieldType {
    type Deserializer = quick_xml_deserialize::ExprfieldTypeDeserializer;
}
impl WithDeserializer for ExprfieldTypeContent {
    type Deserializer = quick_xml_deserialize::ExprfieldTypeContentDeserializer;
}
#[derive(Debug)]
pub struct ValueparamType {
    pub value_mask_type: String,
    pub value_mask_name: String,
    pub value_list_name: String,
}
impl WithSerializer for ValueparamType {
    type Serializer<'x> = quick_xml_serialize::ValueparamTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ValueparamTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ValueparamTypeSerializerState::Init__),
            name: name.unwrap_or("valueparam"),
            is_root,
        })
    }
}
impl WithDeserializer for ValueparamType {
    type Deserializer = quick_xml_deserialize::ValueparamTypeDeserializer;
}
#[derive(Debug)]
pub struct SwitchexprType {
    pub name: String,
    pub content: Vec<SwitchexprTypeContent>,
}
#[derive(Debug)]
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
impl WithSerializer for SwitchexprType {
    type Serializer<'x> = quick_xml_serialize::SwitchexprTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::SwitchexprTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SwitchexprTypeSerializerState::Init__),
            name: name.unwrap_or("switchexpr"),
            is_root,
        })
    }
}
impl WithSerializer for SwitchexprTypeContent {
    type Serializer<'x> = quick_xml_serialize::SwitchexprTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::SwitchexprTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SwitchexprTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for SwitchexprType {
    type Deserializer = quick_xml_deserialize::SwitchexprTypeDeserializer;
}
impl WithDeserializer for SwitchexprTypeContent {
    type Deserializer = quick_xml_deserialize::SwitchexprTypeContentDeserializer;
}
#[derive(Debug)]
pub struct RequestReplyType {
    pub content: Vec<RequestReplyTypeContent>,
}
#[derive(Debug)]
pub enum RequestReplyTypeContent {
    Pad(PadType),
    Field(VarType),
    List(ListType),
    Fd(AnyType),
    Valueparam(ValueparamType),
    Switch(SwitchexprType),
    Doc(DocType),
}
impl WithSerializer for RequestReplyType {
    type Serializer<'x> = quick_xml_serialize::RequestReplyTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::RequestReplyTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::RequestReplyTypeSerializerState::Init__),
            name: name.unwrap_or("RequestReply"),
            is_root,
        })
    }
}
impl WithSerializer for RequestReplyTypeContent {
    type Serializer<'x> = quick_xml_serialize::RequestReplyTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::RequestReplyTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::RequestReplyTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for RequestReplyType {
    type Deserializer = quick_xml_deserialize::RequestReplyTypeDeserializer;
}
impl WithDeserializer for RequestReplyTypeContent {
    type Deserializer = quick_xml_deserialize::RequestReplyTypeContentDeserializer;
}
#[derive(Debug)]
pub struct DocType {
    pub content: Vec<DocTypeContent>,
}
#[derive(Debug)]
pub enum DocTypeContent {
    Brief(String),
    Description(String),
    Example(String),
    Field(FieldType),
    Error(ErrorType),
    See(SeeType),
}
impl WithSerializer for DocType {
    type Serializer<'x> = quick_xml_serialize::DocTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::DocTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DocTypeSerializerState::Init__),
            name: name.unwrap_or("doc"),
            is_root,
        })
    }
}
impl WithSerializer for DocTypeContent {
    type Serializer<'x> = quick_xml_serialize::DocTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::DocTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DocTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for DocType {
    type Deserializer = quick_xml_deserialize::DocTypeDeserializer;
}
impl WithDeserializer for DocTypeContent {
    type Deserializer = quick_xml_deserialize::DocTypeContentDeserializer;
}
#[derive(Debug)]
pub struct EnumItemType {
    pub name: String,
    pub content: EnumItemTypeContent,
}
#[derive(Debug)]
pub enum EnumItemTypeContent {
    Value(DecOrHexIntegerType),
    Bit(i32),
}
impl WithSerializer for EnumItemType {
    type Serializer<'x> = quick_xml_serialize::EnumItemTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::EnumItemTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::EnumItemTypeSerializerState::Init__),
            name: name.unwrap_or("EnumItem"),
            is_root,
        })
    }
}
impl WithSerializer for EnumItemTypeContent {
    type Serializer<'x> = quick_xml_serialize::EnumItemTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::EnumItemTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::EnumItemTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for EnumItemType {
    type Deserializer = quick_xml_deserialize::EnumItemTypeDeserializer;
}
impl WithDeserializer for EnumItemTypeContent {
    type Deserializer = quick_xml_deserialize::EnumItemTypeContentDeserializer;
}
#[derive(Debug)]
pub struct OpType {
    pub op: String,
    pub content: [OpTypeContent; 2usize],
}
#[derive(Debug)]
pub enum OpTypeContent {
    Op(Box<OpType>),
    Unop(UnopType),
    Fieldref(String),
    Enumref(EnumrefType),
    Popcount(PopcountType),
    Sumof(SumofType),
    Value(DecOrHexIntegerType),
    Bit(i32),
}
impl WithSerializer for OpType {
    type Serializer<'x> = quick_xml_serialize::OpTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::OpTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::OpTypeSerializerState::Init__),
            name: name.unwrap_or("Op"),
            is_root,
        })
    }
}
impl WithSerializer for OpTypeContent {
    type Serializer<'x> = quick_xml_serialize::OpTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::OpTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::OpTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for OpType {
    type Deserializer = quick_xml_deserialize::OpTypeDeserializer;
}
impl WithDeserializer for OpTypeContent {
    type Deserializer = quick_xml_deserialize::OpTypeContentDeserializer;
}
#[derive(Debug)]
pub struct UnopType {
    pub op: String,
    pub content: UnopTypeContent,
}
#[derive(Debug)]
pub enum UnopTypeContent {
    Op(Box<OpType>),
    Unop(Box<UnopType>),
    Fieldref(String),
    Enumref(EnumrefType),
    Popcount(PopcountType),
    Sumof(SumofType),
    Value(DecOrHexIntegerType),
    Bit(i32),
}
impl WithSerializer for UnopType {
    type Serializer<'x> = quick_xml_serialize::UnopTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::UnopTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::UnopTypeSerializerState::Init__),
            name: name.unwrap_or("Unop"),
            is_root,
        })
    }
}
impl WithSerializer for UnopTypeContent {
    type Serializer<'x> = quick_xml_serialize::UnopTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::UnopTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::UnopTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for UnopType {
    type Deserializer = quick_xml_deserialize::UnopTypeDeserializer;
}
impl WithDeserializer for UnopTypeContent {
    type Deserializer = quick_xml_deserialize::UnopTypeContentDeserializer;
}
#[derive(Debug)]
pub struct EnumrefType {
    pub ref_: String,
    pub content: String,
}
impl WithSerializer for EnumrefType {
    type Serializer<'x> = quick_xml_serialize::EnumrefTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::EnumrefTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::EnumrefTypeSerializerState::Init__),
            name: name.unwrap_or("Enumref"),
            is_root,
        })
    }
}
impl WithDeserializer for EnumrefType {
    type Deserializer = quick_xml_deserialize::EnumrefTypeDeserializer;
}
#[derive(Debug)]
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
impl WithSerializer for PopcountType {
    type Serializer<'x> = quick_xml_serialize::PopcountTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::PopcountTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::PopcountTypeSerializerState::Init__),
            name: name.unwrap_or("Popcount"),
            is_root,
        })
    }
}
impl WithDeserializer for PopcountType {
    type Deserializer = quick_xml_deserialize::PopcountTypeDeserializer;
}
#[derive(Debug)]
pub struct SumofType {
    pub ref_: String,
}
impl WithSerializer for SumofType {
    type Serializer<'x> = quick_xml_serialize::SumofTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::SumofTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SumofTypeSerializerState::Init__),
            name: name.unwrap_or("Sumof"),
            is_root,
        })
    }
}
impl WithDeserializer for SumofType {
    type Deserializer = quick_xml_deserialize::SumofTypeDeserializer;
}
#[derive(Debug)]
pub enum DecOrHexIntegerType {
    I32(i32),
    String(String),
}
impl SerializeBytes for DecOrHexIntegerType {
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        match self {
            Self::I32(x) => x.serialize_bytes(helper),
            Self::String(x) => x.serialize_bytes(helper),
        }
    }
}
impl DeserializeBytes for DecOrHexIntegerType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let mut errors = Vec::new();
        match i32::deserialize_bytes(helper, bytes) {
            Ok(value) => return Ok(Self::I32(value)),
            Err(error) => errors.push(Box::new(error)),
        }
        match String::deserialize_bytes(helper, bytes) {
            Ok(value) => return Ok(Self::String(value)),
            Err(error) => errors.push(Box::new(error)),
        }
        Err(Error::from(ErrorKind::InvalidUnion(errors.into())))
    }
}
#[derive(Debug)]
pub struct CaseexprType {
    pub name: Option<String>,
    pub content: Vec<CaseexprTypeContent>,
}
#[derive(Debug)]
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
impl WithSerializer for CaseexprType {
    type Serializer<'x> = quick_xml_serialize::CaseexprTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::CaseexprTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::CaseexprTypeSerializerState::Init__),
            name: name.unwrap_or("caseexpr"),
            is_root,
        })
    }
}
impl WithSerializer for CaseexprTypeContent {
    type Serializer<'x> = quick_xml_serialize::CaseexprTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::CaseexprTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::CaseexprTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for CaseexprType {
    type Deserializer = quick_xml_deserialize::CaseexprTypeDeserializer;
}
impl WithDeserializer for CaseexprTypeContent {
    type Deserializer = quick_xml_deserialize::CaseexprTypeContentDeserializer;
}
#[derive(Debug)]
pub struct FieldType {
    pub name: Option<String>,
    pub content: String,
}
impl WithSerializer for FieldType {
    type Serializer<'x> = quick_xml_serialize::FieldTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::FieldTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::FieldTypeSerializerState::Init__),
            name: name.unwrap_or("Field"),
            is_root,
        })
    }
}
impl WithDeserializer for FieldType {
    type Deserializer = quick_xml_deserialize::FieldTypeDeserializer;
}
#[derive(Debug)]
pub struct ErrorType {
    pub type_: Option<String>,
    pub content: String,
}
impl WithSerializer for ErrorType {
    type Serializer<'x> = quick_xml_serialize::ErrorTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ErrorTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ErrorTypeSerializerState::Init__),
            name: name.unwrap_or("Error"),
            is_root,
        })
    }
}
impl WithDeserializer for ErrorType {
    type Deserializer = quick_xml_deserialize::ErrorTypeDeserializer;
}
#[derive(Debug)]
pub struct SeeType {
    pub name: Option<String>,
    pub type_: Option<String>,
}
impl WithSerializer for SeeType {
    type Serializer<'x> = quick_xml_serialize::SeeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::SeeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SeeTypeSerializerState::Init__),
            name: name.unwrap_or("See"),
            is_root,
        })
    }
}
impl WithDeserializer for SeeType {
    type Deserializer = quick_xml_deserialize::SeeTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, ContentDeserializer, DeserializeHelper, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
        ErrorKind, Event, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct XcbTypeDeserializer {
        header: String,
        extension_xname: Option<String>,
        extension_name: Option<String>,
        extension_multiword: bool,
        major_version: Option<i32>,
        minor_version: Option<i32>,
        content: Vec<super::XcbTypeContent>,
        state__: Box<XcbTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum XcbTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::XcbTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl XcbTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut header: Option<String> = None;
            let mut extension_xname: Option<String> = None;
            let mut extension_name: Option<String> = None;
            let mut extension_multiword: Option<bool> = None;
            let mut major_version: Option<i32> = None;
            let mut minor_version: Option<i32> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"header" {
                    helper.read_attrib(&mut header, b"header", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"extension-xname" {
                    helper.read_attrib(&mut extension_xname, b"extension-xname", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"extension-name" {
                    helper.read_attrib(&mut extension_name, b"extension-name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"extension-multiword" {
                    helper.read_attrib(
                        &mut extension_multiword,
                        b"extension-multiword",
                        &attrib.value,
                    )?;
                } else if attrib.key.local_name().as_ref() == b"major-version" {
                    helper.read_attrib(&mut major_version, b"major-version", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"minor-version" {
                    helper.read_attrib(&mut minor_version, b"minor-version", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                header: header.ok_or_else(|| ErrorKind::MissingAttribute("header".into()))?,
                extension_xname: extension_xname,
                extension_name: extension_name,
                extension_multiword: extension_multiword
                    .unwrap_or_else(super::XcbType::default_extension_multiword),
                major_version: major_version,
                minor_version: minor_version,
                content: Vec::new(),
                state__: Box::new(XcbTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: XcbTypeDeserializerState,
        ) -> Result<(), Error> {
            if let XcbTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::XcbTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::XcbTypeContent>,
            fallback: &mut Option<XcbTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use XcbTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::XcbType> for XcbTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::XcbType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::XcbType> {
            use XcbTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::XcbTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::XcbType, Error> {
            let state = replace(&mut *self.state__, XcbTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::XcbType {
                header: self.header,
                extension_xname: self.extension_xname,
                extension_name: self.extension_name,
                extension_multiword: self.extension_multiword,
                major_version: self.major_version,
                minor_version: self.minor_version,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct XcbTypeContentDeserializer {
        state__: Box<XcbTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum XcbTypeContentDeserializerState {
        Init__,
        Request(
            Option<super::RequestType>,
            Option<<super::RequestType as WithDeserializer>::Deserializer>,
            Option<<super::RequestType as WithDeserializer>::Deserializer>,
        ),
        Event(
            Option<super::EventType>,
            Option<<super::EventType as WithDeserializer>::Deserializer>,
            Option<<super::EventType as WithDeserializer>::Deserializer>,
        ),
        Eventcopy(
            Option<super::PacketStructCopyType>,
            Option<<super::PacketStructCopyType as WithDeserializer>::Deserializer>,
            Option<<super::PacketStructCopyType as WithDeserializer>::Deserializer>,
        ),
        Error(
            Option<super::PacketStructType>,
            Option<<super::PacketStructType as WithDeserializer>::Deserializer>,
            Option<<super::PacketStructType as WithDeserializer>::Deserializer>,
        ),
        Errorcopy(
            Option<super::PacketStructCopyType>,
            Option<<super::PacketStructCopyType as WithDeserializer>::Deserializer>,
            Option<<super::PacketStructCopyType as WithDeserializer>::Deserializer>,
        ),
        Struct(
            Option<super::StructType>,
            Option<<super::StructType as WithDeserializer>::Deserializer>,
            Option<<super::StructType as WithDeserializer>::Deserializer>,
        ),
        Union(
            Option<super::StructType>,
            Option<<super::StructType as WithDeserializer>::Deserializer>,
            Option<<super::StructType as WithDeserializer>::Deserializer>,
        ),
        Xidtype(
            Option<super::XidtypeType>,
            Option<<super::XidtypeType as WithDeserializer>::Deserializer>,
            Option<<super::XidtypeType as WithDeserializer>::Deserializer>,
        ),
        Xidunion(
            Option<super::XidunionType>,
            Option<<super::XidunionType as WithDeserializer>::Deserializer>,
            Option<<super::XidunionType as WithDeserializer>::Deserializer>,
        ),
        Enum(
            Option<super::EnumType>,
            Option<<super::EnumType as WithDeserializer>::Deserializer>,
            Option<<super::EnumType as WithDeserializer>::Deserializer>,
        ),
        Typedef(
            Option<super::TypedefType>,
            Option<<super::TypedefType as WithDeserializer>::Deserializer>,
            Option<<super::TypedefType as WithDeserializer>::Deserializer>,
        ),
        Import(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::XcbTypeContent),
        Unknown__,
    }
    impl XcbTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"request" {
                    let output = <super::RequestType as WithDeserializer>::init(helper, event)?;
                    return self.handle_request(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"event" {
                    let output = <super::EventType as WithDeserializer>::init(helper, event)?;
                    return self.handle_event(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"eventcopy" {
                    let output =
                        <super::PacketStructCopyType as WithDeserializer>::init(helper, event)?;
                    return self.handle_eventcopy(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"error" {
                    let output =
                        <super::PacketStructType as WithDeserializer>::init(helper, event)?;
                    return self.handle_error(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"errorcopy" {
                    let output =
                        <super::PacketStructCopyType as WithDeserializer>::init(helper, event)?;
                    return self.handle_errorcopy(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"struct" {
                    let output = <super::StructType as WithDeserializer>::init(helper, event)?;
                    return self.handle_struct_(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"union" {
                    let output = <super::StructType as WithDeserializer>::init(helper, event)?;
                    return self.handle_union_(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"xidtype" {
                    let output = <super::XidtypeType as WithDeserializer>::init(helper, event)?;
                    return self.handle_xidtype(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"xidunion" {
                    let output = <super::XidunionType as WithDeserializer>::init(helper, event)?;
                    return self.handle_xidunion(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"enum" {
                    let output = <super::EnumType as WithDeserializer>::init(helper, event)?;
                    return self.handle_enum_(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"typedef" {
                    let output = <super::TypedefType as WithDeserializer>::init(helper, event)?;
                    return self.handle_typedef(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"import" {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_import(helper, Default::default(), None, output);
                }
            }
            *self.state__ = XcbTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: XcbTypeContentDeserializerState,
        ) -> Result<super::XcbTypeContent, Error> {
            use XcbTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Request(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_request(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Request(
                        helper.finish_element("request", values)?,
                    ))
                }
                S::Event(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_event(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Event(
                        helper.finish_element("event", values)?,
                    ))
                }
                S::Eventcopy(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_eventcopy(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Eventcopy(
                        helper.finish_element("eventcopy", values)?,
                    ))
                }
                S::Error(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_error(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Error(
                        helper.finish_element("error", values)?,
                    ))
                }
                S::Errorcopy(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_errorcopy(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Errorcopy(
                        helper.finish_element("errorcopy", values)?,
                    ))
                }
                S::Struct(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_struct_(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Struct(
                        helper.finish_element("struct", values)?,
                    ))
                }
                S::Union(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_union_(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Union(
                        helper.finish_element("union", values)?,
                    ))
                }
                S::Xidtype(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_xidtype(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Xidtype(
                        helper.finish_element("xidtype", values)?,
                    ))
                }
                S::Xidunion(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_xidunion(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Xidunion(
                        helper.finish_element("xidunion", values)?,
                    ))
                }
                S::Enum(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_enum_(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Enum(
                        helper.finish_element("enum", values)?,
                    ))
                }
                S::Typedef(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_typedef(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Typedef(
                        helper.finish_element("typedef", values)?,
                    ))
                }
                S::Import(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_import(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Import(
                        helper.finish_element("import", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_request(
            values: &mut Option<super::RequestType>,
            value: super::RequestType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"request",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_event(
            values: &mut Option<super::EventType>,
            value: super::EventType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"event",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_eventcopy(
            values: &mut Option<super::PacketStructCopyType>,
            value: super::PacketStructCopyType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"eventcopy",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_error(
            values: &mut Option<super::PacketStructType>,
            value: super::PacketStructType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"error",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_errorcopy(
            values: &mut Option<super::PacketStructCopyType>,
            value: super::PacketStructCopyType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"errorcopy",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_struct_(
            values: &mut Option<super::StructType>,
            value: super::StructType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"struct",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_union_(
            values: &mut Option<super::StructType>,
            value: super::StructType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"union",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_xidtype(
            values: &mut Option<super::XidtypeType>,
            value: super::XidtypeType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"xidtype",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_xidunion(
            values: &mut Option<super::XidunionType>,
            value: super::XidunionType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"xidunion",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_enum_(
            values: &mut Option<super::EnumType>,
            value: super::EnumType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"enum")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_typedef(
            values: &mut Option<super::TypedefType>,
            value: super::TypedefType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"typedef",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_import(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"import",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_request<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::RequestType>,
            fallback: Option<<super::RequestType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::RequestType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use XcbTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_request(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_request(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Request(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Request(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_event<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::EventType>,
            fallback: Option<<super::EventType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::EventType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use XcbTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_event(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_event(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Event(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Event(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_eventcopy<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PacketStructCopyType>,
            fallback: Option<<super::PacketStructCopyType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PacketStructCopyType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use XcbTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_eventcopy(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_eventcopy(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Eventcopy(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Eventcopy(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_error<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PacketStructType>,
            fallback: Option<<super::PacketStructType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PacketStructType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use XcbTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_error(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_error(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Error(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Error(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_errorcopy<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PacketStructCopyType>,
            fallback: Option<<super::PacketStructCopyType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PacketStructCopyType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use XcbTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_errorcopy(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_errorcopy(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Errorcopy(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Errorcopy(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_struct_<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::StructType>,
            fallback: Option<<super::StructType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::StructType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use XcbTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_struct_(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_struct_(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Struct(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Struct(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_union_<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::StructType>,
            fallback: Option<<super::StructType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::StructType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use XcbTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_union_(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_union_(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Union(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Union(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_xidtype<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::XidtypeType>,
            fallback: Option<<super::XidtypeType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::XidtypeType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use XcbTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_xidtype(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_xidtype(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Xidtype(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Xidtype(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_xidunion<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::XidunionType>,
            fallback: Option<<super::XidunionType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::XidunionType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use XcbTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_xidunion(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_xidunion(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Xidunion(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Xidunion(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_enum_<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::EnumType>,
            fallback: Option<<super::EnumType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::EnumType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use XcbTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_enum_(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_enum_(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Enum(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Enum(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_typedef<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::TypedefType>,
            fallback: Option<<super::TypedefType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::TypedefType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use XcbTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_typedef(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_typedef(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Typedef(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Typedef(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_import<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use XcbTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_import(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_import(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Import(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Import(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::XcbTypeContent> for XcbTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::XcbTypeContent> {
            let deserializer = Self {
                state__: Box::new(XcbTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, XcbTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::XcbTypeContent> {
            use XcbTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Request(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_request(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Event(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_event(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Eventcopy(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_eventcopy(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Error(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_error(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Errorcopy(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_errorcopy(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Struct(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_struct_(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Union(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_union_(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Xidtype(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_xidtype(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Xidunion(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_xidunion(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enum(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_enum_(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Typedef(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_typedef(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Import(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_import(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Request(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"request", true)?;
                        match self.handle_request(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Event(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"event", true)?;
                        match self.handle_event(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Eventcopy(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"eventcopy", false)?;
                        match self.handle_eventcopy(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Error(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"error", true)?;
                        match self.handle_error(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Errorcopy(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"errorcopy", false)?;
                        match self.handle_errorcopy(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Struct(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"struct", true)?;
                        match self.handle_struct_(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Union(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"union", true)?;
                        match self.handle_union_(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Xidtype(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"xidtype", false)?;
                        match self.handle_xidtype(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Xidunion(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"xidunion", false)?;
                        match self.handle_xidunion(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Enum(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"enum", false)?;
                        match self.handle_enum_(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Typedef(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"typedef", false)?;
                        match self.handle_typedef(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Import(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"import", false)?;
                        match self.handle_import(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::XcbTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct RequestTypeDeserializer {
        name: String,
        opcode: i32,
        combine_adjacent: Option<bool>,
        content: Vec<super::RequestTypeContent>,
        state__: Box<RequestTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RequestTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::RequestTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl RequestTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut name: Option<String> = None;
            let mut opcode: Option<i32> = None;
            let mut combine_adjacent: Option<bool> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"opcode" {
                    helper.read_attrib(&mut opcode, b"opcode", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"combine-adjacent" {
                    helper.read_attrib(
                        &mut combine_adjacent,
                        b"combine-adjacent",
                        &attrib.value,
                    )?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                opcode: opcode.ok_or_else(|| ErrorKind::MissingAttribute("opcode".into()))?,
                combine_adjacent: combine_adjacent,
                content: Vec::new(),
                state__: Box::new(RequestTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: RequestTypeDeserializerState,
        ) -> Result<(), Error> {
            if let RequestTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::RequestTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::RequestTypeContent>,
            fallback: &mut Option<RequestTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RequestTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RequestType> for RequestTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RequestType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RequestType> {
            use RequestTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::RequestTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::RequestType, Error> {
            let state = replace(&mut *self.state__, RequestTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::RequestType {
                name: self.name,
                opcode: self.opcode,
                combine_adjacent: self.combine_adjacent,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct RequestTypeContentDeserializer {
        state__: Box<RequestTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum RequestTypeContentDeserializerState {
        Init__,
        Pad(
            Option<super::PadType>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
        ),
        Field(
            Option<super::VarType>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
        ),
        List(
            Option<super::ListType>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
        ),
        Fd(
            Option<super::AnyType>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
        ),
        Exprfield(
            Option<super::ExprfieldType>,
            Option<<super::ExprfieldType as WithDeserializer>::Deserializer>,
            Option<<super::ExprfieldType as WithDeserializer>::Deserializer>,
        ),
        Valueparam(
            Option<super::ValueparamType>,
            Option<<super::ValueparamType as WithDeserializer>::Deserializer>,
            Option<<super::ValueparamType as WithDeserializer>::Deserializer>,
        ),
        Switch(
            Option<super::SwitchexprType>,
            Option<<super::SwitchexprType as WithDeserializer>::Deserializer>,
            Option<<super::SwitchexprType as WithDeserializer>::Deserializer>,
        ),
        Reply(
            Option<super::RequestReplyType>,
            Option<<super::RequestReplyType as WithDeserializer>::Deserializer>,
            Option<<super::RequestReplyType as WithDeserializer>::Deserializer>,
        ),
        Doc(
            Option<super::DocType>,
            Option<<super::DocType as WithDeserializer>::Deserializer>,
            Option<<super::DocType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::RequestTypeContent),
        Unknown__,
    }
    impl RequestTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"pad" {
                    let output = <super::PadType as WithDeserializer>::init(helper, event)?;
                    return self.handle_pad(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"field" {
                    let output = <super::VarType as WithDeserializer>::init(helper, event)?;
                    return self.handle_field(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"list" {
                    let output = <super::ListType as WithDeserializer>::init(helper, event)?;
                    return self.handle_list(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"fd" {
                    let output = <super::AnyType as WithDeserializer>::init(helper, event)?;
                    return self.handle_fd(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"exprfield" {
                    let output = <super::ExprfieldType as WithDeserializer>::init(helper, event)?;
                    return self.handle_exprfield(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"valueparam" {
                    let output = <super::ValueparamType as WithDeserializer>::init(helper, event)?;
                    return self.handle_valueparam(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"switch" {
                    let output = <super::SwitchexprType as WithDeserializer>::init(helper, event)?;
                    return self.handle_switch(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"reply" {
                    let output =
                        <super::RequestReplyType as WithDeserializer>::init(helper, event)?;
                    return self.handle_reply(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"doc" {
                    let output = <super::DocType as WithDeserializer>::init(helper, event)?;
                    return self.handle_doc(helper, Default::default(), None, output);
                }
            }
            *self.state__ = RequestTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: RequestTypeContentDeserializerState,
        ) -> Result<super::RequestTypeContent, Error> {
            use RequestTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Pad(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_pad(&mut values, value)?;
                    }
                    Ok(super::RequestTypeContent::Pad(
                        helper.finish_element("pad", values)?,
                    ))
                }
                S::Field(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_field(&mut values, value)?;
                    }
                    Ok(super::RequestTypeContent::Field(
                        helper.finish_element("field", values)?,
                    ))
                }
                S::List(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_list(&mut values, value)?;
                    }
                    Ok(super::RequestTypeContent::List(
                        helper.finish_element("list", values)?,
                    ))
                }
                S::Fd(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_fd(&mut values, value)?;
                    }
                    Ok(super::RequestTypeContent::Fd(
                        helper.finish_element("fd", values)?,
                    ))
                }
                S::Exprfield(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_exprfield(&mut values, value)?;
                    }
                    Ok(super::RequestTypeContent::Exprfield(
                        helper.finish_element("exprfield", values)?,
                    ))
                }
                S::Valueparam(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_valueparam(&mut values, value)?;
                    }
                    Ok(super::RequestTypeContent::Valueparam(
                        helper.finish_element("valueparam", values)?,
                    ))
                }
                S::Switch(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_switch(&mut values, value)?;
                    }
                    Ok(super::RequestTypeContent::Switch(
                        helper.finish_element("switch", values)?,
                    ))
                }
                S::Reply(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_reply(&mut values, value)?;
                    }
                    Ok(super::RequestTypeContent::Reply(
                        helper.finish_element("reply", values)?,
                    ))
                }
                S::Doc(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_doc(&mut values, value)?;
                    }
                    Ok(super::RequestTypeContent::Doc(
                        helper.finish_element("doc", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_pad(
            values: &mut Option<super::PadType>,
            value: super::PadType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"pad")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_field(
            values: &mut Option<super::VarType>,
            value: super::VarType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"field",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_list(
            values: &mut Option<super::ListType>,
            value: super::ListType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"list")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_fd(
            values: &mut Option<super::AnyType>,
            value: super::AnyType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"fd")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_exprfield(
            values: &mut Option<super::ExprfieldType>,
            value: super::ExprfieldType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"exprfield",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_valueparam(
            values: &mut Option<super::ValueparamType>,
            value: super::ValueparamType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"valueparam",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_switch(
            values: &mut Option<super::SwitchexprType>,
            value: super::SwitchexprType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"switch",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_reply(
            values: &mut Option<super::RequestReplyType>,
            value: super::RequestReplyType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"reply",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_doc(
            values: &mut Option<super::DocType>,
            value: super::DocType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"doc")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_pad<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PadType>,
            fallback: Option<<super::PadType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PadType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RequestTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_pad(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pad(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Pad(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Pad(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_field<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::VarType>,
            fallback: Option<<super::VarType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::VarType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RequestTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_field(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_field(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Field(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Field(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_list<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ListType>,
            fallback: Option<<super::ListType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ListType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RequestTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_list(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_list(&mut values, data)?;
                    let data = Self::finish_state(helper, S::List(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::List(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_fd<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnyType>,
            fallback: Option<<super::AnyType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnyType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RequestTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_fd(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fd(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Fd(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Fd(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_exprfield<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ExprfieldType>,
            fallback: Option<<super::ExprfieldType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ExprfieldType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RequestTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_exprfield(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_exprfield(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Exprfield(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Exprfield(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_valueparam<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ValueparamType>,
            fallback: Option<<super::ValueparamType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ValueparamType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RequestTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_valueparam(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_valueparam(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Valueparam(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Valueparam(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_switch<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SwitchexprType>,
            fallback: Option<<super::SwitchexprType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SwitchexprType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RequestTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_switch(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_switch(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Switch(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Switch(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_reply<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::RequestReplyType>,
            fallback: Option<<super::RequestReplyType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::RequestReplyType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RequestTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_reply(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_reply(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Reply(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Reply(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_doc<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::DocType>,
            fallback: Option<<super::DocType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::DocType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RequestTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_doc(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_doc(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Doc(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Doc(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RequestTypeContent> for RequestTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RequestTypeContent> {
            let deserializer = Self {
                state__: Box::new(RequestTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, RequestTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RequestTypeContent> {
            use RequestTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Pad(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_pad(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_field(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_list(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fd(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Exprfield(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_exprfield(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Valueparam(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_valueparam(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Switch(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_switch(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Reply(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_reply(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Doc(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_doc(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Pad(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"pad", false)?;
                        match self.handle_pad(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Field(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"field", false)?;
                        match self.handle_field(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::List(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"list", false)?;
                        match self.handle_list(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Fd(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"fd", true)?;
                        match self.handle_fd(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Exprfield(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"exprfield", false)?;
                        match self.handle_exprfield(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Valueparam(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            None,
                            b"valueparam",
                            false,
                        )?;
                        match self.handle_valueparam(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Switch(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"switch", true)?;
                        match self.handle_switch(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Reply(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"reply", true)?;
                        match self.handle_reply(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Doc(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"doc", false)?;
                        match self.handle_doc(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::RequestTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct EventTypeDeserializer {
        name: String,
        number: i32,
        no_sequence_number: Option<bool>,
        xge: Option<bool>,
        content: Vec<super::EventTypeContent>,
        state__: Box<EventTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum EventTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::EventTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl EventTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut name: Option<String> = None;
            let mut number: Option<i32> = None;
            let mut no_sequence_number: Option<bool> = None;
            let mut xge: Option<bool> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"number" {
                    helper.read_attrib(&mut number, b"number", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"no-sequence-number" {
                    helper.read_attrib(
                        &mut no_sequence_number,
                        b"no-sequence-number",
                        &attrib.value,
                    )?;
                } else if attrib.key.local_name().as_ref() == b"xge" {
                    helper.read_attrib(&mut xge, b"xge", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                number: number.ok_or_else(|| ErrorKind::MissingAttribute("number".into()))?,
                no_sequence_number: no_sequence_number,
                xge: xge,
                content: Vec::new(),
                state__: Box::new(EventTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: EventTypeDeserializerState,
        ) -> Result<(), Error> {
            if let EventTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::EventTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::EventTypeContent>,
            fallback: &mut Option<EventTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use EventTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::EventType> for EventTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EventType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EventType> {
            use EventTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::EventTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::EventType, Error> {
            let state = replace(&mut *self.state__, EventTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::EventType {
                name: self.name,
                number: self.number,
                no_sequence_number: self.no_sequence_number,
                xge: self.xge,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct EventTypeContentDeserializer {
        state__: Box<EventTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum EventTypeContentDeserializerState {
        Init__,
        Pad(
            Option<super::PadType>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
        ),
        Field(
            Option<super::VarType>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
        ),
        List(
            Option<super::ListType>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
        ),
        Fd(
            Option<super::AnyType>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
        ),
        Doc(
            Option<super::DocType>,
            Option<<super::DocType as WithDeserializer>::Deserializer>,
            Option<<super::DocType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::EventTypeContent),
        Unknown__,
    }
    impl EventTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"pad" {
                    let output = <super::PadType as WithDeserializer>::init(helper, event)?;
                    return self.handle_pad(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"field" {
                    let output = <super::VarType as WithDeserializer>::init(helper, event)?;
                    return self.handle_field(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"list" {
                    let output = <super::ListType as WithDeserializer>::init(helper, event)?;
                    return self.handle_list(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"fd" {
                    let output = <super::AnyType as WithDeserializer>::init(helper, event)?;
                    return self.handle_fd(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"doc" {
                    let output = <super::DocType as WithDeserializer>::init(helper, event)?;
                    return self.handle_doc(helper, Default::default(), None, output);
                }
            }
            *self.state__ = EventTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: EventTypeContentDeserializerState,
        ) -> Result<super::EventTypeContent, Error> {
            use EventTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Pad(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_pad(&mut values, value)?;
                    }
                    Ok(super::EventTypeContent::Pad(
                        helper.finish_element("pad", values)?,
                    ))
                }
                S::Field(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_field(&mut values, value)?;
                    }
                    Ok(super::EventTypeContent::Field(
                        helper.finish_element("field", values)?,
                    ))
                }
                S::List(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_list(&mut values, value)?;
                    }
                    Ok(super::EventTypeContent::List(
                        helper.finish_element("list", values)?,
                    ))
                }
                S::Fd(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_fd(&mut values, value)?;
                    }
                    Ok(super::EventTypeContent::Fd(
                        helper.finish_element("fd", values)?,
                    ))
                }
                S::Doc(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_doc(&mut values, value)?;
                    }
                    Ok(super::EventTypeContent::Doc(
                        helper.finish_element("doc", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_pad(
            values: &mut Option<super::PadType>,
            value: super::PadType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"pad")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_field(
            values: &mut Option<super::VarType>,
            value: super::VarType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"field",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_list(
            values: &mut Option<super::ListType>,
            value: super::ListType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"list")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_fd(
            values: &mut Option<super::AnyType>,
            value: super::AnyType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"fd")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_doc(
            values: &mut Option<super::DocType>,
            value: super::DocType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"doc")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_pad<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PadType>,
            fallback: Option<<super::PadType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PadType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use EventTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_pad(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pad(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Pad(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Pad(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_field<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::VarType>,
            fallback: Option<<super::VarType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::VarType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use EventTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_field(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_field(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Field(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Field(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_list<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ListType>,
            fallback: Option<<super::ListType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ListType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use EventTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_list(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_list(&mut values, data)?;
                    let data = Self::finish_state(helper, S::List(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::List(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_fd<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnyType>,
            fallback: Option<<super::AnyType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnyType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use EventTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_fd(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fd(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Fd(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Fd(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_doc<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::DocType>,
            fallback: Option<<super::DocType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::DocType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use EventTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_doc(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_doc(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Doc(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Doc(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::EventTypeContent> for EventTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EventTypeContent> {
            let deserializer = Self {
                state__: Box::new(EventTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, EventTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EventTypeContent> {
            use EventTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Pad(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_pad(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_field(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_list(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fd(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Doc(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_doc(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Pad(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"pad", false)?;
                        match self.handle_pad(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Field(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"field", false)?;
                        match self.handle_field(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::List(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"list", false)?;
                        match self.handle_list(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Fd(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"fd", true)?;
                        match self.handle_fd(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Doc(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"doc", false)?;
                        match self.handle_doc(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::EventTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct PacketStructCopyTypeDeserializer {
        name: String,
        number: i32,
        ref_: String,
        state__: Box<PacketStructCopyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PacketStructCopyTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl PacketStructCopyTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut name: Option<String> = None;
            let mut number: Option<i32> = None;
            let mut ref_: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"number" {
                    helper.read_attrib(&mut number, b"number", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"ref" {
                    helper.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                number: number.ok_or_else(|| ErrorKind::MissingAttribute("number".into()))?,
                ref_: ref_.ok_or_else(|| ErrorKind::MissingAttribute("ref".into()))?,
                state__: Box::new(PacketStructCopyTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: PacketStructCopyTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::PacketStructCopyType> for PacketStructCopyTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PacketStructCopyType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PacketStructCopyType> {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(helper)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                })
            }
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::PacketStructCopyType, Error> {
            let state = replace(
                &mut *self.state__,
                PacketStructCopyTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::PacketStructCopyType {
                name: self.name,
                number: self.number,
                ref_: self.ref_,
            })
        }
    }
    #[derive(Debug)]
    pub struct PacketStructTypeDeserializer {
        name: String,
        number: i32,
        content: Vec<super::PacketStructTypeContent>,
        state__: Box<PacketStructTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PacketStructTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::PacketStructTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl PacketStructTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut name: Option<String> = None;
            let mut number: Option<i32> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"number" {
                    helper.read_attrib(&mut number, b"number", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                number: number.ok_or_else(|| ErrorKind::MissingAttribute("number".into()))?,
                content: Vec::new(),
                state__: Box::new(PacketStructTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: PacketStructTypeDeserializerState,
        ) -> Result<(), Error> {
            if let PacketStructTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::PacketStructTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::PacketStructTypeContent>,
            fallback: &mut Option<PacketStructTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PacketStructTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::PacketStructType> for PacketStructTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PacketStructType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PacketStructType> {
            use PacketStructTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output = <super::PacketStructTypeContent as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::PacketStructType, Error> {
            let state = replace(
                &mut *self.state__,
                PacketStructTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::PacketStructType {
                name: self.name,
                number: self.number,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PacketStructTypeContentDeserializer {
        state__: Box<PacketStructTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum PacketStructTypeContentDeserializerState {
        Init__,
        Pad(
            Option<super::PadType>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
        ),
        Field(
            Option<super::VarType>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
        ),
        List(
            Option<super::ListType>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
        ),
        Fd(
            Option<super::AnyType>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::PacketStructTypeContent),
        Unknown__,
    }
    impl PacketStructTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"pad" {
                    let output = <super::PadType as WithDeserializer>::init(helper, event)?;
                    return self.handle_pad(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"field" {
                    let output = <super::VarType as WithDeserializer>::init(helper, event)?;
                    return self.handle_field(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"list" {
                    let output = <super::ListType as WithDeserializer>::init(helper, event)?;
                    return self.handle_list(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"fd" {
                    let output = <super::AnyType as WithDeserializer>::init(helper, event)?;
                    return self.handle_fd(helper, Default::default(), None, output);
                }
            }
            *self.state__ = PacketStructTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: PacketStructTypeContentDeserializerState,
        ) -> Result<super::PacketStructTypeContent, Error> {
            use PacketStructTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Pad(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_pad(&mut values, value)?;
                    }
                    Ok(super::PacketStructTypeContent::Pad(
                        helper.finish_element("pad", values)?,
                    ))
                }
                S::Field(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_field(&mut values, value)?;
                    }
                    Ok(super::PacketStructTypeContent::Field(
                        helper.finish_element("field", values)?,
                    ))
                }
                S::List(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_list(&mut values, value)?;
                    }
                    Ok(super::PacketStructTypeContent::List(
                        helper.finish_element("list", values)?,
                    ))
                }
                S::Fd(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_fd(&mut values, value)?;
                    }
                    Ok(super::PacketStructTypeContent::Fd(
                        helper.finish_element("fd", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_pad(
            values: &mut Option<super::PadType>,
            value: super::PadType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"pad")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_field(
            values: &mut Option<super::VarType>,
            value: super::VarType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"field",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_list(
            values: &mut Option<super::ListType>,
            value: super::ListType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"list")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_fd(
            values: &mut Option<super::AnyType>,
            value: super::AnyType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"fd")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_pad<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PadType>,
            fallback: Option<<super::PadType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PadType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PacketStructTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_pad(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pad(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Pad(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Pad(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_field<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::VarType>,
            fallback: Option<<super::VarType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::VarType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PacketStructTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_field(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_field(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Field(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Field(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_list<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ListType>,
            fallback: Option<<super::ListType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ListType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PacketStructTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_list(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_list(&mut values, data)?;
                    let data = Self::finish_state(helper, S::List(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::List(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_fd<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnyType>,
            fallback: Option<<super::AnyType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnyType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PacketStructTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_fd(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fd(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Fd(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Fd(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::PacketStructTypeContent>
        for PacketStructTypeContentDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PacketStructTypeContent> {
            let deserializer = Self {
                state__: Box::new(PacketStructTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        PacketStructTypeContentDeserializerState::Init__
                    ) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PacketStructTypeContent> {
            use PacketStructTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Pad(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_pad(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_field(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_list(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fd(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Pad(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"pad", false)?;
                        match self.handle_pad(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Field(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"field", false)?;
                        match self.handle_field(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::List(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"list", false)?;
                        match self.handle_list(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Fd(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"fd", true)?;
                        match self.handle_fd(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::PacketStructTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct StructTypeDeserializer {
        name: String,
        content: Vec<super::StructTypeContent>,
        state__: Box<StructTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum StructTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::StructTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl StructTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut name: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                content: Vec::new(),
                state__: Box::new(StructTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: StructTypeDeserializerState,
        ) -> Result<(), Error> {
            if let StructTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::StructTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::StructTypeContent>,
            fallback: &mut Option<StructTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use StructTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::StructType> for StructTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::StructType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::StructType> {
            use StructTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::StructTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::StructType, Error> {
            let state = replace(&mut *self.state__, StructTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::StructType {
                name: self.name,
                content: helper.finish_vec(1usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct StructTypeContentDeserializer {
        state__: Box<StructTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum StructTypeContentDeserializerState {
        Init__,
        Pad(
            Option<super::PadType>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
        ),
        Field(
            Option<super::VarType>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
        ),
        List(
            Option<super::ListType>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
        ),
        Fd(
            Option<super::AnyType>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
        ),
        Switch(
            Option<super::SwitchexprType>,
            Option<<super::SwitchexprType as WithDeserializer>::Deserializer>,
            Option<<super::SwitchexprType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::StructTypeContent),
        Unknown__,
    }
    impl StructTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"pad" {
                    let output = <super::PadType as WithDeserializer>::init(helper, event)?;
                    return self.handle_pad(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"field" {
                    let output = <super::VarType as WithDeserializer>::init(helper, event)?;
                    return self.handle_field(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"list" {
                    let output = <super::ListType as WithDeserializer>::init(helper, event)?;
                    return self.handle_list(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"fd" {
                    let output = <super::AnyType as WithDeserializer>::init(helper, event)?;
                    return self.handle_fd(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"switch" {
                    let output = <super::SwitchexprType as WithDeserializer>::init(helper, event)?;
                    return self.handle_switch(helper, Default::default(), None, output);
                }
            }
            *self.state__ = StructTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: StructTypeContentDeserializerState,
        ) -> Result<super::StructTypeContent, Error> {
            use StructTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Pad(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_pad(&mut values, value)?;
                    }
                    Ok(super::StructTypeContent::Pad(
                        helper.finish_element("pad", values)?,
                    ))
                }
                S::Field(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_field(&mut values, value)?;
                    }
                    Ok(super::StructTypeContent::Field(
                        helper.finish_element("field", values)?,
                    ))
                }
                S::List(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_list(&mut values, value)?;
                    }
                    Ok(super::StructTypeContent::List(
                        helper.finish_element("list", values)?,
                    ))
                }
                S::Fd(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_fd(&mut values, value)?;
                    }
                    Ok(super::StructTypeContent::Fd(
                        helper.finish_element("fd", values)?,
                    ))
                }
                S::Switch(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_switch(&mut values, value)?;
                    }
                    Ok(super::StructTypeContent::Switch(
                        helper.finish_element("switch", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_pad(
            values: &mut Option<super::PadType>,
            value: super::PadType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"pad")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_field(
            values: &mut Option<super::VarType>,
            value: super::VarType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"field",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_list(
            values: &mut Option<super::ListType>,
            value: super::ListType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"list")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_fd(
            values: &mut Option<super::AnyType>,
            value: super::AnyType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"fd")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_switch(
            values: &mut Option<super::SwitchexprType>,
            value: super::SwitchexprType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"switch",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_pad<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PadType>,
            fallback: Option<<super::PadType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PadType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use StructTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_pad(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pad(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Pad(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Pad(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_field<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::VarType>,
            fallback: Option<<super::VarType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::VarType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use StructTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_field(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_field(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Field(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Field(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_list<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ListType>,
            fallback: Option<<super::ListType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ListType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use StructTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_list(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_list(&mut values, data)?;
                    let data = Self::finish_state(helper, S::List(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::List(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_fd<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnyType>,
            fallback: Option<<super::AnyType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnyType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use StructTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_fd(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fd(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Fd(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Fd(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_switch<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SwitchexprType>,
            fallback: Option<<super::SwitchexprType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SwitchexprType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use StructTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_switch(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_switch(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Switch(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Switch(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::StructTypeContent> for StructTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::StructTypeContent> {
            let deserializer = Self {
                state__: Box::new(StructTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, StructTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::StructTypeContent> {
            use StructTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Pad(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_pad(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_field(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_list(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fd(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Switch(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_switch(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Pad(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"pad", false)?;
                        match self.handle_pad(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Field(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"field", false)?;
                        match self.handle_field(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::List(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"list", false)?;
                        match self.handle_list(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Fd(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"fd", true)?;
                        match self.handle_fd(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Switch(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"switch", true)?;
                        match self.handle_switch(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::StructTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct XidtypeTypeDeserializer {
        name: String,
        state__: Box<XidtypeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum XidtypeTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl XidtypeTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut name: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                state__: Box::new(XidtypeTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: XidtypeTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::XidtypeType> for XidtypeTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::XidtypeType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::XidtypeType> {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(helper)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                })
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::XidtypeType, Error> {
            let state = replace(&mut *self.state__, XidtypeTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::XidtypeType { name: self.name })
        }
    }
    #[derive(Debug)]
    pub struct XidunionTypeDeserializer {
        name: String,
        type_: Vec<String>,
        state__: Box<XidunionTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum XidunionTypeDeserializerState {
        Init__,
        Type(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl XidunionTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut name: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                type_: Vec::new(),
                state__: Box::new(XidunionTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: XidunionTypeDeserializerState,
        ) -> Result<(), Error> {
            use XidunionTypeDeserializerState as S;
            match state {
                S::Type(Some(deserializer)) => self.store_type_(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_type_(&mut self, value: String) -> Result<(), Error> {
            self.type_.push(value);
            Ok(())
        }
        fn handle_type_<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<XidunionTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use XidunionTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else if self.type_.len() < 1usize {
                    fallback.get_or_insert(S::Type(None));
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else {
                    fallback.get_or_insert(S::Type(None));
                    *self.state__ = S::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_type_(data)?;
                    *self.state__ = S::Type(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Type(Some(deserializer)));
                    *self.state__ = S::Type(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::XidunionType> for XidunionTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::XidunionType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::XidunionType> {
            use XidunionTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Type(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_type_(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Type(None);
                        event
                    }
                    (S::Type(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"type", false)?;
                        match self.handle_type_(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::XidunionType, Error> {
            let state = replace(&mut *self.state__, XidunionTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::XidunionType {
                name: self.name,
                type_: helper.finish_vec(1usize, None, self.type_)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct EnumTypeDeserializer {
        name: String,
        content: Vec<super::EnumTypeContent>,
        state__: Box<EnumTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum EnumTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::EnumTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl EnumTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut name: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                content: Vec::new(),
                state__: Box::new(EnumTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: EnumTypeDeserializerState,
        ) -> Result<(), Error> {
            if let EnumTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::EnumTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::EnumTypeContent>,
            fallback: &mut Option<EnumTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use EnumTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::EnumType> for EnumTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EnumType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EnumType> {
            use EnumTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::EnumTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::EnumType, Error> {
            let state = replace(&mut *self.state__, EnumTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::EnumType {
                name: self.name,
                content: helper.finish_vec(1usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct EnumTypeContentDeserializer {
        item: Option<super::EnumItemType>,
        doc: Option<super::DocType>,
        state__: Box<EnumTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    enum EnumTypeContentDeserializerState {
        Init__,
        Item(Option<<super::EnumItemType as WithDeserializer>::Deserializer>),
        Doc(Option<<super::DocType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl EnumTypeContentDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: EnumTypeContentDeserializerState,
        ) -> Result<(), Error> {
            use EnumTypeContentDeserializerState as S;
            match state {
                S::Item(Some(deserializer)) => self.store_item(deserializer.finish(helper)?)?,
                S::Doc(Some(deserializer)) => self.store_doc(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_item(&mut self, value: super::EnumItemType) -> Result<(), Error> {
            if self.item.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"item")))?;
            }
            self.item = Some(value);
            Ok(())
        }
        fn store_doc(&mut self, value: super::DocType) -> Result<(), Error> {
            if self.doc.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"doc")))?;
            }
            self.doc = Some(value);
            Ok(())
        }
        fn handle_item<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::EnumItemType>,
            fallback: &mut Option<EnumTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use EnumTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Item(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_item(data)?;
                    *self.state__ = S::Doc(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Item(Some(deserializer)));
                    *self.state__ = S::Doc(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_doc<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::DocType>,
            fallback: &mut Option<EnumTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use EnumTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Doc(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_doc(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Doc(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::EnumTypeContent> for EnumTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EnumTypeContent> {
            let deserializer = Self {
                item: None,
                doc: None,
                state__: Box::new(EnumTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, EnumTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EnumTypeContent> {
            use EnumTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Item(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_item(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Doc(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_doc(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, event @ Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Item(None);
                        event
                    }
                    (S::Item(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"item", false)?;
                        match self.handle_item(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Doc(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"doc", false)?;
                        match self.handle_doc(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::EnumTypeContent, Error> {
            let state = replace(
                &mut *self.state__,
                EnumTypeContentDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::EnumTypeContent {
                item: helper.finish_element("item", self.item)?,
                doc: self.doc,
            })
        }
    }
    #[derive(Debug)]
    pub struct TypedefTypeDeserializer {
        oldname: String,
        newname: String,
        state__: Box<TypedefTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TypedefTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl TypedefTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut oldname: Option<String> = None;
            let mut newname: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"oldname" {
                    helper.read_attrib(&mut oldname, b"oldname", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"newname" {
                    helper.read_attrib(&mut newname, b"newname", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                oldname: oldname.ok_or_else(|| ErrorKind::MissingAttribute("oldname".into()))?,
                newname: newname.ok_or_else(|| ErrorKind::MissingAttribute("newname".into()))?,
                state__: Box::new(TypedefTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TypedefTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::TypedefType> for TypedefTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TypedefType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TypedefType> {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(helper)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                })
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::TypedefType, Error> {
            let state = replace(&mut *self.state__, TypedefTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::TypedefType {
                oldname: self.oldname,
                newname: self.newname,
            })
        }
    }
    #[derive(Debug)]
    pub struct PadTypeDeserializer {
        bytes: Option<i32>,
        align: Option<i32>,
        state__: Box<PadTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PadTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl PadTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut bytes: Option<i32> = None;
            let mut align: Option<i32> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"bytes" {
                    helper.read_attrib(&mut bytes, b"bytes", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"align" {
                    helper.read_attrib(&mut align, b"align", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                bytes: bytes,
                align: align,
                state__: Box::new(PadTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: PadTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::PadType> for PadTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PadType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PadType> {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(helper)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                })
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::PadType, Error> {
            let state = replace(&mut *self.state__, PadTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::PadType {
                bytes: self.bytes,
                align: self.align,
            })
        }
    }
    #[derive(Debug)]
    pub struct VarTypeDeserializer {
        name: String,
        type_: String,
        enum_: Option<String>,
        altenum: Option<String>,
        mask: Option<String>,
        state__: Box<VarTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum VarTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl VarTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut name: Option<String> = None;
            let mut type_: Option<String> = None;
            let mut enum_: Option<String> = None;
            let mut altenum: Option<String> = None;
            let mut mask: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"type" {
                    helper.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"enum" {
                    helper.read_attrib(&mut enum_, b"enum", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"altenum" {
                    helper.read_attrib(&mut altenum, b"altenum", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"mask" {
                    helper.read_attrib(&mut mask, b"mask", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                type_: type_.ok_or_else(|| ErrorKind::MissingAttribute("type".into()))?,
                enum_: enum_,
                altenum: altenum,
                mask: mask,
                state__: Box::new(VarTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: VarTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::VarType> for VarTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::VarType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::VarType> {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(helper)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                })
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::VarType, Error> {
            let state = replace(&mut *self.state__, VarTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::VarType {
                name: self.name,
                type_: self.type_,
                enum_: self.enum_,
                altenum: self.altenum,
                mask: self.mask,
            })
        }
    }
    #[derive(Debug)]
    pub struct ListTypeDeserializer {
        name: String,
        type_: String,
        enum_: Option<String>,
        altenum: Option<String>,
        mask: Option<String>,
        content: Option<super::ListTypeContent>,
        state__: Box<ListTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ListTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::ListTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ListTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut name: Option<String> = None;
            let mut type_: Option<String> = None;
            let mut enum_: Option<String> = None;
            let mut altenum: Option<String> = None;
            let mut mask: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"type" {
                    helper.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"enum" {
                    helper.read_attrib(&mut enum_, b"enum", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"altenum" {
                    helper.read_attrib(&mut altenum, b"altenum", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"mask" {
                    helper.read_attrib(&mut mask, b"mask", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                type_: type_.ok_or_else(|| ErrorKind::MissingAttribute("type".into()))?,
                enum_: enum_,
                altenum: altenum,
                mask: mask,
                content: None,
                state__: Box::new(ListTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ListTypeDeserializerState,
        ) -> Result<(), Error> {
            if let ListTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ListTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ListTypeContent>,
            fallback: &mut Option<ListTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ListTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ListType> for ListTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ListType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ListType> {
            use ListTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::ListTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ListType, Error> {
            let state = replace(&mut *self.state__, ListTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::ListType {
                name: self.name,
                type_: self.type_,
                enum_: self.enum_,
                altenum: self.altenum,
                mask: self.mask,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct ListTypeContentDeserializer {
        state__: Box<ListTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ListTypeContentDeserializerState {
        Init__,
        Op(
            Option<super::OpType>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
        ),
        Unop(
            Option<super::UnopType>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
        ),
        Fieldref(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Enumref(
            Option<super::EnumrefType>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
        ),
        Popcount(
            Option<super::PopcountType>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
        ),
        Sumof(
            Option<super::SumofType>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
        ),
        Value(
            Option<super::DecOrHexIntegerType>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
        ),
        Bit(
            Option<i32>,
            Option<<i32 as WithDeserializer>::Deserializer>,
            Option<<i32 as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ListTypeContent),
        Unknown__,
    }
    impl ListTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"op" {
                    let output = <super::OpType as WithDeserializer>::init(helper, event)?;
                    return self.handle_op(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"unop" {
                    let output = <super::UnopType as WithDeserializer>::init(helper, event)?;
                    return self.handle_unop(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"fieldref" {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_fieldref(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"enumref" {
                    let output = <super::EnumrefType as WithDeserializer>::init(helper, event)?;
                    return self.handle_enumref(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"popcount" {
                    let output = <super::PopcountType as WithDeserializer>::init(helper, event)?;
                    return self.handle_popcount(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"sumof" {
                    let output = <super::SumofType as WithDeserializer>::init(helper, event)?;
                    return self.handle_sumof(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"value" {
                    let output =
                        <super::DecOrHexIntegerType as WithDeserializer>::init(helper, event)?;
                    return self.handle_value(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"bit" {
                    let output = <i32 as WithDeserializer>::init(helper, event)?;
                    return self.handle_bit(helper, Default::default(), None, output);
                }
            }
            *self.state__ = ListTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: ListTypeContentDeserializerState,
        ) -> Result<super::ListTypeContent, Error> {
            use ListTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Op(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_op(&mut values, value)?;
                    }
                    Ok(super::ListTypeContent::Op(
                        helper.finish_element("op", values)?,
                    ))
                }
                S::Unop(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_unop(&mut values, value)?;
                    }
                    Ok(super::ListTypeContent::Unop(
                        helper.finish_element("unop", values)?,
                    ))
                }
                S::Fieldref(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_fieldref(&mut values, value)?;
                    }
                    Ok(super::ListTypeContent::Fieldref(
                        helper.finish_element("fieldref", values)?,
                    ))
                }
                S::Enumref(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_enumref(&mut values, value)?;
                    }
                    Ok(super::ListTypeContent::Enumref(
                        helper.finish_element("enumref", values)?,
                    ))
                }
                S::Popcount(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_popcount(&mut values, value)?;
                    }
                    Ok(super::ListTypeContent::Popcount(
                        helper.finish_element("popcount", values)?,
                    ))
                }
                S::Sumof(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_sumof(&mut values, value)?;
                    }
                    Ok(super::ListTypeContent::Sumof(
                        helper.finish_element("sumof", values)?,
                    ))
                }
                S::Value(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_value(&mut values, value)?;
                    }
                    Ok(super::ListTypeContent::Value(
                        helper.finish_element("value", values)?,
                    ))
                }
                S::Bit(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_bit(&mut values, value)?;
                    }
                    Ok(super::ListTypeContent::Bit(
                        helper.finish_element("bit", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_op(values: &mut Option<super::OpType>, value: super::OpType) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"op")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_unop(
            values: &mut Option<super::UnopType>,
            value: super::UnopType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"unop")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_fieldref(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"fieldref",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_enumref(
            values: &mut Option<super::EnumrefType>,
            value: super::EnumrefType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"enumref",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_popcount(
            values: &mut Option<super::PopcountType>,
            value: super::PopcountType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"popcount",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_sumof(
            values: &mut Option<super::SumofType>,
            value: super::SumofType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"sumof",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_value(
            values: &mut Option<super::DecOrHexIntegerType>,
            value: super::DecOrHexIntegerType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"value",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_bit(values: &mut Option<i32>, value: i32) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"bit")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_op<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::OpType>,
            fallback: Option<<super::OpType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::OpType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ListTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_op(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_op(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Op(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Op(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_unop<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::UnopType>,
            fallback: Option<<super::UnopType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::UnopType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ListTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_unop(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_unop(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Unop(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Unop(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_fieldref<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ListTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_fieldref(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fieldref(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Fieldref(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Fieldref(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_enumref<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::EnumrefType>,
            fallback: Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::EnumrefType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ListTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_enumref(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_enumref(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Enumref(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Enumref(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_popcount<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PopcountType>,
            fallback: Option<<super::PopcountType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PopcountType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ListTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_popcount(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_popcount(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Popcount(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Popcount(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_sumof<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SumofType>,
            fallback: Option<<super::SumofType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SumofType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ListTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_sumof(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sumof(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Sumof(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Sumof(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_value<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::DecOrHexIntegerType>,
            fallback: Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::DecOrHexIntegerType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ListTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_value(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_value(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Value(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Value(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_bit<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<i32>,
            fallback: Option<<i32 as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, i32>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ListTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_bit(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bit(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Bit(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Bit(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ListTypeContent> for ListTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ListTypeContent> {
            let deserializer = Self {
                state__: Box::new(ListTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, ListTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ListTypeContent> {
            use ListTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Op(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_op(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_unop(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fieldref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_enumref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_popcount(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_sumof(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_value(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_bit(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Op(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"op", false)?;
                        match self.handle_op(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Unop(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"unop", false)?;
                        match self.handle_unop(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Fieldref(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"fieldref", false)?;
                        match self.handle_fieldref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Enumref(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"enumref", false)?;
                        match self.handle_enumref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Popcount(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"popcount", false)?;
                        match self.handle_popcount(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Sumof(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"sumof", false)?;
                        match self.handle_sumof(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Value(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"value", false)?;
                        match self.handle_value(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Bit(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"bit", false)?;
                        match self.handle_bit(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::ListTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct AnyTypeDeserializer {
        state__: Box<AnyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AnyTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl AnyTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            Ok(Self {
                state__: Box::new(AnyTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: AnyTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::AnyType> for AnyTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnyType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnyType> {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(helper)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else if matches!(&event, Event::Text(_) | Event::CData(_)) {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::None,
                    allow_any: true,
                })
            } else {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::Break(event),
                    allow_any: true,
                })
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::AnyType, Error> {
            let state = replace(&mut *self.state__, AnyTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::AnyType {})
        }
    }
    #[derive(Debug)]
    pub struct ExprfieldTypeDeserializer {
        name: String,
        type_: String,
        enum_: Option<String>,
        altenum: Option<String>,
        mask: Option<String>,
        content: Option<super::ExprfieldTypeContent>,
        state__: Box<ExprfieldTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ExprfieldTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::ExprfieldTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ExprfieldTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut name: Option<String> = None;
            let mut type_: Option<String> = None;
            let mut enum_: Option<String> = None;
            let mut altenum: Option<String> = None;
            let mut mask: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"type" {
                    helper.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"enum" {
                    helper.read_attrib(&mut enum_, b"enum", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"altenum" {
                    helper.read_attrib(&mut altenum, b"altenum", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"mask" {
                    helper.read_attrib(&mut mask, b"mask", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                type_: type_.ok_or_else(|| ErrorKind::MissingAttribute("type".into()))?,
                enum_: enum_,
                altenum: altenum,
                mask: mask,
                content: None,
                state__: Box::new(ExprfieldTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ExprfieldTypeDeserializerState,
        ) -> Result<(), Error> {
            if let ExprfieldTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ExprfieldTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ExprfieldTypeContent>,
            fallback: &mut Option<ExprfieldTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExprfieldTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ExprfieldType> for ExprfieldTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExprfieldType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExprfieldType> {
            use ExprfieldTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::ExprfieldTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ExprfieldType, Error> {
            let state = replace(
                &mut *self.state__,
                ExprfieldTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ExprfieldType {
                name: self.name,
                type_: self.type_,
                enum_: self.enum_,
                altenum: self.altenum,
                mask: self.mask,
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ExprfieldTypeContentDeserializer {
        state__: Box<ExprfieldTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ExprfieldTypeContentDeserializerState {
        Init__,
        Op(
            Option<super::OpType>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
        ),
        Unop(
            Option<super::UnopType>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
        ),
        Fieldref(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Enumref(
            Option<super::EnumrefType>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
        ),
        Popcount(
            Option<super::PopcountType>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
        ),
        Sumof(
            Option<super::SumofType>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
        ),
        Value(
            Option<super::DecOrHexIntegerType>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
        ),
        Bit(
            Option<i32>,
            Option<<i32 as WithDeserializer>::Deserializer>,
            Option<<i32 as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ExprfieldTypeContent),
        Unknown__,
    }
    impl ExprfieldTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"op" {
                    let output = <super::OpType as WithDeserializer>::init(helper, event)?;
                    return self.handle_op(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"unop" {
                    let output = <super::UnopType as WithDeserializer>::init(helper, event)?;
                    return self.handle_unop(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"fieldref" {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_fieldref(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"enumref" {
                    let output = <super::EnumrefType as WithDeserializer>::init(helper, event)?;
                    return self.handle_enumref(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"popcount" {
                    let output = <super::PopcountType as WithDeserializer>::init(helper, event)?;
                    return self.handle_popcount(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"sumof" {
                    let output = <super::SumofType as WithDeserializer>::init(helper, event)?;
                    return self.handle_sumof(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"value" {
                    let output =
                        <super::DecOrHexIntegerType as WithDeserializer>::init(helper, event)?;
                    return self.handle_value(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"bit" {
                    let output = <i32 as WithDeserializer>::init(helper, event)?;
                    return self.handle_bit(helper, Default::default(), None, output);
                }
            }
            *self.state__ = ExprfieldTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: ExprfieldTypeContentDeserializerState,
        ) -> Result<super::ExprfieldTypeContent, Error> {
            use ExprfieldTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Op(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_op(&mut values, value)?;
                    }
                    Ok(super::ExprfieldTypeContent::Op(
                        helper.finish_element("op", values)?,
                    ))
                }
                S::Unop(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_unop(&mut values, value)?;
                    }
                    Ok(super::ExprfieldTypeContent::Unop(
                        helper.finish_element("unop", values)?,
                    ))
                }
                S::Fieldref(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_fieldref(&mut values, value)?;
                    }
                    Ok(super::ExprfieldTypeContent::Fieldref(
                        helper.finish_element("fieldref", values)?,
                    ))
                }
                S::Enumref(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_enumref(&mut values, value)?;
                    }
                    Ok(super::ExprfieldTypeContent::Enumref(
                        helper.finish_element("enumref", values)?,
                    ))
                }
                S::Popcount(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_popcount(&mut values, value)?;
                    }
                    Ok(super::ExprfieldTypeContent::Popcount(
                        helper.finish_element("popcount", values)?,
                    ))
                }
                S::Sumof(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_sumof(&mut values, value)?;
                    }
                    Ok(super::ExprfieldTypeContent::Sumof(
                        helper.finish_element("sumof", values)?,
                    ))
                }
                S::Value(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_value(&mut values, value)?;
                    }
                    Ok(super::ExprfieldTypeContent::Value(
                        helper.finish_element("value", values)?,
                    ))
                }
                S::Bit(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_bit(&mut values, value)?;
                    }
                    Ok(super::ExprfieldTypeContent::Bit(
                        helper.finish_element("bit", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_op(values: &mut Option<super::OpType>, value: super::OpType) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"op")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_unop(
            values: &mut Option<super::UnopType>,
            value: super::UnopType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"unop")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_fieldref(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"fieldref",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_enumref(
            values: &mut Option<super::EnumrefType>,
            value: super::EnumrefType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"enumref",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_popcount(
            values: &mut Option<super::PopcountType>,
            value: super::PopcountType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"popcount",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_sumof(
            values: &mut Option<super::SumofType>,
            value: super::SumofType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"sumof",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_value(
            values: &mut Option<super::DecOrHexIntegerType>,
            value: super::DecOrHexIntegerType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"value",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_bit(values: &mut Option<i32>, value: i32) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"bit")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_op<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::OpType>,
            fallback: Option<<super::OpType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::OpType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExprfieldTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_op(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_op(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Op(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Op(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_unop<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::UnopType>,
            fallback: Option<<super::UnopType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::UnopType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExprfieldTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_unop(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_unop(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Unop(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Unop(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_fieldref<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExprfieldTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_fieldref(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fieldref(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Fieldref(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Fieldref(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_enumref<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::EnumrefType>,
            fallback: Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::EnumrefType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExprfieldTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_enumref(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_enumref(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Enumref(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Enumref(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_popcount<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PopcountType>,
            fallback: Option<<super::PopcountType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PopcountType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExprfieldTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_popcount(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_popcount(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Popcount(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Popcount(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_sumof<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SumofType>,
            fallback: Option<<super::SumofType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SumofType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExprfieldTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_sumof(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sumof(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Sumof(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Sumof(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_value<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::DecOrHexIntegerType>,
            fallback: Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::DecOrHexIntegerType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExprfieldTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_value(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_value(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Value(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Value(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_bit<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<i32>,
            fallback: Option<<i32 as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, i32>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ExprfieldTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_bit(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bit(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Bit(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Bit(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ExprfieldTypeContent> for ExprfieldTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExprfieldTypeContent> {
            let deserializer = Self {
                state__: Box::new(ExprfieldTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, ExprfieldTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExprfieldTypeContent> {
            use ExprfieldTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Op(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_op(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_unop(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fieldref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_enumref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_popcount(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_sumof(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_value(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_bit(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Op(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"op", false)?;
                        match self.handle_op(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Unop(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"unop", false)?;
                        match self.handle_unop(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Fieldref(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"fieldref", false)?;
                        match self.handle_fieldref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Enumref(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"enumref", false)?;
                        match self.handle_enumref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Popcount(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"popcount", false)?;
                        match self.handle_popcount(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Sumof(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"sumof", false)?;
                        match self.handle_sumof(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Value(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"value", false)?;
                        match self.handle_value(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Bit(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"bit", false)?;
                        match self.handle_bit(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::ExprfieldTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct ValueparamTypeDeserializer {
        value_mask_type: String,
        value_mask_name: String,
        value_list_name: String,
        state__: Box<ValueparamTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ValueparamTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl ValueparamTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut value_mask_type: Option<String> = None;
            let mut value_mask_name: Option<String> = None;
            let mut value_list_name: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"value-mask-type" {
                    helper.read_attrib(&mut value_mask_type, b"value-mask-type", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"value-mask-name" {
                    helper.read_attrib(&mut value_mask_name, b"value-mask-name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"value-list-name" {
                    helper.read_attrib(&mut value_list_name, b"value-list-name", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                value_mask_type: value_mask_type
                    .ok_or_else(|| ErrorKind::MissingAttribute("value-mask-type".into()))?,
                value_mask_name: value_mask_name
                    .ok_or_else(|| ErrorKind::MissingAttribute("value-mask-name".into()))?,
                value_list_name: value_list_name
                    .ok_or_else(|| ErrorKind::MissingAttribute("value-list-name".into()))?,
                state__: Box::new(ValueparamTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ValueparamTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::ValueparamType> for ValueparamTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ValueparamType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ValueparamType> {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(helper)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                })
            }
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::ValueparamType, Error> {
            let state = replace(
                &mut *self.state__,
                ValueparamTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ValueparamType {
                value_mask_type: self.value_mask_type,
                value_mask_name: self.value_mask_name,
                value_list_name: self.value_list_name,
            })
        }
    }
    #[derive(Debug)]
    pub struct SwitchexprTypeDeserializer {
        name: String,
        content: Vec<super::SwitchexprTypeContent>,
        state__: Box<SwitchexprTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SwitchexprTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::SwitchexprTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl SwitchexprTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut name: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                content: Vec::new(),
                state__: Box::new(SwitchexprTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SwitchexprTypeDeserializerState,
        ) -> Result<(), Error> {
            if let SwitchexprTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::SwitchexprTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::SwitchexprTypeContent>,
            fallback: &mut Option<SwitchexprTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SwitchexprTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SwitchexprType> for SwitchexprTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SwitchexprType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SwitchexprType> {
            use SwitchexprTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output = <super::SwitchexprTypeContent as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::SwitchexprType, Error> {
            let state = replace(
                &mut *self.state__,
                SwitchexprTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::SwitchexprType {
                name: self.name,
                content: helper.finish_vec(2usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct SwitchexprTypeContentDeserializer {
        state__: Box<SwitchexprTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum SwitchexprTypeContentDeserializerState {
        Init__,
        Op(
            Option<super::OpType>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
        ),
        Unop(
            Option<super::UnopType>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
        ),
        Fieldref(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Enumref(
            Option<super::EnumrefType>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
        ),
        Popcount(
            Option<super::PopcountType>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
        ),
        Sumof(
            Option<super::SumofType>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
        ),
        Value(
            Option<super::DecOrHexIntegerType>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
        ),
        Bit(
            Option<i32>,
            Option<<i32 as WithDeserializer>::Deserializer>,
            Option<<i32 as WithDeserializer>::Deserializer>,
        ),
        Bitcase(
            Option<super::CaseexprType>,
            Option<<super::CaseexprType as WithDeserializer>::Deserializer>,
            Option<<super::CaseexprType as WithDeserializer>::Deserializer>,
        ),
        Pad(
            Option<super::PadType>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
        ),
        Field(
            Option<super::VarType>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
        ),
        List(
            Option<super::ListType>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
        ),
        Fd(
            Option<super::AnyType>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::SwitchexprTypeContent),
        Unknown__,
    }
    impl SwitchexprTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"op" {
                    let output = <super::OpType as WithDeserializer>::init(helper, event)?;
                    return self.handle_op(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"unop" {
                    let output = <super::UnopType as WithDeserializer>::init(helper, event)?;
                    return self.handle_unop(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"fieldref" {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_fieldref(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"enumref" {
                    let output = <super::EnumrefType as WithDeserializer>::init(helper, event)?;
                    return self.handle_enumref(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"popcount" {
                    let output = <super::PopcountType as WithDeserializer>::init(helper, event)?;
                    return self.handle_popcount(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"sumof" {
                    let output = <super::SumofType as WithDeserializer>::init(helper, event)?;
                    return self.handle_sumof(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"value" {
                    let output =
                        <super::DecOrHexIntegerType as WithDeserializer>::init(helper, event)?;
                    return self.handle_value(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"bit" {
                    let output = <i32 as WithDeserializer>::init(helper, event)?;
                    return self.handle_bit(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"bitcase" {
                    let output = <super::CaseexprType as WithDeserializer>::init(helper, event)?;
                    return self.handle_bitcase(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"pad" {
                    let output = <super::PadType as WithDeserializer>::init(helper, event)?;
                    return self.handle_pad(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"field" {
                    let output = <super::VarType as WithDeserializer>::init(helper, event)?;
                    return self.handle_field(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"list" {
                    let output = <super::ListType as WithDeserializer>::init(helper, event)?;
                    return self.handle_list(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"fd" {
                    let output = <super::AnyType as WithDeserializer>::init(helper, event)?;
                    return self.handle_fd(helper, Default::default(), None, output);
                }
            }
            *self.state__ = SwitchexprTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: SwitchexprTypeContentDeserializerState,
        ) -> Result<super::SwitchexprTypeContent, Error> {
            use SwitchexprTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Op(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_op(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Op(
                        helper.finish_element("op", values)?,
                    ))
                }
                S::Unop(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_unop(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Unop(
                        helper.finish_element("unop", values)?,
                    ))
                }
                S::Fieldref(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_fieldref(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Fieldref(
                        helper.finish_element("fieldref", values)?,
                    ))
                }
                S::Enumref(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_enumref(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Enumref(
                        helper.finish_element("enumref", values)?,
                    ))
                }
                S::Popcount(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_popcount(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Popcount(
                        helper.finish_element("popcount", values)?,
                    ))
                }
                S::Sumof(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_sumof(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Sumof(
                        helper.finish_element("sumof", values)?,
                    ))
                }
                S::Value(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_value(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Value(
                        helper.finish_element("value", values)?,
                    ))
                }
                S::Bit(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_bit(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Bit(
                        helper.finish_element("bit", values)?,
                    ))
                }
                S::Bitcase(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_bitcase(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Bitcase(
                        helper.finish_element("bitcase", values)?,
                    ))
                }
                S::Pad(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_pad(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Pad(
                        helper.finish_element("pad", values)?,
                    ))
                }
                S::Field(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_field(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Field(
                        helper.finish_element("field", values)?,
                    ))
                }
                S::List(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_list(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::List(
                        helper.finish_element("list", values)?,
                    ))
                }
                S::Fd(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_fd(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Fd(
                        helper.finish_element("fd", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_op(values: &mut Option<super::OpType>, value: super::OpType) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"op")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_unop(
            values: &mut Option<super::UnopType>,
            value: super::UnopType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"unop")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_fieldref(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"fieldref",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_enumref(
            values: &mut Option<super::EnumrefType>,
            value: super::EnumrefType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"enumref",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_popcount(
            values: &mut Option<super::PopcountType>,
            value: super::PopcountType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"popcount",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_sumof(
            values: &mut Option<super::SumofType>,
            value: super::SumofType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"sumof",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_value(
            values: &mut Option<super::DecOrHexIntegerType>,
            value: super::DecOrHexIntegerType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"value",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_bit(values: &mut Option<i32>, value: i32) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"bit")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_bitcase(
            values: &mut Option<super::CaseexprType>,
            value: super::CaseexprType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"bitcase",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_pad(
            values: &mut Option<super::PadType>,
            value: super::PadType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"pad")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_field(
            values: &mut Option<super::VarType>,
            value: super::VarType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"field",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_list(
            values: &mut Option<super::ListType>,
            value: super::ListType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"list")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_fd(
            values: &mut Option<super::AnyType>,
            value: super::AnyType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"fd")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_op<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::OpType>,
            fallback: Option<<super::OpType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::OpType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SwitchexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_op(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_op(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Op(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Op(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_unop<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::UnopType>,
            fallback: Option<<super::UnopType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::UnopType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SwitchexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_unop(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_unop(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Unop(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Unop(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_fieldref<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SwitchexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_fieldref(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fieldref(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Fieldref(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Fieldref(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_enumref<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::EnumrefType>,
            fallback: Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::EnumrefType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SwitchexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_enumref(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_enumref(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Enumref(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Enumref(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_popcount<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PopcountType>,
            fallback: Option<<super::PopcountType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PopcountType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SwitchexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_popcount(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_popcount(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Popcount(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Popcount(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_sumof<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SumofType>,
            fallback: Option<<super::SumofType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SumofType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SwitchexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_sumof(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sumof(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Sumof(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Sumof(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_value<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::DecOrHexIntegerType>,
            fallback: Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::DecOrHexIntegerType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SwitchexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_value(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_value(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Value(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Value(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_bit<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<i32>,
            fallback: Option<<i32 as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, i32>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SwitchexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_bit(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bit(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Bit(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Bit(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_bitcase<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::CaseexprType>,
            fallback: Option<<super::CaseexprType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::CaseexprType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SwitchexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_bitcase(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bitcase(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Bitcase(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Bitcase(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_pad<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PadType>,
            fallback: Option<<super::PadType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PadType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SwitchexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_pad(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pad(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Pad(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Pad(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_field<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::VarType>,
            fallback: Option<<super::VarType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::VarType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SwitchexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_field(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_field(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Field(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Field(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_list<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ListType>,
            fallback: Option<<super::ListType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ListType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SwitchexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_list(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_list(&mut values, data)?;
                    let data = Self::finish_state(helper, S::List(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::List(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_fd<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnyType>,
            fallback: Option<<super::AnyType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnyType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SwitchexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_fd(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fd(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Fd(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Fd(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SwitchexprTypeContent> for SwitchexprTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SwitchexprTypeContent> {
            let deserializer = Self {
                state__: Box::new(SwitchexprTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, SwitchexprTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SwitchexprTypeContent> {
            use SwitchexprTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Op(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_op(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_unop(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fieldref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_enumref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_popcount(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_sumof(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_value(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_bit(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bitcase(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_bitcase(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Pad(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_pad(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_field(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_list(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fd(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Op(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"op", false)?;
                        match self.handle_op(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Unop(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"unop", false)?;
                        match self.handle_unop(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Fieldref(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"fieldref", false)?;
                        match self.handle_fieldref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Enumref(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"enumref", false)?;
                        match self.handle_enumref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Popcount(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"popcount", false)?;
                        match self.handle_popcount(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Sumof(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"sumof", false)?;
                        match self.handle_sumof(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Value(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"value", false)?;
                        match self.handle_value(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Bit(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"bit", false)?;
                        match self.handle_bit(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Bitcase(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"bitcase", true)?;
                        match self.handle_bitcase(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Pad(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"pad", false)?;
                        match self.handle_pad(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Field(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"field", false)?;
                        match self.handle_field(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::List(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"list", false)?;
                        match self.handle_list(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Fd(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"fd", true)?;
                        match self.handle_fd(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::SwitchexprTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct RequestReplyTypeDeserializer {
        content: Vec<super::RequestReplyTypeContent>,
        state__: Box<RequestReplyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RequestReplyTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::RequestReplyTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl RequestReplyTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: Vec::new(),
                state__: Box::new(RequestReplyTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: RequestReplyTypeDeserializerState,
        ) -> Result<(), Error> {
            if let RequestReplyTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::RequestReplyTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::RequestReplyTypeContent>,
            fallback: &mut Option<RequestReplyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RequestReplyTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RequestReplyType> for RequestReplyTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RequestReplyType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RequestReplyType> {
            use RequestReplyTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output = <super::RequestReplyTypeContent as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::RequestReplyType, Error> {
            let state = replace(
                &mut *self.state__,
                RequestReplyTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::RequestReplyType {
                content: helper.finish_vec(1usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct RequestReplyTypeContentDeserializer {
        state__: Box<RequestReplyTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum RequestReplyTypeContentDeserializerState {
        Init__,
        Pad(
            Option<super::PadType>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
        ),
        Field(
            Option<super::VarType>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
        ),
        List(
            Option<super::ListType>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
        ),
        Fd(
            Option<super::AnyType>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
        ),
        Valueparam(
            Option<super::ValueparamType>,
            Option<<super::ValueparamType as WithDeserializer>::Deserializer>,
            Option<<super::ValueparamType as WithDeserializer>::Deserializer>,
        ),
        Switch(
            Option<super::SwitchexprType>,
            Option<<super::SwitchexprType as WithDeserializer>::Deserializer>,
            Option<<super::SwitchexprType as WithDeserializer>::Deserializer>,
        ),
        Doc(
            Option<super::DocType>,
            Option<<super::DocType as WithDeserializer>::Deserializer>,
            Option<<super::DocType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::RequestReplyTypeContent),
        Unknown__,
    }
    impl RequestReplyTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"pad" {
                    let output = <super::PadType as WithDeserializer>::init(helper, event)?;
                    return self.handle_pad(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"field" {
                    let output = <super::VarType as WithDeserializer>::init(helper, event)?;
                    return self.handle_field(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"list" {
                    let output = <super::ListType as WithDeserializer>::init(helper, event)?;
                    return self.handle_list(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"fd" {
                    let output = <super::AnyType as WithDeserializer>::init(helper, event)?;
                    return self.handle_fd(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"valueparam" {
                    let output = <super::ValueparamType as WithDeserializer>::init(helper, event)?;
                    return self.handle_valueparam(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"switch" {
                    let output = <super::SwitchexprType as WithDeserializer>::init(helper, event)?;
                    return self.handle_switch(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"doc" {
                    let output = <super::DocType as WithDeserializer>::init(helper, event)?;
                    return self.handle_doc(helper, Default::default(), None, output);
                }
            }
            *self.state__ = RequestReplyTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: RequestReplyTypeContentDeserializerState,
        ) -> Result<super::RequestReplyTypeContent, Error> {
            use RequestReplyTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Pad(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_pad(&mut values, value)?;
                    }
                    Ok(super::RequestReplyTypeContent::Pad(
                        helper.finish_element("pad", values)?,
                    ))
                }
                S::Field(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_field(&mut values, value)?;
                    }
                    Ok(super::RequestReplyTypeContent::Field(
                        helper.finish_element("field", values)?,
                    ))
                }
                S::List(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_list(&mut values, value)?;
                    }
                    Ok(super::RequestReplyTypeContent::List(
                        helper.finish_element("list", values)?,
                    ))
                }
                S::Fd(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_fd(&mut values, value)?;
                    }
                    Ok(super::RequestReplyTypeContent::Fd(
                        helper.finish_element("fd", values)?,
                    ))
                }
                S::Valueparam(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_valueparam(&mut values, value)?;
                    }
                    Ok(super::RequestReplyTypeContent::Valueparam(
                        helper.finish_element("valueparam", values)?,
                    ))
                }
                S::Switch(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_switch(&mut values, value)?;
                    }
                    Ok(super::RequestReplyTypeContent::Switch(
                        helper.finish_element("switch", values)?,
                    ))
                }
                S::Doc(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_doc(&mut values, value)?;
                    }
                    Ok(super::RequestReplyTypeContent::Doc(
                        helper.finish_element("doc", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_pad(
            values: &mut Option<super::PadType>,
            value: super::PadType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"pad")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_field(
            values: &mut Option<super::VarType>,
            value: super::VarType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"field",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_list(
            values: &mut Option<super::ListType>,
            value: super::ListType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"list")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_fd(
            values: &mut Option<super::AnyType>,
            value: super::AnyType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"fd")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_valueparam(
            values: &mut Option<super::ValueparamType>,
            value: super::ValueparamType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"valueparam",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_switch(
            values: &mut Option<super::SwitchexprType>,
            value: super::SwitchexprType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"switch",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_doc(
            values: &mut Option<super::DocType>,
            value: super::DocType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"doc")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_pad<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PadType>,
            fallback: Option<<super::PadType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PadType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RequestReplyTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_pad(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pad(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Pad(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Pad(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_field<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::VarType>,
            fallback: Option<<super::VarType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::VarType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RequestReplyTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_field(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_field(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Field(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Field(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_list<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ListType>,
            fallback: Option<<super::ListType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ListType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RequestReplyTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_list(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_list(&mut values, data)?;
                    let data = Self::finish_state(helper, S::List(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::List(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_fd<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnyType>,
            fallback: Option<<super::AnyType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnyType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RequestReplyTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_fd(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fd(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Fd(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Fd(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_valueparam<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ValueparamType>,
            fallback: Option<<super::ValueparamType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ValueparamType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RequestReplyTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_valueparam(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_valueparam(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Valueparam(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Valueparam(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_switch<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SwitchexprType>,
            fallback: Option<<super::SwitchexprType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SwitchexprType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RequestReplyTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_switch(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_switch(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Switch(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Switch(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_doc<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::DocType>,
            fallback: Option<<super::DocType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::DocType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RequestReplyTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_doc(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_doc(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Doc(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Doc(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RequestReplyTypeContent>
        for RequestReplyTypeContentDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RequestReplyTypeContent> {
            let deserializer = Self {
                state__: Box::new(RequestReplyTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        RequestReplyTypeContentDeserializerState::Init__
                    ) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RequestReplyTypeContent> {
            use RequestReplyTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Pad(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_pad(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_field(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_list(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fd(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Valueparam(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_valueparam(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Switch(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_switch(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Doc(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_doc(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Pad(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"pad", false)?;
                        match self.handle_pad(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Field(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"field", false)?;
                        match self.handle_field(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::List(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"list", false)?;
                        match self.handle_list(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Fd(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"fd", true)?;
                        match self.handle_fd(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Valueparam(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            None,
                            b"valueparam",
                            false,
                        )?;
                        match self.handle_valueparam(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Switch(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"switch", true)?;
                        match self.handle_switch(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Doc(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"doc", false)?;
                        match self.handle_doc(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::RequestReplyTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct DocTypeDeserializer {
        content: Vec<super::DocTypeContent>,
        state__: Box<DocTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DocTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::DocTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl DocTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: Vec::new(),
                state__: Box::new(DocTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: DocTypeDeserializerState,
        ) -> Result<(), Error> {
            if let DocTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::DocTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::DocTypeContent>,
            fallback: &mut Option<DocTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DocTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DocType> for DocTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocType> {
            use DocTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::DocTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::DocType, Error> {
            let state = replace(&mut *self.state__, DocTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::DocType {
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DocTypeContentDeserializer {
        state__: Box<DocTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum DocTypeContentDeserializerState {
        Init__,
        Brief(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Description(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Example(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Field(
            Option<super::FieldType>,
            Option<<super::FieldType as WithDeserializer>::Deserializer>,
            Option<<super::FieldType as WithDeserializer>::Deserializer>,
        ),
        Error(
            Option<super::ErrorType>,
            Option<<super::ErrorType as WithDeserializer>::Deserializer>,
            Option<<super::ErrorType as WithDeserializer>::Deserializer>,
        ),
        See(
            Option<super::SeeType>,
            Option<<super::SeeType as WithDeserializer>::Deserializer>,
            Option<<super::SeeType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::DocTypeContent),
        Unknown__,
    }
    impl DocTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"brief" {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_brief(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"description" {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_description(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"example" {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_example(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"field" {
                    let output = <super::FieldType as WithDeserializer>::init(helper, event)?;
                    return self.handle_field(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"error" {
                    let output = <super::ErrorType as WithDeserializer>::init(helper, event)?;
                    return self.handle_error(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"see" {
                    let output = <super::SeeType as WithDeserializer>::init(helper, event)?;
                    return self.handle_see(helper, Default::default(), None, output);
                }
            }
            *self.state__ = DocTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: DocTypeContentDeserializerState,
        ) -> Result<super::DocTypeContent, Error> {
            use DocTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Brief(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_brief(&mut values, value)?;
                    }
                    Ok(super::DocTypeContent::Brief(
                        helper.finish_element("brief", values)?,
                    ))
                }
                S::Description(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_description(&mut values, value)?;
                    }
                    Ok(super::DocTypeContent::Description(
                        helper.finish_element("description", values)?,
                    ))
                }
                S::Example(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_example(&mut values, value)?;
                    }
                    Ok(super::DocTypeContent::Example(
                        helper.finish_element("example", values)?,
                    ))
                }
                S::Field(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_field(&mut values, value)?;
                    }
                    Ok(super::DocTypeContent::Field(
                        helper.finish_element("field", values)?,
                    ))
                }
                S::Error(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_error(&mut values, value)?;
                    }
                    Ok(super::DocTypeContent::Error(
                        helper.finish_element("error", values)?,
                    ))
                }
                S::See(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_see(&mut values, value)?;
                    }
                    Ok(super::DocTypeContent::See(
                        helper.finish_element("see", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_brief(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"brief",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_description(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"description",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_example(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"example",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_field(
            values: &mut Option<super::FieldType>,
            value: super::FieldType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"field",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_error(
            values: &mut Option<super::ErrorType>,
            value: super::ErrorType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"error",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_see(
            values: &mut Option<super::SeeType>,
            value: super::SeeType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"see")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_brief<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DocTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_brief(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_brief(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Brief(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Brief(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_description<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DocTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_description(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_description(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Description(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Description(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_example<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DocTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_example(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_example(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Example(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Example(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_field<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::FieldType>,
            fallback: Option<<super::FieldType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::FieldType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DocTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_field(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_field(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Field(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Field(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_error<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ErrorType>,
            fallback: Option<<super::ErrorType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ErrorType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DocTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_error(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_error(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Error(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Error(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_see<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SeeType>,
            fallback: Option<<super::SeeType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SeeType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DocTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_see(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_see(&mut values, data)?;
                    let data = Self::finish_state(helper, S::See(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::See(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DocTypeContent> for DocTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocTypeContent> {
            let deserializer = Self {
                state__: Box::new(DocTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, DocTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocTypeContent> {
            use DocTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Brief(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_brief(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Description(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_description(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Example(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_example(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_field(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Error(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_error(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::See(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_see(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Brief(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"brief", false)?;
                        match self.handle_brief(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Description(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            None,
                            b"description",
                            false,
                        )?;
                        match self.handle_description(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Example(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"example", false)?;
                        match self.handle_example(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Field(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"field", false)?;
                        match self.handle_field(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Error(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"error", false)?;
                        match self.handle_error(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::See(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"see", false)?;
                        match self.handle_see(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, Event::Text(_) | Event::CData(_)) => {
                        *self.state__ = state;
                        break (DeserializerEvent::None, false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::DocTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct EnumItemTypeDeserializer {
        name: String,
        content: Option<super::EnumItemTypeContent>,
        state__: Box<EnumItemTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum EnumItemTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::EnumItemTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl EnumItemTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut name: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                content: None,
                state__: Box::new(EnumItemTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: EnumItemTypeDeserializerState,
        ) -> Result<(), Error> {
            if let EnumItemTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::EnumItemTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::EnumItemTypeContent>,
            fallback: &mut Option<EnumItemTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use EnumItemTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::EnumItemType> for EnumItemTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EnumItemType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EnumItemType> {
            use EnumItemTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::EnumItemTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::EnumItemType, Error> {
            let state = replace(&mut *self.state__, EnumItemTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::EnumItemType {
                name: self.name,
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct EnumItemTypeContentDeserializer {
        state__: Box<EnumItemTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum EnumItemTypeContentDeserializerState {
        Init__,
        Value(
            Option<super::DecOrHexIntegerType>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
        ),
        Bit(
            Option<i32>,
            Option<<i32 as WithDeserializer>::Deserializer>,
            Option<<i32 as WithDeserializer>::Deserializer>,
        ),
        Done__(super::EnumItemTypeContent),
        Unknown__,
    }
    impl EnumItemTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"value" {
                    let output =
                        <super::DecOrHexIntegerType as WithDeserializer>::init(helper, event)?;
                    return self.handle_value(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"bit" {
                    let output = <i32 as WithDeserializer>::init(helper, event)?;
                    return self.handle_bit(helper, Default::default(), None, output);
                }
            }
            *self.state__ = EnumItemTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: EnumItemTypeContentDeserializerState,
        ) -> Result<super::EnumItemTypeContent, Error> {
            use EnumItemTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Value(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_value(&mut values, value)?;
                    }
                    Ok(super::EnumItemTypeContent::Value(
                        helper.finish_element("value", values)?,
                    ))
                }
                S::Bit(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_bit(&mut values, value)?;
                    }
                    Ok(super::EnumItemTypeContent::Bit(
                        helper.finish_element("bit", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_value(
            values: &mut Option<super::DecOrHexIntegerType>,
            value: super::DecOrHexIntegerType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"value",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_bit(values: &mut Option<i32>, value: i32) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"bit")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_value<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::DecOrHexIntegerType>,
            fallback: Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::DecOrHexIntegerType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use EnumItemTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_value(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_value(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Value(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Value(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_bit<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<i32>,
            fallback: Option<<i32 as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, i32>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use EnumItemTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_bit(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bit(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Bit(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Bit(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::EnumItemTypeContent> for EnumItemTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EnumItemTypeContent> {
            let deserializer = Self {
                state__: Box::new(EnumItemTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, EnumItemTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EnumItemTypeContent> {
            use EnumItemTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Value(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_value(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_bit(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Value(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"value", false)?;
                        match self.handle_value(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Bit(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"bit", false)?;
                        match self.handle_bit(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::EnumItemTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct OpTypeDeserializer {
        op: String,
        content: Vec<super::OpTypeContent>,
        state__: Box<OpTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum OpTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::OpTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl OpTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut op: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"op" {
                    helper.read_attrib(&mut op, b"op", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                op: op.ok_or_else(|| ErrorKind::MissingAttribute("op".into()))?,
                content: Vec::new(),
                state__: Box::new(OpTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: OpTypeDeserializerState,
        ) -> Result<(), Error> {
            if let OpTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::OpTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::OpTypeContent>,
            fallback: &mut Option<OpTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OpTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    if self.content.len() < 1usize {
                        *fallback = Some(S::Content__(deserializer));
                        *self.state__ = S::Next__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    } else {
                        *self.state__ = S::Content__(deserializer);
                        Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                    }
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::OpType> for OpTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OpType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OpType> {
            use OpTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::OpTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::OpType, Error> {
            let state = replace(&mut *self.state__, OpTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::OpType {
                op: self.op,
                content: helper.finish_arr::<_, 2usize>(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct OpTypeContentDeserializer {
        state__: Box<OpTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum OpTypeContentDeserializerState {
        Init__,
        Op(
            Option<super::OpType>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
        ),
        Unop(
            Option<super::UnopType>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
        ),
        Fieldref(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Enumref(
            Option<super::EnumrefType>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
        ),
        Popcount(
            Option<super::PopcountType>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
        ),
        Sumof(
            Option<super::SumofType>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
        ),
        Value(
            Option<super::DecOrHexIntegerType>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
        ),
        Bit(
            Option<i32>,
            Option<<i32 as WithDeserializer>::Deserializer>,
            Option<<i32 as WithDeserializer>::Deserializer>,
        ),
        Done__(super::OpTypeContent),
        Unknown__,
    }
    impl OpTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"op" {
                    let output = <super::OpType as WithDeserializer>::init(helper, event)?;
                    return self.handle_op(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"unop" {
                    let output = <super::UnopType as WithDeserializer>::init(helper, event)?;
                    return self.handle_unop(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"fieldref" {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_fieldref(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"enumref" {
                    let output = <super::EnumrefType as WithDeserializer>::init(helper, event)?;
                    return self.handle_enumref(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"popcount" {
                    let output = <super::PopcountType as WithDeserializer>::init(helper, event)?;
                    return self.handle_popcount(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"sumof" {
                    let output = <super::SumofType as WithDeserializer>::init(helper, event)?;
                    return self.handle_sumof(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"value" {
                    let output =
                        <super::DecOrHexIntegerType as WithDeserializer>::init(helper, event)?;
                    return self.handle_value(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"bit" {
                    let output = <i32 as WithDeserializer>::init(helper, event)?;
                    return self.handle_bit(helper, Default::default(), None, output);
                }
            }
            *self.state__ = OpTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: OpTypeContentDeserializerState,
        ) -> Result<super::OpTypeContent, Error> {
            use OpTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Op(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_op(&mut values, value)?;
                    }
                    Ok(super::OpTypeContent::Op(Box::new(
                        helper.finish_element("op", values)?,
                    )))
                }
                S::Unop(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_unop(&mut values, value)?;
                    }
                    Ok(super::OpTypeContent::Unop(
                        helper.finish_element("unop", values)?,
                    ))
                }
                S::Fieldref(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_fieldref(&mut values, value)?;
                    }
                    Ok(super::OpTypeContent::Fieldref(
                        helper.finish_element("fieldref", values)?,
                    ))
                }
                S::Enumref(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_enumref(&mut values, value)?;
                    }
                    Ok(super::OpTypeContent::Enumref(
                        helper.finish_element("enumref", values)?,
                    ))
                }
                S::Popcount(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_popcount(&mut values, value)?;
                    }
                    Ok(super::OpTypeContent::Popcount(
                        helper.finish_element("popcount", values)?,
                    ))
                }
                S::Sumof(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_sumof(&mut values, value)?;
                    }
                    Ok(super::OpTypeContent::Sumof(
                        helper.finish_element("sumof", values)?,
                    ))
                }
                S::Value(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_value(&mut values, value)?;
                    }
                    Ok(super::OpTypeContent::Value(
                        helper.finish_element("value", values)?,
                    ))
                }
                S::Bit(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_bit(&mut values, value)?;
                    }
                    Ok(super::OpTypeContent::Bit(
                        helper.finish_element("bit", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_op(values: &mut Option<super::OpType>, value: super::OpType) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"op")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_unop(
            values: &mut Option<super::UnopType>,
            value: super::UnopType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"unop")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_fieldref(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"fieldref",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_enumref(
            values: &mut Option<super::EnumrefType>,
            value: super::EnumrefType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"enumref",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_popcount(
            values: &mut Option<super::PopcountType>,
            value: super::PopcountType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"popcount",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_sumof(
            values: &mut Option<super::SumofType>,
            value: super::SumofType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"sumof",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_value(
            values: &mut Option<super::DecOrHexIntegerType>,
            value: super::DecOrHexIntegerType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"value",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_bit(values: &mut Option<i32>, value: i32) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"bit")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_op<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::OpType>,
            fallback: Option<<super::OpType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::OpType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OpTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_op(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_op(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Op(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Op(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_unop<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::UnopType>,
            fallback: Option<<super::UnopType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::UnopType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OpTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_unop(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_unop(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Unop(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Unop(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_fieldref<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OpTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_fieldref(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fieldref(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Fieldref(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Fieldref(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_enumref<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::EnumrefType>,
            fallback: Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::EnumrefType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OpTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_enumref(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_enumref(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Enumref(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Enumref(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_popcount<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PopcountType>,
            fallback: Option<<super::PopcountType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PopcountType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OpTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_popcount(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_popcount(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Popcount(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Popcount(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_sumof<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SumofType>,
            fallback: Option<<super::SumofType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SumofType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OpTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_sumof(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sumof(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Sumof(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Sumof(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_value<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::DecOrHexIntegerType>,
            fallback: Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::DecOrHexIntegerType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OpTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_value(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_value(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Value(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Value(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_bit<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<i32>,
            fallback: Option<<i32 as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, i32>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use OpTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_bit(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bit(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Bit(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Bit(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::OpTypeContent> for OpTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OpTypeContent> {
            let deserializer = Self {
                state__: Box::new(OpTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, OpTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OpTypeContent> {
            use OpTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Op(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_op(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_unop(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fieldref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_enumref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_popcount(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_sumof(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_value(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_bit(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Op(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"op", false)?;
                        match self.handle_op(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Unop(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"unop", false)?;
                        match self.handle_unop(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Fieldref(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"fieldref", false)?;
                        match self.handle_fieldref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Enumref(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"enumref", false)?;
                        match self.handle_enumref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Popcount(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"popcount", false)?;
                        match self.handle_popcount(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Sumof(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"sumof", false)?;
                        match self.handle_sumof(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Value(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"value", false)?;
                        match self.handle_value(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Bit(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"bit", false)?;
                        match self.handle_bit(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::OpTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct UnopTypeDeserializer {
        op: String,
        content: Option<super::UnopTypeContent>,
        state__: Box<UnopTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum UnopTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::UnopTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl UnopTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut op: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"op" {
                    helper.read_attrib(&mut op, b"op", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                op: op.ok_or_else(|| ErrorKind::MissingAttribute("op".into()))?,
                content: None,
                state__: Box::new(UnopTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: UnopTypeDeserializerState,
        ) -> Result<(), Error> {
            if let UnopTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::UnopTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::UnopTypeContent>,
            fallback: &mut Option<UnopTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use UnopTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::UnopType> for UnopTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UnopType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UnopType> {
            use UnopTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::UnopTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::UnopType, Error> {
            let state = replace(&mut *self.state__, UnopTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::UnopType {
                op: self.op,
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct UnopTypeContentDeserializer {
        state__: Box<UnopTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum UnopTypeContentDeserializerState {
        Init__,
        Op(
            Option<super::OpType>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
        ),
        Unop(
            Option<super::UnopType>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
        ),
        Fieldref(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Enumref(
            Option<super::EnumrefType>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
        ),
        Popcount(
            Option<super::PopcountType>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
        ),
        Sumof(
            Option<super::SumofType>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
        ),
        Value(
            Option<super::DecOrHexIntegerType>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
        ),
        Bit(
            Option<i32>,
            Option<<i32 as WithDeserializer>::Deserializer>,
            Option<<i32 as WithDeserializer>::Deserializer>,
        ),
        Done__(super::UnopTypeContent),
        Unknown__,
    }
    impl UnopTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"op" {
                    let output = <super::OpType as WithDeserializer>::init(helper, event)?;
                    return self.handle_op(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"unop" {
                    let output = <super::UnopType as WithDeserializer>::init(helper, event)?;
                    return self.handle_unop(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"fieldref" {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_fieldref(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"enumref" {
                    let output = <super::EnumrefType as WithDeserializer>::init(helper, event)?;
                    return self.handle_enumref(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"popcount" {
                    let output = <super::PopcountType as WithDeserializer>::init(helper, event)?;
                    return self.handle_popcount(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"sumof" {
                    let output = <super::SumofType as WithDeserializer>::init(helper, event)?;
                    return self.handle_sumof(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"value" {
                    let output =
                        <super::DecOrHexIntegerType as WithDeserializer>::init(helper, event)?;
                    return self.handle_value(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"bit" {
                    let output = <i32 as WithDeserializer>::init(helper, event)?;
                    return self.handle_bit(helper, Default::default(), None, output);
                }
            }
            *self.state__ = UnopTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: UnopTypeContentDeserializerState,
        ) -> Result<super::UnopTypeContent, Error> {
            use UnopTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Op(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_op(&mut values, value)?;
                    }
                    Ok(super::UnopTypeContent::Op(Box::new(
                        helper.finish_element("op", values)?,
                    )))
                }
                S::Unop(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_unop(&mut values, value)?;
                    }
                    Ok(super::UnopTypeContent::Unop(Box::new(
                        helper.finish_element("unop", values)?,
                    )))
                }
                S::Fieldref(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_fieldref(&mut values, value)?;
                    }
                    Ok(super::UnopTypeContent::Fieldref(
                        helper.finish_element("fieldref", values)?,
                    ))
                }
                S::Enumref(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_enumref(&mut values, value)?;
                    }
                    Ok(super::UnopTypeContent::Enumref(
                        helper.finish_element("enumref", values)?,
                    ))
                }
                S::Popcount(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_popcount(&mut values, value)?;
                    }
                    Ok(super::UnopTypeContent::Popcount(
                        helper.finish_element("popcount", values)?,
                    ))
                }
                S::Sumof(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_sumof(&mut values, value)?;
                    }
                    Ok(super::UnopTypeContent::Sumof(
                        helper.finish_element("sumof", values)?,
                    ))
                }
                S::Value(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_value(&mut values, value)?;
                    }
                    Ok(super::UnopTypeContent::Value(
                        helper.finish_element("value", values)?,
                    ))
                }
                S::Bit(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_bit(&mut values, value)?;
                    }
                    Ok(super::UnopTypeContent::Bit(
                        helper.finish_element("bit", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_op(values: &mut Option<super::OpType>, value: super::OpType) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"op")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_unop(
            values: &mut Option<super::UnopType>,
            value: super::UnopType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"unop")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_fieldref(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"fieldref",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_enumref(
            values: &mut Option<super::EnumrefType>,
            value: super::EnumrefType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"enumref",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_popcount(
            values: &mut Option<super::PopcountType>,
            value: super::PopcountType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"popcount",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_sumof(
            values: &mut Option<super::SumofType>,
            value: super::SumofType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"sumof",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_value(
            values: &mut Option<super::DecOrHexIntegerType>,
            value: super::DecOrHexIntegerType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"value",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_bit(values: &mut Option<i32>, value: i32) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"bit")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_op<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::OpType>,
            fallback: Option<<super::OpType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::OpType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use UnopTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_op(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_op(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Op(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Op(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_unop<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::UnopType>,
            fallback: Option<<super::UnopType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::UnopType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use UnopTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_unop(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_unop(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Unop(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Unop(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_fieldref<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use UnopTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_fieldref(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fieldref(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Fieldref(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Fieldref(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_enumref<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::EnumrefType>,
            fallback: Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::EnumrefType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use UnopTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_enumref(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_enumref(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Enumref(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Enumref(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_popcount<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PopcountType>,
            fallback: Option<<super::PopcountType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PopcountType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use UnopTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_popcount(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_popcount(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Popcount(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Popcount(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_sumof<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SumofType>,
            fallback: Option<<super::SumofType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SumofType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use UnopTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_sumof(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sumof(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Sumof(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Sumof(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_value<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::DecOrHexIntegerType>,
            fallback: Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::DecOrHexIntegerType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use UnopTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_value(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_value(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Value(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Value(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_bit<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<i32>,
            fallback: Option<<i32 as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, i32>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use UnopTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_bit(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bit(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Bit(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Bit(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::UnopTypeContent> for UnopTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UnopTypeContent> {
            let deserializer = Self {
                state__: Box::new(UnopTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, UnopTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UnopTypeContent> {
            use UnopTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Op(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_op(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_unop(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fieldref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_enumref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_popcount(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_sumof(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_value(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_bit(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Op(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"op", false)?;
                        match self.handle_op(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Unop(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"unop", false)?;
                        match self.handle_unop(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Fieldref(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"fieldref", false)?;
                        match self.handle_fieldref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Enumref(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"enumref", false)?;
                        match self.handle_enumref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Popcount(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"popcount", false)?;
                        match self.handle_popcount(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Sumof(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"sumof", false)?;
                        match self.handle_sumof(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Value(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"value", false)?;
                        match self.handle_value(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Bit(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"bit", false)?;
                        match self.handle_bit(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::UnopTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct EnumrefTypeDeserializer {
        ref_: String,
        content: Option<String>,
        state__: Box<EnumrefTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum EnumrefTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl EnumrefTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut ref_: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"ref" {
                    helper.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                ref_: ref_.ok_or_else(|| ErrorKind::MissingAttribute("ref".into()))?,
                content: None,
                state__: Box::new(EnumrefTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: EnumrefTypeDeserializerState,
        ) -> Result<(), Error> {
            if let EnumrefTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::EnumrefType> {
            use EnumrefTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::EnumrefType> for EnumrefTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EnumrefType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EnumrefType> {
            use EnumrefTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::EnumrefType, Error> {
            let state = replace(&mut *self.state__, EnumrefTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::EnumrefType {
                ref_: self.ref_,
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PopcountTypeDeserializer {
        state__: Box<PopcountTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum PopcountTypeDeserializerState {
        Init__,
        Op(
            Option<super::OpType>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
        ),
        Unop(
            Option<super::UnopType>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
        ),
        Fieldref(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Enumref(
            Option<super::EnumrefType>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
        ),
        Popcount(
            Option<super::PopcountType>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
        ),
        Sumof(
            Option<super::SumofType>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
        ),
        Value(
            Option<super::DecOrHexIntegerType>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
        ),
        Bit(
            Option<i32>,
            Option<<i32 as WithDeserializer>::Deserializer>,
            Option<<i32 as WithDeserializer>::Deserializer>,
        ),
        Done__(super::PopcountType),
        Unknown__,
    }
    impl PopcountTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"op" {
                    let output = <super::OpType as WithDeserializer>::init(helper, event)?;
                    return self.handle_op(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"unop" {
                    let output = <super::UnopType as WithDeserializer>::init(helper, event)?;
                    return self.handle_unop(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"fieldref" {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_fieldref(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"enumref" {
                    let output = <super::EnumrefType as WithDeserializer>::init(helper, event)?;
                    return self.handle_enumref(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"popcount" {
                    let output = <super::PopcountType as WithDeserializer>::init(helper, event)?;
                    return self.handle_popcount(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"sumof" {
                    let output = <super::SumofType as WithDeserializer>::init(helper, event)?;
                    return self.handle_sumof(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"value" {
                    let output =
                        <super::DecOrHexIntegerType as WithDeserializer>::init(helper, event)?;
                    return self.handle_value(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"bit" {
                    let output = <i32 as WithDeserializer>::init(helper, event)?;
                    return self.handle_bit(helper, Default::default(), None, output);
                }
            }
            *self.state__ = PopcountTypeDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                state__: Box::new(PopcountTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: PopcountTypeDeserializerState,
        ) -> Result<super::PopcountType, Error> {
            use PopcountTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Op(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_op(&mut values, value)?;
                    }
                    Ok(super::PopcountType::Op(Box::new(
                        helper.finish_element("op", values)?,
                    )))
                }
                S::Unop(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_unop(&mut values, value)?;
                    }
                    Ok(super::PopcountType::Unop(Box::new(
                        helper.finish_element("unop", values)?,
                    )))
                }
                S::Fieldref(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_fieldref(&mut values, value)?;
                    }
                    Ok(super::PopcountType::Fieldref(
                        helper.finish_element("fieldref", values)?,
                    ))
                }
                S::Enumref(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_enumref(&mut values, value)?;
                    }
                    Ok(super::PopcountType::Enumref(
                        helper.finish_element("enumref", values)?,
                    ))
                }
                S::Popcount(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_popcount(&mut values, value)?;
                    }
                    Ok(super::PopcountType::Popcount(Box::new(
                        helper.finish_element("popcount", values)?,
                    )))
                }
                S::Sumof(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_sumof(&mut values, value)?;
                    }
                    Ok(super::PopcountType::Sumof(
                        helper.finish_element("sumof", values)?,
                    ))
                }
                S::Value(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_value(&mut values, value)?;
                    }
                    Ok(super::PopcountType::Value(
                        helper.finish_element("value", values)?,
                    ))
                }
                S::Bit(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_bit(&mut values, value)?;
                    }
                    Ok(super::PopcountType::Bit(
                        helper.finish_element("bit", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_op(values: &mut Option<super::OpType>, value: super::OpType) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"op")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_unop(
            values: &mut Option<super::UnopType>,
            value: super::UnopType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"unop")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_fieldref(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"fieldref",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_enumref(
            values: &mut Option<super::EnumrefType>,
            value: super::EnumrefType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"enumref",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_popcount(
            values: &mut Option<super::PopcountType>,
            value: super::PopcountType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"popcount",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_sumof(
            values: &mut Option<super::SumofType>,
            value: super::SumofType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"sumof",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_value(
            values: &mut Option<super::DecOrHexIntegerType>,
            value: super::DecOrHexIntegerType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"value",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_bit(values: &mut Option<i32>, value: i32) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"bit")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_op<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::OpType>,
            fallback: Option<<super::OpType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::OpType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PopcountTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_op(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_op(&mut values, data)?;
                    *self.state__ = S::Op(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Op(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
        fn handle_unop<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::UnopType>,
            fallback: Option<<super::UnopType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::UnopType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PopcountTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_unop(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_unop(&mut values, data)?;
                    *self.state__ = S::Unop(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Unop(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
        fn handle_fieldref<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PopcountTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_fieldref(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fieldref(&mut values, data)?;
                    *self.state__ = S::Fieldref(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Fieldref(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
        fn handle_enumref<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::EnumrefType>,
            fallback: Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::EnumrefType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PopcountTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_enumref(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_enumref(&mut values, data)?;
                    *self.state__ = S::Enumref(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Enumref(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
        fn handle_popcount<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PopcountType>,
            fallback: Option<<super::PopcountType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PopcountType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PopcountTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_popcount(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_popcount(&mut values, data)?;
                    *self.state__ = S::Popcount(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Popcount(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
        fn handle_sumof<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SumofType>,
            fallback: Option<<super::SumofType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SumofType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PopcountTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_sumof(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sumof(&mut values, data)?;
                    *self.state__ = S::Sumof(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Sumof(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
        fn handle_value<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::DecOrHexIntegerType>,
            fallback: Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::DecOrHexIntegerType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PopcountTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_value(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_value(&mut values, data)?;
                    *self.state__ = S::Value(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Value(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
        fn handle_bit<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<i32>,
            fallback: Option<<i32 as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, i32>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PopcountTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_bit(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bit(&mut values, data)?;
                    *self.state__ = S::Bit(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Bit(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::PopcountType> for PopcountTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PopcountType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PopcountType> {
            use PopcountTypeDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Op(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_op(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_unop(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fieldref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_enumref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_popcount(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_sumof(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_value(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_bit(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Op(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"op", false)?;
                        match self.handle_op(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Unop(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"unop", false)?;
                        match self.handle_unop(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Fieldref(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"fieldref", false)?;
                        match self.handle_fieldref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Enumref(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"enumref", false)?;
                        match self.handle_enumref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Popcount(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"popcount", false)?;
                        match self.handle_popcount(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Sumof(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"sumof", false)?;
                        match self.handle_sumof(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Value(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"value", false)?;
                        match self.handle_value(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Bit(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"bit", false)?;
                        match self.handle_bit(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::PopcountType, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct SumofTypeDeserializer {
        ref_: String,
        state__: Box<SumofTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SumofTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl SumofTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut ref_: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"ref" {
                    helper.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                ref_: ref_.ok_or_else(|| ErrorKind::MissingAttribute("ref".into()))?,
                state__: Box::new(SumofTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SumofTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::SumofType> for SumofTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SumofType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SumofType> {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(helper)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                })
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::SumofType, Error> {
            let state = replace(&mut *self.state__, SumofTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::SumofType { ref_: self.ref_ })
        }
    }
    #[derive(Debug)]
    pub struct CaseexprTypeDeserializer {
        name: Option<String>,
        content: Vec<super::CaseexprTypeContent>,
        state__: Box<CaseexprTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CaseexprTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::CaseexprTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl CaseexprTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut name: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                name: name,
                content: Vec::new(),
                state__: Box::new(CaseexprTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: CaseexprTypeDeserializerState,
        ) -> Result<(), Error> {
            if let CaseexprTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::CaseexprTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::CaseexprTypeContent>,
            fallback: &mut Option<CaseexprTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CaseexprTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::CaseexprType> for CaseexprTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CaseexprType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CaseexprType> {
            use CaseexprTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::CaseexprTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::CaseexprType, Error> {
            let state = replace(&mut *self.state__, CaseexprTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::CaseexprType {
                name: self.name,
                content: helper.finish_vec(2usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct CaseexprTypeContentDeserializer {
        state__: Box<CaseexprTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum CaseexprTypeContentDeserializerState {
        Init__,
        Op(
            Option<super::OpType>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
        ),
        Unop(
            Option<super::UnopType>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
        ),
        Fieldref(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Enumref(
            Option<super::EnumrefType>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
        ),
        Popcount(
            Option<super::PopcountType>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
        ),
        Sumof(
            Option<super::SumofType>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
        ),
        Value(
            Option<super::DecOrHexIntegerType>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
        ),
        Bit(
            Option<i32>,
            Option<<i32 as WithDeserializer>::Deserializer>,
            Option<<i32 as WithDeserializer>::Deserializer>,
        ),
        Pad(
            Option<super::PadType>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
        ),
        Field(
            Option<super::VarType>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
        ),
        List(
            Option<super::ListType>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
        ),
        Fd(
            Option<super::AnyType>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
        ),
        Switch(
            Option<super::SwitchexprType>,
            Option<<super::SwitchexprType as WithDeserializer>::Deserializer>,
            Option<<super::SwitchexprType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::CaseexprTypeContent),
        Unknown__,
    }
    impl CaseexprTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"op" {
                    let output = <super::OpType as WithDeserializer>::init(helper, event)?;
                    return self.handle_op(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"unop" {
                    let output = <super::UnopType as WithDeserializer>::init(helper, event)?;
                    return self.handle_unop(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"fieldref" {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_fieldref(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"enumref" {
                    let output = <super::EnumrefType as WithDeserializer>::init(helper, event)?;
                    return self.handle_enumref(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"popcount" {
                    let output = <super::PopcountType as WithDeserializer>::init(helper, event)?;
                    return self.handle_popcount(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"sumof" {
                    let output = <super::SumofType as WithDeserializer>::init(helper, event)?;
                    return self.handle_sumof(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"value" {
                    let output =
                        <super::DecOrHexIntegerType as WithDeserializer>::init(helper, event)?;
                    return self.handle_value(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"bit" {
                    let output = <i32 as WithDeserializer>::init(helper, event)?;
                    return self.handle_bit(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"pad" {
                    let output = <super::PadType as WithDeserializer>::init(helper, event)?;
                    return self.handle_pad(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"field" {
                    let output = <super::VarType as WithDeserializer>::init(helper, event)?;
                    return self.handle_field(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"list" {
                    let output = <super::ListType as WithDeserializer>::init(helper, event)?;
                    return self.handle_list(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"fd" {
                    let output = <super::AnyType as WithDeserializer>::init(helper, event)?;
                    return self.handle_fd(helper, Default::default(), None, output);
                }
                if x.name().local_name().as_ref() == b"switch" {
                    let output = <super::SwitchexprType as WithDeserializer>::init(helper, event)?;
                    return self.handle_switch(helper, Default::default(), None, output);
                }
            }
            *self.state__ = CaseexprTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: CaseexprTypeContentDeserializerState,
        ) -> Result<super::CaseexprTypeContent, Error> {
            use CaseexprTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Op(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_op(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Op(
                        helper.finish_element("op", values)?,
                    ))
                }
                S::Unop(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_unop(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Unop(
                        helper.finish_element("unop", values)?,
                    ))
                }
                S::Fieldref(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_fieldref(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Fieldref(
                        helper.finish_element("fieldref", values)?,
                    ))
                }
                S::Enumref(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_enumref(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Enumref(
                        helper.finish_element("enumref", values)?,
                    ))
                }
                S::Popcount(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_popcount(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Popcount(
                        helper.finish_element("popcount", values)?,
                    ))
                }
                S::Sumof(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_sumof(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Sumof(
                        helper.finish_element("sumof", values)?,
                    ))
                }
                S::Value(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_value(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Value(
                        helper.finish_element("value", values)?,
                    ))
                }
                S::Bit(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_bit(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Bit(
                        helper.finish_element("bit", values)?,
                    ))
                }
                S::Pad(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_pad(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Pad(
                        helper.finish_element("pad", values)?,
                    ))
                }
                S::Field(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_field(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Field(
                        helper.finish_element("field", values)?,
                    ))
                }
                S::List(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_list(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::List(
                        helper.finish_element("list", values)?,
                    ))
                }
                S::Fd(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_fd(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Fd(
                        helper.finish_element("fd", values)?,
                    ))
                }
                S::Switch(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_switch(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Switch(
                        helper.finish_element("switch", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_op(values: &mut Option<super::OpType>, value: super::OpType) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"op")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_unop(
            values: &mut Option<super::UnopType>,
            value: super::UnopType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"unop")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_fieldref(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"fieldref",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_enumref(
            values: &mut Option<super::EnumrefType>,
            value: super::EnumrefType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"enumref",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_popcount(
            values: &mut Option<super::PopcountType>,
            value: super::PopcountType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"popcount",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_sumof(
            values: &mut Option<super::SumofType>,
            value: super::SumofType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"sumof",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_value(
            values: &mut Option<super::DecOrHexIntegerType>,
            value: super::DecOrHexIntegerType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"value",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_bit(values: &mut Option<i32>, value: i32) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"bit")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_pad(
            values: &mut Option<super::PadType>,
            value: super::PadType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"pad")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_field(
            values: &mut Option<super::VarType>,
            value: super::VarType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"field",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_list(
            values: &mut Option<super::ListType>,
            value: super::ListType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"list")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_fd(
            values: &mut Option<super::AnyType>,
            value: super::AnyType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"fd")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_switch(
            values: &mut Option<super::SwitchexprType>,
            value: super::SwitchexprType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"switch",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_op<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::OpType>,
            fallback: Option<<super::OpType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::OpType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CaseexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_op(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_op(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Op(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Op(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_unop<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::UnopType>,
            fallback: Option<<super::UnopType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::UnopType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CaseexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_unop(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_unop(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Unop(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Unop(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_fieldref<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CaseexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_fieldref(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fieldref(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Fieldref(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Fieldref(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_enumref<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::EnumrefType>,
            fallback: Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::EnumrefType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CaseexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_enumref(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_enumref(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Enumref(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Enumref(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_popcount<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PopcountType>,
            fallback: Option<<super::PopcountType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PopcountType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CaseexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_popcount(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_popcount(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Popcount(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Popcount(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_sumof<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SumofType>,
            fallback: Option<<super::SumofType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SumofType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CaseexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_sumof(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sumof(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Sumof(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Sumof(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_value<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::DecOrHexIntegerType>,
            fallback: Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::DecOrHexIntegerType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CaseexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_value(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_value(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Value(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Value(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_bit<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<i32>,
            fallback: Option<<i32 as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, i32>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CaseexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_bit(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bit(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Bit(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Bit(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_pad<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PadType>,
            fallback: Option<<super::PadType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PadType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CaseexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_pad(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pad(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Pad(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Pad(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_field<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::VarType>,
            fallback: Option<<super::VarType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::VarType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CaseexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_field(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_field(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Field(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Field(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_list<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ListType>,
            fallback: Option<<super::ListType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ListType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CaseexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_list(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_list(&mut values, data)?;
                    let data = Self::finish_state(helper, S::List(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::List(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_fd<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnyType>,
            fallback: Option<<super::AnyType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnyType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CaseexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_fd(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fd(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Fd(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Fd(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_switch<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SwitchexprType>,
            fallback: Option<<super::SwitchexprType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SwitchexprType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use CaseexprTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_switch(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_switch(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Switch(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Switch(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::CaseexprTypeContent> for CaseexprTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CaseexprTypeContent> {
            let deserializer = Self {
                state__: Box::new(CaseexprTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, CaseexprTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CaseexprTypeContent> {
            use CaseexprTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Op(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_op(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_unop(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fieldref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_enumref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_popcount(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_sumof(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_value(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_bit(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Pad(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_pad(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_field(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_list(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_fd(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Switch(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_switch(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Op(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"op", false)?;
                        match self.handle_op(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Unop(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"unop", false)?;
                        match self.handle_unop(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Fieldref(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"fieldref", false)?;
                        match self.handle_fieldref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Enumref(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"enumref", false)?;
                        match self.handle_enumref(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Popcount(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"popcount", false)?;
                        match self.handle_popcount(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Sumof(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"sumof", false)?;
                        match self.handle_sumof(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Value(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"value", false)?;
                        match self.handle_value(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Bit(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"bit", false)?;
                        match self.handle_bit(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Pad(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"pad", false)?;
                        match self.handle_pad(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Field(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"field", false)?;
                        match self.handle_field(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::List(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"list", false)?;
                        match self.handle_list(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Fd(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"fd", true)?;
                        match self.handle_fd(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Switch(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            helper.init_start_tag_deserializer(event, None, b"switch", true)?;
                        match self.handle_switch(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::CaseexprTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct FieldTypeDeserializer {
        name: Option<String>,
        content: Option<String>,
        state__: Box<FieldTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FieldTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl FieldTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut name: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                name: name,
                content: None,
                state__: Box::new(FieldTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: FieldTypeDeserializerState,
        ) -> Result<(), Error> {
            if let FieldTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::FieldType> {
            use FieldTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::FieldType> for FieldTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FieldType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FieldType> {
            use FieldTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::FieldType, Error> {
            let state = replace(&mut *self.state__, FieldTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::FieldType {
                name: self.name,
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ErrorTypeDeserializer {
        type_: Option<String>,
        content: Option<String>,
        state__: Box<ErrorTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ErrorTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ErrorTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut type_: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"type" {
                    helper.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                type_: type_,
                content: None,
                state__: Box::new(ErrorTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ErrorTypeDeserializerState,
        ) -> Result<(), Error> {
            if let ErrorTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::ErrorType> {
            use ErrorTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ErrorType> for ErrorTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ErrorType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ErrorType> {
            use ErrorTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ErrorType, Error> {
            let state = replace(&mut *self.state__, ErrorTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::ErrorType {
                type_: self.type_,
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct SeeTypeDeserializer {
        name: Option<String>,
        type_: Option<String>,
        state__: Box<SeeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SeeTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl SeeTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut name: Option<String> = None;
            let mut type_: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"type" {
                    helper.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                name: name,
                type_: type_,
                state__: Box::new(SeeTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SeeTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::SeeType> for SeeTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SeeType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SeeType> {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(helper)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                })
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::SeeType, Error> {
            let state = replace(&mut *self.state__, SeeTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::SeeType {
                name: self.name,
                type_: self.type_,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, Error, Event, IterSerializer, SerializeHelper, Serializer,
        WithSerializer,
    };
    #[derive(Debug)]
    pub struct XcbTypeSerializer<'ser> {
        pub(super) value: &'ser super::XcbType,
        pub(super) state: Box<XcbTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum XcbTypeSerializerState<'ser> {
        Init__,
        Content__(IterSerializer<'ser, &'ser [super::XcbTypeContent], super::XcbTypeContent>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> XcbTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    XcbTypeSerializerState::Init__ => {
                        *self.state = XcbTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib(&mut bytes, "header", &self.value.header)?;
                        helper.write_attrib_opt(
                            &mut bytes,
                            "extension-xname",
                            &self.value.extension_xname,
                        )?;
                        helper.write_attrib_opt(
                            &mut bytes,
                            "extension-name",
                            &self.value.extension_name,
                        )?;
                        helper.write_attrib(
                            &mut bytes,
                            "extension-multiword",
                            &self.value.extension_multiword,
                        )?;
                        helper.write_attrib_opt(
                            &mut bytes,
                            "major-version",
                            &self.value.major_version,
                        )?;
                        helper.write_attrib_opt(
                            &mut bytes,
                            "minor-version",
                            &self.value.minor_version,
                        )?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    XcbTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = XcbTypeSerializerState::End__,
                    },
                    XcbTypeSerializerState::End__ => {
                        *self.state = XcbTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    XcbTypeSerializerState::Done__ => return Ok(None),
                    XcbTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for XcbTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = XcbTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct XcbTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::XcbTypeContent,
        pub(super) state: Box<XcbTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum XcbTypeContentSerializerState<'ser> {
        Init__,
        Request(<super::RequestType as WithSerializer>::Serializer<'ser>),
        Event(<super::EventType as WithSerializer>::Serializer<'ser>),
        Eventcopy(<super::PacketStructCopyType as WithSerializer>::Serializer<'ser>),
        Error(<super::PacketStructType as WithSerializer>::Serializer<'ser>),
        Errorcopy(<super::PacketStructCopyType as WithSerializer>::Serializer<'ser>),
        Struct(<super::StructType as WithSerializer>::Serializer<'ser>),
        Union(<super::StructType as WithSerializer>::Serializer<'ser>),
        Xidtype(<super::XidtypeType as WithSerializer>::Serializer<'ser>),
        Xidunion(<super::XidunionType as WithSerializer>::Serializer<'ser>),
        Enum(<super::EnumType as WithSerializer>::Serializer<'ser>),
        Typedef(<super::TypedefType as WithSerializer>::Serializer<'ser>),
        Import(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> XcbTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    XcbTypeContentSerializerState::Init__ => match self.value {
                        super::XcbTypeContent::Request(x) => {
                            *self.state = XcbTypeContentSerializerState::Request(
                                WithSerializer::serializer(x, Some("request"), false)?,
                            )
                        }
                        super::XcbTypeContent::Event(x) => {
                            *self.state = XcbTypeContentSerializerState::Event(
                                WithSerializer::serializer(x, Some("event"), false)?,
                            )
                        }
                        super::XcbTypeContent::Eventcopy(x) => {
                            *self.state = XcbTypeContentSerializerState::Eventcopy(
                                WithSerializer::serializer(x, Some("eventcopy"), false)?,
                            )
                        }
                        super::XcbTypeContent::Error(x) => {
                            *self.state = XcbTypeContentSerializerState::Error(
                                WithSerializer::serializer(x, Some("error"), false)?,
                            )
                        }
                        super::XcbTypeContent::Errorcopy(x) => {
                            *self.state = XcbTypeContentSerializerState::Errorcopy(
                                WithSerializer::serializer(x, Some("errorcopy"), false)?,
                            )
                        }
                        super::XcbTypeContent::Struct(x) => {
                            *self.state = XcbTypeContentSerializerState::Struct(
                                WithSerializer::serializer(x, Some("struct"), false)?,
                            )
                        }
                        super::XcbTypeContent::Union(x) => {
                            *self.state = XcbTypeContentSerializerState::Union(
                                WithSerializer::serializer(x, Some("union"), false)?,
                            )
                        }
                        super::XcbTypeContent::Xidtype(x) => {
                            *self.state = XcbTypeContentSerializerState::Xidtype(
                                WithSerializer::serializer(x, Some("xidtype"), false)?,
                            )
                        }
                        super::XcbTypeContent::Xidunion(x) => {
                            *self.state = XcbTypeContentSerializerState::Xidunion(
                                WithSerializer::serializer(x, Some("xidunion"), false)?,
                            )
                        }
                        super::XcbTypeContent::Enum(x) => {
                            *self.state = XcbTypeContentSerializerState::Enum(
                                WithSerializer::serializer(x, Some("enum"), false)?,
                            )
                        }
                        super::XcbTypeContent::Typedef(x) => {
                            *self.state = XcbTypeContentSerializerState::Typedef(
                                WithSerializer::serializer(x, Some("typedef"), false)?,
                            )
                        }
                        super::XcbTypeContent::Import(x) => {
                            *self.state = XcbTypeContentSerializerState::Import(
                                WithSerializer::serializer(x, Some("import"), false)?,
                            )
                        }
                    },
                    XcbTypeContentSerializerState::Request(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = XcbTypeContentSerializerState::Done__,
                        }
                    }
                    XcbTypeContentSerializerState::Event(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = XcbTypeContentSerializerState::Done__,
                    },
                    XcbTypeContentSerializerState::Eventcopy(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = XcbTypeContentSerializerState::Done__,
                        }
                    }
                    XcbTypeContentSerializerState::Error(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = XcbTypeContentSerializerState::Done__,
                    },
                    XcbTypeContentSerializerState::Errorcopy(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = XcbTypeContentSerializerState::Done__,
                        }
                    }
                    XcbTypeContentSerializerState::Struct(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = XcbTypeContentSerializerState::Done__,
                        }
                    }
                    XcbTypeContentSerializerState::Union(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = XcbTypeContentSerializerState::Done__,
                    },
                    XcbTypeContentSerializerState::Xidtype(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = XcbTypeContentSerializerState::Done__,
                        }
                    }
                    XcbTypeContentSerializerState::Xidunion(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = XcbTypeContentSerializerState::Done__,
                        }
                    }
                    XcbTypeContentSerializerState::Enum(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = XcbTypeContentSerializerState::Done__,
                    },
                    XcbTypeContentSerializerState::Typedef(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = XcbTypeContentSerializerState::Done__,
                        }
                    }
                    XcbTypeContentSerializerState::Import(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = XcbTypeContentSerializerState::Done__,
                        }
                    }
                    XcbTypeContentSerializerState::Done__ => return Ok(None),
                    XcbTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for XcbTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = XcbTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct RequestTypeSerializer<'ser> {
        pub(super) value: &'ser super::RequestType,
        pub(super) state: Box<RequestTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum RequestTypeSerializerState<'ser> {
        Init__,
        Content__(
            IterSerializer<'ser, &'ser [super::RequestTypeContent], super::RequestTypeContent>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RequestTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    RequestTypeSerializerState::Init__ => {
                        *self.state = RequestTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        helper.write_attrib(&mut bytes, "opcode", &self.value.opcode)?;
                        helper.write_attrib_opt(
                            &mut bytes,
                            "combine-adjacent",
                            &self.value.combine_adjacent,
                        )?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    RequestTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = RequestTypeSerializerState::End__,
                    },
                    RequestTypeSerializerState::End__ => {
                        *self.state = RequestTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    RequestTypeSerializerState::Done__ => return Ok(None),
                    RequestTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for RequestTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = RequestTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct RequestTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::RequestTypeContent,
        pub(super) state: Box<RequestTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum RequestTypeContentSerializerState<'ser> {
        Init__,
        Pad(<super::PadType as WithSerializer>::Serializer<'ser>),
        Field(<super::VarType as WithSerializer>::Serializer<'ser>),
        List(<super::ListType as WithSerializer>::Serializer<'ser>),
        Fd(<super::AnyType as WithSerializer>::Serializer<'ser>),
        Exprfield(<super::ExprfieldType as WithSerializer>::Serializer<'ser>),
        Valueparam(<super::ValueparamType as WithSerializer>::Serializer<'ser>),
        Switch(<super::SwitchexprType as WithSerializer>::Serializer<'ser>),
        Reply(<super::RequestReplyType as WithSerializer>::Serializer<'ser>),
        Doc(<super::DocType as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RequestTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    RequestTypeContentSerializerState::Init__ => match self.value {
                        super::RequestTypeContent::Pad(x) => {
                            *self.state = RequestTypeContentSerializerState::Pad(
                                WithSerializer::serializer(x, Some("pad"), false)?,
                            )
                        }
                        super::RequestTypeContent::Field(x) => {
                            *self.state = RequestTypeContentSerializerState::Field(
                                WithSerializer::serializer(x, Some("field"), false)?,
                            )
                        }
                        super::RequestTypeContent::List(x) => {
                            *self.state = RequestTypeContentSerializerState::List(
                                WithSerializer::serializer(x, Some("list"), false)?,
                            )
                        }
                        super::RequestTypeContent::Fd(x) => {
                            *self.state = RequestTypeContentSerializerState::Fd(
                                WithSerializer::serializer(x, Some("fd"), false)?,
                            )
                        }
                        super::RequestTypeContent::Exprfield(x) => {
                            *self.state = RequestTypeContentSerializerState::Exprfield(
                                WithSerializer::serializer(x, Some("exprfield"), false)?,
                            )
                        }
                        super::RequestTypeContent::Valueparam(x) => {
                            *self.state = RequestTypeContentSerializerState::Valueparam(
                                WithSerializer::serializer(x, Some("valueparam"), false)?,
                            )
                        }
                        super::RequestTypeContent::Switch(x) => {
                            *self.state = RequestTypeContentSerializerState::Switch(
                                WithSerializer::serializer(x, Some("switch"), false)?,
                            )
                        }
                        super::RequestTypeContent::Reply(x) => {
                            *self.state = RequestTypeContentSerializerState::Reply(
                                WithSerializer::serializer(x, Some("reply"), false)?,
                            )
                        }
                        super::RequestTypeContent::Doc(x) => {
                            *self.state = RequestTypeContentSerializerState::Doc(
                                WithSerializer::serializer(x, Some("doc"), false)?,
                            )
                        }
                    },
                    RequestTypeContentSerializerState::Pad(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestTypeContentSerializerState::Done__,
                        }
                    }
                    RequestTypeContentSerializerState::Field(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestTypeContentSerializerState::Done__,
                        }
                    }
                    RequestTypeContentSerializerState::List(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestTypeContentSerializerState::Done__,
                        }
                    }
                    RequestTypeContentSerializerState::Fd(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestTypeContentSerializerState::Done__,
                        }
                    }
                    RequestTypeContentSerializerState::Exprfield(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestTypeContentSerializerState::Done__,
                        }
                    }
                    RequestTypeContentSerializerState::Valueparam(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestTypeContentSerializerState::Done__,
                        }
                    }
                    RequestTypeContentSerializerState::Switch(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestTypeContentSerializerState::Done__,
                        }
                    }
                    RequestTypeContentSerializerState::Reply(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestTypeContentSerializerState::Done__,
                        }
                    }
                    RequestTypeContentSerializerState::Doc(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestTypeContentSerializerState::Done__,
                        }
                    }
                    RequestTypeContentSerializerState::Done__ => return Ok(None),
                    RequestTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for RequestTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = RequestTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct EventTypeSerializer<'ser> {
        pub(super) value: &'ser super::EventType,
        pub(super) state: Box<EventTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum EventTypeSerializerState<'ser> {
        Init__,
        Content__(IterSerializer<'ser, &'ser [super::EventTypeContent], super::EventTypeContent>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> EventTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EventTypeSerializerState::Init__ => {
                        *self.state = EventTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        helper.write_attrib(&mut bytes, "number", &self.value.number)?;
                        helper.write_attrib_opt(
                            &mut bytes,
                            "no-sequence-number",
                            &self.value.no_sequence_number,
                        )?;
                        helper.write_attrib_opt(&mut bytes, "xge", &self.value.xge)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    EventTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = EventTypeSerializerState::End__,
                    },
                    EventTypeSerializerState::End__ => {
                        *self.state = EventTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    EventTypeSerializerState::Done__ => return Ok(None),
                    EventTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for EventTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = EventTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct EventTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::EventTypeContent,
        pub(super) state: Box<EventTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum EventTypeContentSerializerState<'ser> {
        Init__,
        Pad(<super::PadType as WithSerializer>::Serializer<'ser>),
        Field(<super::VarType as WithSerializer>::Serializer<'ser>),
        List(<super::ListType as WithSerializer>::Serializer<'ser>),
        Fd(<super::AnyType as WithSerializer>::Serializer<'ser>),
        Doc(<super::DocType as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> EventTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EventTypeContentSerializerState::Init__ => match self.value {
                        super::EventTypeContent::Pad(x) => {
                            *self.state = EventTypeContentSerializerState::Pad(
                                WithSerializer::serializer(x, Some("pad"), false)?,
                            )
                        }
                        super::EventTypeContent::Field(x) => {
                            *self.state = EventTypeContentSerializerState::Field(
                                WithSerializer::serializer(x, Some("field"), false)?,
                            )
                        }
                        super::EventTypeContent::List(x) => {
                            *self.state = EventTypeContentSerializerState::List(
                                WithSerializer::serializer(x, Some("list"), false)?,
                            )
                        }
                        super::EventTypeContent::Fd(x) => {
                            *self.state = EventTypeContentSerializerState::Fd(
                                WithSerializer::serializer(x, Some("fd"), false)?,
                            )
                        }
                        super::EventTypeContent::Doc(x) => {
                            *self.state = EventTypeContentSerializerState::Doc(
                                WithSerializer::serializer(x, Some("doc"), false)?,
                            )
                        }
                    },
                    EventTypeContentSerializerState::Pad(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = EventTypeContentSerializerState::Done__,
                    },
                    EventTypeContentSerializerState::Field(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = EventTypeContentSerializerState::Done__,
                        }
                    }
                    EventTypeContentSerializerState::List(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = EventTypeContentSerializerState::Done__,
                        }
                    }
                    EventTypeContentSerializerState::Fd(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = EventTypeContentSerializerState::Done__,
                    },
                    EventTypeContentSerializerState::Doc(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = EventTypeContentSerializerState::Done__,
                    },
                    EventTypeContentSerializerState::Done__ => return Ok(None),
                    EventTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for EventTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = EventTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PacketStructCopyTypeSerializer<'ser> {
        pub(super) value: &'ser super::PacketStructCopyType,
        pub(super) state: Box<PacketStructCopyTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum PacketStructCopyTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PacketStructCopyTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PacketStructCopyTypeSerializerState::Init__ => {
                        *self.state = PacketStructCopyTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        helper.write_attrib(&mut bytes, "number", &self.value.number)?;
                        helper.write_attrib(&mut bytes, "ref", &self.value.ref_)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    PacketStructCopyTypeSerializerState::Done__ => return Ok(None),
                    PacketStructCopyTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for PacketStructCopyTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PacketStructCopyTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PacketStructTypeSerializer<'ser> {
        pub(super) value: &'ser super::PacketStructType,
        pub(super) state: Box<PacketStructTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum PacketStructTypeSerializerState<'ser> {
        Init__,
        Content__(
            IterSerializer<
                'ser,
                &'ser [super::PacketStructTypeContent],
                super::PacketStructTypeContent,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PacketStructTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PacketStructTypeSerializerState::Init__ => {
                        *self.state = PacketStructTypeSerializerState::Content__(
                            IterSerializer::new(&self.value.content[..], None, false),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        helper.write_attrib(&mut bytes, "number", &self.value.number)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    PacketStructTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PacketStructTypeSerializerState::End__,
                        }
                    }
                    PacketStructTypeSerializerState::End__ => {
                        *self.state = PacketStructTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    PacketStructTypeSerializerState::Done__ => return Ok(None),
                    PacketStructTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for PacketStructTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PacketStructTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PacketStructTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::PacketStructTypeContent,
        pub(super) state: Box<PacketStructTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum PacketStructTypeContentSerializerState<'ser> {
        Init__,
        Pad(<super::PadType as WithSerializer>::Serializer<'ser>),
        Field(<super::VarType as WithSerializer>::Serializer<'ser>),
        List(<super::ListType as WithSerializer>::Serializer<'ser>),
        Fd(<super::AnyType as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PacketStructTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PacketStructTypeContentSerializerState::Init__ => match self.value {
                        super::PacketStructTypeContent::Pad(x) => {
                            *self.state = PacketStructTypeContentSerializerState::Pad(
                                WithSerializer::serializer(x, Some("pad"), false)?,
                            )
                        }
                        super::PacketStructTypeContent::Field(x) => {
                            *self.state = PacketStructTypeContentSerializerState::Field(
                                WithSerializer::serializer(x, Some("field"), false)?,
                            )
                        }
                        super::PacketStructTypeContent::List(x) => {
                            *self.state = PacketStructTypeContentSerializerState::List(
                                WithSerializer::serializer(x, Some("list"), false)?,
                            )
                        }
                        super::PacketStructTypeContent::Fd(x) => {
                            *self.state = PacketStructTypeContentSerializerState::Fd(
                                WithSerializer::serializer(x, Some("fd"), false)?,
                            )
                        }
                    },
                    PacketStructTypeContentSerializerState::Pad(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PacketStructTypeContentSerializerState::Done__,
                        }
                    }
                    PacketStructTypeContentSerializerState::Field(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PacketStructTypeContentSerializerState::Done__,
                        }
                    }
                    PacketStructTypeContentSerializerState::List(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PacketStructTypeContentSerializerState::Done__,
                        }
                    }
                    PacketStructTypeContentSerializerState::Fd(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PacketStructTypeContentSerializerState::Done__,
                        }
                    }
                    PacketStructTypeContentSerializerState::Done__ => return Ok(None),
                    PacketStructTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for PacketStructTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PacketStructTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct StructTypeSerializer<'ser> {
        pub(super) value: &'ser super::StructType,
        pub(super) state: Box<StructTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum StructTypeSerializerState<'ser> {
        Init__,
        Content__(IterSerializer<'ser, &'ser [super::StructTypeContent], super::StructTypeContent>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> StructTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    StructTypeSerializerState::Init__ => {
                        *self.state = StructTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    StructTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = StructTypeSerializerState::End__,
                    },
                    StructTypeSerializerState::End__ => {
                        *self.state = StructTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    StructTypeSerializerState::Done__ => return Ok(None),
                    StructTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for StructTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = StructTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct StructTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::StructTypeContent,
        pub(super) state: Box<StructTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum StructTypeContentSerializerState<'ser> {
        Init__,
        Pad(<super::PadType as WithSerializer>::Serializer<'ser>),
        Field(<super::VarType as WithSerializer>::Serializer<'ser>),
        List(<super::ListType as WithSerializer>::Serializer<'ser>),
        Fd(<super::AnyType as WithSerializer>::Serializer<'ser>),
        Switch(<super::SwitchexprType as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> StructTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    StructTypeContentSerializerState::Init__ => match self.value {
                        super::StructTypeContent::Pad(x) => {
                            *self.state = StructTypeContentSerializerState::Pad(
                                WithSerializer::serializer(x, Some("pad"), false)?,
                            )
                        }
                        super::StructTypeContent::Field(x) => {
                            *self.state = StructTypeContentSerializerState::Field(
                                WithSerializer::serializer(x, Some("field"), false)?,
                            )
                        }
                        super::StructTypeContent::List(x) => {
                            *self.state = StructTypeContentSerializerState::List(
                                WithSerializer::serializer(x, Some("list"), false)?,
                            )
                        }
                        super::StructTypeContent::Fd(x) => {
                            *self.state = StructTypeContentSerializerState::Fd(
                                WithSerializer::serializer(x, Some("fd"), false)?,
                            )
                        }
                        super::StructTypeContent::Switch(x) => {
                            *self.state = StructTypeContentSerializerState::Switch(
                                WithSerializer::serializer(x, Some("switch"), false)?,
                            )
                        }
                    },
                    StructTypeContentSerializerState::Pad(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = StructTypeContentSerializerState::Done__,
                        }
                    }
                    StructTypeContentSerializerState::Field(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = StructTypeContentSerializerState::Done__,
                        }
                    }
                    StructTypeContentSerializerState::List(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = StructTypeContentSerializerState::Done__,
                        }
                    }
                    StructTypeContentSerializerState::Fd(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = StructTypeContentSerializerState::Done__,
                    },
                    StructTypeContentSerializerState::Switch(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = StructTypeContentSerializerState::Done__,
                        }
                    }
                    StructTypeContentSerializerState::Done__ => return Ok(None),
                    StructTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for StructTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = StructTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct XidtypeTypeSerializer<'ser> {
        pub(super) value: &'ser super::XidtypeType,
        pub(super) state: Box<XidtypeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum XidtypeTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> XidtypeTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    XidtypeTypeSerializerState::Init__ => {
                        *self.state = XidtypeTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    XidtypeTypeSerializerState::Done__ => return Ok(None),
                    XidtypeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for XidtypeTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = XidtypeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct XidunionTypeSerializer<'ser> {
        pub(super) value: &'ser super::XidunionType,
        pub(super) state: Box<XidunionTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum XidunionTypeSerializerState<'ser> {
        Init__,
        Type(IterSerializer<'ser, &'ser [String], String>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> XidunionTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    XidunionTypeSerializerState::Init__ => {
                        *self.state = XidunionTypeSerializerState::Type(IterSerializer::new(
                            &self.value.type_[..],
                            Some("type"),
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    XidunionTypeSerializerState::Type(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = XidunionTypeSerializerState::End__,
                    },
                    XidunionTypeSerializerState::End__ => {
                        *self.state = XidunionTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    XidunionTypeSerializerState::Done__ => return Ok(None),
                    XidunionTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for XidunionTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = XidunionTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct EnumTypeSerializer<'ser> {
        pub(super) value: &'ser super::EnumType,
        pub(super) state: Box<EnumTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum EnumTypeSerializerState<'ser> {
        Init__,
        Content__(IterSerializer<'ser, &'ser [super::EnumTypeContent], super::EnumTypeContent>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> EnumTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EnumTypeSerializerState::Init__ => {
                        *self.state = EnumTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    EnumTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = EnumTypeSerializerState::End__,
                    },
                    EnumTypeSerializerState::End__ => {
                        *self.state = EnumTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    EnumTypeSerializerState::Done__ => return Ok(None),
                    EnumTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for EnumTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = EnumTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct EnumTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::EnumTypeContent,
        pub(super) state: Box<EnumTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum EnumTypeContentSerializerState<'ser> {
        Init__,
        Item(<super::EnumItemType as WithSerializer>::Serializer<'ser>),
        Doc(IterSerializer<'ser, Option<&'ser super::DocType>, super::DocType>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> EnumTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EnumTypeContentSerializerState::Init__ => {
                        *self.state = EnumTypeContentSerializerState::Item(
                            WithSerializer::serializer(&self.value.item, Some("item"), false)?,
                        );
                    }
                    EnumTypeContentSerializerState::Item(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = EnumTypeContentSerializerState::Doc(IterSerializer::new(
                                self.value.doc.as_ref(),
                                Some("doc"),
                                false,
                            ))
                        }
                    },
                    EnumTypeContentSerializerState::Doc(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = EnumTypeContentSerializerState::Done__,
                    },
                    EnumTypeContentSerializerState::Done__ => return Ok(None),
                    EnumTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for EnumTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = EnumTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TypedefTypeSerializer<'ser> {
        pub(super) value: &'ser super::TypedefType,
        pub(super) state: Box<TypedefTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TypedefTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TypedefTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TypedefTypeSerializerState::Init__ => {
                        *self.state = TypedefTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib(&mut bytes, "oldname", &self.value.oldname)?;
                        helper.write_attrib(&mut bytes, "newname", &self.value.newname)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    TypedefTypeSerializerState::Done__ => return Ok(None),
                    TypedefTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TypedefTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TypedefTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PadTypeSerializer<'ser> {
        pub(super) value: &'ser super::PadType,
        pub(super) state: Box<PadTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum PadTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PadTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PadTypeSerializerState::Init__ => {
                        *self.state = PadTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib_opt(&mut bytes, "bytes", &self.value.bytes)?;
                        helper.write_attrib_opt(&mut bytes, "align", &self.value.align)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    PadTypeSerializerState::Done__ => return Ok(None),
                    PadTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for PadTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PadTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct VarTypeSerializer<'ser> {
        pub(super) value: &'ser super::VarType,
        pub(super) state: Box<VarTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum VarTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> VarTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    VarTypeSerializerState::Init__ => {
                        *self.state = VarTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        helper.write_attrib(&mut bytes, "type", &self.value.type_)?;
                        helper.write_attrib_opt(&mut bytes, "enum", &self.value.enum_)?;
                        helper.write_attrib_opt(&mut bytes, "altenum", &self.value.altenum)?;
                        helper.write_attrib_opt(&mut bytes, "mask", &self.value.mask)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    VarTypeSerializerState::Done__ => return Ok(None),
                    VarTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for VarTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = VarTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ListTypeSerializer<'ser> {
        pub(super) value: &'ser super::ListType,
        pub(super) state: Box<ListTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ListTypeSerializerState<'ser> {
        Init__,
        Content__(
            IterSerializer<'ser, Option<&'ser super::ListTypeContent>, super::ListTypeContent>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ListTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ListTypeSerializerState::Init__ => {
                        *self.state = ListTypeSerializerState::Content__(IterSerializer::new(
                            self.value.content.as_ref(),
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        helper.write_attrib(&mut bytes, "type", &self.value.type_)?;
                        helper.write_attrib_opt(&mut bytes, "enum", &self.value.enum_)?;
                        helper.write_attrib_opt(&mut bytes, "altenum", &self.value.altenum)?;
                        helper.write_attrib_opt(&mut bytes, "mask", &self.value.mask)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ListTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ListTypeSerializerState::End__,
                    },
                    ListTypeSerializerState::End__ => {
                        *self.state = ListTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ListTypeSerializerState::Done__ => return Ok(None),
                    ListTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ListTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ListTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ListTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::ListTypeContent,
        pub(super) state: Box<ListTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum ListTypeContentSerializerState<'ser> {
        Init__,
        Op(<super::OpType as WithSerializer>::Serializer<'ser>),
        Unop(<super::UnopType as WithSerializer>::Serializer<'ser>),
        Fieldref(<String as WithSerializer>::Serializer<'ser>),
        Enumref(<super::EnumrefType as WithSerializer>::Serializer<'ser>),
        Popcount(<super::PopcountType as WithSerializer>::Serializer<'ser>),
        Sumof(<super::SumofType as WithSerializer>::Serializer<'ser>),
        Value(<super::DecOrHexIntegerType as WithSerializer>::Serializer<'ser>),
        Bit(<i32 as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ListTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ListTypeContentSerializerState::Init__ => match self.value {
                        super::ListTypeContent::Op(x) => {
                            *self.state = ListTypeContentSerializerState::Op(
                                WithSerializer::serializer(x, Some("op"), false)?,
                            )
                        }
                        super::ListTypeContent::Unop(x) => {
                            *self.state = ListTypeContentSerializerState::Unop(
                                WithSerializer::serializer(x, Some("unop"), false)?,
                            )
                        }
                        super::ListTypeContent::Fieldref(x) => {
                            *self.state = ListTypeContentSerializerState::Fieldref(
                                WithSerializer::serializer(x, Some("fieldref"), false)?,
                            )
                        }
                        super::ListTypeContent::Enumref(x) => {
                            *self.state = ListTypeContentSerializerState::Enumref(
                                WithSerializer::serializer(x, Some("enumref"), false)?,
                            )
                        }
                        super::ListTypeContent::Popcount(x) => {
                            *self.state = ListTypeContentSerializerState::Popcount(
                                WithSerializer::serializer(x, Some("popcount"), false)?,
                            )
                        }
                        super::ListTypeContent::Sumof(x) => {
                            *self.state = ListTypeContentSerializerState::Sumof(
                                WithSerializer::serializer(x, Some("sumof"), false)?,
                            )
                        }
                        super::ListTypeContent::Value(x) => {
                            *self.state = ListTypeContentSerializerState::Value(
                                WithSerializer::serializer(x, Some("value"), false)?,
                            )
                        }
                        super::ListTypeContent::Bit(x) => {
                            *self.state = ListTypeContentSerializerState::Bit(
                                WithSerializer::serializer(x, Some("bit"), false)?,
                            )
                        }
                    },
                    ListTypeContentSerializerState::Op(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ListTypeContentSerializerState::Done__,
                    },
                    ListTypeContentSerializerState::Unop(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ListTypeContentSerializerState::Done__,
                    },
                    ListTypeContentSerializerState::Fieldref(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ListTypeContentSerializerState::Done__,
                        }
                    }
                    ListTypeContentSerializerState::Enumref(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ListTypeContentSerializerState::Done__,
                        }
                    }
                    ListTypeContentSerializerState::Popcount(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ListTypeContentSerializerState::Done__,
                        }
                    }
                    ListTypeContentSerializerState::Sumof(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ListTypeContentSerializerState::Done__,
                        }
                    }
                    ListTypeContentSerializerState::Value(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ListTypeContentSerializerState::Done__,
                        }
                    }
                    ListTypeContentSerializerState::Bit(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ListTypeContentSerializerState::Done__,
                    },
                    ListTypeContentSerializerState::Done__ => return Ok(None),
                    ListTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ListTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ListTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct AnyTypeSerializer<'ser> {
        pub(super) value: &'ser super::AnyType,
        pub(super) state: Box<AnyTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum AnyTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> AnyTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    AnyTypeSerializerState::Init__ => {
                        *self.state = AnyTypeSerializerState::Done__;
                        let bytes = BytesStart::new(self.name);
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    AnyTypeSerializerState::Done__ => return Ok(None),
                    AnyTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for AnyTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = AnyTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ExprfieldTypeSerializer<'ser> {
        pub(super) value: &'ser super::ExprfieldType,
        pub(super) state: Box<ExprfieldTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ExprfieldTypeSerializerState<'ser> {
        Init__,
        Content__(<super::ExprfieldTypeContent as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ExprfieldTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ExprfieldTypeSerializerState::Init__ => {
                        *self.state = ExprfieldTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        helper.write_attrib(&mut bytes, "type", &self.value.type_)?;
                        helper.write_attrib_opt(&mut bytes, "enum", &self.value.enum_)?;
                        helper.write_attrib_opt(&mut bytes, "altenum", &self.value.altenum)?;
                        helper.write_attrib_opt(&mut bytes, "mask", &self.value.mask)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ExprfieldTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ExprfieldTypeSerializerState::End__,
                        }
                    }
                    ExprfieldTypeSerializerState::End__ => {
                        *self.state = ExprfieldTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ExprfieldTypeSerializerState::Done__ => return Ok(None),
                    ExprfieldTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ExprfieldTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ExprfieldTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ExprfieldTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::ExprfieldTypeContent,
        pub(super) state: Box<ExprfieldTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum ExprfieldTypeContentSerializerState<'ser> {
        Init__,
        Op(<super::OpType as WithSerializer>::Serializer<'ser>),
        Unop(<super::UnopType as WithSerializer>::Serializer<'ser>),
        Fieldref(<String as WithSerializer>::Serializer<'ser>),
        Enumref(<super::EnumrefType as WithSerializer>::Serializer<'ser>),
        Popcount(<super::PopcountType as WithSerializer>::Serializer<'ser>),
        Sumof(<super::SumofType as WithSerializer>::Serializer<'ser>),
        Value(<super::DecOrHexIntegerType as WithSerializer>::Serializer<'ser>),
        Bit(<i32 as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ExprfieldTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ExprfieldTypeContentSerializerState::Init__ => match self.value {
                        super::ExprfieldTypeContent::Op(x) => {
                            *self.state = ExprfieldTypeContentSerializerState::Op(
                                WithSerializer::serializer(x, Some("op"), false)?,
                            )
                        }
                        super::ExprfieldTypeContent::Unop(x) => {
                            *self.state = ExprfieldTypeContentSerializerState::Unop(
                                WithSerializer::serializer(x, Some("unop"), false)?,
                            )
                        }
                        super::ExprfieldTypeContent::Fieldref(x) => {
                            *self.state = ExprfieldTypeContentSerializerState::Fieldref(
                                WithSerializer::serializer(x, Some("fieldref"), false)?,
                            )
                        }
                        super::ExprfieldTypeContent::Enumref(x) => {
                            *self.state = ExprfieldTypeContentSerializerState::Enumref(
                                WithSerializer::serializer(x, Some("enumref"), false)?,
                            )
                        }
                        super::ExprfieldTypeContent::Popcount(x) => {
                            *self.state = ExprfieldTypeContentSerializerState::Popcount(
                                WithSerializer::serializer(x, Some("popcount"), false)?,
                            )
                        }
                        super::ExprfieldTypeContent::Sumof(x) => {
                            *self.state = ExprfieldTypeContentSerializerState::Sumof(
                                WithSerializer::serializer(x, Some("sumof"), false)?,
                            )
                        }
                        super::ExprfieldTypeContent::Value(x) => {
                            *self.state = ExprfieldTypeContentSerializerState::Value(
                                WithSerializer::serializer(x, Some("value"), false)?,
                            )
                        }
                        super::ExprfieldTypeContent::Bit(x) => {
                            *self.state = ExprfieldTypeContentSerializerState::Bit(
                                WithSerializer::serializer(x, Some("bit"), false)?,
                            )
                        }
                    },
                    ExprfieldTypeContentSerializerState::Op(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ExprfieldTypeContentSerializerState::Done__,
                        }
                    }
                    ExprfieldTypeContentSerializerState::Unop(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ExprfieldTypeContentSerializerState::Done__,
                        }
                    }
                    ExprfieldTypeContentSerializerState::Fieldref(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ExprfieldTypeContentSerializerState::Done__,
                        }
                    }
                    ExprfieldTypeContentSerializerState::Enumref(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ExprfieldTypeContentSerializerState::Done__,
                        }
                    }
                    ExprfieldTypeContentSerializerState::Popcount(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ExprfieldTypeContentSerializerState::Done__,
                        }
                    }
                    ExprfieldTypeContentSerializerState::Sumof(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ExprfieldTypeContentSerializerState::Done__,
                        }
                    }
                    ExprfieldTypeContentSerializerState::Value(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ExprfieldTypeContentSerializerState::Done__,
                        }
                    }
                    ExprfieldTypeContentSerializerState::Bit(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ExprfieldTypeContentSerializerState::Done__,
                        }
                    }
                    ExprfieldTypeContentSerializerState::Done__ => return Ok(None),
                    ExprfieldTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ExprfieldTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ExprfieldTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ValueparamTypeSerializer<'ser> {
        pub(super) value: &'ser super::ValueparamType,
        pub(super) state: Box<ValueparamTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ValueparamTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ValueparamTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ValueparamTypeSerializerState::Init__ => {
                        *self.state = ValueparamTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib(
                            &mut bytes,
                            "value-mask-type",
                            &self.value.value_mask_type,
                        )?;
                        helper.write_attrib(
                            &mut bytes,
                            "value-mask-name",
                            &self.value.value_mask_name,
                        )?;
                        helper.write_attrib(
                            &mut bytes,
                            "value-list-name",
                            &self.value.value_list_name,
                        )?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    ValueparamTypeSerializerState::Done__ => return Ok(None),
                    ValueparamTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ValueparamTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ValueparamTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SwitchexprTypeSerializer<'ser> {
        pub(super) value: &'ser super::SwitchexprType,
        pub(super) state: Box<SwitchexprTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SwitchexprTypeSerializerState<'ser> {
        Init__,
        Content__(
            IterSerializer<
                'ser,
                &'ser [super::SwitchexprTypeContent],
                super::SwitchexprTypeContent,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SwitchexprTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SwitchexprTypeSerializerState::Init__ => {
                        *self.state = SwitchexprTypeSerializerState::Content__(
                            IterSerializer::new(&self.value.content[..], None, false),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SwitchexprTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeSerializerState::End__,
                        }
                    }
                    SwitchexprTypeSerializerState::End__ => {
                        *self.state = SwitchexprTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SwitchexprTypeSerializerState::Done__ => return Ok(None),
                    SwitchexprTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for SwitchexprTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SwitchexprTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SwitchexprTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::SwitchexprTypeContent,
        pub(super) state: Box<SwitchexprTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum SwitchexprTypeContentSerializerState<'ser> {
        Init__,
        Op(<super::OpType as WithSerializer>::Serializer<'ser>),
        Unop(<super::UnopType as WithSerializer>::Serializer<'ser>),
        Fieldref(<String as WithSerializer>::Serializer<'ser>),
        Enumref(<super::EnumrefType as WithSerializer>::Serializer<'ser>),
        Popcount(<super::PopcountType as WithSerializer>::Serializer<'ser>),
        Sumof(<super::SumofType as WithSerializer>::Serializer<'ser>),
        Value(<super::DecOrHexIntegerType as WithSerializer>::Serializer<'ser>),
        Bit(<i32 as WithSerializer>::Serializer<'ser>),
        Bitcase(<super::CaseexprType as WithSerializer>::Serializer<'ser>),
        Pad(<super::PadType as WithSerializer>::Serializer<'ser>),
        Field(<super::VarType as WithSerializer>::Serializer<'ser>),
        List(<super::ListType as WithSerializer>::Serializer<'ser>),
        Fd(<super::AnyType as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SwitchexprTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SwitchexprTypeContentSerializerState::Init__ => match self.value {
                        super::SwitchexprTypeContent::Op(x) => {
                            *self.state = SwitchexprTypeContentSerializerState::Op(
                                WithSerializer::serializer(x, Some("op"), false)?,
                            )
                        }
                        super::SwitchexprTypeContent::Unop(x) => {
                            *self.state = SwitchexprTypeContentSerializerState::Unop(
                                WithSerializer::serializer(x, Some("unop"), false)?,
                            )
                        }
                        super::SwitchexprTypeContent::Fieldref(x) => {
                            *self.state = SwitchexprTypeContentSerializerState::Fieldref(
                                WithSerializer::serializer(x, Some("fieldref"), false)?,
                            )
                        }
                        super::SwitchexprTypeContent::Enumref(x) => {
                            *self.state = SwitchexprTypeContentSerializerState::Enumref(
                                WithSerializer::serializer(x, Some("enumref"), false)?,
                            )
                        }
                        super::SwitchexprTypeContent::Popcount(x) => {
                            *self.state = SwitchexprTypeContentSerializerState::Popcount(
                                WithSerializer::serializer(x, Some("popcount"), false)?,
                            )
                        }
                        super::SwitchexprTypeContent::Sumof(x) => {
                            *self.state = SwitchexprTypeContentSerializerState::Sumof(
                                WithSerializer::serializer(x, Some("sumof"), false)?,
                            )
                        }
                        super::SwitchexprTypeContent::Value(x) => {
                            *self.state = SwitchexprTypeContentSerializerState::Value(
                                WithSerializer::serializer(x, Some("value"), false)?,
                            )
                        }
                        super::SwitchexprTypeContent::Bit(x) => {
                            *self.state = SwitchexprTypeContentSerializerState::Bit(
                                WithSerializer::serializer(x, Some("bit"), false)?,
                            )
                        }
                        super::SwitchexprTypeContent::Bitcase(x) => {
                            *self.state = SwitchexprTypeContentSerializerState::Bitcase(
                                WithSerializer::serializer(x, Some("bitcase"), false)?,
                            )
                        }
                        super::SwitchexprTypeContent::Pad(x) => {
                            *self.state = SwitchexprTypeContentSerializerState::Pad(
                                WithSerializer::serializer(x, Some("pad"), false)?,
                            )
                        }
                        super::SwitchexprTypeContent::Field(x) => {
                            *self.state = SwitchexprTypeContentSerializerState::Field(
                                WithSerializer::serializer(x, Some("field"), false)?,
                            )
                        }
                        super::SwitchexprTypeContent::List(x) => {
                            *self.state = SwitchexprTypeContentSerializerState::List(
                                WithSerializer::serializer(x, Some("list"), false)?,
                            )
                        }
                        super::SwitchexprTypeContent::Fd(x) => {
                            *self.state = SwitchexprTypeContentSerializerState::Fd(
                                WithSerializer::serializer(x, Some("fd"), false)?,
                            )
                        }
                    },
                    SwitchexprTypeContentSerializerState::Op(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                        }
                    }
                    SwitchexprTypeContentSerializerState::Unop(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                        }
                    }
                    SwitchexprTypeContentSerializerState::Fieldref(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                        }
                    }
                    SwitchexprTypeContentSerializerState::Enumref(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                        }
                    }
                    SwitchexprTypeContentSerializerState::Popcount(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                        }
                    }
                    SwitchexprTypeContentSerializerState::Sumof(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                        }
                    }
                    SwitchexprTypeContentSerializerState::Value(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                        }
                    }
                    SwitchexprTypeContentSerializerState::Bit(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                        }
                    }
                    SwitchexprTypeContentSerializerState::Bitcase(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                        }
                    }
                    SwitchexprTypeContentSerializerState::Pad(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                        }
                    }
                    SwitchexprTypeContentSerializerState::Field(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                        }
                    }
                    SwitchexprTypeContentSerializerState::List(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                        }
                    }
                    SwitchexprTypeContentSerializerState::Fd(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                        }
                    }
                    SwitchexprTypeContentSerializerState::Done__ => return Ok(None),
                    SwitchexprTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for SwitchexprTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SwitchexprTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct RequestReplyTypeSerializer<'ser> {
        pub(super) value: &'ser super::RequestReplyType,
        pub(super) state: Box<RequestReplyTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum RequestReplyTypeSerializerState<'ser> {
        Init__,
        Content__(
            IterSerializer<
                'ser,
                &'ser [super::RequestReplyTypeContent],
                super::RequestReplyTypeContent,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RequestReplyTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    RequestReplyTypeSerializerState::Init__ => {
                        *self.state = RequestReplyTypeSerializerState::Content__(
                            IterSerializer::new(&self.value.content[..], None, false),
                        );
                        let bytes = BytesStart::new(self.name);
                        return Ok(Some(Event::Start(bytes)));
                    }
                    RequestReplyTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestReplyTypeSerializerState::End__,
                        }
                    }
                    RequestReplyTypeSerializerState::End__ => {
                        *self.state = RequestReplyTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    RequestReplyTypeSerializerState::Done__ => return Ok(None),
                    RequestReplyTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for RequestReplyTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = RequestReplyTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct RequestReplyTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::RequestReplyTypeContent,
        pub(super) state: Box<RequestReplyTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum RequestReplyTypeContentSerializerState<'ser> {
        Init__,
        Pad(<super::PadType as WithSerializer>::Serializer<'ser>),
        Field(<super::VarType as WithSerializer>::Serializer<'ser>),
        List(<super::ListType as WithSerializer>::Serializer<'ser>),
        Fd(<super::AnyType as WithSerializer>::Serializer<'ser>),
        Valueparam(<super::ValueparamType as WithSerializer>::Serializer<'ser>),
        Switch(<super::SwitchexprType as WithSerializer>::Serializer<'ser>),
        Doc(<super::DocType as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RequestReplyTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    RequestReplyTypeContentSerializerState::Init__ => match self.value {
                        super::RequestReplyTypeContent::Pad(x) => {
                            *self.state = RequestReplyTypeContentSerializerState::Pad(
                                WithSerializer::serializer(x, Some("pad"), false)?,
                            )
                        }
                        super::RequestReplyTypeContent::Field(x) => {
                            *self.state = RequestReplyTypeContentSerializerState::Field(
                                WithSerializer::serializer(x, Some("field"), false)?,
                            )
                        }
                        super::RequestReplyTypeContent::List(x) => {
                            *self.state = RequestReplyTypeContentSerializerState::List(
                                WithSerializer::serializer(x, Some("list"), false)?,
                            )
                        }
                        super::RequestReplyTypeContent::Fd(x) => {
                            *self.state = RequestReplyTypeContentSerializerState::Fd(
                                WithSerializer::serializer(x, Some("fd"), false)?,
                            )
                        }
                        super::RequestReplyTypeContent::Valueparam(x) => {
                            *self.state = RequestReplyTypeContentSerializerState::Valueparam(
                                WithSerializer::serializer(x, Some("valueparam"), false)?,
                            )
                        }
                        super::RequestReplyTypeContent::Switch(x) => {
                            *self.state = RequestReplyTypeContentSerializerState::Switch(
                                WithSerializer::serializer(x, Some("switch"), false)?,
                            )
                        }
                        super::RequestReplyTypeContent::Doc(x) => {
                            *self.state = RequestReplyTypeContentSerializerState::Doc(
                                WithSerializer::serializer(x, Some("doc"), false)?,
                            )
                        }
                    },
                    RequestReplyTypeContentSerializerState::Pad(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestReplyTypeContentSerializerState::Done__,
                        }
                    }
                    RequestReplyTypeContentSerializerState::Field(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestReplyTypeContentSerializerState::Done__,
                        }
                    }
                    RequestReplyTypeContentSerializerState::List(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestReplyTypeContentSerializerState::Done__,
                        }
                    }
                    RequestReplyTypeContentSerializerState::Fd(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestReplyTypeContentSerializerState::Done__,
                        }
                    }
                    RequestReplyTypeContentSerializerState::Valueparam(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestReplyTypeContentSerializerState::Done__,
                        }
                    }
                    RequestReplyTypeContentSerializerState::Switch(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestReplyTypeContentSerializerState::Done__,
                        }
                    }
                    RequestReplyTypeContentSerializerState::Doc(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestReplyTypeContentSerializerState::Done__,
                        }
                    }
                    RequestReplyTypeContentSerializerState::Done__ => return Ok(None),
                    RequestReplyTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for RequestReplyTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = RequestReplyTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DocTypeSerializer<'ser> {
        pub(super) value: &'ser super::DocType,
        pub(super) state: Box<DocTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DocTypeSerializerState<'ser> {
        Init__,
        Content__(IterSerializer<'ser, &'ser [super::DocTypeContent], super::DocTypeContent>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DocTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DocTypeSerializerState::Init__ => {
                        *self.state = DocTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let bytes = BytesStart::new(self.name);
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DocTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DocTypeSerializerState::End__,
                    },
                    DocTypeSerializerState::End__ => {
                        *self.state = DocTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DocTypeSerializerState::Done__ => return Ok(None),
                    DocTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for DocTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DocTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DocTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::DocTypeContent,
        pub(super) state: Box<DocTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum DocTypeContentSerializerState<'ser> {
        Init__,
        Brief(<String as WithSerializer>::Serializer<'ser>),
        Description(<String as WithSerializer>::Serializer<'ser>),
        Example(<String as WithSerializer>::Serializer<'ser>),
        Field(<super::FieldType as WithSerializer>::Serializer<'ser>),
        Error(<super::ErrorType as WithSerializer>::Serializer<'ser>),
        See(<super::SeeType as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DocTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DocTypeContentSerializerState::Init__ => match self.value {
                        super::DocTypeContent::Brief(x) => {
                            *self.state = DocTypeContentSerializerState::Brief(
                                WithSerializer::serializer(x, Some("brief"), false)?,
                            )
                        }
                        super::DocTypeContent::Description(x) => {
                            *self.state = DocTypeContentSerializerState::Description(
                                WithSerializer::serializer(x, Some("description"), false)?,
                            )
                        }
                        super::DocTypeContent::Example(x) => {
                            *self.state = DocTypeContentSerializerState::Example(
                                WithSerializer::serializer(x, Some("example"), false)?,
                            )
                        }
                        super::DocTypeContent::Field(x) => {
                            *self.state = DocTypeContentSerializerState::Field(
                                WithSerializer::serializer(x, Some("field"), false)?,
                            )
                        }
                        super::DocTypeContent::Error(x) => {
                            *self.state = DocTypeContentSerializerState::Error(
                                WithSerializer::serializer(x, Some("error"), false)?,
                            )
                        }
                        super::DocTypeContent::See(x) => {
                            *self.state = DocTypeContentSerializerState::See(
                                WithSerializer::serializer(x, Some("see"), false)?,
                            )
                        }
                    },
                    DocTypeContentSerializerState::Brief(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DocTypeContentSerializerState::Done__,
                    },
                    DocTypeContentSerializerState::Description(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = DocTypeContentSerializerState::Done__,
                        }
                    }
                    DocTypeContentSerializerState::Example(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = DocTypeContentSerializerState::Done__,
                        }
                    }
                    DocTypeContentSerializerState::Field(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DocTypeContentSerializerState::Done__,
                    },
                    DocTypeContentSerializerState::Error(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DocTypeContentSerializerState::Done__,
                    },
                    DocTypeContentSerializerState::See(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DocTypeContentSerializerState::Done__,
                    },
                    DocTypeContentSerializerState::Done__ => return Ok(None),
                    DocTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for DocTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DocTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct EnumItemTypeSerializer<'ser> {
        pub(super) value: &'ser super::EnumItemType,
        pub(super) state: Box<EnumItemTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum EnumItemTypeSerializerState<'ser> {
        Init__,
        Content__(<super::EnumItemTypeContent as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> EnumItemTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EnumItemTypeSerializerState::Init__ => {
                        *self.state = EnumItemTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    EnumItemTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = EnumItemTypeSerializerState::End__,
                        }
                    }
                    EnumItemTypeSerializerState::End__ => {
                        *self.state = EnumItemTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    EnumItemTypeSerializerState::Done__ => return Ok(None),
                    EnumItemTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for EnumItemTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = EnumItemTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct EnumItemTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::EnumItemTypeContent,
        pub(super) state: Box<EnumItemTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum EnumItemTypeContentSerializerState<'ser> {
        Init__,
        Value(<super::DecOrHexIntegerType as WithSerializer>::Serializer<'ser>),
        Bit(<i32 as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> EnumItemTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EnumItemTypeContentSerializerState::Init__ => match self.value {
                        super::EnumItemTypeContent::Value(x) => {
                            *self.state = EnumItemTypeContentSerializerState::Value(
                                WithSerializer::serializer(x, Some("value"), false)?,
                            )
                        }
                        super::EnumItemTypeContent::Bit(x) => {
                            *self.state = EnumItemTypeContentSerializerState::Bit(
                                WithSerializer::serializer(x, Some("bit"), false)?,
                            )
                        }
                    },
                    EnumItemTypeContentSerializerState::Value(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = EnumItemTypeContentSerializerState::Done__,
                        }
                    }
                    EnumItemTypeContentSerializerState::Bit(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = EnumItemTypeContentSerializerState::Done__,
                        }
                    }
                    EnumItemTypeContentSerializerState::Done__ => return Ok(None),
                    EnumItemTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for EnumItemTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = EnumItemTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct OpTypeSerializer<'ser> {
        pub(super) value: &'ser super::OpType,
        pub(super) state: Box<OpTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum OpTypeSerializerState<'ser> {
        Init__,
        Content__(IterSerializer<'ser, &'ser [super::OpTypeContent], super::OpTypeContent>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> OpTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    OpTypeSerializerState::Init__ => {
                        *self.state = OpTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib(&mut bytes, "op", &self.value.op)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    OpTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = OpTypeSerializerState::End__,
                    },
                    OpTypeSerializerState::End__ => {
                        *self.state = OpTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    OpTypeSerializerState::Done__ => return Ok(None),
                    OpTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for OpTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = OpTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct OpTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::OpTypeContent,
        pub(super) state: Box<OpTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum OpTypeContentSerializerState<'ser> {
        Init__,
        Op(<super::OpType as WithSerializer>::Serializer<'ser>),
        Unop(<super::UnopType as WithSerializer>::Serializer<'ser>),
        Fieldref(<String as WithSerializer>::Serializer<'ser>),
        Enumref(<super::EnumrefType as WithSerializer>::Serializer<'ser>),
        Popcount(<super::PopcountType as WithSerializer>::Serializer<'ser>),
        Sumof(<super::SumofType as WithSerializer>::Serializer<'ser>),
        Value(<super::DecOrHexIntegerType as WithSerializer>::Serializer<'ser>),
        Bit(<i32 as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> OpTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    OpTypeContentSerializerState::Init__ => match self.value {
                        super::OpTypeContent::Op(x) => {
                            *self.state = OpTypeContentSerializerState::Op(
                                WithSerializer::serializer(&**x, Some("op"), false)?,
                            )
                        }
                        super::OpTypeContent::Unop(x) => {
                            *self.state = OpTypeContentSerializerState::Unop(
                                WithSerializer::serializer(x, Some("unop"), false)?,
                            )
                        }
                        super::OpTypeContent::Fieldref(x) => {
                            *self.state = OpTypeContentSerializerState::Fieldref(
                                WithSerializer::serializer(x, Some("fieldref"), false)?,
                            )
                        }
                        super::OpTypeContent::Enumref(x) => {
                            *self.state = OpTypeContentSerializerState::Enumref(
                                WithSerializer::serializer(x, Some("enumref"), false)?,
                            )
                        }
                        super::OpTypeContent::Popcount(x) => {
                            *self.state = OpTypeContentSerializerState::Popcount(
                                WithSerializer::serializer(x, Some("popcount"), false)?,
                            )
                        }
                        super::OpTypeContent::Sumof(x) => {
                            *self.state = OpTypeContentSerializerState::Sumof(
                                WithSerializer::serializer(x, Some("sumof"), false)?,
                            )
                        }
                        super::OpTypeContent::Value(x) => {
                            *self.state = OpTypeContentSerializerState::Value(
                                WithSerializer::serializer(x, Some("value"), false)?,
                            )
                        }
                        super::OpTypeContent::Bit(x) => {
                            *self.state = OpTypeContentSerializerState::Bit(
                                WithSerializer::serializer(x, Some("bit"), false)?,
                            )
                        }
                    },
                    OpTypeContentSerializerState::Op(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = OpTypeContentSerializerState::Done__,
                    },
                    OpTypeContentSerializerState::Unop(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = OpTypeContentSerializerState::Done__,
                    },
                    OpTypeContentSerializerState::Fieldref(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = OpTypeContentSerializerState::Done__,
                        }
                    }
                    OpTypeContentSerializerState::Enumref(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = OpTypeContentSerializerState::Done__,
                        }
                    }
                    OpTypeContentSerializerState::Popcount(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = OpTypeContentSerializerState::Done__,
                        }
                    }
                    OpTypeContentSerializerState::Sumof(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = OpTypeContentSerializerState::Done__,
                    },
                    OpTypeContentSerializerState::Value(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = OpTypeContentSerializerState::Done__,
                    },
                    OpTypeContentSerializerState::Bit(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = OpTypeContentSerializerState::Done__,
                    },
                    OpTypeContentSerializerState::Done__ => return Ok(None),
                    OpTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for OpTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = OpTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct UnopTypeSerializer<'ser> {
        pub(super) value: &'ser super::UnopType,
        pub(super) state: Box<UnopTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum UnopTypeSerializerState<'ser> {
        Init__,
        Content__(<super::UnopTypeContent as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> UnopTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    UnopTypeSerializerState::Init__ => {
                        *self.state = UnopTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib(&mut bytes, "op", &self.value.op)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    UnopTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = UnopTypeSerializerState::End__,
                    },
                    UnopTypeSerializerState::End__ => {
                        *self.state = UnopTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    UnopTypeSerializerState::Done__ => return Ok(None),
                    UnopTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for UnopTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = UnopTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct UnopTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::UnopTypeContent,
        pub(super) state: Box<UnopTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum UnopTypeContentSerializerState<'ser> {
        Init__,
        Op(<super::OpType as WithSerializer>::Serializer<'ser>),
        Unop(<super::UnopType as WithSerializer>::Serializer<'ser>),
        Fieldref(<String as WithSerializer>::Serializer<'ser>),
        Enumref(<super::EnumrefType as WithSerializer>::Serializer<'ser>),
        Popcount(<super::PopcountType as WithSerializer>::Serializer<'ser>),
        Sumof(<super::SumofType as WithSerializer>::Serializer<'ser>),
        Value(<super::DecOrHexIntegerType as WithSerializer>::Serializer<'ser>),
        Bit(<i32 as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> UnopTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    UnopTypeContentSerializerState::Init__ => match self.value {
                        super::UnopTypeContent::Op(x) => {
                            *self.state = UnopTypeContentSerializerState::Op(
                                WithSerializer::serializer(&**x, Some("op"), false)?,
                            )
                        }
                        super::UnopTypeContent::Unop(x) => {
                            *self.state = UnopTypeContentSerializerState::Unop(
                                WithSerializer::serializer(&**x, Some("unop"), false)?,
                            )
                        }
                        super::UnopTypeContent::Fieldref(x) => {
                            *self.state = UnopTypeContentSerializerState::Fieldref(
                                WithSerializer::serializer(x, Some("fieldref"), false)?,
                            )
                        }
                        super::UnopTypeContent::Enumref(x) => {
                            *self.state = UnopTypeContentSerializerState::Enumref(
                                WithSerializer::serializer(x, Some("enumref"), false)?,
                            )
                        }
                        super::UnopTypeContent::Popcount(x) => {
                            *self.state = UnopTypeContentSerializerState::Popcount(
                                WithSerializer::serializer(x, Some("popcount"), false)?,
                            )
                        }
                        super::UnopTypeContent::Sumof(x) => {
                            *self.state = UnopTypeContentSerializerState::Sumof(
                                WithSerializer::serializer(x, Some("sumof"), false)?,
                            )
                        }
                        super::UnopTypeContent::Value(x) => {
                            *self.state = UnopTypeContentSerializerState::Value(
                                WithSerializer::serializer(x, Some("value"), false)?,
                            )
                        }
                        super::UnopTypeContent::Bit(x) => {
                            *self.state = UnopTypeContentSerializerState::Bit(
                                WithSerializer::serializer(x, Some("bit"), false)?,
                            )
                        }
                    },
                    UnopTypeContentSerializerState::Op(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = UnopTypeContentSerializerState::Done__,
                    },
                    UnopTypeContentSerializerState::Unop(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = UnopTypeContentSerializerState::Done__,
                    },
                    UnopTypeContentSerializerState::Fieldref(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = UnopTypeContentSerializerState::Done__,
                        }
                    }
                    UnopTypeContentSerializerState::Enumref(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = UnopTypeContentSerializerState::Done__,
                        }
                    }
                    UnopTypeContentSerializerState::Popcount(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = UnopTypeContentSerializerState::Done__,
                        }
                    }
                    UnopTypeContentSerializerState::Sumof(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = UnopTypeContentSerializerState::Done__,
                        }
                    }
                    UnopTypeContentSerializerState::Value(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = UnopTypeContentSerializerState::Done__,
                        }
                    }
                    UnopTypeContentSerializerState::Bit(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = UnopTypeContentSerializerState::Done__,
                    },
                    UnopTypeContentSerializerState::Done__ => return Ok(None),
                    UnopTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for UnopTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = UnopTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct EnumrefTypeSerializer<'ser> {
        pub(super) value: &'ser super::EnumrefType,
        pub(super) state: Box<EnumrefTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum EnumrefTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> EnumrefTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EnumrefTypeSerializerState::Init__ => {
                        *self.state = EnumrefTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib(&mut bytes, "ref", &self.value.ref_)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    EnumrefTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = EnumrefTypeSerializerState::End__,
                    },
                    EnumrefTypeSerializerState::End__ => {
                        *self.state = EnumrefTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    EnumrefTypeSerializerState::Done__ => return Ok(None),
                    EnumrefTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for EnumrefTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = EnumrefTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PopcountTypeSerializer<'ser> {
        pub(super) value: &'ser super::PopcountType,
        pub(super) state: Box<PopcountTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum PopcountTypeSerializerState<'ser> {
        Init__,
        Op(<super::OpType as WithSerializer>::Serializer<'ser>),
        Unop(<super::UnopType as WithSerializer>::Serializer<'ser>),
        Fieldref(<String as WithSerializer>::Serializer<'ser>),
        Enumref(<super::EnumrefType as WithSerializer>::Serializer<'ser>),
        Popcount(<super::PopcountType as WithSerializer>::Serializer<'ser>),
        Sumof(<super::SumofType as WithSerializer>::Serializer<'ser>),
        Value(<super::DecOrHexIntegerType as WithSerializer>::Serializer<'ser>),
        Bit(<i32 as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PopcountTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PopcountTypeSerializerState::Init__ => {
                        match self.value {
                            super::PopcountType::Op(x) => {
                                *self.state = PopcountTypeSerializerState::Op(
                                    WithSerializer::serializer(&**x, Some("op"), self.is_root)?,
                                )
                            }
                            super::PopcountType::Unop(x) => {
                                *self.state = PopcountTypeSerializerState::Unop(
                                    WithSerializer::serializer(&**x, Some("unop"), self.is_root)?,
                                )
                            }
                            super::PopcountType::Fieldref(x) => {
                                *self.state = PopcountTypeSerializerState::Fieldref(
                                    WithSerializer::serializer(x, Some("fieldref"), self.is_root)?,
                                )
                            }
                            super::PopcountType::Enumref(x) => {
                                *self.state = PopcountTypeSerializerState::Enumref(
                                    WithSerializer::serializer(x, Some("enumref"), self.is_root)?,
                                )
                            }
                            super::PopcountType::Popcount(x) => {
                                *self.state = PopcountTypeSerializerState::Popcount(
                                    WithSerializer::serializer(
                                        &**x,
                                        Some("popcount"),
                                        self.is_root,
                                    )?,
                                )
                            }
                            super::PopcountType::Sumof(x) => {
                                *self.state = PopcountTypeSerializerState::Sumof(
                                    WithSerializer::serializer(x, Some("sumof"), self.is_root)?,
                                )
                            }
                            super::PopcountType::Value(x) => {
                                *self.state = PopcountTypeSerializerState::Value(
                                    WithSerializer::serializer(x, Some("value"), self.is_root)?,
                                )
                            }
                            super::PopcountType::Bit(x) => {
                                *self.state = PopcountTypeSerializerState::Bit(
                                    WithSerializer::serializer(x, Some("bit"), self.is_root)?,
                                )
                            }
                        }
                        let bytes = BytesStart::new(self.name);
                        return Ok(Some(Event::Start(bytes)));
                    }
                    PopcountTypeSerializerState::Op(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PopcountTypeSerializerState::End__,
                    },
                    PopcountTypeSerializerState::Unop(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PopcountTypeSerializerState::End__,
                    },
                    PopcountTypeSerializerState::Fieldref(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PopcountTypeSerializerState::End__,
                        }
                    }
                    PopcountTypeSerializerState::Enumref(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PopcountTypeSerializerState::End__,
                    },
                    PopcountTypeSerializerState::Popcount(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PopcountTypeSerializerState::End__,
                        }
                    }
                    PopcountTypeSerializerState::Sumof(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PopcountTypeSerializerState::End__,
                    },
                    PopcountTypeSerializerState::Value(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PopcountTypeSerializerState::End__,
                    },
                    PopcountTypeSerializerState::Bit(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PopcountTypeSerializerState::End__,
                    },
                    PopcountTypeSerializerState::End__ => {
                        *self.state = PopcountTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    PopcountTypeSerializerState::Done__ => return Ok(None),
                    PopcountTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for PopcountTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PopcountTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SumofTypeSerializer<'ser> {
        pub(super) value: &'ser super::SumofType,
        pub(super) state: Box<SumofTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SumofTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SumofTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SumofTypeSerializerState::Init__ => {
                        *self.state = SumofTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib(&mut bytes, "ref", &self.value.ref_)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    SumofTypeSerializerState::Done__ => return Ok(None),
                    SumofTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for SumofTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SumofTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct CaseexprTypeSerializer<'ser> {
        pub(super) value: &'ser super::CaseexprType,
        pub(super) state: Box<CaseexprTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum CaseexprTypeSerializerState<'ser> {
        Init__,
        Content__(
            IterSerializer<'ser, &'ser [super::CaseexprTypeContent], super::CaseexprTypeContent>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CaseexprTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CaseexprTypeSerializerState::Init__ => {
                        *self.state = CaseexprTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib_opt(&mut bytes, "name", &self.value.name)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    CaseexprTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CaseexprTypeSerializerState::End__,
                        }
                    }
                    CaseexprTypeSerializerState::End__ => {
                        *self.state = CaseexprTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    CaseexprTypeSerializerState::Done__ => return Ok(None),
                    CaseexprTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for CaseexprTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CaseexprTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct CaseexprTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::CaseexprTypeContent,
        pub(super) state: Box<CaseexprTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum CaseexprTypeContentSerializerState<'ser> {
        Init__,
        Op(<super::OpType as WithSerializer>::Serializer<'ser>),
        Unop(<super::UnopType as WithSerializer>::Serializer<'ser>),
        Fieldref(<String as WithSerializer>::Serializer<'ser>),
        Enumref(<super::EnumrefType as WithSerializer>::Serializer<'ser>),
        Popcount(<super::PopcountType as WithSerializer>::Serializer<'ser>),
        Sumof(<super::SumofType as WithSerializer>::Serializer<'ser>),
        Value(<super::DecOrHexIntegerType as WithSerializer>::Serializer<'ser>),
        Bit(<i32 as WithSerializer>::Serializer<'ser>),
        Pad(<super::PadType as WithSerializer>::Serializer<'ser>),
        Field(<super::VarType as WithSerializer>::Serializer<'ser>),
        List(<super::ListType as WithSerializer>::Serializer<'ser>),
        Fd(<super::AnyType as WithSerializer>::Serializer<'ser>),
        Switch(<super::SwitchexprType as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CaseexprTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CaseexprTypeContentSerializerState::Init__ => match self.value {
                        super::CaseexprTypeContent::Op(x) => {
                            *self.state = CaseexprTypeContentSerializerState::Op(
                                WithSerializer::serializer(x, Some("op"), false)?,
                            )
                        }
                        super::CaseexprTypeContent::Unop(x) => {
                            *self.state = CaseexprTypeContentSerializerState::Unop(
                                WithSerializer::serializer(x, Some("unop"), false)?,
                            )
                        }
                        super::CaseexprTypeContent::Fieldref(x) => {
                            *self.state = CaseexprTypeContentSerializerState::Fieldref(
                                WithSerializer::serializer(x, Some("fieldref"), false)?,
                            )
                        }
                        super::CaseexprTypeContent::Enumref(x) => {
                            *self.state = CaseexprTypeContentSerializerState::Enumref(
                                WithSerializer::serializer(x, Some("enumref"), false)?,
                            )
                        }
                        super::CaseexprTypeContent::Popcount(x) => {
                            *self.state = CaseexprTypeContentSerializerState::Popcount(
                                WithSerializer::serializer(x, Some("popcount"), false)?,
                            )
                        }
                        super::CaseexprTypeContent::Sumof(x) => {
                            *self.state = CaseexprTypeContentSerializerState::Sumof(
                                WithSerializer::serializer(x, Some("sumof"), false)?,
                            )
                        }
                        super::CaseexprTypeContent::Value(x) => {
                            *self.state = CaseexprTypeContentSerializerState::Value(
                                WithSerializer::serializer(x, Some("value"), false)?,
                            )
                        }
                        super::CaseexprTypeContent::Bit(x) => {
                            *self.state = CaseexprTypeContentSerializerState::Bit(
                                WithSerializer::serializer(x, Some("bit"), false)?,
                            )
                        }
                        super::CaseexprTypeContent::Pad(x) => {
                            *self.state = CaseexprTypeContentSerializerState::Pad(
                                WithSerializer::serializer(x, Some("pad"), false)?,
                            )
                        }
                        super::CaseexprTypeContent::Field(x) => {
                            *self.state = CaseexprTypeContentSerializerState::Field(
                                WithSerializer::serializer(x, Some("field"), false)?,
                            )
                        }
                        super::CaseexprTypeContent::List(x) => {
                            *self.state = CaseexprTypeContentSerializerState::List(
                                WithSerializer::serializer(x, Some("list"), false)?,
                            )
                        }
                        super::CaseexprTypeContent::Fd(x) => {
                            *self.state = CaseexprTypeContentSerializerState::Fd(
                                WithSerializer::serializer(x, Some("fd"), false)?,
                            )
                        }
                        super::CaseexprTypeContent::Switch(x) => {
                            *self.state = CaseexprTypeContentSerializerState::Switch(
                                WithSerializer::serializer(x, Some("switch"), false)?,
                            )
                        }
                    },
                    CaseexprTypeContentSerializerState::Op(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CaseexprTypeContentSerializerState::Done__,
                        }
                    }
                    CaseexprTypeContentSerializerState::Unop(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CaseexprTypeContentSerializerState::Done__,
                        }
                    }
                    CaseexprTypeContentSerializerState::Fieldref(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CaseexprTypeContentSerializerState::Done__,
                        }
                    }
                    CaseexprTypeContentSerializerState::Enumref(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CaseexprTypeContentSerializerState::Done__,
                        }
                    }
                    CaseexprTypeContentSerializerState::Popcount(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CaseexprTypeContentSerializerState::Done__,
                        }
                    }
                    CaseexprTypeContentSerializerState::Sumof(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CaseexprTypeContentSerializerState::Done__,
                        }
                    }
                    CaseexprTypeContentSerializerState::Value(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CaseexprTypeContentSerializerState::Done__,
                        }
                    }
                    CaseexprTypeContentSerializerState::Bit(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CaseexprTypeContentSerializerState::Done__,
                        }
                    }
                    CaseexprTypeContentSerializerState::Pad(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CaseexprTypeContentSerializerState::Done__,
                        }
                    }
                    CaseexprTypeContentSerializerState::Field(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CaseexprTypeContentSerializerState::Done__,
                        }
                    }
                    CaseexprTypeContentSerializerState::List(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CaseexprTypeContentSerializerState::Done__,
                        }
                    }
                    CaseexprTypeContentSerializerState::Fd(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CaseexprTypeContentSerializerState::Done__,
                        }
                    }
                    CaseexprTypeContentSerializerState::Switch(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CaseexprTypeContentSerializerState::Done__,
                        }
                    }
                    CaseexprTypeContentSerializerState::Done__ => return Ok(None),
                    CaseexprTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for CaseexprTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CaseexprTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct FieldTypeSerializer<'ser> {
        pub(super) value: &'ser super::FieldType,
        pub(super) state: Box<FieldTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum FieldTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FieldTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FieldTypeSerializerState::Init__ => {
                        *self.state = FieldTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib_opt(&mut bytes, "name", &self.value.name)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    FieldTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FieldTypeSerializerState::End__,
                    },
                    FieldTypeSerializerState::End__ => {
                        *self.state = FieldTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    FieldTypeSerializerState::Done__ => return Ok(None),
                    FieldTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for FieldTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = FieldTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ErrorTypeSerializer<'ser> {
        pub(super) value: &'ser super::ErrorType,
        pub(super) state: Box<ErrorTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ErrorTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ErrorTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ErrorTypeSerializerState::Init__ => {
                        *self.state = ErrorTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib_opt(&mut bytes, "type", &self.value.type_)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ErrorTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ErrorTypeSerializerState::End__,
                    },
                    ErrorTypeSerializerState::End__ => {
                        *self.state = ErrorTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ErrorTypeSerializerState::Done__ => return Ok(None),
                    ErrorTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ErrorTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ErrorTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SeeTypeSerializer<'ser> {
        pub(super) value: &'ser super::SeeType,
        pub(super) state: Box<SeeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SeeTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SeeTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SeeTypeSerializerState::Init__ => {
                        *self.state = SeeTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.write_attrib_opt(&mut bytes, "name", &self.value.name)?;
                        helper.write_attrib_opt(&mut bytes, "type", &self.value.type_)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    SeeTypeSerializerState::Done__ => return Ok(None),
                    SeeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for SeeTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SeeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
