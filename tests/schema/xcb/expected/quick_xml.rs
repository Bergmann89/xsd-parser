use std::borrow::Cow;
use xsd_parser::{
    models::schema::Namespace,
    quick_xml::{
        DeserializeBytes, Error, ErrorKind, SerializeBytes, WithDeserializer, WithSerializer,
        XmlReader,
    },
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
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
            name: name.unwrap_or("xs:anyType"),
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
    pub content: [OpTypeContent; 8usize],
}
#[derive(Debug)]
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
    pub content: [UnopTypeContent; 8usize],
}
#[derive(Debug)]
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
    fn serialize_bytes(&self) -> Result<Option<Cow<'_, str>>, Error> {
        match self {
            Self::I32(x) => x.serialize_bytes(),
            Self::String(x) => x.serialize_bytes(),
        }
    }
}
impl DeserializeBytes for DecOrHexIntegerType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: XmlReader,
    {
        let mut errors = Vec::new();
        match i32::deserialize_bytes(reader, bytes) {
            Ok(value) => return Ok(Self::I32(value)),
            Err(error) => errors.push(Box::new(error)),
        }
        match String::deserialize_bytes(reader, bytes) {
            Ok(value) => return Ok(Self::String(value)),
            Err(error) => errors.push(Box::new(error)),
        }
        Err(reader.map_error(ErrorKind::InvalidUnion(errors.into())))
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
    use xsd_parser::quick_xml::{
        filter_xmlns_attributes, BytesStart, ContentDeserializer, DeserializeReader, Deserializer,
        DeserializerArtifact, DeserializerEvent, DeserializerOutput, DeserializerResult,
        ElementHandlerOutput, Error, ErrorKind, Event, RawByteStr, WithDeserializer,
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
        state: Box<XcbTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum XcbTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::XcbTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl XcbTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut header: Option<String> = None;
            let mut extension_xname: Option<String> = None;
            let mut extension_name: Option<String> = None;
            let mut extension_multiword: Option<bool> = None;
            let mut major_version: Option<i32> = None;
            let mut minor_version: Option<i32> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"header" {
                    reader.read_attrib(&mut header, b"header", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"extension-xname" {
                    reader.read_attrib(&mut extension_xname, b"extension-xname", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"extension-name" {
                    reader.read_attrib(&mut extension_name, b"extension-name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"extension-multiword" {
                    reader.read_attrib(
                        &mut extension_multiword,
                        b"extension-multiword",
                        &attrib.value,
                    )?;
                } else if attrib.key.local_name().as_ref() == b"major-version" {
                    reader.read_attrib(&mut major_version, b"major-version", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"minor-version" {
                    reader.read_attrib(&mut minor_version, b"minor-version", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                header: header.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("header".into()))
                })?,
                extension_xname: extension_xname,
                extension_name: extension_name,
                extension_multiword: extension_multiword
                    .unwrap_or_else(super::XcbType::default_extension_multiword),
                major_version: major_version,
                minor_version: minor_version,
                content: Vec::new(),
                state: Box::new(XcbTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: XcbTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let XcbTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::XcbTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::XcbTypeContent>,
            fallback: &mut Option<XcbTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback.take().unwrap_or(XcbTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = XcbTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = XcbTypeDeserializerState::Content__(deserializer);
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(XcbTypeDeserializerState::Content__(deserializer));
                            *self.state = XcbTypeDeserializerState::Next__;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::XcbType> for XcbTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::XcbType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::XcbType>
        where
            R: DeserializeReader,
        {
            use XcbTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::XcbTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::XcbType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, XcbTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::XcbType {
                header: self.header,
                extension_xname: self.extension_xname,
                extension_name: self.extension_name,
                extension_multiword: self.extension_multiword,
                major_version: self.major_version,
                minor_version: self.minor_version,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct XcbTypeContentDeserializer {
        state: Box<XcbTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum XcbTypeContentDeserializerState {
        Init__,
        Request(
            Option<super::RequestType>,
            Option<<super::RequestType as WithDeserializer>::Deserializer>,
        ),
        Event(
            Option<super::EventType>,
            Option<<super::EventType as WithDeserializer>::Deserializer>,
        ),
        Eventcopy(
            Option<super::PacketStructCopyType>,
            Option<<super::PacketStructCopyType as WithDeserializer>::Deserializer>,
        ),
        Error(
            Option<super::PacketStructType>,
            Option<<super::PacketStructType as WithDeserializer>::Deserializer>,
        ),
        Errorcopy(
            Option<super::PacketStructCopyType>,
            Option<<super::PacketStructCopyType as WithDeserializer>::Deserializer>,
        ),
        Struct(
            Option<super::StructType>,
            Option<<super::StructType as WithDeserializer>::Deserializer>,
        ),
        Union(
            Option<super::StructType>,
            Option<<super::StructType as WithDeserializer>::Deserializer>,
        ),
        Xidtype(
            Option<super::XidtypeType>,
            Option<<super::XidtypeType as WithDeserializer>::Deserializer>,
        ),
        Xidunion(
            Option<super::XidunionType>,
            Option<<super::XidunionType as WithDeserializer>::Deserializer>,
        ),
        Enum(
            Option<super::EnumType>,
            Option<<super::EnumType as WithDeserializer>::Deserializer>,
        ),
        Typedef(
            Option<super::TypedefType>,
            Option<<super::TypedefType as WithDeserializer>::Deserializer>,
        ),
        Import(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::XcbTypeContent),
        Unknown__,
    }
    impl XcbTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<XcbTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"request" {
                    let output = <super::RequestType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_request(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"event" {
                    let output =
                        <super::EventType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_event(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"eventcopy" {
                    let output =
                        <super::PacketStructCopyType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    return self.handle_eventcopy(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"error" {
                    let output = <super::PacketStructType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_error(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"errorcopy" {
                    let output =
                        <super::PacketStructCopyType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    return self.handle_errorcopy(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"struct" {
                    let output =
                        <super::StructType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_struct_(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"union" {
                    let output =
                        <super::StructType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_union_(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"xidtype" {
                    let output = <super::XidtypeType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_xidtype(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"xidunion" {
                    let output = <super::XidunionType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_xidunion(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"enum" {
                    let output =
                        <super::EnumType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_enum_(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"typedef" {
                    let output = <super::TypedefType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_typedef(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"import" {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_import(reader, Default::default(), output, &mut *fallback);
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(XcbTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: XcbTypeContentDeserializerState,
        ) -> Result<super::XcbTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use XcbTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Request(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_request(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Request(values.ok_or_else(|| {
                        ErrorKind::MissingElement("request".into())
                    })?))
                }
                S::Event(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_event(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Event(values.ok_or_else(|| {
                        ErrorKind::MissingElement("event".into())
                    })?))
                }
                S::Eventcopy(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_eventcopy(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Eventcopy(values.ok_or_else(
                        || ErrorKind::MissingElement("eventcopy".into()),
                    )?))
                }
                S::Error(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_error(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Error(values.ok_or_else(|| {
                        ErrorKind::MissingElement("error".into())
                    })?))
                }
                S::Errorcopy(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_errorcopy(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Errorcopy(values.ok_or_else(
                        || ErrorKind::MissingElement("errorcopy".into()),
                    )?))
                }
                S::Struct(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_struct_(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Struct(values.ok_or_else(|| {
                        ErrorKind::MissingElement("struct".into())
                    })?))
                }
                S::Union(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_union_(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Union(values.ok_or_else(|| {
                        ErrorKind::MissingElement("union".into())
                    })?))
                }
                S::Xidtype(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_xidtype(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Xidtype(values.ok_or_else(|| {
                        ErrorKind::MissingElement("xidtype".into())
                    })?))
                }
                S::Xidunion(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_xidunion(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Xidunion(values.ok_or_else(
                        || ErrorKind::MissingElement("xidunion".into()),
                    )?))
                }
                S::Enum(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_enum_(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Enum(
                        values.ok_or_else(|| ErrorKind::MissingElement("enum".into()))?,
                    ))
                }
                S::Typedef(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_typedef(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Typedef(values.ok_or_else(|| {
                        ErrorKind::MissingElement("typedef".into())
                    })?))
                }
                S::Import(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_import(&mut values, value)?;
                    }
                    Ok(super::XcbTypeContent::Import(values.ok_or_else(|| {
                        ErrorKind::MissingElement("import".into())
                    })?))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_request<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::RequestType>,
            output: DeserializerOutput<'de, super::RequestType>,
            fallback: &mut Option<XcbTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => XcbTypeContentDeserializerState::Request(values, None),
                    Some(XcbTypeContentDeserializerState::Request(_, Some(deserializer))) => {
                        XcbTypeContentDeserializerState::Request(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(XcbTypeContentDeserializerState::Request(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_request(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_request(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        XcbTypeContentDeserializerState::Request(values, None),
                    )?;
                    *self.state = XcbTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        XcbTypeContentDeserializerState::Request(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_event<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::EventType>,
            output: DeserializerOutput<'de, super::EventType>,
            fallback: &mut Option<XcbTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => XcbTypeContentDeserializerState::Event(values, None),
                    Some(XcbTypeContentDeserializerState::Event(_, Some(deserializer))) => {
                        XcbTypeContentDeserializerState::Event(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(XcbTypeContentDeserializerState::Event(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_event(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_event(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        XcbTypeContentDeserializerState::Event(values, None),
                    )?;
                    *self.state = XcbTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        XcbTypeContentDeserializerState::Event(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_eventcopy<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PacketStructCopyType>,
            output: DeserializerOutput<'de, super::PacketStructCopyType>,
            fallback: &mut Option<XcbTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => XcbTypeContentDeserializerState::Eventcopy(values, None),
                    Some(XcbTypeContentDeserializerState::Eventcopy(_, Some(deserializer))) => {
                        XcbTypeContentDeserializerState::Eventcopy(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(XcbTypeContentDeserializerState::Eventcopy(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_eventcopy(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_eventcopy(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        XcbTypeContentDeserializerState::Eventcopy(values, None),
                    )?;
                    *self.state = XcbTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        XcbTypeContentDeserializerState::Eventcopy(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_error<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PacketStructType>,
            output: DeserializerOutput<'de, super::PacketStructType>,
            fallback: &mut Option<XcbTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => XcbTypeContentDeserializerState::Error(values, None),
                    Some(XcbTypeContentDeserializerState::Error(_, Some(deserializer))) => {
                        XcbTypeContentDeserializerState::Error(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(XcbTypeContentDeserializerState::Error(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_error(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_error(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        XcbTypeContentDeserializerState::Error(values, None),
                    )?;
                    *self.state = XcbTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        XcbTypeContentDeserializerState::Error(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_errorcopy<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PacketStructCopyType>,
            output: DeserializerOutput<'de, super::PacketStructCopyType>,
            fallback: &mut Option<XcbTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => XcbTypeContentDeserializerState::Errorcopy(values, None),
                    Some(XcbTypeContentDeserializerState::Errorcopy(_, Some(deserializer))) => {
                        XcbTypeContentDeserializerState::Errorcopy(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(XcbTypeContentDeserializerState::Errorcopy(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_errorcopy(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_errorcopy(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        XcbTypeContentDeserializerState::Errorcopy(values, None),
                    )?;
                    *self.state = XcbTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        XcbTypeContentDeserializerState::Errorcopy(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_struct_<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::StructType>,
            output: DeserializerOutput<'de, super::StructType>,
            fallback: &mut Option<XcbTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => XcbTypeContentDeserializerState::Struct(values, None),
                    Some(XcbTypeContentDeserializerState::Struct(_, Some(deserializer))) => {
                        XcbTypeContentDeserializerState::Struct(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(XcbTypeContentDeserializerState::Struct(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_struct_(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_struct_(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        XcbTypeContentDeserializerState::Struct(values, None),
                    )?;
                    *self.state = XcbTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        XcbTypeContentDeserializerState::Struct(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_union_<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::StructType>,
            output: DeserializerOutput<'de, super::StructType>,
            fallback: &mut Option<XcbTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => XcbTypeContentDeserializerState::Union(values, None),
                    Some(XcbTypeContentDeserializerState::Union(_, Some(deserializer))) => {
                        XcbTypeContentDeserializerState::Union(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(XcbTypeContentDeserializerState::Union(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_union_(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_union_(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        XcbTypeContentDeserializerState::Union(values, None),
                    )?;
                    *self.state = XcbTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        XcbTypeContentDeserializerState::Union(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_xidtype<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::XidtypeType>,
            output: DeserializerOutput<'de, super::XidtypeType>,
            fallback: &mut Option<XcbTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => XcbTypeContentDeserializerState::Xidtype(values, None),
                    Some(XcbTypeContentDeserializerState::Xidtype(_, Some(deserializer))) => {
                        XcbTypeContentDeserializerState::Xidtype(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(XcbTypeContentDeserializerState::Xidtype(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_xidtype(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_xidtype(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        XcbTypeContentDeserializerState::Xidtype(values, None),
                    )?;
                    *self.state = XcbTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        XcbTypeContentDeserializerState::Xidtype(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_xidunion<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::XidunionType>,
            output: DeserializerOutput<'de, super::XidunionType>,
            fallback: &mut Option<XcbTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => XcbTypeContentDeserializerState::Xidunion(values, None),
                    Some(XcbTypeContentDeserializerState::Xidunion(_, Some(deserializer))) => {
                        XcbTypeContentDeserializerState::Xidunion(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(XcbTypeContentDeserializerState::Xidunion(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_xidunion(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_xidunion(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        XcbTypeContentDeserializerState::Xidunion(values, None),
                    )?;
                    *self.state = XcbTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        XcbTypeContentDeserializerState::Xidunion(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_enum_<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::EnumType>,
            output: DeserializerOutput<'de, super::EnumType>,
            fallback: &mut Option<XcbTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => XcbTypeContentDeserializerState::Enum(values, None),
                    Some(XcbTypeContentDeserializerState::Enum(_, Some(deserializer))) => {
                        XcbTypeContentDeserializerState::Enum(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(XcbTypeContentDeserializerState::Enum(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_enum_(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_enum_(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        XcbTypeContentDeserializerState::Enum(values, None),
                    )?;
                    *self.state = XcbTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = XcbTypeContentDeserializerState::Enum(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_typedef<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::TypedefType>,
            output: DeserializerOutput<'de, super::TypedefType>,
            fallback: &mut Option<XcbTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => XcbTypeContentDeserializerState::Typedef(values, None),
                    Some(XcbTypeContentDeserializerState::Typedef(_, Some(deserializer))) => {
                        XcbTypeContentDeserializerState::Typedef(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(XcbTypeContentDeserializerState::Typedef(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_typedef(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_typedef(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        XcbTypeContentDeserializerState::Typedef(values, None),
                    )?;
                    *self.state = XcbTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        XcbTypeContentDeserializerState::Typedef(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_import<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<XcbTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => XcbTypeContentDeserializerState::Import(values, None),
                    Some(XcbTypeContentDeserializerState::Import(_, Some(deserializer))) => {
                        XcbTypeContentDeserializerState::Import(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(XcbTypeContentDeserializerState::Import(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_import(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_import(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        XcbTypeContentDeserializerState::Import(values, None),
                    )?;
                    *self.state = XcbTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        XcbTypeContentDeserializerState::Import(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::XcbTypeContent> for XcbTypeContentDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::XcbTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(XcbTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, XcbTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::XcbTypeContent>
        where
            R: DeserializeReader,
        {
            use XcbTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Request(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_request(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Event(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_event(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Eventcopy(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_eventcopy(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Error(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_error(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Errorcopy(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_errorcopy(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Struct(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_struct_(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Union(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_union_(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Xidtype(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_xidtype(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Xidunion(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_xidunion(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enum(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_enum_(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Typedef(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_typedef(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Import(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_import(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::Request(values, None), event) => {
                        let output = <super::RequestType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_request(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Event(values, None), event) => {
                        let output = <super::EventType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_event(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Eventcopy(values, None), event) => {
                        let output =
                            <super::PacketStructCopyType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_eventcopy(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Error(values, None), event) => {
                        let output =
                            <super::PacketStructType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_error(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Errorcopy(values, None), event) => {
                        let output =
                            <super::PacketStructCopyType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_errorcopy(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Struct(values, None), event) => {
                        let output = <super::StructType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_struct_(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Union(values, None), event) => {
                        let output = <super::StructType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_union_(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Xidtype(values, None), event) => {
                        let output = <super::XidtypeType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_xidtype(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Xidunion(values, None), event) => {
                        let output = <super::XidunionType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_xidunion(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enum(values, None), event) => {
                        let output = <super::EnumType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_enum_(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Typedef(values, None), event) => {
                        let output = <super::TypedefType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_typedef(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Import(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_import(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::XcbTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct RequestTypeDeserializer {
        name: String,
        opcode: i32,
        combine_adjacent: Option<bool>,
        content: Vec<super::RequestTypeContent>,
        state: Box<RequestTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RequestTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::RequestTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl RequestTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<String> = None;
            let mut opcode: Option<i32> = None;
            let mut combine_adjacent: Option<bool> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"opcode" {
                    reader.read_attrib(&mut opcode, b"opcode", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"combine-adjacent" {
                    reader.read_attrib(
                        &mut combine_adjacent,
                        b"combine-adjacent",
                        &attrib.value,
                    )?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                name: name
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("name".into())))?,
                opcode: opcode.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("opcode".into()))
                })?,
                combine_adjacent: combine_adjacent,
                content: Vec::new(),
                state: Box::new(RequestTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: RequestTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let RequestTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::RequestTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::RequestTypeContent>,
            fallback: &mut Option<RequestTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(RequestTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = RequestTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let can_have_more = self.content.len().saturating_add(1) < 9usize;
                    let ret = if can_have_more {
                        ElementHandlerOutput::from_event(event, allow_any)
                    } else {
                        ElementHandlerOutput::from_event_end(event, allow_any)
                    };
                    match (can_have_more, &ret) {
                        (true, ElementHandlerOutput::Continue { .. }) => {
                            fallback.get_or_insert(RequestTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = RequestTypeDeserializerState::Next__;
                        }
                        (false, _) | (_, ElementHandlerOutput::Break { .. }) => {
                            *self.state = RequestTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RequestType> for RequestTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::RequestType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RequestType>
        where
            R: DeserializeReader,
        {
            use RequestTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::RequestTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::RequestType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, RequestTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::RequestType {
                name: self.name,
                opcode: self.opcode,
                combine_adjacent: self.combine_adjacent,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct RequestTypeContentDeserializer {
        state: Box<RequestTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum RequestTypeContentDeserializerState {
        Init__,
        Pad(
            Option<super::PadType>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
        ),
        Field(
            Option<super::VarType>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
        ),
        List(
            Option<super::ListType>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
        ),
        Fd(
            Option<super::AnyType>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
        ),
        Exprfield(
            Option<super::ExprfieldType>,
            Option<<super::ExprfieldType as WithDeserializer>::Deserializer>,
        ),
        Valueparam(
            Option<super::ValueparamType>,
            Option<<super::ValueparamType as WithDeserializer>::Deserializer>,
        ),
        Switch(
            Option<super::SwitchexprType>,
            Option<<super::SwitchexprType as WithDeserializer>::Deserializer>,
        ),
        Reply(
            Option<super::RequestReplyType>,
            Option<<super::RequestReplyType as WithDeserializer>::Deserializer>,
        ),
        Doc(
            Option<super::DocType>,
            Option<<super::DocType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::RequestTypeContent),
        Unknown__,
    }
    impl RequestTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<RequestTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"pad" {
                    let output =
                        <super::PadType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_pad(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"field" {
                    let output =
                        <super::VarType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_field(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"list" {
                    let output =
                        <super::ListType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_list(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"fd" {
                    let output =
                        <super::AnyType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_fd(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"exprfield" {
                    let output = <super::ExprfieldType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_exprfield(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"valueparam" {
                    let output = <super::ValueparamType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_valueparam(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"switch" {
                    let output = <super::SwitchexprType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_switch(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"reply" {
                    let output = <super::RequestReplyType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_reply(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"doc" {
                    let output =
                        <super::DocType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_doc(reader, Default::default(), output, &mut *fallback);
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(RequestTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: RequestTypeContentDeserializerState,
        ) -> Result<super::RequestTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use RequestTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Pad(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_pad(&mut values, value)?;
                    }
                    Ok(super::RequestTypeContent::Pad(
                        values.ok_or_else(|| ErrorKind::MissingElement("pad".into()))?,
                    ))
                }
                S::Field(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_field(&mut values, value)?;
                    }
                    Ok(super::RequestTypeContent::Field(values.ok_or_else(
                        || ErrorKind::MissingElement("field".into()),
                    )?))
                }
                S::List(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_list(&mut values, value)?;
                    }
                    Ok(super::RequestTypeContent::List(
                        values.ok_or_else(|| ErrorKind::MissingElement("list".into()))?,
                    ))
                }
                S::Fd(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_fd(&mut values, value)?;
                    }
                    Ok(super::RequestTypeContent::Fd(
                        values.ok_or_else(|| ErrorKind::MissingElement("fd".into()))?,
                    ))
                }
                S::Exprfield(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_exprfield(&mut values, value)?;
                    }
                    Ok(super::RequestTypeContent::Exprfield(values.ok_or_else(
                        || ErrorKind::MissingElement("exprfield".into()),
                    )?))
                }
                S::Valueparam(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_valueparam(&mut values, value)?;
                    }
                    Ok(super::RequestTypeContent::Valueparam(values.ok_or_else(
                        || ErrorKind::MissingElement("valueparam".into()),
                    )?))
                }
                S::Switch(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_switch(&mut values, value)?;
                    }
                    Ok(super::RequestTypeContent::Switch(values.ok_or_else(
                        || ErrorKind::MissingElement("switch".into()),
                    )?))
                }
                S::Reply(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_reply(&mut values, value)?;
                    }
                    Ok(super::RequestTypeContent::Reply(values.ok_or_else(
                        || ErrorKind::MissingElement("reply".into()),
                    )?))
                }
                S::Doc(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_doc(&mut values, value)?;
                    }
                    Ok(super::RequestTypeContent::Doc(
                        values.ok_or_else(|| ErrorKind::MissingElement("doc".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_pad<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PadType>,
            output: DeserializerOutput<'de, super::PadType>,
            fallback: &mut Option<RequestTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => RequestTypeContentDeserializerState::Pad(values, None),
                    Some(RequestTypeContentDeserializerState::Pad(_, Some(deserializer))) => {
                        RequestTypeContentDeserializerState::Pad(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(RequestTypeContentDeserializerState::Pad(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_pad(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pad(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        RequestTypeContentDeserializerState::Pad(values, None),
                    )?;
                    *self.state = RequestTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        RequestTypeContentDeserializerState::Pad(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_field<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::VarType>,
            output: DeserializerOutput<'de, super::VarType>,
            fallback: &mut Option<RequestTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => RequestTypeContentDeserializerState::Field(values, None),
                    Some(RequestTypeContentDeserializerState::Field(_, Some(deserializer))) => {
                        RequestTypeContentDeserializerState::Field(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(RequestTypeContentDeserializerState::Field(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_field(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_field(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        RequestTypeContentDeserializerState::Field(values, None),
                    )?;
                    *self.state = RequestTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        RequestTypeContentDeserializerState::Field(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_list<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ListType>,
            output: DeserializerOutput<'de, super::ListType>,
            fallback: &mut Option<RequestTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => RequestTypeContentDeserializerState::List(values, None),
                    Some(RequestTypeContentDeserializerState::List(_, Some(deserializer))) => {
                        RequestTypeContentDeserializerState::List(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(RequestTypeContentDeserializerState::List(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_list(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_list(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        RequestTypeContentDeserializerState::List(values, None),
                    )?;
                    *self.state = RequestTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        RequestTypeContentDeserializerState::List(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_fd<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnyType>,
            output: DeserializerOutput<'de, super::AnyType>,
            fallback: &mut Option<RequestTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => RequestTypeContentDeserializerState::Fd(values, None),
                    Some(RequestTypeContentDeserializerState::Fd(_, Some(deserializer))) => {
                        RequestTypeContentDeserializerState::Fd(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(RequestTypeContentDeserializerState::Fd(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_fd(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fd(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        RequestTypeContentDeserializerState::Fd(values, None),
                    )?;
                    *self.state = RequestTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        RequestTypeContentDeserializerState::Fd(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_exprfield<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ExprfieldType>,
            output: DeserializerOutput<'de, super::ExprfieldType>,
            fallback: &mut Option<RequestTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => RequestTypeContentDeserializerState::Exprfield(values, None),
                    Some(RequestTypeContentDeserializerState::Exprfield(_, Some(deserializer))) => {
                        RequestTypeContentDeserializerState::Exprfield(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(RequestTypeContentDeserializerState::Exprfield(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_exprfield(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_exprfield(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        RequestTypeContentDeserializerState::Exprfield(values, None),
                    )?;
                    *self.state = RequestTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        RequestTypeContentDeserializerState::Exprfield(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_valueparam<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ValueparamType>,
            output: DeserializerOutput<'de, super::ValueparamType>,
            fallback: &mut Option<RequestTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => RequestTypeContentDeserializerState::Valueparam(values, None),
                    Some(RequestTypeContentDeserializerState::Valueparam(
                        _,
                        Some(deserializer),
                    )) => {
                        RequestTypeContentDeserializerState::Valueparam(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(RequestTypeContentDeserializerState::Valueparam(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_valueparam(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_valueparam(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        RequestTypeContentDeserializerState::Valueparam(values, None),
                    )?;
                    *self.state = RequestTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        RequestTypeContentDeserializerState::Valueparam(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_switch<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SwitchexprType>,
            output: DeserializerOutput<'de, super::SwitchexprType>,
            fallback: &mut Option<RequestTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => RequestTypeContentDeserializerState::Switch(values, None),
                    Some(RequestTypeContentDeserializerState::Switch(_, Some(deserializer))) => {
                        RequestTypeContentDeserializerState::Switch(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(RequestTypeContentDeserializerState::Switch(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_switch(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_switch(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        RequestTypeContentDeserializerState::Switch(values, None),
                    )?;
                    *self.state = RequestTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        RequestTypeContentDeserializerState::Switch(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_reply<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::RequestReplyType>,
            output: DeserializerOutput<'de, super::RequestReplyType>,
            fallback: &mut Option<RequestTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => RequestTypeContentDeserializerState::Reply(values, None),
                    Some(RequestTypeContentDeserializerState::Reply(_, Some(deserializer))) => {
                        RequestTypeContentDeserializerState::Reply(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(RequestTypeContentDeserializerState::Reply(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_reply(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_reply(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        RequestTypeContentDeserializerState::Reply(values, None),
                    )?;
                    *self.state = RequestTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        RequestTypeContentDeserializerState::Reply(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_doc<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::DocType>,
            output: DeserializerOutput<'de, super::DocType>,
            fallback: &mut Option<RequestTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => RequestTypeContentDeserializerState::Doc(values, None),
                    Some(RequestTypeContentDeserializerState::Doc(_, Some(deserializer))) => {
                        RequestTypeContentDeserializerState::Doc(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(RequestTypeContentDeserializerState::Doc(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_doc(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_doc(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        RequestTypeContentDeserializerState::Doc(values, None),
                    )?;
                    *self.state = RequestTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        RequestTypeContentDeserializerState::Doc(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RequestTypeContent> for RequestTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RequestTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(RequestTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, RequestTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RequestTypeContent>
        where
            R: DeserializeReader,
        {
            use RequestTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Pad(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pad(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_field(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_list(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fd(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Exprfield(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_exprfield(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Valueparam(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_valueparam(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Switch(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_switch(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Reply(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_reply(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Doc(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_doc(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::Pad(values, None), event) => {
                        let output = <super::PadType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_pad(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, None), event) => {
                        let output = <super::VarType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_field(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, None), event) => {
                        let output = <super::ListType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_list(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, None), event) => {
                        let output = <super::AnyType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_fd(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Exprfield(values, None), event) => {
                        let output =
                            <super::ExprfieldType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_exprfield(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Valueparam(values, None), event) => {
                        let output =
                            <super::ValueparamType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_valueparam(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Switch(values, None), event) => {
                        let output =
                            <super::SwitchexprType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_switch(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Reply(values, None), event) => {
                        let output =
                            <super::RequestReplyType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_reply(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Doc(values, None), event) => {
                        let output = <super::DocType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_doc(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::RequestTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct EventTypeDeserializer {
        name: String,
        number: i32,
        no_sequence_number: Option<bool>,
        xge: Option<bool>,
        content: Vec<super::EventTypeContent>,
        state: Box<EventTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum EventTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::EventTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl EventTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<String> = None;
            let mut number: Option<i32> = None;
            let mut no_sequence_number: Option<bool> = None;
            let mut xge: Option<bool> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"number" {
                    reader.read_attrib(&mut number, b"number", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"no-sequence-number" {
                    reader.read_attrib(
                        &mut no_sequence_number,
                        b"no-sequence-number",
                        &attrib.value,
                    )?;
                } else if attrib.key.local_name().as_ref() == b"xge" {
                    reader.read_attrib(&mut xge, b"xge", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                name: name
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("name".into())))?,
                number: number.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("number".into()))
                })?,
                no_sequence_number: no_sequence_number,
                xge: xge,
                content: Vec::new(),
                state: Box::new(EventTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: EventTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let EventTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::EventTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::EventTypeContent>,
            fallback: &mut Option<EventTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(EventTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = EventTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = EventTypeDeserializerState::Content__(deserializer);
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(EventTypeDeserializerState::Content__(deserializer));
                            *self.state = EventTypeDeserializerState::Next__;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::EventType> for EventTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::EventType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EventType>
        where
            R: DeserializeReader,
        {
            use EventTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::EventTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::EventType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, EventTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::EventType {
                name: self.name,
                number: self.number,
                no_sequence_number: self.no_sequence_number,
                xge: self.xge,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct EventTypeContentDeserializer {
        state: Box<EventTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum EventTypeContentDeserializerState {
        Init__,
        Pad(
            Option<super::PadType>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
        ),
        Field(
            Option<super::VarType>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
        ),
        List(
            Option<super::ListType>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
        ),
        Fd(
            Option<super::AnyType>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
        ),
        Doc(
            Option<super::DocType>,
            Option<<super::DocType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::EventTypeContent),
        Unknown__,
    }
    impl EventTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<EventTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"pad" {
                    let output =
                        <super::PadType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_pad(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"field" {
                    let output =
                        <super::VarType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_field(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"list" {
                    let output =
                        <super::ListType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_list(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"fd" {
                    let output =
                        <super::AnyType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_fd(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"doc" {
                    let output =
                        <super::DocType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_doc(reader, Default::default(), output, &mut *fallback);
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(EventTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: EventTypeContentDeserializerState,
        ) -> Result<super::EventTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use EventTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Pad(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_pad(&mut values, value)?;
                    }
                    Ok(super::EventTypeContent::Pad(
                        values.ok_or_else(|| ErrorKind::MissingElement("pad".into()))?,
                    ))
                }
                S::Field(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_field(&mut values, value)?;
                    }
                    Ok(super::EventTypeContent::Field(values.ok_or_else(|| {
                        ErrorKind::MissingElement("field".into())
                    })?))
                }
                S::List(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_list(&mut values, value)?;
                    }
                    Ok(super::EventTypeContent::List(
                        values.ok_or_else(|| ErrorKind::MissingElement("list".into()))?,
                    ))
                }
                S::Fd(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_fd(&mut values, value)?;
                    }
                    Ok(super::EventTypeContent::Fd(
                        values.ok_or_else(|| ErrorKind::MissingElement("fd".into()))?,
                    ))
                }
                S::Doc(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_doc(&mut values, value)?;
                    }
                    Ok(super::EventTypeContent::Doc(
                        values.ok_or_else(|| ErrorKind::MissingElement("doc".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_pad<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PadType>,
            output: DeserializerOutput<'de, super::PadType>,
            fallback: &mut Option<EventTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => EventTypeContentDeserializerState::Pad(values, None),
                    Some(EventTypeContentDeserializerState::Pad(_, Some(deserializer))) => {
                        EventTypeContentDeserializerState::Pad(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(EventTypeContentDeserializerState::Pad(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_pad(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pad(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        EventTypeContentDeserializerState::Pad(values, None),
                    )?;
                    *self.state = EventTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        EventTypeContentDeserializerState::Pad(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_field<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::VarType>,
            output: DeserializerOutput<'de, super::VarType>,
            fallback: &mut Option<EventTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => EventTypeContentDeserializerState::Field(values, None),
                    Some(EventTypeContentDeserializerState::Field(_, Some(deserializer))) => {
                        EventTypeContentDeserializerState::Field(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(EventTypeContentDeserializerState::Field(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_field(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_field(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        EventTypeContentDeserializerState::Field(values, None),
                    )?;
                    *self.state = EventTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        EventTypeContentDeserializerState::Field(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_list<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ListType>,
            output: DeserializerOutput<'de, super::ListType>,
            fallback: &mut Option<EventTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => EventTypeContentDeserializerState::List(values, None),
                    Some(EventTypeContentDeserializerState::List(_, Some(deserializer))) => {
                        EventTypeContentDeserializerState::List(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(EventTypeContentDeserializerState::List(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_list(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_list(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        EventTypeContentDeserializerState::List(values, None),
                    )?;
                    *self.state = EventTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        EventTypeContentDeserializerState::List(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_fd<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnyType>,
            output: DeserializerOutput<'de, super::AnyType>,
            fallback: &mut Option<EventTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => EventTypeContentDeserializerState::Fd(values, None),
                    Some(EventTypeContentDeserializerState::Fd(_, Some(deserializer))) => {
                        EventTypeContentDeserializerState::Fd(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(EventTypeContentDeserializerState::Fd(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_fd(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fd(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        EventTypeContentDeserializerState::Fd(values, None),
                    )?;
                    *self.state = EventTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = EventTypeContentDeserializerState::Fd(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_doc<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::DocType>,
            output: DeserializerOutput<'de, super::DocType>,
            fallback: &mut Option<EventTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => EventTypeContentDeserializerState::Doc(values, None),
                    Some(EventTypeContentDeserializerState::Doc(_, Some(deserializer))) => {
                        EventTypeContentDeserializerState::Doc(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(EventTypeContentDeserializerState::Doc(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_doc(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_doc(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        EventTypeContentDeserializerState::Doc(values, None),
                    )?;
                    *self.state = EventTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        EventTypeContentDeserializerState::Doc(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::EventTypeContent> for EventTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EventTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(EventTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, EventTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EventTypeContent>
        where
            R: DeserializeReader,
        {
            use EventTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Pad(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pad(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_field(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_list(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fd(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Doc(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_doc(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::Pad(values, None), event) => {
                        let output = <super::PadType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_pad(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, None), event) => {
                        let output = <super::VarType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_field(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, None), event) => {
                        let output = <super::ListType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_list(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, None), event) => {
                        let output = <super::AnyType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_fd(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Doc(values, None), event) => {
                        let output = <super::DocType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_doc(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::EventTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct PacketStructCopyTypeDeserializer {
        name: String,
        number: i32,
        ref_: String,
        state: Box<PacketStructCopyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PacketStructCopyTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl PacketStructCopyTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<String> = None;
            let mut number: Option<i32> = None;
            let mut ref_: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"number" {
                    reader.read_attrib(&mut number, b"number", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"ref" {
                    reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                name: name
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("name".into())))?,
                number: number.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("number".into()))
                })?,
                ref_: ref_
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("ref".into())))?,
                state: Box::new(PacketStructCopyTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: PacketStructCopyTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::PacketStructCopyType> for PacketStructCopyTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PacketStructCopyType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PacketStructCopyType>
        where
            R: DeserializeReader,
        {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(reader)?),
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
        fn finish<R>(mut self, reader: &R) -> Result<super::PacketStructCopyType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                PacketStructCopyTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
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
        state: Box<PacketStructTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PacketStructTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::PacketStructTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl PacketStructTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<String> = None;
            let mut number: Option<i32> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"number" {
                    reader.read_attrib(&mut number, b"number", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                name: name
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("name".into())))?,
                number: number.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("number".into()))
                })?,
                content: Vec::new(),
                state: Box::new(PacketStructTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: PacketStructTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let PacketStructTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::PacketStructTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::PacketStructTypeContent>,
            fallback: &mut Option<PacketStructTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(PacketStructTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = PacketStructTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                PacketStructTypeDeserializerState::Content__(deserializer);
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(PacketStructTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = PacketStructTypeDeserializerState::Next__;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::PacketStructType> for PacketStructTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PacketStructType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PacketStructType>
        where
            R: DeserializeReader,
        {
            use PacketStructTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output = < super :: PacketStructTypeContent as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::PacketStructType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                PacketStructTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::PacketStructType {
                name: self.name,
                number: self.number,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct PacketStructTypeContentDeserializer {
        state: Box<PacketStructTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum PacketStructTypeContentDeserializerState {
        Init__,
        Pad(
            Option<super::PadType>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
        ),
        Field(
            Option<super::VarType>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
        ),
        List(
            Option<super::ListType>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
        ),
        Fd(
            Option<super::AnyType>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::PacketStructTypeContent),
        Unknown__,
    }
    impl PacketStructTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<PacketStructTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"pad" {
                    let output =
                        <super::PadType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_pad(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"field" {
                    let output =
                        <super::VarType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_field(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"list" {
                    let output =
                        <super::ListType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_list(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"fd" {
                    let output =
                        <super::AnyType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_fd(reader, Default::default(), output, &mut *fallback);
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(PacketStructTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: PacketStructTypeContentDeserializerState,
        ) -> Result<super::PacketStructTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use PacketStructTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Pad(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_pad(&mut values, value)?;
                    }
                    Ok(super::PacketStructTypeContent::Pad(
                        values.ok_or_else(|| ErrorKind::MissingElement("pad".into()))?,
                    ))
                }
                S::Field(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_field(&mut values, value)?;
                    }
                    Ok(super::PacketStructTypeContent::Field(values.ok_or_else(
                        || ErrorKind::MissingElement("field".into()),
                    )?))
                }
                S::List(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_list(&mut values, value)?;
                    }
                    Ok(super::PacketStructTypeContent::List(
                        values.ok_or_else(|| ErrorKind::MissingElement("list".into()))?,
                    ))
                }
                S::Fd(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_fd(&mut values, value)?;
                    }
                    Ok(super::PacketStructTypeContent::Fd(
                        values.ok_or_else(|| ErrorKind::MissingElement("fd".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_pad<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PadType>,
            output: DeserializerOutput<'de, super::PadType>,
            fallback: &mut Option<PacketStructTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => PacketStructTypeContentDeserializerState::Pad(values, None),
                    Some(PacketStructTypeContentDeserializerState::Pad(_, Some(deserializer))) => {
                        PacketStructTypeContentDeserializerState::Pad(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(PacketStructTypeContentDeserializerState::Pad(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_pad(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pad(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        PacketStructTypeContentDeserializerState::Pad(values, None),
                    )?;
                    *self.state = PacketStructTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        PacketStructTypeContentDeserializerState::Pad(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_field<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::VarType>,
            output: DeserializerOutput<'de, super::VarType>,
            fallback: &mut Option<PacketStructTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => PacketStructTypeContentDeserializerState::Field(values, None),
                    Some(PacketStructTypeContentDeserializerState::Field(
                        _,
                        Some(deserializer),
                    )) => {
                        PacketStructTypeContentDeserializerState::Field(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(PacketStructTypeContentDeserializerState::Field(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_field(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_field(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        PacketStructTypeContentDeserializerState::Field(values, None),
                    )?;
                    *self.state = PacketStructTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        PacketStructTypeContentDeserializerState::Field(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_list<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ListType>,
            output: DeserializerOutput<'de, super::ListType>,
            fallback: &mut Option<PacketStructTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => PacketStructTypeContentDeserializerState::List(values, None),
                    Some(PacketStructTypeContentDeserializerState::List(_, Some(deserializer))) => {
                        PacketStructTypeContentDeserializerState::List(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(PacketStructTypeContentDeserializerState::List(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_list(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_list(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        PacketStructTypeContentDeserializerState::List(values, None),
                    )?;
                    *self.state = PacketStructTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        PacketStructTypeContentDeserializerState::List(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_fd<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnyType>,
            output: DeserializerOutput<'de, super::AnyType>,
            fallback: &mut Option<PacketStructTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => PacketStructTypeContentDeserializerState::Fd(values, None),
                    Some(PacketStructTypeContentDeserializerState::Fd(_, Some(deserializer))) => {
                        PacketStructTypeContentDeserializerState::Fd(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(PacketStructTypeContentDeserializerState::Fd(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_fd(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fd(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        PacketStructTypeContentDeserializerState::Fd(values, None),
                    )?;
                    *self.state = PacketStructTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        PacketStructTypeContentDeserializerState::Fd(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::PacketStructTypeContent>
        for PacketStructTypeContentDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PacketStructTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(PacketStructTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, PacketStructTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PacketStructTypeContent>
        where
            R: DeserializeReader,
        {
            use PacketStructTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Pad(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pad(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_field(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_list(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fd(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::Pad(values, None), event) => {
                        let output = <super::PadType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_pad(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, None), event) => {
                        let output = <super::VarType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_field(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, None), event) => {
                        let output = <super::ListType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_list(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, None), event) => {
                        let output = <super::AnyType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_fd(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::PacketStructTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct StructTypeDeserializer {
        name: String,
        content: Vec<super::StructTypeContent>,
        state: Box<StructTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum StructTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::StructTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl StructTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                name: name
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("name".into())))?,
                content: Vec::new(),
                state: Box::new(StructTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: StructTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let StructTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::StructTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::StructTypeContent>,
            fallback: &mut Option<StructTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(StructTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = StructTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let can_have_more = self.content.len().saturating_add(1) < 5usize;
                    let ret = if can_have_more {
                        ElementHandlerOutput::from_event(event, allow_any)
                    } else {
                        ElementHandlerOutput::from_event_end(event, allow_any)
                    };
                    match (can_have_more, &ret) {
                        (true, ElementHandlerOutput::Continue { .. }) => {
                            fallback.get_or_insert(StructTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = StructTypeDeserializerState::Next__;
                        }
                        (false, _) | (_, ElementHandlerOutput::Break { .. }) => {
                            *self.state = StructTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::StructType> for StructTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::StructType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::StructType>
        where
            R: DeserializeReader,
        {
            use StructTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::StructTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::StructType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, StructTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::StructType {
                name: self.name,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct StructTypeContentDeserializer {
        state: Box<StructTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum StructTypeContentDeserializerState {
        Init__,
        Pad(
            Option<super::PadType>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
        ),
        Field(
            Option<super::VarType>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
        ),
        List(
            Option<super::ListType>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
        ),
        Fd(
            Option<super::AnyType>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
        ),
        Switch(
            Option<super::SwitchexprType>,
            Option<<super::SwitchexprType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::StructTypeContent),
        Unknown__,
    }
    impl StructTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<StructTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"pad" {
                    let output =
                        <super::PadType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_pad(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"field" {
                    let output =
                        <super::VarType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_field(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"list" {
                    let output =
                        <super::ListType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_list(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"fd" {
                    let output =
                        <super::AnyType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_fd(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"switch" {
                    let output = <super::SwitchexprType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_switch(reader, Default::default(), output, &mut *fallback);
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(StructTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: StructTypeContentDeserializerState,
        ) -> Result<super::StructTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use StructTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Pad(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_pad(&mut values, value)?;
                    }
                    Ok(super::StructTypeContent::Pad(
                        values.ok_or_else(|| ErrorKind::MissingElement("pad".into()))?,
                    ))
                }
                S::Field(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_field(&mut values, value)?;
                    }
                    Ok(super::StructTypeContent::Field(values.ok_or_else(
                        || ErrorKind::MissingElement("field".into()),
                    )?))
                }
                S::List(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_list(&mut values, value)?;
                    }
                    Ok(super::StructTypeContent::List(
                        values.ok_or_else(|| ErrorKind::MissingElement("list".into()))?,
                    ))
                }
                S::Fd(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_fd(&mut values, value)?;
                    }
                    Ok(super::StructTypeContent::Fd(
                        values.ok_or_else(|| ErrorKind::MissingElement("fd".into()))?,
                    ))
                }
                S::Switch(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_switch(&mut values, value)?;
                    }
                    Ok(super::StructTypeContent::Switch(values.ok_or_else(
                        || ErrorKind::MissingElement("switch".into()),
                    )?))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_pad<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PadType>,
            output: DeserializerOutput<'de, super::PadType>,
            fallback: &mut Option<StructTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => StructTypeContentDeserializerState::Pad(values, None),
                    Some(StructTypeContentDeserializerState::Pad(_, Some(deserializer))) => {
                        StructTypeContentDeserializerState::Pad(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(StructTypeContentDeserializerState::Pad(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_pad(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pad(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        StructTypeContentDeserializerState::Pad(values, None),
                    )?;
                    *self.state = StructTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        StructTypeContentDeserializerState::Pad(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_field<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::VarType>,
            output: DeserializerOutput<'de, super::VarType>,
            fallback: &mut Option<StructTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => StructTypeContentDeserializerState::Field(values, None),
                    Some(StructTypeContentDeserializerState::Field(_, Some(deserializer))) => {
                        StructTypeContentDeserializerState::Field(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(StructTypeContentDeserializerState::Field(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_field(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_field(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        StructTypeContentDeserializerState::Field(values, None),
                    )?;
                    *self.state = StructTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        StructTypeContentDeserializerState::Field(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_list<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ListType>,
            output: DeserializerOutput<'de, super::ListType>,
            fallback: &mut Option<StructTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => StructTypeContentDeserializerState::List(values, None),
                    Some(StructTypeContentDeserializerState::List(_, Some(deserializer))) => {
                        StructTypeContentDeserializerState::List(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(StructTypeContentDeserializerState::List(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_list(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_list(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        StructTypeContentDeserializerState::List(values, None),
                    )?;
                    *self.state = StructTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        StructTypeContentDeserializerState::List(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_fd<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnyType>,
            output: DeserializerOutput<'de, super::AnyType>,
            fallback: &mut Option<StructTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => StructTypeContentDeserializerState::Fd(values, None),
                    Some(StructTypeContentDeserializerState::Fd(_, Some(deserializer))) => {
                        StructTypeContentDeserializerState::Fd(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(StructTypeContentDeserializerState::Fd(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_fd(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fd(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        StructTypeContentDeserializerState::Fd(values, None),
                    )?;
                    *self.state = StructTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        StructTypeContentDeserializerState::Fd(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_switch<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SwitchexprType>,
            output: DeserializerOutput<'de, super::SwitchexprType>,
            fallback: &mut Option<StructTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => StructTypeContentDeserializerState::Switch(values, None),
                    Some(StructTypeContentDeserializerState::Switch(_, Some(deserializer))) => {
                        StructTypeContentDeserializerState::Switch(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(StructTypeContentDeserializerState::Switch(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_switch(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_switch(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        StructTypeContentDeserializerState::Switch(values, None),
                    )?;
                    *self.state = StructTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        StructTypeContentDeserializerState::Switch(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::StructTypeContent> for StructTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::StructTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(StructTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, StructTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::StructTypeContent>
        where
            R: DeserializeReader,
        {
            use StructTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Pad(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pad(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_field(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_list(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fd(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Switch(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_switch(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::Pad(values, None), event) => {
                        let output = <super::PadType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_pad(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, None), event) => {
                        let output = <super::VarType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_field(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, None), event) => {
                        let output = <super::ListType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_list(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, None), event) => {
                        let output = <super::AnyType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_fd(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Switch(values, None), event) => {
                        let output =
                            <super::SwitchexprType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_switch(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::StructTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct XidtypeTypeDeserializer {
        name: String,
        state: Box<XidtypeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum XidtypeTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl XidtypeTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                name: name
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("name".into())))?,
                state: Box::new(XidtypeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: XidtypeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::XidtypeType> for XidtypeTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::XidtypeType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::XidtypeType>
        where
            R: DeserializeReader,
        {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(reader)?),
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
        fn finish<R>(mut self, reader: &R) -> Result<super::XidtypeType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, XidtypeTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::XidtypeType { name: self.name })
        }
    }
    #[derive(Debug)]
    pub struct XidunionTypeDeserializer {
        name: String,
        type_: Vec<String>,
        state: Box<XidunionTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum XidunionTypeDeserializerState {
        Init__,
        Type(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl XidunionTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                name: name
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("name".into())))?,
                type_: Vec::new(),
                state: Box::new(XidunionTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: XidunionTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use XidunionTypeDeserializerState as S;
            match state {
                S::Type(Some(deserializer)) => self.store_type_(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_type_(&mut self, value: String) -> Result<(), Error> {
            self.type_.push(value);
            Ok(())
        }
        fn handle_type_<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<XidunionTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.type_.len() < 1usize {
                    *self.state = XidunionTypeDeserializerState::Type(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(XidunionTypeDeserializerState::Type(None));
                    *self.state = XidunionTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_type_(data)?;
                    *self.state = XidunionTypeDeserializerState::Type(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(XidunionTypeDeserializerState::Type(Some(
                                deserializer,
                            )));
                            if self.type_.len().saturating_add(1) < 1usize {
                                *self.state = XidunionTypeDeserializerState::Type(None);
                            } else {
                                *self.state = XidunionTypeDeserializerState::Done__;
                            }
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = XidunionTypeDeserializerState::Type(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::XidunionType> for XidunionTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::XidunionType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::XidunionType>
        where
            R: DeserializeReader,
        {
            use XidunionTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Type(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_type_(reader, output, &mut fallback)? {
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
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = XidunionTypeDeserializerState::Type(None);
                        event
                    }
                    (S::Type(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"type") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_type_(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::XidunionType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, XidunionTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::XidunionType {
                name: self.name,
                type_: self.type_,
            })
        }
    }
    #[derive(Debug)]
    pub struct EnumTypeDeserializer {
        name: String,
        content: Vec<super::EnumTypeContent>,
        state: Box<EnumTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum EnumTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::EnumTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl EnumTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                name: name
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("name".into())))?,
                content: Vec::new(),
                state: Box::new(EnumTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: EnumTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let EnumTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::EnumTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::EnumTypeContent>,
            fallback: &mut Option<EnumTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback.take().unwrap_or(EnumTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = EnumTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = EnumTypeDeserializerState::Content__(deserializer);
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(EnumTypeDeserializerState::Content__(deserializer));
                            *self.state = EnumTypeDeserializerState::Next__;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::EnumType> for EnumTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::EnumType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EnumType>
        where
            R: DeserializeReader,
        {
            use EnumTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::EnumTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::EnumType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, EnumTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::EnumType {
                name: self.name,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct EnumTypeContentDeserializer {
        item: Option<super::EnumItemType>,
        doc: Option<super::DocType>,
        state: Box<EnumTypeContentDeserializerState>,
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
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: EnumTypeContentDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use EnumTypeContentDeserializerState as S;
            match state {
                S::Item(Some(deserializer)) => self.store_item(deserializer.finish(reader)?)?,
                S::Doc(Some(deserializer)) => self.store_doc(deserializer.finish(reader)?)?,
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
        fn handle_item<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::EnumItemType>,
            fallback: &mut Option<EnumTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.item.is_some() {
                    fallback.get_or_insert(EnumTypeContentDeserializerState::Item(None));
                    *self.state = EnumTypeContentDeserializerState::Doc(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = EnumTypeContentDeserializerState::Item(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_item(data)?;
                    *self.state = EnumTypeContentDeserializerState::Doc(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(EnumTypeContentDeserializerState::Item(Some(
                                deserializer,
                            )));
                            *self.state = EnumTypeContentDeserializerState::Doc(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                EnumTypeContentDeserializerState::Item(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_doc<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DocType>,
            fallback: &mut Option<EnumTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(EnumTypeContentDeserializerState::Doc(None));
                *self.state = EnumTypeContentDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_doc(data)?;
                    *self.state = EnumTypeContentDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(EnumTypeContentDeserializerState::Doc(Some(
                                deserializer,
                            )));
                            *self.state = EnumTypeContentDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = EnumTypeContentDeserializerState::Doc(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::EnumTypeContent> for EnumTypeContentDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::EnumTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                item: None,
                doc: None,
                state: Box::new(EnumTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, EnumTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EnumTypeContent>
        where
            R: DeserializeReader,
        {
            use EnumTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Item(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_item(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_doc(reader, output, &mut fallback)? {
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
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = EnumTypeContentDeserializerState::Item(None);
                        event
                    }
                    (S::Item(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"item") {
                            let output =
                                <super::EnumItemType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_item(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Doc(None);
                            event
                        }
                    }
                    (S::Doc(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"doc") {
                            let output = <super::DocType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_doc(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::EnumTypeContent, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                EnumTypeContentDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::EnumTypeContent {
                item: self
                    .item
                    .ok_or_else(|| ErrorKind::MissingElement("item".into()))?,
                doc: self.doc,
            })
        }
    }
    #[derive(Debug)]
    pub struct TypedefTypeDeserializer {
        oldname: String,
        newname: String,
        state: Box<TypedefTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TypedefTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl TypedefTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut oldname: Option<String> = None;
            let mut newname: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"oldname" {
                    reader.read_attrib(&mut oldname, b"oldname", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"newname" {
                    reader.read_attrib(&mut newname, b"newname", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                oldname: oldname.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("oldname".into()))
                })?,
                newname: newname.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("newname".into()))
                })?,
                state: Box::new(TypedefTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TypedefTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::TypedefType> for TypedefTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::TypedefType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TypedefType>
        where
            R: DeserializeReader,
        {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(reader)?),
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
        fn finish<R>(mut self, reader: &R) -> Result<super::TypedefType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, TypedefTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
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
        state: Box<PadTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PadTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl PadTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut bytes: Option<i32> = None;
            let mut align: Option<i32> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"bytes" {
                    reader.read_attrib(&mut bytes, b"bytes", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"align" {
                    reader.read_attrib(&mut align, b"align", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                bytes: bytes,
                align: align,
                state: Box::new(PadTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: PadTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::PadType> for PadTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::PadType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PadType>
        where
            R: DeserializeReader,
        {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(reader)?),
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
        fn finish<R>(mut self, reader: &R) -> Result<super::PadType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, PadTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
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
        state: Box<VarTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum VarTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl VarTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<String> = None;
            let mut type_: Option<String> = None;
            let mut enum_: Option<String> = None;
            let mut altenum: Option<String> = None;
            let mut mask: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"type" {
                    reader.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"enum" {
                    reader.read_attrib(&mut enum_, b"enum", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"altenum" {
                    reader.read_attrib(&mut altenum, b"altenum", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"mask" {
                    reader.read_attrib(&mut mask, b"mask", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                name: name
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("name".into())))?,
                type_: type_
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("type".into())))?,
                enum_: enum_,
                altenum: altenum,
                mask: mask,
                state: Box::new(VarTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: VarTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::VarType> for VarTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::VarType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::VarType>
        where
            R: DeserializeReader,
        {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(reader)?),
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
        fn finish<R>(mut self, reader: &R) -> Result<super::VarType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, VarTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
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
        state: Box<ListTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ListTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::ListTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ListTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<String> = None;
            let mut type_: Option<String> = None;
            let mut enum_: Option<String> = None;
            let mut altenum: Option<String> = None;
            let mut mask: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"type" {
                    reader.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"enum" {
                    reader.read_attrib(&mut enum_, b"enum", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"altenum" {
                    reader.read_attrib(&mut altenum, b"altenum", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"mask" {
                    reader.read_attrib(&mut mask, b"mask", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                name: name
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("name".into())))?,
                type_: type_
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("type".into())))?,
                enum_: enum_,
                altenum: altenum,
                mask: mask,
                content: None,
                state: Box::new(ListTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ListTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let ListTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
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
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ListTypeContent>,
            fallback: &mut Option<ListTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback.take().unwrap_or(ListTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = ListTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = ListTypeDeserializerState::Content__(deserializer);
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ListType> for ListTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ListType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ListType>
        where
            R: DeserializeReader,
        {
            use ListTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::ListTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ListType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, ListTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
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
        state: Box<ListTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ListTypeContentDeserializerState {
        Init__,
        Op(
            Option<super::OpType>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
        ),
        Unop(
            Option<super::UnopType>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
        ),
        Fieldref(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Enumref(
            Option<super::EnumrefType>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
        ),
        Popcount(
            Option<super::PopcountType>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
        ),
        Sumof(
            Option<super::SumofType>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
        ),
        Value(
            Option<super::DecOrHexIntegerType>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
        ),
        Bit(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Done__(super::ListTypeContent),
        Unknown__,
    }
    impl ListTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<ListTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"op" {
                    let output =
                        <super::OpType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_op(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"unop" {
                    let output =
                        <super::UnopType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_unop(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"fieldref" {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_fieldref(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"enumref" {
                    let output = <super::EnumrefType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_enumref(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"popcount" {
                    let output = <super::PopcountType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_popcount(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"sumof" {
                    let output =
                        <super::SumofType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_sumof(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"value" {
                    let output =
                        <super::DecOrHexIntegerType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    return self.handle_value(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"bit" {
                    let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_bit(reader, Default::default(), output, &mut *fallback);
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(ListTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: ListTypeContentDeserializerState,
        ) -> Result<super::ListTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use ListTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Op(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_op(&mut values, value)?;
                    }
                    Ok(super::ListTypeContent::Op(
                        values.ok_or_else(|| ErrorKind::MissingElement("op".into()))?,
                    ))
                }
                S::Unop(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_unop(&mut values, value)?;
                    }
                    Ok(super::ListTypeContent::Unop(
                        values.ok_or_else(|| ErrorKind::MissingElement("unop".into()))?,
                    ))
                }
                S::Fieldref(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_fieldref(&mut values, value)?;
                    }
                    Ok(super::ListTypeContent::Fieldref(values.ok_or_else(
                        || ErrorKind::MissingElement("fieldref".into()),
                    )?))
                }
                S::Enumref(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_enumref(&mut values, value)?;
                    }
                    Ok(super::ListTypeContent::Enumref(values.ok_or_else(
                        || ErrorKind::MissingElement("enumref".into()),
                    )?))
                }
                S::Popcount(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_popcount(&mut values, value)?;
                    }
                    Ok(super::ListTypeContent::Popcount(values.ok_or_else(
                        || ErrorKind::MissingElement("popcount".into()),
                    )?))
                }
                S::Sumof(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_sumof(&mut values, value)?;
                    }
                    Ok(super::ListTypeContent::Sumof(values.ok_or_else(|| {
                        ErrorKind::MissingElement("sumof".into())
                    })?))
                }
                S::Value(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_value(&mut values, value)?;
                    }
                    Ok(super::ListTypeContent::Value(values.ok_or_else(|| {
                        ErrorKind::MissingElement("value".into())
                    })?))
                }
                S::Bit(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_bit(&mut values, value)?;
                    }
                    Ok(super::ListTypeContent::Bit(
                        values.ok_or_else(|| ErrorKind::MissingElement("bit".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_op<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::OpType>,
            output: DeserializerOutput<'de, super::OpType>,
            fallback: &mut Option<ListTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => ListTypeContentDeserializerState::Op(values, None),
                    Some(ListTypeContentDeserializerState::Op(_, Some(deserializer))) => {
                        ListTypeContentDeserializerState::Op(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(ListTypeContentDeserializerState::Op(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_op(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_op(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        ListTypeContentDeserializerState::Op(values, None),
                    )?;
                    *self.state = ListTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = ListTypeContentDeserializerState::Op(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_unop<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::UnopType>,
            output: DeserializerOutput<'de, super::UnopType>,
            fallback: &mut Option<ListTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => ListTypeContentDeserializerState::Unop(values, None),
                    Some(ListTypeContentDeserializerState::Unop(_, Some(deserializer))) => {
                        ListTypeContentDeserializerState::Unop(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(ListTypeContentDeserializerState::Unop(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_unop(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_unop(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        ListTypeContentDeserializerState::Unop(values, None),
                    )?;
                    *self.state = ListTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        ListTypeContentDeserializerState::Unop(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_fieldref<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ListTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => ListTypeContentDeserializerState::Fieldref(values, None),
                    Some(ListTypeContentDeserializerState::Fieldref(_, Some(deserializer))) => {
                        ListTypeContentDeserializerState::Fieldref(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(ListTypeContentDeserializerState::Fieldref(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_fieldref(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fieldref(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        ListTypeContentDeserializerState::Fieldref(values, None),
                    )?;
                    *self.state = ListTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        ListTypeContentDeserializerState::Fieldref(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_enumref<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::EnumrefType>,
            output: DeserializerOutput<'de, super::EnumrefType>,
            fallback: &mut Option<ListTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => ListTypeContentDeserializerState::Enumref(values, None),
                    Some(ListTypeContentDeserializerState::Enumref(_, Some(deserializer))) => {
                        ListTypeContentDeserializerState::Enumref(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(ListTypeContentDeserializerState::Enumref(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_enumref(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_enumref(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        ListTypeContentDeserializerState::Enumref(values, None),
                    )?;
                    *self.state = ListTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        ListTypeContentDeserializerState::Enumref(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_popcount<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PopcountType>,
            output: DeserializerOutput<'de, super::PopcountType>,
            fallback: &mut Option<ListTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => ListTypeContentDeserializerState::Popcount(values, None),
                    Some(ListTypeContentDeserializerState::Popcount(_, Some(deserializer))) => {
                        ListTypeContentDeserializerState::Popcount(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(ListTypeContentDeserializerState::Popcount(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_popcount(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_popcount(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        ListTypeContentDeserializerState::Popcount(values, None),
                    )?;
                    *self.state = ListTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        ListTypeContentDeserializerState::Popcount(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_sumof<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SumofType>,
            output: DeserializerOutput<'de, super::SumofType>,
            fallback: &mut Option<ListTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => ListTypeContentDeserializerState::Sumof(values, None),
                    Some(ListTypeContentDeserializerState::Sumof(_, Some(deserializer))) => {
                        ListTypeContentDeserializerState::Sumof(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(ListTypeContentDeserializerState::Sumof(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_sumof(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sumof(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        ListTypeContentDeserializerState::Sumof(values, None),
                    )?;
                    *self.state = ListTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        ListTypeContentDeserializerState::Sumof(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_value<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::DecOrHexIntegerType>,
            output: DeserializerOutput<'de, super::DecOrHexIntegerType>,
            fallback: &mut Option<ListTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => ListTypeContentDeserializerState::Value(values, None),
                    Some(ListTypeContentDeserializerState::Value(_, Some(deserializer))) => {
                        ListTypeContentDeserializerState::Value(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(ListTypeContentDeserializerState::Value(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_value(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_value(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        ListTypeContentDeserializerState::Value(values, None),
                    )?;
                    *self.state = ListTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        ListTypeContentDeserializerState::Value(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_bit<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<ListTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => ListTypeContentDeserializerState::Bit(values, None),
                    Some(ListTypeContentDeserializerState::Bit(_, Some(deserializer))) => {
                        ListTypeContentDeserializerState::Bit(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(ListTypeContentDeserializerState::Bit(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_bit(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bit(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        ListTypeContentDeserializerState::Bit(values, None),
                    )?;
                    *self.state = ListTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = ListTypeContentDeserializerState::Bit(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ListTypeContent> for ListTypeContentDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ListTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(ListTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, ListTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ListTypeContent>
        where
            R: DeserializeReader,
        {
            use ListTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Op(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_op(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_unop(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fieldref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_enumref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_popcount(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_sumof(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_bit(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::Op(values, None), event) => {
                        let output =
                            <super::OpType as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_op(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, None), event) => {
                        let output = <super::UnopType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_unop(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_fieldref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, None), event) => {
                        let output = <super::EnumrefType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_enumref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, None), event) => {
                        let output = <super::PopcountType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_popcount(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, None), event) => {
                        let output = <super::SumofType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_sumof(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, None), event) => {
                        let output =
                            <super::DecOrHexIntegerType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, None), event) => {
                        let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_bit(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::ListTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct AnyTypeDeserializer {
        state: Box<AnyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AnyTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl AnyTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            Ok(Self {
                state: Box::new(AnyTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: AnyTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::AnyType> for AnyTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::AnyType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnyType>
        where
            R: DeserializeReader,
        {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(reader)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::Break(event),
                    allow_any: true,
                })
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::AnyType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, AnyTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
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
        state: Box<ExprfieldTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ExprfieldTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::ExprfieldTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ExprfieldTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<String> = None;
            let mut type_: Option<String> = None;
            let mut enum_: Option<String> = None;
            let mut altenum: Option<String> = None;
            let mut mask: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"type" {
                    reader.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"enum" {
                    reader.read_attrib(&mut enum_, b"enum", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"altenum" {
                    reader.read_attrib(&mut altenum, b"altenum", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"mask" {
                    reader.read_attrib(&mut mask, b"mask", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                name: name
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("name".into())))?,
                type_: type_
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("type".into())))?,
                enum_: enum_,
                altenum: altenum,
                mask: mask,
                content: None,
                state: Box::new(ExprfieldTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ExprfieldTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let ExprfieldTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
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
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ExprfieldTypeContent>,
            fallback: &mut Option<ExprfieldTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(ExprfieldTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = ExprfieldTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = ExprfieldTypeDeserializerState::Content__(deserializer);
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ExprfieldType> for ExprfieldTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ExprfieldType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExprfieldType>
        where
            R: DeserializeReader,
        {
            use ExprfieldTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::ExprfieldTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ExprfieldType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, ExprfieldTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::ExprfieldType {
                name: self.name,
                type_: self.type_,
                enum_: self.enum_,
                altenum: self.altenum,
                mask: self.mask,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ExprfieldTypeContentDeserializer {
        state: Box<ExprfieldTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ExprfieldTypeContentDeserializerState {
        Init__,
        Op(
            Option<super::OpType>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
        ),
        Unop(
            Option<super::UnopType>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
        ),
        Fieldref(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Enumref(
            Option<super::EnumrefType>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
        ),
        Popcount(
            Option<super::PopcountType>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
        ),
        Sumof(
            Option<super::SumofType>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
        ),
        Value(
            Option<super::DecOrHexIntegerType>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
        ),
        Bit(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Done__(super::ExprfieldTypeContent),
        Unknown__,
    }
    impl ExprfieldTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<ExprfieldTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"op" {
                    let output =
                        <super::OpType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_op(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"unop" {
                    let output =
                        <super::UnopType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_unop(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"fieldref" {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_fieldref(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"enumref" {
                    let output = <super::EnumrefType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_enumref(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"popcount" {
                    let output = <super::PopcountType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_popcount(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"sumof" {
                    let output =
                        <super::SumofType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_sumof(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"value" {
                    let output =
                        <super::DecOrHexIntegerType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    return self.handle_value(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"bit" {
                    let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_bit(reader, Default::default(), output, &mut *fallback);
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(ExprfieldTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: ExprfieldTypeContentDeserializerState,
        ) -> Result<super::ExprfieldTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use ExprfieldTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Op(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_op(&mut values, value)?;
                    }
                    Ok(super::ExprfieldTypeContent::Op(
                        values.ok_or_else(|| ErrorKind::MissingElement("op".into()))?,
                    ))
                }
                S::Unop(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_unop(&mut values, value)?;
                    }
                    Ok(super::ExprfieldTypeContent::Unop(
                        values.ok_or_else(|| ErrorKind::MissingElement("unop".into()))?,
                    ))
                }
                S::Fieldref(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_fieldref(&mut values, value)?;
                    }
                    Ok(super::ExprfieldTypeContent::Fieldref(values.ok_or_else(
                        || ErrorKind::MissingElement("fieldref".into()),
                    )?))
                }
                S::Enumref(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_enumref(&mut values, value)?;
                    }
                    Ok(super::ExprfieldTypeContent::Enumref(values.ok_or_else(
                        || ErrorKind::MissingElement("enumref".into()),
                    )?))
                }
                S::Popcount(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_popcount(&mut values, value)?;
                    }
                    Ok(super::ExprfieldTypeContent::Popcount(values.ok_or_else(
                        || ErrorKind::MissingElement("popcount".into()),
                    )?))
                }
                S::Sumof(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_sumof(&mut values, value)?;
                    }
                    Ok(super::ExprfieldTypeContent::Sumof(values.ok_or_else(
                        || ErrorKind::MissingElement("sumof".into()),
                    )?))
                }
                S::Value(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_value(&mut values, value)?;
                    }
                    Ok(super::ExprfieldTypeContent::Value(values.ok_or_else(
                        || ErrorKind::MissingElement("value".into()),
                    )?))
                }
                S::Bit(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_bit(&mut values, value)?;
                    }
                    Ok(super::ExprfieldTypeContent::Bit(
                        values.ok_or_else(|| ErrorKind::MissingElement("bit".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_op<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::OpType>,
            output: DeserializerOutput<'de, super::OpType>,
            fallback: &mut Option<ExprfieldTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => ExprfieldTypeContentDeserializerState::Op(values, None),
                    Some(ExprfieldTypeContentDeserializerState::Op(_, Some(deserializer))) => {
                        ExprfieldTypeContentDeserializerState::Op(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(ExprfieldTypeContentDeserializerState::Op(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_op(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_op(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        ExprfieldTypeContentDeserializerState::Op(values, None),
                    )?;
                    *self.state = ExprfieldTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        ExprfieldTypeContentDeserializerState::Op(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_unop<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::UnopType>,
            output: DeserializerOutput<'de, super::UnopType>,
            fallback: &mut Option<ExprfieldTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => ExprfieldTypeContentDeserializerState::Unop(values, None),
                    Some(ExprfieldTypeContentDeserializerState::Unop(_, Some(deserializer))) => {
                        ExprfieldTypeContentDeserializerState::Unop(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(ExprfieldTypeContentDeserializerState::Unop(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_unop(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_unop(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        ExprfieldTypeContentDeserializerState::Unop(values, None),
                    )?;
                    *self.state = ExprfieldTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        ExprfieldTypeContentDeserializerState::Unop(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_fieldref<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ExprfieldTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => ExprfieldTypeContentDeserializerState::Fieldref(values, None),
                    Some(ExprfieldTypeContentDeserializerState::Fieldref(
                        _,
                        Some(deserializer),
                    )) => {
                        ExprfieldTypeContentDeserializerState::Fieldref(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(ExprfieldTypeContentDeserializerState::Fieldref(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_fieldref(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fieldref(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        ExprfieldTypeContentDeserializerState::Fieldref(values, None),
                    )?;
                    *self.state = ExprfieldTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        ExprfieldTypeContentDeserializerState::Fieldref(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_enumref<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::EnumrefType>,
            output: DeserializerOutput<'de, super::EnumrefType>,
            fallback: &mut Option<ExprfieldTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => ExprfieldTypeContentDeserializerState::Enumref(values, None),
                    Some(ExprfieldTypeContentDeserializerState::Enumref(_, Some(deserializer))) => {
                        ExprfieldTypeContentDeserializerState::Enumref(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(ExprfieldTypeContentDeserializerState::Enumref(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_enumref(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_enumref(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        ExprfieldTypeContentDeserializerState::Enumref(values, None),
                    )?;
                    *self.state = ExprfieldTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        ExprfieldTypeContentDeserializerState::Enumref(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_popcount<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PopcountType>,
            output: DeserializerOutput<'de, super::PopcountType>,
            fallback: &mut Option<ExprfieldTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => ExprfieldTypeContentDeserializerState::Popcount(values, None),
                    Some(ExprfieldTypeContentDeserializerState::Popcount(
                        _,
                        Some(deserializer),
                    )) => {
                        ExprfieldTypeContentDeserializerState::Popcount(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(ExprfieldTypeContentDeserializerState::Popcount(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_popcount(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_popcount(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        ExprfieldTypeContentDeserializerState::Popcount(values, None),
                    )?;
                    *self.state = ExprfieldTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        ExprfieldTypeContentDeserializerState::Popcount(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_sumof<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SumofType>,
            output: DeserializerOutput<'de, super::SumofType>,
            fallback: &mut Option<ExprfieldTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => ExprfieldTypeContentDeserializerState::Sumof(values, None),
                    Some(ExprfieldTypeContentDeserializerState::Sumof(_, Some(deserializer))) => {
                        ExprfieldTypeContentDeserializerState::Sumof(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(ExprfieldTypeContentDeserializerState::Sumof(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_sumof(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sumof(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        ExprfieldTypeContentDeserializerState::Sumof(values, None),
                    )?;
                    *self.state = ExprfieldTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        ExprfieldTypeContentDeserializerState::Sumof(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_value<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::DecOrHexIntegerType>,
            output: DeserializerOutput<'de, super::DecOrHexIntegerType>,
            fallback: &mut Option<ExprfieldTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => ExprfieldTypeContentDeserializerState::Value(values, None),
                    Some(ExprfieldTypeContentDeserializerState::Value(_, Some(deserializer))) => {
                        ExprfieldTypeContentDeserializerState::Value(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(ExprfieldTypeContentDeserializerState::Value(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_value(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_value(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        ExprfieldTypeContentDeserializerState::Value(values, None),
                    )?;
                    *self.state = ExprfieldTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        ExprfieldTypeContentDeserializerState::Value(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_bit<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<ExprfieldTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => ExprfieldTypeContentDeserializerState::Bit(values, None),
                    Some(ExprfieldTypeContentDeserializerState::Bit(_, Some(deserializer))) => {
                        ExprfieldTypeContentDeserializerState::Bit(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(ExprfieldTypeContentDeserializerState::Bit(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_bit(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bit(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        ExprfieldTypeContentDeserializerState::Bit(values, None),
                    )?;
                    *self.state = ExprfieldTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        ExprfieldTypeContentDeserializerState::Bit(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ExprfieldTypeContent> for ExprfieldTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExprfieldTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(ExprfieldTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, ExprfieldTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExprfieldTypeContent>
        where
            R: DeserializeReader,
        {
            use ExprfieldTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Op(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_op(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_unop(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fieldref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_enumref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_popcount(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_sumof(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_bit(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::Op(values, None), event) => {
                        let output =
                            <super::OpType as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_op(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, None), event) => {
                        let output = <super::UnopType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_unop(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_fieldref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, None), event) => {
                        let output = <super::EnumrefType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_enumref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, None), event) => {
                        let output = <super::PopcountType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_popcount(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, None), event) => {
                        let output = <super::SumofType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_sumof(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, None), event) => {
                        let output =
                            <super::DecOrHexIntegerType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, None), event) => {
                        let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_bit(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::ExprfieldTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct ValueparamTypeDeserializer {
        value_mask_type: String,
        value_mask_name: String,
        value_list_name: String,
        state: Box<ValueparamTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ValueparamTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl ValueparamTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut value_mask_type: Option<String> = None;
            let mut value_mask_name: Option<String> = None;
            let mut value_list_name: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"value-mask-type" {
                    reader.read_attrib(&mut value_mask_type, b"value-mask-type", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"value-mask-name" {
                    reader.read_attrib(&mut value_mask_name, b"value-mask-name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"value-list-name" {
                    reader.read_attrib(&mut value_list_name, b"value-list-name", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                value_mask_type: value_mask_type.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("value-mask-type".into()))
                })?,
                value_mask_name: value_mask_name.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("value-mask-name".into()))
                })?,
                value_list_name: value_list_name.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("value-list-name".into()))
                })?,
                state: Box::new(ValueparamTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ValueparamTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::ValueparamType> for ValueparamTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ValueparamType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ValueparamType>
        where
            R: DeserializeReader,
        {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(reader)?),
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
        fn finish<R>(mut self, reader: &R) -> Result<super::ValueparamType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, ValueparamTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
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
        state: Box<SwitchexprTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SwitchexprTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::SwitchexprTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl SwitchexprTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                name: name
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("name".into())))?,
                content: Vec::new(),
                state: Box::new(SwitchexprTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SwitchexprTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let SwitchexprTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::SwitchexprTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SwitchexprTypeContent>,
            fallback: &mut Option<SwitchexprTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(SwitchexprTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = SwitchexprTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let can_have_more = self.content.len().saturating_add(1) < 13usize;
                    let ret = if can_have_more {
                        ElementHandlerOutput::from_event(event, allow_any)
                    } else {
                        ElementHandlerOutput::from_event_end(event, allow_any)
                    };
                    match (can_have_more, &ret) {
                        (true, ElementHandlerOutput::Continue { .. }) => {
                            fallback.get_or_insert(SwitchexprTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = SwitchexprTypeDeserializerState::Next__;
                        }
                        (false, _) | (_, ElementHandlerOutput::Break { .. }) => {
                            *self.state = SwitchexprTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SwitchexprType> for SwitchexprTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::SwitchexprType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SwitchexprType>
        where
            R: DeserializeReader,
        {
            use SwitchexprTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::SwitchexprTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::SwitchexprType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, SwitchexprTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::SwitchexprType {
                name: self.name,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct SwitchexprTypeContentDeserializer {
        state: Box<SwitchexprTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum SwitchexprTypeContentDeserializerState {
        Init__,
        Op(
            Option<super::OpType>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
        ),
        Unop(
            Option<super::UnopType>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
        ),
        Fieldref(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Enumref(
            Option<super::EnumrefType>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
        ),
        Popcount(
            Option<super::PopcountType>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
        ),
        Sumof(
            Option<super::SumofType>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
        ),
        Value(
            Option<super::DecOrHexIntegerType>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
        ),
        Bit(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Bitcase(
            Option<super::CaseexprType>,
            Option<<super::CaseexprType as WithDeserializer>::Deserializer>,
        ),
        Pad(
            Option<super::PadType>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
        ),
        Field(
            Option<super::VarType>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
        ),
        List(
            Option<super::ListType>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
        ),
        Fd(
            Option<super::AnyType>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::SwitchexprTypeContent),
        Unknown__,
    }
    impl SwitchexprTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<SwitchexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"op" {
                    let output =
                        <super::OpType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_op(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"unop" {
                    let output =
                        <super::UnopType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_unop(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"fieldref" {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_fieldref(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"enumref" {
                    let output = <super::EnumrefType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_enumref(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"popcount" {
                    let output = <super::PopcountType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_popcount(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"sumof" {
                    let output =
                        <super::SumofType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_sumof(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"value" {
                    let output =
                        <super::DecOrHexIntegerType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    return self.handle_value(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"bit" {
                    let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_bit(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"bitcase" {
                    let output = <super::CaseexprType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_bitcase(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"pad" {
                    let output =
                        <super::PadType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_pad(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"field" {
                    let output =
                        <super::VarType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_field(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"list" {
                    let output =
                        <super::ListType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_list(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"fd" {
                    let output =
                        <super::AnyType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_fd(reader, Default::default(), output, &mut *fallback);
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(SwitchexprTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: SwitchexprTypeContentDeserializerState,
        ) -> Result<super::SwitchexprTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use SwitchexprTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Op(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_op(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Op(
                        values.ok_or_else(|| ErrorKind::MissingElement("op".into()))?,
                    ))
                }
                S::Unop(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_unop(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Unop(
                        values.ok_or_else(|| ErrorKind::MissingElement("unop".into()))?,
                    ))
                }
                S::Fieldref(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_fieldref(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Fieldref(values.ok_or_else(
                        || ErrorKind::MissingElement("fieldref".into()),
                    )?))
                }
                S::Enumref(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_enumref(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Enumref(values.ok_or_else(
                        || ErrorKind::MissingElement("enumref".into()),
                    )?))
                }
                S::Popcount(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_popcount(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Popcount(values.ok_or_else(
                        || ErrorKind::MissingElement("popcount".into()),
                    )?))
                }
                S::Sumof(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_sumof(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Sumof(values.ok_or_else(
                        || ErrorKind::MissingElement("sumof".into()),
                    )?))
                }
                S::Value(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_value(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Value(values.ok_or_else(
                        || ErrorKind::MissingElement("value".into()),
                    )?))
                }
                S::Bit(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_bit(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Bit(
                        values.ok_or_else(|| ErrorKind::MissingElement("bit".into()))?,
                    ))
                }
                S::Bitcase(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_bitcase(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Bitcase(values.ok_or_else(
                        || ErrorKind::MissingElement("bitcase".into()),
                    )?))
                }
                S::Pad(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_pad(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Pad(
                        values.ok_or_else(|| ErrorKind::MissingElement("pad".into()))?,
                    ))
                }
                S::Field(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_field(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Field(values.ok_or_else(
                        || ErrorKind::MissingElement("field".into()),
                    )?))
                }
                S::List(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_list(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::List(
                        values.ok_or_else(|| ErrorKind::MissingElement("list".into()))?,
                    ))
                }
                S::Fd(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_fd(&mut values, value)?;
                    }
                    Ok(super::SwitchexprTypeContent::Fd(
                        values.ok_or_else(|| ErrorKind::MissingElement("fd".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_op<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::OpType>,
            output: DeserializerOutput<'de, super::OpType>,
            fallback: &mut Option<SwitchexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => SwitchexprTypeContentDeserializerState::Op(values, None),
                    Some(SwitchexprTypeContentDeserializerState::Op(_, Some(deserializer))) => {
                        SwitchexprTypeContentDeserializerState::Op(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(SwitchexprTypeContentDeserializerState::Op(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_op(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_op(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        SwitchexprTypeContentDeserializerState::Op(values, None),
                    )?;
                    *self.state = SwitchexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        SwitchexprTypeContentDeserializerState::Op(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_unop<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::UnopType>,
            output: DeserializerOutput<'de, super::UnopType>,
            fallback: &mut Option<SwitchexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => SwitchexprTypeContentDeserializerState::Unop(values, None),
                    Some(SwitchexprTypeContentDeserializerState::Unop(_, Some(deserializer))) => {
                        SwitchexprTypeContentDeserializerState::Unop(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(SwitchexprTypeContentDeserializerState::Unop(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_unop(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_unop(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        SwitchexprTypeContentDeserializerState::Unop(values, None),
                    )?;
                    *self.state = SwitchexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        SwitchexprTypeContentDeserializerState::Unop(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_fieldref<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<SwitchexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => SwitchexprTypeContentDeserializerState::Fieldref(values, None),
                    Some(SwitchexprTypeContentDeserializerState::Fieldref(
                        _,
                        Some(deserializer),
                    )) => {
                        SwitchexprTypeContentDeserializerState::Fieldref(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(SwitchexprTypeContentDeserializerState::Fieldref(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_fieldref(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fieldref(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        SwitchexprTypeContentDeserializerState::Fieldref(values, None),
                    )?;
                    *self.state = SwitchexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = SwitchexprTypeContentDeserializerState::Fieldref(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_enumref<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::EnumrefType>,
            output: DeserializerOutput<'de, super::EnumrefType>,
            fallback: &mut Option<SwitchexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => SwitchexprTypeContentDeserializerState::Enumref(values, None),
                    Some(SwitchexprTypeContentDeserializerState::Enumref(
                        _,
                        Some(deserializer),
                    )) => {
                        SwitchexprTypeContentDeserializerState::Enumref(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(SwitchexprTypeContentDeserializerState::Enumref(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_enumref(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_enumref(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        SwitchexprTypeContentDeserializerState::Enumref(values, None),
                    )?;
                    *self.state = SwitchexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        SwitchexprTypeContentDeserializerState::Enumref(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_popcount<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PopcountType>,
            output: DeserializerOutput<'de, super::PopcountType>,
            fallback: &mut Option<SwitchexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => SwitchexprTypeContentDeserializerState::Popcount(values, None),
                    Some(SwitchexprTypeContentDeserializerState::Popcount(
                        _,
                        Some(deserializer),
                    )) => {
                        SwitchexprTypeContentDeserializerState::Popcount(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(SwitchexprTypeContentDeserializerState::Popcount(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_popcount(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_popcount(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        SwitchexprTypeContentDeserializerState::Popcount(values, None),
                    )?;
                    *self.state = SwitchexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = SwitchexprTypeContentDeserializerState::Popcount(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_sumof<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SumofType>,
            output: DeserializerOutput<'de, super::SumofType>,
            fallback: &mut Option<SwitchexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => SwitchexprTypeContentDeserializerState::Sumof(values, None),
                    Some(SwitchexprTypeContentDeserializerState::Sumof(_, Some(deserializer))) => {
                        SwitchexprTypeContentDeserializerState::Sumof(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(SwitchexprTypeContentDeserializerState::Sumof(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_sumof(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sumof(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        SwitchexprTypeContentDeserializerState::Sumof(values, None),
                    )?;
                    *self.state = SwitchexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        SwitchexprTypeContentDeserializerState::Sumof(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_value<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::DecOrHexIntegerType>,
            output: DeserializerOutput<'de, super::DecOrHexIntegerType>,
            fallback: &mut Option<SwitchexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => SwitchexprTypeContentDeserializerState::Value(values, None),
                    Some(SwitchexprTypeContentDeserializerState::Value(_, Some(deserializer))) => {
                        SwitchexprTypeContentDeserializerState::Value(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(SwitchexprTypeContentDeserializerState::Value(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_value(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_value(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        SwitchexprTypeContentDeserializerState::Value(values, None),
                    )?;
                    *self.state = SwitchexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        SwitchexprTypeContentDeserializerState::Value(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_bit<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<SwitchexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => SwitchexprTypeContentDeserializerState::Bit(values, None),
                    Some(SwitchexprTypeContentDeserializerState::Bit(_, Some(deserializer))) => {
                        SwitchexprTypeContentDeserializerState::Bit(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(SwitchexprTypeContentDeserializerState::Bit(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_bit(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bit(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        SwitchexprTypeContentDeserializerState::Bit(values, None),
                    )?;
                    *self.state = SwitchexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        SwitchexprTypeContentDeserializerState::Bit(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_bitcase<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::CaseexprType>,
            output: DeserializerOutput<'de, super::CaseexprType>,
            fallback: &mut Option<SwitchexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => SwitchexprTypeContentDeserializerState::Bitcase(values, None),
                    Some(SwitchexprTypeContentDeserializerState::Bitcase(
                        _,
                        Some(deserializer),
                    )) => {
                        SwitchexprTypeContentDeserializerState::Bitcase(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(SwitchexprTypeContentDeserializerState::Bitcase(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_bitcase(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bitcase(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        SwitchexprTypeContentDeserializerState::Bitcase(values, None),
                    )?;
                    *self.state = SwitchexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        SwitchexprTypeContentDeserializerState::Bitcase(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_pad<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PadType>,
            output: DeserializerOutput<'de, super::PadType>,
            fallback: &mut Option<SwitchexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => SwitchexprTypeContentDeserializerState::Pad(values, None),
                    Some(SwitchexprTypeContentDeserializerState::Pad(_, Some(deserializer))) => {
                        SwitchexprTypeContentDeserializerState::Pad(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(SwitchexprTypeContentDeserializerState::Pad(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_pad(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pad(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        SwitchexprTypeContentDeserializerState::Pad(values, None),
                    )?;
                    *self.state = SwitchexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        SwitchexprTypeContentDeserializerState::Pad(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_field<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::VarType>,
            output: DeserializerOutput<'de, super::VarType>,
            fallback: &mut Option<SwitchexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => SwitchexprTypeContentDeserializerState::Field(values, None),
                    Some(SwitchexprTypeContentDeserializerState::Field(_, Some(deserializer))) => {
                        SwitchexprTypeContentDeserializerState::Field(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(SwitchexprTypeContentDeserializerState::Field(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_field(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_field(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        SwitchexprTypeContentDeserializerState::Field(values, None),
                    )?;
                    *self.state = SwitchexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        SwitchexprTypeContentDeserializerState::Field(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_list<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ListType>,
            output: DeserializerOutput<'de, super::ListType>,
            fallback: &mut Option<SwitchexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => SwitchexprTypeContentDeserializerState::List(values, None),
                    Some(SwitchexprTypeContentDeserializerState::List(_, Some(deserializer))) => {
                        SwitchexprTypeContentDeserializerState::List(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(SwitchexprTypeContentDeserializerState::List(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_list(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_list(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        SwitchexprTypeContentDeserializerState::List(values, None),
                    )?;
                    *self.state = SwitchexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        SwitchexprTypeContentDeserializerState::List(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_fd<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnyType>,
            output: DeserializerOutput<'de, super::AnyType>,
            fallback: &mut Option<SwitchexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => SwitchexprTypeContentDeserializerState::Fd(values, None),
                    Some(SwitchexprTypeContentDeserializerState::Fd(_, Some(deserializer))) => {
                        SwitchexprTypeContentDeserializerState::Fd(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(SwitchexprTypeContentDeserializerState::Fd(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_fd(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fd(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        SwitchexprTypeContentDeserializerState::Fd(values, None),
                    )?;
                    *self.state = SwitchexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        SwitchexprTypeContentDeserializerState::Fd(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SwitchexprTypeContent> for SwitchexprTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SwitchexprTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(SwitchexprTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, SwitchexprTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SwitchexprTypeContent>
        where
            R: DeserializeReader,
        {
            use SwitchexprTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Op(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_op(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_unop(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fieldref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_enumref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_popcount(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_sumof(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_bit(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bitcase(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_bitcase(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Pad(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pad(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_field(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_list(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fd(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::Op(values, None), event) => {
                        let output =
                            <super::OpType as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_op(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, None), event) => {
                        let output = <super::UnopType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_unop(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_fieldref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, None), event) => {
                        let output = <super::EnumrefType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_enumref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, None), event) => {
                        let output = <super::PopcountType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_popcount(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, None), event) => {
                        let output = <super::SumofType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_sumof(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, None), event) => {
                        let output =
                            <super::DecOrHexIntegerType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, None), event) => {
                        let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_bit(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bitcase(values, None), event) => {
                        let output = <super::CaseexprType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_bitcase(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Pad(values, None), event) => {
                        let output = <super::PadType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_pad(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, None), event) => {
                        let output = <super::VarType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_field(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, None), event) => {
                        let output = <super::ListType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_list(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, None), event) => {
                        let output = <super::AnyType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_fd(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::SwitchexprTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct RequestReplyTypeDeserializer {
        content: Vec<super::RequestReplyTypeContent>,
        state: Box<RequestReplyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RequestReplyTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::RequestReplyTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl RequestReplyTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                content: Vec::new(),
                state: Box::new(RequestReplyTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: RequestReplyTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let RequestReplyTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::RequestReplyTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::RequestReplyTypeContent>,
            fallback: &mut Option<RequestReplyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(RequestReplyTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = RequestReplyTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let can_have_more = self.content.len().saturating_add(1) < 7usize;
                    let ret = if can_have_more {
                        ElementHandlerOutput::from_event(event, allow_any)
                    } else {
                        ElementHandlerOutput::from_event_end(event, allow_any)
                    };
                    match (can_have_more, &ret) {
                        (true, ElementHandlerOutput::Continue { .. }) => {
                            fallback.get_or_insert(RequestReplyTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = RequestReplyTypeDeserializerState::Next__;
                        }
                        (false, _) | (_, ElementHandlerOutput::Break { .. }) => {
                            *self.state =
                                RequestReplyTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RequestReplyType> for RequestReplyTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RequestReplyType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RequestReplyType>
        where
            R: DeserializeReader,
        {
            use RequestReplyTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output = < super :: RequestReplyTypeContent as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::RequestReplyType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                RequestReplyTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::RequestReplyType {
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct RequestReplyTypeContentDeserializer {
        state: Box<RequestReplyTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum RequestReplyTypeContentDeserializerState {
        Init__,
        Pad(
            Option<super::PadType>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
        ),
        Field(
            Option<super::VarType>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
        ),
        List(
            Option<super::ListType>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
        ),
        Fd(
            Option<super::AnyType>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
        ),
        Valueparam(
            Option<super::ValueparamType>,
            Option<<super::ValueparamType as WithDeserializer>::Deserializer>,
        ),
        Switch(
            Option<super::SwitchexprType>,
            Option<<super::SwitchexprType as WithDeserializer>::Deserializer>,
        ),
        Doc(
            Option<super::DocType>,
            Option<<super::DocType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::RequestReplyTypeContent),
        Unknown__,
    }
    impl RequestReplyTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<RequestReplyTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"pad" {
                    let output =
                        <super::PadType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_pad(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"field" {
                    let output =
                        <super::VarType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_field(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"list" {
                    let output =
                        <super::ListType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_list(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"fd" {
                    let output =
                        <super::AnyType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_fd(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"valueparam" {
                    let output = <super::ValueparamType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_valueparam(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"switch" {
                    let output = <super::SwitchexprType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_switch(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"doc" {
                    let output =
                        <super::DocType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_doc(reader, Default::default(), output, &mut *fallback);
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(RequestReplyTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: RequestReplyTypeContentDeserializerState,
        ) -> Result<super::RequestReplyTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use RequestReplyTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Pad(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_pad(&mut values, value)?;
                    }
                    Ok(super::RequestReplyTypeContent::Pad(
                        values.ok_or_else(|| ErrorKind::MissingElement("pad".into()))?,
                    ))
                }
                S::Field(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_field(&mut values, value)?;
                    }
                    Ok(super::RequestReplyTypeContent::Field(values.ok_or_else(
                        || ErrorKind::MissingElement("field".into()),
                    )?))
                }
                S::List(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_list(&mut values, value)?;
                    }
                    Ok(super::RequestReplyTypeContent::List(
                        values.ok_or_else(|| ErrorKind::MissingElement("list".into()))?,
                    ))
                }
                S::Fd(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_fd(&mut values, value)?;
                    }
                    Ok(super::RequestReplyTypeContent::Fd(
                        values.ok_or_else(|| ErrorKind::MissingElement("fd".into()))?,
                    ))
                }
                S::Valueparam(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_valueparam(&mut values, value)?;
                    }
                    Ok(super::RequestReplyTypeContent::Valueparam(
                        values.ok_or_else(|| ErrorKind::MissingElement("valueparam".into()))?,
                    ))
                }
                S::Switch(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_switch(&mut values, value)?;
                    }
                    Ok(super::RequestReplyTypeContent::Switch(values.ok_or_else(
                        || ErrorKind::MissingElement("switch".into()),
                    )?))
                }
                S::Doc(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_doc(&mut values, value)?;
                    }
                    Ok(super::RequestReplyTypeContent::Doc(
                        values.ok_or_else(|| ErrorKind::MissingElement("doc".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_pad<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PadType>,
            output: DeserializerOutput<'de, super::PadType>,
            fallback: &mut Option<RequestReplyTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => RequestReplyTypeContentDeserializerState::Pad(values, None),
                    Some(RequestReplyTypeContentDeserializerState::Pad(_, Some(deserializer))) => {
                        RequestReplyTypeContentDeserializerState::Pad(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(RequestReplyTypeContentDeserializerState::Pad(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_pad(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pad(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        RequestReplyTypeContentDeserializerState::Pad(values, None),
                    )?;
                    *self.state = RequestReplyTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        RequestReplyTypeContentDeserializerState::Pad(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_field<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::VarType>,
            output: DeserializerOutput<'de, super::VarType>,
            fallback: &mut Option<RequestReplyTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => RequestReplyTypeContentDeserializerState::Field(values, None),
                    Some(RequestReplyTypeContentDeserializerState::Field(
                        _,
                        Some(deserializer),
                    )) => {
                        RequestReplyTypeContentDeserializerState::Field(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(RequestReplyTypeContentDeserializerState::Field(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_field(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_field(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        RequestReplyTypeContentDeserializerState::Field(values, None),
                    )?;
                    *self.state = RequestReplyTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        RequestReplyTypeContentDeserializerState::Field(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_list<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ListType>,
            output: DeserializerOutput<'de, super::ListType>,
            fallback: &mut Option<RequestReplyTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => RequestReplyTypeContentDeserializerState::List(values, None),
                    Some(RequestReplyTypeContentDeserializerState::List(_, Some(deserializer))) => {
                        RequestReplyTypeContentDeserializerState::List(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(RequestReplyTypeContentDeserializerState::List(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_list(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_list(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        RequestReplyTypeContentDeserializerState::List(values, None),
                    )?;
                    *self.state = RequestReplyTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        RequestReplyTypeContentDeserializerState::List(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_fd<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnyType>,
            output: DeserializerOutput<'de, super::AnyType>,
            fallback: &mut Option<RequestReplyTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => RequestReplyTypeContentDeserializerState::Fd(values, None),
                    Some(RequestReplyTypeContentDeserializerState::Fd(_, Some(deserializer))) => {
                        RequestReplyTypeContentDeserializerState::Fd(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(RequestReplyTypeContentDeserializerState::Fd(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_fd(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fd(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        RequestReplyTypeContentDeserializerState::Fd(values, None),
                    )?;
                    *self.state = RequestReplyTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        RequestReplyTypeContentDeserializerState::Fd(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_valueparam<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ValueparamType>,
            output: DeserializerOutput<'de, super::ValueparamType>,
            fallback: &mut Option<RequestReplyTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => RequestReplyTypeContentDeserializerState::Valueparam(values, None),
                    Some(RequestReplyTypeContentDeserializerState::Valueparam(
                        _,
                        Some(deserializer),
                    )) => RequestReplyTypeContentDeserializerState::Valueparam(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(RequestReplyTypeContentDeserializerState::Valueparam(
                    _,
                    Some(deserializer),
                )) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_valueparam(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_valueparam(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        RequestReplyTypeContentDeserializerState::Valueparam(values, None),
                    )?;
                    *self.state = RequestReplyTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = RequestReplyTypeContentDeserializerState::Valueparam(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_switch<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SwitchexprType>,
            output: DeserializerOutput<'de, super::SwitchexprType>,
            fallback: &mut Option<RequestReplyTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => RequestReplyTypeContentDeserializerState::Switch(values, None),
                    Some(RequestReplyTypeContentDeserializerState::Switch(
                        _,
                        Some(deserializer),
                    )) => {
                        RequestReplyTypeContentDeserializerState::Switch(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(RequestReplyTypeContentDeserializerState::Switch(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_switch(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_switch(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        RequestReplyTypeContentDeserializerState::Switch(values, None),
                    )?;
                    *self.state = RequestReplyTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = RequestReplyTypeContentDeserializerState::Switch(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_doc<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::DocType>,
            output: DeserializerOutput<'de, super::DocType>,
            fallback: &mut Option<RequestReplyTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => RequestReplyTypeContentDeserializerState::Doc(values, None),
                    Some(RequestReplyTypeContentDeserializerState::Doc(_, Some(deserializer))) => {
                        RequestReplyTypeContentDeserializerState::Doc(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(RequestReplyTypeContentDeserializerState::Doc(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_doc(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_doc(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        RequestReplyTypeContentDeserializerState::Doc(values, None),
                    )?;
                    *self.state = RequestReplyTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        RequestReplyTypeContentDeserializerState::Doc(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RequestReplyTypeContent>
        for RequestReplyTypeContentDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RequestReplyTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(RequestReplyTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, RequestReplyTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RequestReplyTypeContent>
        where
            R: DeserializeReader,
        {
            use RequestReplyTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Pad(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pad(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_field(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_list(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fd(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Valueparam(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_valueparam(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Switch(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_switch(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Doc(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_doc(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::Pad(values, None), event) => {
                        let output = <super::PadType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_pad(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, None), event) => {
                        let output = <super::VarType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_field(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, None), event) => {
                        let output = <super::ListType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_list(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, None), event) => {
                        let output = <super::AnyType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_fd(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Valueparam(values, None), event) => {
                        let output =
                            <super::ValueparamType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_valueparam(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Switch(values, None), event) => {
                        let output =
                            <super::SwitchexprType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_switch(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Doc(values, None), event) => {
                        let output = <super::DocType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_doc(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::RequestReplyTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct DocTypeDeserializer {
        content: Vec<super::DocTypeContent>,
        state: Box<DocTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DocTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::DocTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl DocTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                content: Vec::new(),
                state: Box::new(DocTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DocTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let DocTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::DocTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DocTypeContent>,
            fallback: &mut Option<DocTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback.take().unwrap_or(DocTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = DocTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DocTypeDeserializerState::Content__(deserializer);
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(DocTypeDeserializerState::Content__(deserializer));
                            *self.state = DocTypeDeserializerState::Next__;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DocType> for DocTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::DocType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocType>
        where
            R: DeserializeReader,
        {
            use DocTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::DocTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::DocType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, DocTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::DocType {
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct DocTypeContentDeserializer {
        state: Box<DocTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum DocTypeContentDeserializerState {
        Init__,
        Brief(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Description(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Example(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Field(
            Option<super::FieldType>,
            Option<<super::FieldType as WithDeserializer>::Deserializer>,
        ),
        Error(
            Option<super::ErrorType>,
            Option<<super::ErrorType as WithDeserializer>::Deserializer>,
        ),
        See(
            Option<super::SeeType>,
            Option<<super::SeeType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::DocTypeContent),
        Unknown__,
    }
    impl DocTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<DocTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"brief" {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_brief(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"description" {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_description(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"example" {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_example(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"field" {
                    let output =
                        <super::FieldType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_field(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"error" {
                    let output =
                        <super::ErrorType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_error(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"see" {
                    let output =
                        <super::SeeType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_see(reader, Default::default(), output, &mut *fallback);
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(DocTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: DocTypeContentDeserializerState,
        ) -> Result<super::DocTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use DocTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Brief(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_brief(&mut values, value)?;
                    }
                    Ok(super::DocTypeContent::Brief(values.ok_or_else(|| {
                        ErrorKind::MissingElement("brief".into())
                    })?))
                }
                S::Description(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_description(&mut values, value)?;
                    }
                    Ok(super::DocTypeContent::Description(values.ok_or_else(
                        || ErrorKind::MissingElement("description".into()),
                    )?))
                }
                S::Example(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_example(&mut values, value)?;
                    }
                    Ok(super::DocTypeContent::Example(values.ok_or_else(|| {
                        ErrorKind::MissingElement("example".into())
                    })?))
                }
                S::Field(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_field(&mut values, value)?;
                    }
                    Ok(super::DocTypeContent::Field(values.ok_or_else(|| {
                        ErrorKind::MissingElement("field".into())
                    })?))
                }
                S::Error(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_error(&mut values, value)?;
                    }
                    Ok(super::DocTypeContent::Error(values.ok_or_else(|| {
                        ErrorKind::MissingElement("error".into())
                    })?))
                }
                S::See(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_see(&mut values, value)?;
                    }
                    Ok(super::DocTypeContent::See(
                        values.ok_or_else(|| ErrorKind::MissingElement("see".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_brief<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DocTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => DocTypeContentDeserializerState::Brief(values, None),
                    Some(DocTypeContentDeserializerState::Brief(_, Some(deserializer))) => {
                        DocTypeContentDeserializerState::Brief(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(DocTypeContentDeserializerState::Brief(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_brief(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_brief(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        DocTypeContentDeserializerState::Brief(values, None),
                    )?;
                    *self.state = DocTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        DocTypeContentDeserializerState::Brief(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_description<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DocTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => DocTypeContentDeserializerState::Description(values, None),
                    Some(DocTypeContentDeserializerState::Description(_, Some(deserializer))) => {
                        DocTypeContentDeserializerState::Description(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(DocTypeContentDeserializerState::Description(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_description(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_description(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        DocTypeContentDeserializerState::Description(values, None),
                    )?;
                    *self.state = DocTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        DocTypeContentDeserializerState::Description(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_example<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DocTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => DocTypeContentDeserializerState::Example(values, None),
                    Some(DocTypeContentDeserializerState::Example(_, Some(deserializer))) => {
                        DocTypeContentDeserializerState::Example(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(DocTypeContentDeserializerState::Example(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_example(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_example(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        DocTypeContentDeserializerState::Example(values, None),
                    )?;
                    *self.state = DocTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        DocTypeContentDeserializerState::Example(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_field<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::FieldType>,
            output: DeserializerOutput<'de, super::FieldType>,
            fallback: &mut Option<DocTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => DocTypeContentDeserializerState::Field(values, None),
                    Some(DocTypeContentDeserializerState::Field(_, Some(deserializer))) => {
                        DocTypeContentDeserializerState::Field(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(DocTypeContentDeserializerState::Field(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_field(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_field(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        DocTypeContentDeserializerState::Field(values, None),
                    )?;
                    *self.state = DocTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        DocTypeContentDeserializerState::Field(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_error<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ErrorType>,
            output: DeserializerOutput<'de, super::ErrorType>,
            fallback: &mut Option<DocTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => DocTypeContentDeserializerState::Error(values, None),
                    Some(DocTypeContentDeserializerState::Error(_, Some(deserializer))) => {
                        DocTypeContentDeserializerState::Error(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(DocTypeContentDeserializerState::Error(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_error(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_error(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        DocTypeContentDeserializerState::Error(values, None),
                    )?;
                    *self.state = DocTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        DocTypeContentDeserializerState::Error(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_see<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SeeType>,
            output: DeserializerOutput<'de, super::SeeType>,
            fallback: &mut Option<DocTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => DocTypeContentDeserializerState::See(values, None),
                    Some(DocTypeContentDeserializerState::See(_, Some(deserializer))) => {
                        DocTypeContentDeserializerState::See(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(DocTypeContentDeserializerState::See(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_see(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_see(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        DocTypeContentDeserializerState::See(values, None),
                    )?;
                    *self.state = DocTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = DocTypeContentDeserializerState::See(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DocTypeContent> for DocTypeContentDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::DocTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(DocTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, DocTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocTypeContent>
        where
            R: DeserializeReader,
        {
            use DocTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Brief(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_brief(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Description(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_description(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Example(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_example(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_field(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Error(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_error(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::See(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_see(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::Brief(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_brief(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Description(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_description(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Example(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_example(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, None), event) => {
                        let output = <super::FieldType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_field(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Error(values, None), event) => {
                        let output = <super::ErrorType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_error(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::See(values, None), event) => {
                        let output = <super::SeeType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_see(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::DocTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct EnumItemTypeDeserializer {
        name: String,
        content: Option<super::EnumItemTypeContent>,
        state: Box<EnumItemTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum EnumItemTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::EnumItemTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl EnumItemTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                name: name
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("name".into())))?,
                content: None,
                state: Box::new(EnumItemTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: EnumItemTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let EnumItemTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
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
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::EnumItemTypeContent>,
            fallback: &mut Option<EnumItemTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(EnumItemTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = EnumItemTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = EnumItemTypeDeserializerState::Content__(deserializer);
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::EnumItemType> for EnumItemTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::EnumItemType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EnumItemType>
        where
            R: DeserializeReader,
        {
            use EnumItemTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::EnumItemTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::EnumItemType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, EnumItemTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::EnumItemType {
                name: self.name,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct EnumItemTypeContentDeserializer {
        state: Box<EnumItemTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum EnumItemTypeContentDeserializerState {
        Init__,
        Value(
            Option<super::DecOrHexIntegerType>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
        ),
        Bit(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Done__(super::EnumItemTypeContent),
        Unknown__,
    }
    impl EnumItemTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<EnumItemTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"value" {
                    let output =
                        <super::DecOrHexIntegerType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    return self.handle_value(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"bit" {
                    let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_bit(reader, Default::default(), output, &mut *fallback);
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(EnumItemTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: EnumItemTypeContentDeserializerState,
        ) -> Result<super::EnumItemTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use EnumItemTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Value(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_value(&mut values, value)?;
                    }
                    Ok(super::EnumItemTypeContent::Value(values.ok_or_else(
                        || ErrorKind::MissingElement("value".into()),
                    )?))
                }
                S::Bit(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_bit(&mut values, value)?;
                    }
                    Ok(super::EnumItemTypeContent::Bit(
                        values.ok_or_else(|| ErrorKind::MissingElement("bit".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_value<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::DecOrHexIntegerType>,
            output: DeserializerOutput<'de, super::DecOrHexIntegerType>,
            fallback: &mut Option<EnumItemTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => EnumItemTypeContentDeserializerState::Value(values, None),
                    Some(EnumItemTypeContentDeserializerState::Value(_, Some(deserializer))) => {
                        EnumItemTypeContentDeserializerState::Value(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(EnumItemTypeContentDeserializerState::Value(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_value(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_value(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        EnumItemTypeContentDeserializerState::Value(values, None),
                    )?;
                    *self.state = EnumItemTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        EnumItemTypeContentDeserializerState::Value(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_bit<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<EnumItemTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => EnumItemTypeContentDeserializerState::Bit(values, None),
                    Some(EnumItemTypeContentDeserializerState::Bit(_, Some(deserializer))) => {
                        EnumItemTypeContentDeserializerState::Bit(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(EnumItemTypeContentDeserializerState::Bit(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_bit(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bit(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        EnumItemTypeContentDeserializerState::Bit(values, None),
                    )?;
                    *self.state = EnumItemTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        EnumItemTypeContentDeserializerState::Bit(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::EnumItemTypeContent> for EnumItemTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EnumItemTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(EnumItemTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, EnumItemTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EnumItemTypeContent>
        where
            R: DeserializeReader,
        {
            use EnumItemTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Value(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_bit(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::Value(values, None), event) => {
                        let output =
                            <super::DecOrHexIntegerType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, None), event) => {
                        let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_bit(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::EnumItemTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct OpTypeDeserializer {
        op: String,
        content: Vec<super::OpTypeContent>,
        state: Box<OpTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum OpTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::OpTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl OpTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut op: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"op" {
                    reader.read_attrib(&mut op, b"op", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                op: op.ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("op".into())))?,
                content: Vec::new(),
                state: Box::new(OpTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: OpTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let OpTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::OpTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::OpTypeContent>,
            fallback: &mut Option<OpTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback.take().unwrap_or(OpTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = OpTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let can_have_more = self.content.len().saturating_add(1) < 8usize;
                    let ret = if can_have_more {
                        ElementHandlerOutput::from_event(event, allow_any)
                    } else {
                        ElementHandlerOutput::from_event_end(event, allow_any)
                    };
                    match (can_have_more, &ret) {
                        (true, ElementHandlerOutput::Continue { .. }) => {
                            fallback
                                .get_or_insert(OpTypeDeserializerState::Content__(deserializer));
                            *self.state = OpTypeDeserializerState::Next__;
                        }
                        (false, _) | (_, ElementHandlerOutput::Break { .. }) => {
                            *self.state = OpTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::OpType> for OpTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::OpType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OpType>
        where
            R: DeserializeReader,
        {
            use OpTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::OpTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::OpType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, OpTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::OpType {
                op: self.op,
                content: self.content.try_into().map_err(|vec: Vec<_>| {
                    ErrorKind::InsufficientSize {
                        min: 8usize,
                        max: 8usize,
                        actual: vec.len(),
                    }
                })?,
            })
        }
    }
    #[derive(Debug)]
    pub struct OpTypeContentDeserializer {
        state: Box<OpTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum OpTypeContentDeserializerState {
        Init__,
        Op(
            Option<super::OpType>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
        ),
        Unop(
            Option<super::UnopType>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
        ),
        Fieldref(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Enumref(
            Option<super::EnumrefType>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
        ),
        Popcount(
            Option<super::PopcountType>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
        ),
        Sumof(
            Option<super::SumofType>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
        ),
        Value(
            Option<super::DecOrHexIntegerType>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
        ),
        Bit(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Done__(super::OpTypeContent),
        Unknown__,
    }
    impl OpTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<OpTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"op" {
                    let output =
                        <super::OpType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_op(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"unop" {
                    let output =
                        <super::UnopType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_unop(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"fieldref" {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_fieldref(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"enumref" {
                    let output = <super::EnumrefType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_enumref(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"popcount" {
                    let output = <super::PopcountType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_popcount(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"sumof" {
                    let output =
                        <super::SumofType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_sumof(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"value" {
                    let output =
                        <super::DecOrHexIntegerType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    return self.handle_value(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"bit" {
                    let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_bit(reader, Default::default(), output, &mut *fallback);
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(OpTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: OpTypeContentDeserializerState,
        ) -> Result<super::OpTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use OpTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Op(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_op(&mut values, value)?;
                    }
                    Ok(super::OpTypeContent::Op(Box::new(
                        values.ok_or_else(|| ErrorKind::MissingElement("op".into()))?,
                    )))
                }
                S::Unop(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_unop(&mut values, value)?;
                    }
                    Ok(super::OpTypeContent::Unop(Box::new(values.ok_or_else(
                        || ErrorKind::MissingElement("unop".into()),
                    )?)))
                }
                S::Fieldref(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_fieldref(&mut values, value)?;
                    }
                    Ok(super::OpTypeContent::Fieldref(values.ok_or_else(|| {
                        ErrorKind::MissingElement("fieldref".into())
                    })?))
                }
                S::Enumref(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_enumref(&mut values, value)?;
                    }
                    Ok(super::OpTypeContent::Enumref(values.ok_or_else(|| {
                        ErrorKind::MissingElement("enumref".into())
                    })?))
                }
                S::Popcount(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_popcount(&mut values, value)?;
                    }
                    Ok(super::OpTypeContent::Popcount(Box::new(
                        values.ok_or_else(|| ErrorKind::MissingElement("popcount".into()))?,
                    )))
                }
                S::Sumof(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_sumof(&mut values, value)?;
                    }
                    Ok(super::OpTypeContent::Sumof(values.ok_or_else(|| {
                        ErrorKind::MissingElement("sumof".into())
                    })?))
                }
                S::Value(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_value(&mut values, value)?;
                    }
                    Ok(super::OpTypeContent::Value(values.ok_or_else(|| {
                        ErrorKind::MissingElement("value".into())
                    })?))
                }
                S::Bit(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_bit(&mut values, value)?;
                    }
                    Ok(super::OpTypeContent::Bit(
                        values.ok_or_else(|| ErrorKind::MissingElement("bit".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_op<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::OpType>,
            output: DeserializerOutput<'de, super::OpType>,
            fallback: &mut Option<OpTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => OpTypeContentDeserializerState::Op(values, None),
                    Some(OpTypeContentDeserializerState::Op(_, Some(deserializer))) => {
                        OpTypeContentDeserializerState::Op(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(OpTypeContentDeserializerState::Op(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_op(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_op(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        OpTypeContentDeserializerState::Op(values, None),
                    )?;
                    *self.state = OpTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = OpTypeContentDeserializerState::Op(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_unop<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::UnopType>,
            output: DeserializerOutput<'de, super::UnopType>,
            fallback: &mut Option<OpTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => OpTypeContentDeserializerState::Unop(values, None),
                    Some(OpTypeContentDeserializerState::Unop(_, Some(deserializer))) => {
                        OpTypeContentDeserializerState::Unop(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(OpTypeContentDeserializerState::Unop(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_unop(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_unop(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        OpTypeContentDeserializerState::Unop(values, None),
                    )?;
                    *self.state = OpTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = OpTypeContentDeserializerState::Unop(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_fieldref<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<OpTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => OpTypeContentDeserializerState::Fieldref(values, None),
                    Some(OpTypeContentDeserializerState::Fieldref(_, Some(deserializer))) => {
                        OpTypeContentDeserializerState::Fieldref(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(OpTypeContentDeserializerState::Fieldref(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_fieldref(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fieldref(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        OpTypeContentDeserializerState::Fieldref(values, None),
                    )?;
                    *self.state = OpTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        OpTypeContentDeserializerState::Fieldref(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_enumref<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::EnumrefType>,
            output: DeserializerOutput<'de, super::EnumrefType>,
            fallback: &mut Option<OpTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => OpTypeContentDeserializerState::Enumref(values, None),
                    Some(OpTypeContentDeserializerState::Enumref(_, Some(deserializer))) => {
                        OpTypeContentDeserializerState::Enumref(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(OpTypeContentDeserializerState::Enumref(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_enumref(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_enumref(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        OpTypeContentDeserializerState::Enumref(values, None),
                    )?;
                    *self.state = OpTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        OpTypeContentDeserializerState::Enumref(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_popcount<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PopcountType>,
            output: DeserializerOutput<'de, super::PopcountType>,
            fallback: &mut Option<OpTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => OpTypeContentDeserializerState::Popcount(values, None),
                    Some(OpTypeContentDeserializerState::Popcount(_, Some(deserializer))) => {
                        OpTypeContentDeserializerState::Popcount(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(OpTypeContentDeserializerState::Popcount(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_popcount(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_popcount(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        OpTypeContentDeserializerState::Popcount(values, None),
                    )?;
                    *self.state = OpTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        OpTypeContentDeserializerState::Popcount(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_sumof<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SumofType>,
            output: DeserializerOutput<'de, super::SumofType>,
            fallback: &mut Option<OpTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => OpTypeContentDeserializerState::Sumof(values, None),
                    Some(OpTypeContentDeserializerState::Sumof(_, Some(deserializer))) => {
                        OpTypeContentDeserializerState::Sumof(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(OpTypeContentDeserializerState::Sumof(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_sumof(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sumof(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        OpTypeContentDeserializerState::Sumof(values, None),
                    )?;
                    *self.state = OpTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = OpTypeContentDeserializerState::Sumof(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_value<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::DecOrHexIntegerType>,
            output: DeserializerOutput<'de, super::DecOrHexIntegerType>,
            fallback: &mut Option<OpTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => OpTypeContentDeserializerState::Value(values, None),
                    Some(OpTypeContentDeserializerState::Value(_, Some(deserializer))) => {
                        OpTypeContentDeserializerState::Value(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(OpTypeContentDeserializerState::Value(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_value(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_value(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        OpTypeContentDeserializerState::Value(values, None),
                    )?;
                    *self.state = OpTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = OpTypeContentDeserializerState::Value(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_bit<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<OpTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => OpTypeContentDeserializerState::Bit(values, None),
                    Some(OpTypeContentDeserializerState::Bit(_, Some(deserializer))) => {
                        OpTypeContentDeserializerState::Bit(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(OpTypeContentDeserializerState::Bit(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_bit(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bit(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        OpTypeContentDeserializerState::Bit(values, None),
                    )?;
                    *self.state = OpTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = OpTypeContentDeserializerState::Bit(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::OpTypeContent> for OpTypeContentDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::OpTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(OpTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, OpTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OpTypeContent>
        where
            R: DeserializeReader,
        {
            use OpTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Op(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_op(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_unop(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fieldref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_enumref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_popcount(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_sumof(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_bit(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::Op(values, None), event) => {
                        let output =
                            <super::OpType as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_op(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, None), event) => {
                        let output = <super::UnopType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_unop(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_fieldref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, None), event) => {
                        let output = <super::EnumrefType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_enumref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, None), event) => {
                        let output = <super::PopcountType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_popcount(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, None), event) => {
                        let output = <super::SumofType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_sumof(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, None), event) => {
                        let output =
                            <super::DecOrHexIntegerType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, None), event) => {
                        let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_bit(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::OpTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct UnopTypeDeserializer {
        op: String,
        content: Vec<super::UnopTypeContent>,
        state: Box<UnopTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum UnopTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::UnopTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl UnopTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut op: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"op" {
                    reader.read_attrib(&mut op, b"op", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                op: op.ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("op".into())))?,
                content: Vec::new(),
                state: Box::new(UnopTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: UnopTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let UnopTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::UnopTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::UnopTypeContent>,
            fallback: &mut Option<UnopTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback.take().unwrap_or(UnopTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = UnopTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let can_have_more = self.content.len().saturating_add(1) < 8usize;
                    let ret = if can_have_more {
                        ElementHandlerOutput::from_event(event, allow_any)
                    } else {
                        ElementHandlerOutput::from_event_end(event, allow_any)
                    };
                    match (can_have_more, &ret) {
                        (true, ElementHandlerOutput::Continue { .. }) => {
                            fallback
                                .get_or_insert(UnopTypeDeserializerState::Content__(deserializer));
                            *self.state = UnopTypeDeserializerState::Next__;
                        }
                        (false, _) | (_, ElementHandlerOutput::Break { .. }) => {
                            *self.state = UnopTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::UnopType> for UnopTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::UnopType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UnopType>
        where
            R: DeserializeReader,
        {
            use UnopTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::UnopTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::UnopType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, UnopTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::UnopType {
                op: self.op,
                content: self.content.try_into().map_err(|vec: Vec<_>| {
                    ErrorKind::InsufficientSize {
                        min: 8usize,
                        max: 8usize,
                        actual: vec.len(),
                    }
                })?,
            })
        }
    }
    #[derive(Debug)]
    pub struct UnopTypeContentDeserializer {
        state: Box<UnopTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum UnopTypeContentDeserializerState {
        Init__,
        Op(
            Option<super::OpType>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
        ),
        Unop(
            Option<super::UnopType>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
        ),
        Fieldref(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Enumref(
            Option<super::EnumrefType>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
        ),
        Popcount(
            Option<super::PopcountType>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
        ),
        Sumof(
            Option<super::SumofType>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
        ),
        Value(
            Option<super::DecOrHexIntegerType>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
        ),
        Bit(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Done__(super::UnopTypeContent),
        Unknown__,
    }
    impl UnopTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<UnopTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"op" {
                    let output =
                        <super::OpType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_op(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"unop" {
                    let output =
                        <super::UnopType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_unop(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"fieldref" {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_fieldref(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"enumref" {
                    let output = <super::EnumrefType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_enumref(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"popcount" {
                    let output = <super::PopcountType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_popcount(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"sumof" {
                    let output =
                        <super::SumofType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_sumof(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"value" {
                    let output =
                        <super::DecOrHexIntegerType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    return self.handle_value(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"bit" {
                    let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_bit(reader, Default::default(), output, &mut *fallback);
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(UnopTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: UnopTypeContentDeserializerState,
        ) -> Result<super::UnopTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use UnopTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Op(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_op(&mut values, value)?;
                    }
                    Ok(super::UnopTypeContent::Op(Box::new(
                        values.ok_or_else(|| ErrorKind::MissingElement("op".into()))?,
                    )))
                }
                S::Unop(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_unop(&mut values, value)?;
                    }
                    Ok(super::UnopTypeContent::Unop(Box::new(values.ok_or_else(
                        || ErrorKind::MissingElement("unop".into()),
                    )?)))
                }
                S::Fieldref(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_fieldref(&mut values, value)?;
                    }
                    Ok(super::UnopTypeContent::Fieldref(values.ok_or_else(
                        || ErrorKind::MissingElement("fieldref".into()),
                    )?))
                }
                S::Enumref(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_enumref(&mut values, value)?;
                    }
                    Ok(super::UnopTypeContent::Enumref(values.ok_or_else(
                        || ErrorKind::MissingElement("enumref".into()),
                    )?))
                }
                S::Popcount(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_popcount(&mut values, value)?;
                    }
                    Ok(super::UnopTypeContent::Popcount(Box::new(
                        values.ok_or_else(|| ErrorKind::MissingElement("popcount".into()))?,
                    )))
                }
                S::Sumof(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_sumof(&mut values, value)?;
                    }
                    Ok(super::UnopTypeContent::Sumof(values.ok_or_else(|| {
                        ErrorKind::MissingElement("sumof".into())
                    })?))
                }
                S::Value(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_value(&mut values, value)?;
                    }
                    Ok(super::UnopTypeContent::Value(values.ok_or_else(|| {
                        ErrorKind::MissingElement("value".into())
                    })?))
                }
                S::Bit(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_bit(&mut values, value)?;
                    }
                    Ok(super::UnopTypeContent::Bit(
                        values.ok_or_else(|| ErrorKind::MissingElement("bit".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_op<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::OpType>,
            output: DeserializerOutput<'de, super::OpType>,
            fallback: &mut Option<UnopTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => UnopTypeContentDeserializerState::Op(values, None),
                    Some(UnopTypeContentDeserializerState::Op(_, Some(deserializer))) => {
                        UnopTypeContentDeserializerState::Op(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(UnopTypeContentDeserializerState::Op(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_op(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_op(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        UnopTypeContentDeserializerState::Op(values, None),
                    )?;
                    *self.state = UnopTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = UnopTypeContentDeserializerState::Op(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_unop<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::UnopType>,
            output: DeserializerOutput<'de, super::UnopType>,
            fallback: &mut Option<UnopTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => UnopTypeContentDeserializerState::Unop(values, None),
                    Some(UnopTypeContentDeserializerState::Unop(_, Some(deserializer))) => {
                        UnopTypeContentDeserializerState::Unop(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(UnopTypeContentDeserializerState::Unop(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_unop(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_unop(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        UnopTypeContentDeserializerState::Unop(values, None),
                    )?;
                    *self.state = UnopTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        UnopTypeContentDeserializerState::Unop(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_fieldref<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<UnopTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => UnopTypeContentDeserializerState::Fieldref(values, None),
                    Some(UnopTypeContentDeserializerState::Fieldref(_, Some(deserializer))) => {
                        UnopTypeContentDeserializerState::Fieldref(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(UnopTypeContentDeserializerState::Fieldref(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_fieldref(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fieldref(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        UnopTypeContentDeserializerState::Fieldref(values, None),
                    )?;
                    *self.state = UnopTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        UnopTypeContentDeserializerState::Fieldref(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_enumref<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::EnumrefType>,
            output: DeserializerOutput<'de, super::EnumrefType>,
            fallback: &mut Option<UnopTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => UnopTypeContentDeserializerState::Enumref(values, None),
                    Some(UnopTypeContentDeserializerState::Enumref(_, Some(deserializer))) => {
                        UnopTypeContentDeserializerState::Enumref(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(UnopTypeContentDeserializerState::Enumref(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_enumref(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_enumref(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        UnopTypeContentDeserializerState::Enumref(values, None),
                    )?;
                    *self.state = UnopTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        UnopTypeContentDeserializerState::Enumref(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_popcount<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PopcountType>,
            output: DeserializerOutput<'de, super::PopcountType>,
            fallback: &mut Option<UnopTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => UnopTypeContentDeserializerState::Popcount(values, None),
                    Some(UnopTypeContentDeserializerState::Popcount(_, Some(deserializer))) => {
                        UnopTypeContentDeserializerState::Popcount(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(UnopTypeContentDeserializerState::Popcount(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_popcount(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_popcount(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        UnopTypeContentDeserializerState::Popcount(values, None),
                    )?;
                    *self.state = UnopTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        UnopTypeContentDeserializerState::Popcount(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_sumof<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SumofType>,
            output: DeserializerOutput<'de, super::SumofType>,
            fallback: &mut Option<UnopTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => UnopTypeContentDeserializerState::Sumof(values, None),
                    Some(UnopTypeContentDeserializerState::Sumof(_, Some(deserializer))) => {
                        UnopTypeContentDeserializerState::Sumof(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(UnopTypeContentDeserializerState::Sumof(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_sumof(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sumof(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        UnopTypeContentDeserializerState::Sumof(values, None),
                    )?;
                    *self.state = UnopTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        UnopTypeContentDeserializerState::Sumof(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_value<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::DecOrHexIntegerType>,
            output: DeserializerOutput<'de, super::DecOrHexIntegerType>,
            fallback: &mut Option<UnopTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => UnopTypeContentDeserializerState::Value(values, None),
                    Some(UnopTypeContentDeserializerState::Value(_, Some(deserializer))) => {
                        UnopTypeContentDeserializerState::Value(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(UnopTypeContentDeserializerState::Value(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_value(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_value(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        UnopTypeContentDeserializerState::Value(values, None),
                    )?;
                    *self.state = UnopTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        UnopTypeContentDeserializerState::Value(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_bit<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<UnopTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => UnopTypeContentDeserializerState::Bit(values, None),
                    Some(UnopTypeContentDeserializerState::Bit(_, Some(deserializer))) => {
                        UnopTypeContentDeserializerState::Bit(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(UnopTypeContentDeserializerState::Bit(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_bit(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bit(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        UnopTypeContentDeserializerState::Bit(values, None),
                    )?;
                    *self.state = UnopTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = UnopTypeContentDeserializerState::Bit(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::UnopTypeContent> for UnopTypeContentDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::UnopTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(UnopTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, UnopTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UnopTypeContent>
        where
            R: DeserializeReader,
        {
            use UnopTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Op(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_op(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_unop(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fieldref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_enumref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_popcount(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_sumof(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_bit(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::Op(values, None), event) => {
                        let output =
                            <super::OpType as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_op(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, None), event) => {
                        let output = <super::UnopType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_unop(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_fieldref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, None), event) => {
                        let output = <super::EnumrefType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_enumref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, None), event) => {
                        let output = <super::PopcountType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_popcount(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, None), event) => {
                        let output = <super::SumofType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_sumof(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, None), event) => {
                        let output =
                            <super::DecOrHexIntegerType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, None), event) => {
                        let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_bit(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::UnopTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct EnumrefTypeDeserializer {
        ref_: String,
        content: Option<String>,
        state: Box<EnumrefTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum EnumrefTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl EnumrefTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut ref_: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"ref" {
                    reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                ref_: ref_
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("ref".into())))?,
                content: None,
                state: Box::new(EnumrefTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: EnumrefTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let EnumrefTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
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
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::EnumrefType>
        where
            R: DeserializeReader,
        {
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
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
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
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::EnumrefType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EnumrefType>
        where
            R: DeserializeReader,
        {
            use EnumrefTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::EnumrefType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, EnumrefTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::EnumrefType {
                ref_: self.ref_,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PopcountTypeDeserializer {
        state: Box<PopcountTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum PopcountTypeDeserializerState {
        Init__,
        Op(
            Option<super::OpType>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
        ),
        Unop(
            Option<super::UnopType>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
        ),
        Fieldref(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Enumref(
            Option<super::EnumrefType>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
        ),
        Popcount(
            Option<super::PopcountType>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
        ),
        Sumof(
            Option<super::SumofType>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
        ),
        Value(
            Option<super::DecOrHexIntegerType>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
        ),
        Bit(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Done__(super::PopcountType),
        Unknown__,
    }
    impl PopcountTypeDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<PopcountTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"op" {
                    let output =
                        <super::OpType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_op(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"unop" {
                    let output =
                        <super::UnopType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_unop(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"fieldref" {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_fieldref(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"enumref" {
                    let output = <super::EnumrefType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_enumref(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"popcount" {
                    let output = <super::PopcountType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_popcount(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"sumof" {
                    let output =
                        <super::SumofType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_sumof(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"value" {
                    let output =
                        <super::DecOrHexIntegerType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    return self.handle_value(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"bit" {
                    let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_bit(reader, Default::default(), output, &mut *fallback);
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(PopcountTypeDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                state: Box::new(PopcountTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            reader: &R,
            state: PopcountTypeDeserializerState,
        ) -> Result<super::PopcountType, Error>
        where
            R: DeserializeReader,
        {
            use PopcountTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Op(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_op(&mut values, value)?;
                    }
                    Ok(super::PopcountType::Op(Box::new(
                        values.ok_or_else(|| ErrorKind::MissingElement("op".into()))?,
                    )))
                }
                S::Unop(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_unop(&mut values, value)?;
                    }
                    Ok(super::PopcountType::Unop(Box::new(values.ok_or_else(
                        || ErrorKind::MissingElement("unop".into()),
                    )?)))
                }
                S::Fieldref(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_fieldref(&mut values, value)?;
                    }
                    Ok(super::PopcountType::Fieldref(values.ok_or_else(|| {
                        ErrorKind::MissingElement("fieldref".into())
                    })?))
                }
                S::Enumref(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_enumref(&mut values, value)?;
                    }
                    Ok(super::PopcountType::Enumref(values.ok_or_else(|| {
                        ErrorKind::MissingElement("enumref".into())
                    })?))
                }
                S::Popcount(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_popcount(&mut values, value)?;
                    }
                    Ok(super::PopcountType::Popcount(Box::new(values.ok_or_else(
                        || ErrorKind::MissingElement("popcount".into()),
                    )?)))
                }
                S::Sumof(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_sumof(&mut values, value)?;
                    }
                    Ok(super::PopcountType::Sumof(values.ok_or_else(|| {
                        ErrorKind::MissingElement("sumof".into())
                    })?))
                }
                S::Value(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_value(&mut values, value)?;
                    }
                    Ok(super::PopcountType::Value(values.ok_or_else(|| {
                        ErrorKind::MissingElement("value".into())
                    })?))
                }
                S::Bit(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_bit(&mut values, value)?;
                    }
                    Ok(super::PopcountType::Bit(
                        values.ok_or_else(|| ErrorKind::MissingElement("bit".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_op<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::OpType>,
            output: DeserializerOutput<'de, super::OpType>,
            fallback: &mut Option<PopcountTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => PopcountTypeDeserializerState::Op(values, None),
                    Some(PopcountTypeDeserializerState::Op(_, Some(deserializer))) => {
                        PopcountTypeDeserializerState::Op(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(PopcountTypeDeserializerState::Op(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_op(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_op(&mut values, data)?;
                    *self.state = PopcountTypeDeserializerState::Op(values, None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = PopcountTypeDeserializerState::Op(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_unop<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::UnopType>,
            output: DeserializerOutput<'de, super::UnopType>,
            fallback: &mut Option<PopcountTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => PopcountTypeDeserializerState::Unop(values, None),
                    Some(PopcountTypeDeserializerState::Unop(_, Some(deserializer))) => {
                        PopcountTypeDeserializerState::Unop(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(PopcountTypeDeserializerState::Unop(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_unop(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_unop(&mut values, data)?;
                    *self.state = PopcountTypeDeserializerState::Unop(values, None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = PopcountTypeDeserializerState::Unop(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_fieldref<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<PopcountTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => PopcountTypeDeserializerState::Fieldref(values, None),
                    Some(PopcountTypeDeserializerState::Fieldref(_, Some(deserializer))) => {
                        PopcountTypeDeserializerState::Fieldref(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(PopcountTypeDeserializerState::Fieldref(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_fieldref(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fieldref(&mut values, data)?;
                    *self.state = PopcountTypeDeserializerState::Fieldref(values, None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        PopcountTypeDeserializerState::Fieldref(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_enumref<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::EnumrefType>,
            output: DeserializerOutput<'de, super::EnumrefType>,
            fallback: &mut Option<PopcountTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => PopcountTypeDeserializerState::Enumref(values, None),
                    Some(PopcountTypeDeserializerState::Enumref(_, Some(deserializer))) => {
                        PopcountTypeDeserializerState::Enumref(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(PopcountTypeDeserializerState::Enumref(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_enumref(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_enumref(&mut values, data)?;
                    *self.state = PopcountTypeDeserializerState::Enumref(values, None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        PopcountTypeDeserializerState::Enumref(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_popcount<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PopcountType>,
            output: DeserializerOutput<'de, super::PopcountType>,
            fallback: &mut Option<PopcountTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => PopcountTypeDeserializerState::Popcount(values, None),
                    Some(PopcountTypeDeserializerState::Popcount(_, Some(deserializer))) => {
                        PopcountTypeDeserializerState::Popcount(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(PopcountTypeDeserializerState::Popcount(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_popcount(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_popcount(&mut values, data)?;
                    *self.state = PopcountTypeDeserializerState::Popcount(values, None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        PopcountTypeDeserializerState::Popcount(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_sumof<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SumofType>,
            output: DeserializerOutput<'de, super::SumofType>,
            fallback: &mut Option<PopcountTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => PopcountTypeDeserializerState::Sumof(values, None),
                    Some(PopcountTypeDeserializerState::Sumof(_, Some(deserializer))) => {
                        PopcountTypeDeserializerState::Sumof(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(PopcountTypeDeserializerState::Sumof(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_sumof(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sumof(&mut values, data)?;
                    *self.state = PopcountTypeDeserializerState::Sumof(values, None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = PopcountTypeDeserializerState::Sumof(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_value<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::DecOrHexIntegerType>,
            output: DeserializerOutput<'de, super::DecOrHexIntegerType>,
            fallback: &mut Option<PopcountTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => PopcountTypeDeserializerState::Value(values, None),
                    Some(PopcountTypeDeserializerState::Value(_, Some(deserializer))) => {
                        PopcountTypeDeserializerState::Value(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(PopcountTypeDeserializerState::Value(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_value(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_value(&mut values, data)?;
                    *self.state = PopcountTypeDeserializerState::Value(values, None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = PopcountTypeDeserializerState::Value(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_bit<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<PopcountTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => PopcountTypeDeserializerState::Bit(values, None),
                    Some(PopcountTypeDeserializerState::Bit(_, Some(deserializer))) => {
                        PopcountTypeDeserializerState::Bit(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(PopcountTypeDeserializerState::Bit(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_bit(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bit(&mut values, data)?;
                    *self.state = PopcountTypeDeserializerState::Bit(values, None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = PopcountTypeDeserializerState::Bit(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::PopcountType> for PopcountTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::PopcountType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PopcountType>
        where
            R: DeserializeReader,
        {
            use PopcountTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Op(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_op(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_unop(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fieldref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_enumref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_popcount(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_sumof(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_bit(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::Op(values, None), event) => {
                        let output =
                            <super::OpType as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_op(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, None), event) => {
                        let output = <super::UnopType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_unop(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_fieldref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, None), event) => {
                        let output = <super::EnumrefType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_enumref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, None), event) => {
                        let output = <super::PopcountType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_popcount(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, None), event) => {
                        let output = <super::SumofType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_sumof(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, None), event) => {
                        let output =
                            <super::DecOrHexIntegerType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, None), event) => {
                        let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_bit(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::PopcountType, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct SumofTypeDeserializer {
        ref_: String,
        state: Box<SumofTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SumofTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl SumofTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut ref_: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"ref" {
                    reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                ref_: ref_
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("ref".into())))?,
                state: Box::new(SumofTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SumofTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::SumofType> for SumofTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::SumofType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SumofType>
        where
            R: DeserializeReader,
        {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(reader)?),
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
        fn finish<R>(mut self, reader: &R) -> Result<super::SumofType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, SumofTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::SumofType { ref_: self.ref_ })
        }
    }
    #[derive(Debug)]
    pub struct CaseexprTypeDeserializer {
        name: Option<String>,
        content: Vec<super::CaseexprTypeContent>,
        state: Box<CaseexprTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CaseexprTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::CaseexprTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl CaseexprTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                name: name,
                content: Vec::new(),
                state: Box::new(CaseexprTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: CaseexprTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let CaseexprTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::CaseexprTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::CaseexprTypeContent>,
            fallback: &mut Option<CaseexprTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(CaseexprTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = CaseexprTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = CaseexprTypeDeserializerState::Content__(deserializer);
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(CaseexprTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = CaseexprTypeDeserializerState::Next__;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::CaseexprType> for CaseexprTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::CaseexprType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CaseexprType>
        where
            R: DeserializeReader,
        {
            use CaseexprTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::CaseexprTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::CaseexprType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, CaseexprTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::CaseexprType {
                name: self.name,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct CaseexprTypeContentDeserializer {
        state: Box<CaseexprTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum CaseexprTypeContentDeserializerState {
        Init__,
        Op(
            Option<super::OpType>,
            Option<<super::OpType as WithDeserializer>::Deserializer>,
        ),
        Unop(
            Option<super::UnopType>,
            Option<<super::UnopType as WithDeserializer>::Deserializer>,
        ),
        Fieldref(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Enumref(
            Option<super::EnumrefType>,
            Option<<super::EnumrefType as WithDeserializer>::Deserializer>,
        ),
        Popcount(
            Option<super::PopcountType>,
            Option<<super::PopcountType as WithDeserializer>::Deserializer>,
        ),
        Sumof(
            Option<super::SumofType>,
            Option<<super::SumofType as WithDeserializer>::Deserializer>,
        ),
        Value(
            Option<super::DecOrHexIntegerType>,
            Option<<super::DecOrHexIntegerType as WithDeserializer>::Deserializer>,
        ),
        Bit(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Pad(
            Option<super::PadType>,
            Option<<super::PadType as WithDeserializer>::Deserializer>,
        ),
        Field(
            Option<super::VarType>,
            Option<<super::VarType as WithDeserializer>::Deserializer>,
        ),
        List(
            Option<super::ListType>,
            Option<<super::ListType as WithDeserializer>::Deserializer>,
        ),
        Fd(
            Option<super::AnyType>,
            Option<<super::AnyType as WithDeserializer>::Deserializer>,
        ),
        Switch(
            Option<super::SwitchexprType>,
            Option<<super::SwitchexprType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::CaseexprTypeContent),
        Unknown__,
    }
    impl CaseexprTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<CaseexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if x.name().local_name().as_ref() == b"op" {
                    let output =
                        <super::OpType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_op(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"unop" {
                    let output =
                        <super::UnopType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_unop(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"fieldref" {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_fieldref(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"enumref" {
                    let output = <super::EnumrefType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_enumref(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"popcount" {
                    let output = <super::PopcountType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_popcount(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if x.name().local_name().as_ref() == b"sumof" {
                    let output =
                        <super::SumofType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_sumof(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"value" {
                    let output =
                        <super::DecOrHexIntegerType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    return self.handle_value(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"bit" {
                    let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_bit(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"pad" {
                    let output =
                        <super::PadType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_pad(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"field" {
                    let output =
                        <super::VarType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_field(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"list" {
                    let output =
                        <super::ListType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_list(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"fd" {
                    let output =
                        <super::AnyType as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_fd(reader, Default::default(), output, &mut *fallback);
                }
                if x.name().local_name().as_ref() == b"switch" {
                    let output = <super::SwitchexprType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_switch(reader, Default::default(), output, &mut *fallback);
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(CaseexprTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: CaseexprTypeContentDeserializerState,
        ) -> Result<super::CaseexprTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use CaseexprTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Op(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_op(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Op(
                        values.ok_or_else(|| ErrorKind::MissingElement("op".into()))?,
                    ))
                }
                S::Unop(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_unop(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Unop(
                        values.ok_or_else(|| ErrorKind::MissingElement("unop".into()))?,
                    ))
                }
                S::Fieldref(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_fieldref(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Fieldref(values.ok_or_else(
                        || ErrorKind::MissingElement("fieldref".into()),
                    )?))
                }
                S::Enumref(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_enumref(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Enumref(values.ok_or_else(
                        || ErrorKind::MissingElement("enumref".into()),
                    )?))
                }
                S::Popcount(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_popcount(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Popcount(values.ok_or_else(
                        || ErrorKind::MissingElement("popcount".into()),
                    )?))
                }
                S::Sumof(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_sumof(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Sumof(values.ok_or_else(
                        || ErrorKind::MissingElement("sumof".into()),
                    )?))
                }
                S::Value(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_value(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Value(values.ok_or_else(
                        || ErrorKind::MissingElement("value".into()),
                    )?))
                }
                S::Bit(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_bit(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Bit(
                        values.ok_or_else(|| ErrorKind::MissingElement("bit".into()))?,
                    ))
                }
                S::Pad(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_pad(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Pad(
                        values.ok_or_else(|| ErrorKind::MissingElement("pad".into()))?,
                    ))
                }
                S::Field(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_field(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Field(values.ok_or_else(
                        || ErrorKind::MissingElement("field".into()),
                    )?))
                }
                S::List(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_list(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::List(
                        values.ok_or_else(|| ErrorKind::MissingElement("list".into()))?,
                    ))
                }
                S::Fd(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_fd(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Fd(
                        values.ok_or_else(|| ErrorKind::MissingElement("fd".into()))?,
                    ))
                }
                S::Switch(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_switch(&mut values, value)?;
                    }
                    Ok(super::CaseexprTypeContent::Switch(values.ok_or_else(
                        || ErrorKind::MissingElement("switch".into()),
                    )?))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_op<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::OpType>,
            output: DeserializerOutput<'de, super::OpType>,
            fallback: &mut Option<CaseexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => CaseexprTypeContentDeserializerState::Op(values, None),
                    Some(CaseexprTypeContentDeserializerState::Op(_, Some(deserializer))) => {
                        CaseexprTypeContentDeserializerState::Op(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(CaseexprTypeContentDeserializerState::Op(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_op(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_op(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        CaseexprTypeContentDeserializerState::Op(values, None),
                    )?;
                    *self.state = CaseexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        CaseexprTypeContentDeserializerState::Op(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_unop<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::UnopType>,
            output: DeserializerOutput<'de, super::UnopType>,
            fallback: &mut Option<CaseexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => CaseexprTypeContentDeserializerState::Unop(values, None),
                    Some(CaseexprTypeContentDeserializerState::Unop(_, Some(deserializer))) => {
                        CaseexprTypeContentDeserializerState::Unop(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(CaseexprTypeContentDeserializerState::Unop(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_unop(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_unop(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        CaseexprTypeContentDeserializerState::Unop(values, None),
                    )?;
                    *self.state = CaseexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        CaseexprTypeContentDeserializerState::Unop(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_fieldref<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<CaseexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => CaseexprTypeContentDeserializerState::Fieldref(values, None),
                    Some(CaseexprTypeContentDeserializerState::Fieldref(_, Some(deserializer))) => {
                        CaseexprTypeContentDeserializerState::Fieldref(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(CaseexprTypeContentDeserializerState::Fieldref(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_fieldref(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fieldref(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        CaseexprTypeContentDeserializerState::Fieldref(values, None),
                    )?;
                    *self.state = CaseexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        CaseexprTypeContentDeserializerState::Fieldref(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_enumref<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::EnumrefType>,
            output: DeserializerOutput<'de, super::EnumrefType>,
            fallback: &mut Option<CaseexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => CaseexprTypeContentDeserializerState::Enumref(values, None),
                    Some(CaseexprTypeContentDeserializerState::Enumref(_, Some(deserializer))) => {
                        CaseexprTypeContentDeserializerState::Enumref(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(CaseexprTypeContentDeserializerState::Enumref(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_enumref(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_enumref(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        CaseexprTypeContentDeserializerState::Enumref(values, None),
                    )?;
                    *self.state = CaseexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        CaseexprTypeContentDeserializerState::Enumref(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_popcount<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PopcountType>,
            output: DeserializerOutput<'de, super::PopcountType>,
            fallback: &mut Option<CaseexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => CaseexprTypeContentDeserializerState::Popcount(values, None),
                    Some(CaseexprTypeContentDeserializerState::Popcount(_, Some(deserializer))) => {
                        CaseexprTypeContentDeserializerState::Popcount(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(CaseexprTypeContentDeserializerState::Popcount(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_popcount(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_popcount(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        CaseexprTypeContentDeserializerState::Popcount(values, None),
                    )?;
                    *self.state = CaseexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        CaseexprTypeContentDeserializerState::Popcount(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_sumof<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SumofType>,
            output: DeserializerOutput<'de, super::SumofType>,
            fallback: &mut Option<CaseexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => CaseexprTypeContentDeserializerState::Sumof(values, None),
                    Some(CaseexprTypeContentDeserializerState::Sumof(_, Some(deserializer))) => {
                        CaseexprTypeContentDeserializerState::Sumof(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(CaseexprTypeContentDeserializerState::Sumof(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_sumof(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_sumof(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        CaseexprTypeContentDeserializerState::Sumof(values, None),
                    )?;
                    *self.state = CaseexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        CaseexprTypeContentDeserializerState::Sumof(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_value<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::DecOrHexIntegerType>,
            output: DeserializerOutput<'de, super::DecOrHexIntegerType>,
            fallback: &mut Option<CaseexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => CaseexprTypeContentDeserializerState::Value(values, None),
                    Some(CaseexprTypeContentDeserializerState::Value(_, Some(deserializer))) => {
                        CaseexprTypeContentDeserializerState::Value(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(CaseexprTypeContentDeserializerState::Value(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_value(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_value(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        CaseexprTypeContentDeserializerState::Value(values, None),
                    )?;
                    *self.state = CaseexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        CaseexprTypeContentDeserializerState::Value(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_bit<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<CaseexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => CaseexprTypeContentDeserializerState::Bit(values, None),
                    Some(CaseexprTypeContentDeserializerState::Bit(_, Some(deserializer))) => {
                        CaseexprTypeContentDeserializerState::Bit(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(CaseexprTypeContentDeserializerState::Bit(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_bit(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bit(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        CaseexprTypeContentDeserializerState::Bit(values, None),
                    )?;
                    *self.state = CaseexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        CaseexprTypeContentDeserializerState::Bit(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_pad<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PadType>,
            output: DeserializerOutput<'de, super::PadType>,
            fallback: &mut Option<CaseexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => CaseexprTypeContentDeserializerState::Pad(values, None),
                    Some(CaseexprTypeContentDeserializerState::Pad(_, Some(deserializer))) => {
                        CaseexprTypeContentDeserializerState::Pad(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(CaseexprTypeContentDeserializerState::Pad(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_pad(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pad(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        CaseexprTypeContentDeserializerState::Pad(values, None),
                    )?;
                    *self.state = CaseexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        CaseexprTypeContentDeserializerState::Pad(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_field<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::VarType>,
            output: DeserializerOutput<'de, super::VarType>,
            fallback: &mut Option<CaseexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => CaseexprTypeContentDeserializerState::Field(values, None),
                    Some(CaseexprTypeContentDeserializerState::Field(_, Some(deserializer))) => {
                        CaseexprTypeContentDeserializerState::Field(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(CaseexprTypeContentDeserializerState::Field(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_field(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_field(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        CaseexprTypeContentDeserializerState::Field(values, None),
                    )?;
                    *self.state = CaseexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        CaseexprTypeContentDeserializerState::Field(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_list<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::ListType>,
            output: DeserializerOutput<'de, super::ListType>,
            fallback: &mut Option<CaseexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => CaseexprTypeContentDeserializerState::List(values, None),
                    Some(CaseexprTypeContentDeserializerState::List(_, Some(deserializer))) => {
                        CaseexprTypeContentDeserializerState::List(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(CaseexprTypeContentDeserializerState::List(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_list(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_list(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        CaseexprTypeContentDeserializerState::List(values, None),
                    )?;
                    *self.state = CaseexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        CaseexprTypeContentDeserializerState::List(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_fd<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::AnyType>,
            output: DeserializerOutput<'de, super::AnyType>,
            fallback: &mut Option<CaseexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => CaseexprTypeContentDeserializerState::Fd(values, None),
                    Some(CaseexprTypeContentDeserializerState::Fd(_, Some(deserializer))) => {
                        CaseexprTypeContentDeserializerState::Fd(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(CaseexprTypeContentDeserializerState::Fd(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_fd(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_fd(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        CaseexprTypeContentDeserializerState::Fd(values, None),
                    )?;
                    *self.state = CaseexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        CaseexprTypeContentDeserializerState::Fd(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_switch<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SwitchexprType>,
            output: DeserializerOutput<'de, super::SwitchexprType>,
            fallback: &mut Option<CaseexprTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => CaseexprTypeContentDeserializerState::Switch(values, None),
                    Some(CaseexprTypeContentDeserializerState::Switch(_, Some(deserializer))) => {
                        CaseexprTypeContentDeserializerState::Switch(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(CaseexprTypeContentDeserializerState::Switch(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_switch(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_switch(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        CaseexprTypeContentDeserializerState::Switch(values, None),
                    )?;
                    *self.state = CaseexprTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        CaseexprTypeContentDeserializerState::Switch(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::CaseexprTypeContent> for CaseexprTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CaseexprTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(CaseexprTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, CaseexprTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CaseexprTypeContent>
        where
            R: DeserializeReader,
        {
            use CaseexprTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Op(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_op(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_unop(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fieldref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_enumref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_popcount(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_sumof(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_bit(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Pad(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pad(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_field(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_list(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fd(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Switch(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_switch(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::Op(values, None), event) => {
                        let output =
                            <super::OpType as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_op(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unop(values, None), event) => {
                        let output = <super::UnopType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_unop(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fieldref(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_fieldref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Enumref(values, None), event) => {
                        let output = <super::EnumrefType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_enumref(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Popcount(values, None), event) => {
                        let output = <super::PopcountType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_popcount(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Sumof(values, None), event) => {
                        let output = <super::SumofType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_sumof(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Value(values, None), event) => {
                        let output =
                            <super::DecOrHexIntegerType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Bit(values, None), event) => {
                        let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_bit(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Pad(values, None), event) => {
                        let output = <super::PadType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_pad(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Field(values, None), event) => {
                        let output = <super::VarType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_field(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::List(values, None), event) => {
                        let output = <super::ListType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_list(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Fd(values, None), event) => {
                        let output = <super::AnyType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_fd(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Switch(values, None), event) => {
                        let output =
                            <super::SwitchexprType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_switch(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::CaseexprTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct FieldTypeDeserializer {
        name: Option<String>,
        content: Option<String>,
        state: Box<FieldTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FieldTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl FieldTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                name: name,
                content: None,
                state: Box::new(FieldTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: FieldTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let FieldTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
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
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::FieldType>
        where
            R: DeserializeReader,
        {
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
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
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
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::FieldType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FieldType>
        where
            R: DeserializeReader,
        {
            use FieldTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::FieldType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, FieldTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::FieldType {
                name: self.name,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ErrorTypeDeserializer {
        type_: Option<String>,
        content: Option<String>,
        state: Box<ErrorTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ErrorTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ErrorTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut type_: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"type" {
                    reader.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                type_: type_,
                content: None,
                state: Box::new(ErrorTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ErrorTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let ErrorTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
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
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::ErrorType>
        where
            R: DeserializeReader,
        {
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
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
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
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ErrorType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ErrorType>
        where
            R: DeserializeReader,
        {
            use ErrorTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ErrorType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, ErrorTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::ErrorType {
                type_: self.type_,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct SeeTypeDeserializer {
        name: Option<String>,
        type_: Option<String>,
        state: Box<SeeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SeeTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl SeeTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<String> = None;
            let mut type_: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"type" {
                    reader.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                name: name,
                type_: type_,
                state: Box::new(SeeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SeeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::SeeType> for SeeTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::SeeType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SeeType>
        where
            R: DeserializeReader,
        {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(reader)?),
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
        fn finish<R>(mut self, reader: &R) -> Result<super::SeeType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, SeeTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::SeeType {
                name: self.name,
                type_: self.type_,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use core::iter::Iterator;
    use xsd_parser::quick_xml::{
        write_attrib, write_attrib_opt, BytesEnd, BytesStart, Error, Event, IterSerializer,
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    XcbTypeSerializerState::Init__ => {
                        *self.state = XcbTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "header", &self.value.header)?;
                        write_attrib_opt(
                            &mut bytes,
                            "extension-xname",
                            &self.value.extension_xname,
                        )?;
                        write_attrib_opt(&mut bytes, "extension-name", &self.value.extension_name)?;
                        write_attrib(
                            &mut bytes,
                            "extension-multiword",
                            &self.value.extension_multiword,
                        )?;
                        write_attrib_opt(&mut bytes, "major-version", &self.value.major_version)?;
                        write_attrib_opt(&mut bytes, "minor-version", &self.value.minor_version)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    XcbTypeSerializerState::Content__(x) => match x.next().transpose()? {
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
    impl<'ser> Iterator for XcbTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                    XcbTypeContentSerializerState::Request(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = XcbTypeContentSerializerState::Done__,
                    },
                    XcbTypeContentSerializerState::Event(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = XcbTypeContentSerializerState::Done__,
                    },
                    XcbTypeContentSerializerState::Eventcopy(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = XcbTypeContentSerializerState::Done__,
                    },
                    XcbTypeContentSerializerState::Error(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = XcbTypeContentSerializerState::Done__,
                    },
                    XcbTypeContentSerializerState::Errorcopy(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = XcbTypeContentSerializerState::Done__,
                    },
                    XcbTypeContentSerializerState::Struct(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = XcbTypeContentSerializerState::Done__,
                    },
                    XcbTypeContentSerializerState::Union(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = XcbTypeContentSerializerState::Done__,
                    },
                    XcbTypeContentSerializerState::Xidtype(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = XcbTypeContentSerializerState::Done__,
                    },
                    XcbTypeContentSerializerState::Xidunion(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = XcbTypeContentSerializerState::Done__,
                    },
                    XcbTypeContentSerializerState::Enum(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = XcbTypeContentSerializerState::Done__,
                    },
                    XcbTypeContentSerializerState::Typedef(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = XcbTypeContentSerializerState::Done__,
                    },
                    XcbTypeContentSerializerState::Import(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = XcbTypeContentSerializerState::Done__,
                    },
                    XcbTypeContentSerializerState::Done__ => return Ok(None),
                    XcbTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for XcbTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    RequestTypeSerializerState::Init__ => {
                        *self.state = RequestTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "name", &self.value.name)?;
                        write_attrib(&mut bytes, "opcode", &self.value.opcode)?;
                        write_attrib_opt(
                            &mut bytes,
                            "combine-adjacent",
                            &self.value.combine_adjacent,
                        )?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    RequestTypeSerializerState::Content__(x) => match x.next().transpose()? {
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
    impl<'ser> Iterator for RequestTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                    RequestTypeContentSerializerState::Pad(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = RequestTypeContentSerializerState::Done__,
                    },
                    RequestTypeContentSerializerState::Field(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = RequestTypeContentSerializerState::Done__,
                    },
                    RequestTypeContentSerializerState::List(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = RequestTypeContentSerializerState::Done__,
                    },
                    RequestTypeContentSerializerState::Fd(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = RequestTypeContentSerializerState::Done__,
                    },
                    RequestTypeContentSerializerState::Exprfield(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestTypeContentSerializerState::Done__,
                        }
                    }
                    RequestTypeContentSerializerState::Valueparam(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestTypeContentSerializerState::Done__,
                        }
                    }
                    RequestTypeContentSerializerState::Switch(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = RequestTypeContentSerializerState::Done__,
                    },
                    RequestTypeContentSerializerState::Reply(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = RequestTypeContentSerializerState::Done__,
                    },
                    RequestTypeContentSerializerState::Doc(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = RequestTypeContentSerializerState::Done__,
                    },
                    RequestTypeContentSerializerState::Done__ => return Ok(None),
                    RequestTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for RequestTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EventTypeSerializerState::Init__ => {
                        *self.state = EventTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "name", &self.value.name)?;
                        write_attrib(&mut bytes, "number", &self.value.number)?;
                        write_attrib_opt(
                            &mut bytes,
                            "no-sequence-number",
                            &self.value.no_sequence_number,
                        )?;
                        write_attrib_opt(&mut bytes, "xge", &self.value.xge)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    EventTypeSerializerState::Content__(x) => match x.next().transpose()? {
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
    impl<'ser> Iterator for EventTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                    EventTypeContentSerializerState::Pad(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = EventTypeContentSerializerState::Done__,
                    },
                    EventTypeContentSerializerState::Field(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = EventTypeContentSerializerState::Done__,
                    },
                    EventTypeContentSerializerState::List(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = EventTypeContentSerializerState::Done__,
                    },
                    EventTypeContentSerializerState::Fd(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = EventTypeContentSerializerState::Done__,
                    },
                    EventTypeContentSerializerState::Doc(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = EventTypeContentSerializerState::Done__,
                    },
                    EventTypeContentSerializerState::Done__ => return Ok(None),
                    EventTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for EventTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PacketStructCopyTypeSerializerState::Init__ => {
                        *self.state = PacketStructCopyTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "name", &self.value.name)?;
                        write_attrib(&mut bytes, "number", &self.value.number)?;
                        write_attrib(&mut bytes, "ref", &self.value.ref_)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    PacketStructCopyTypeSerializerState::Done__ => return Ok(None),
                    PacketStructCopyTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for PacketStructCopyTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PacketStructTypeSerializerState::Init__ => {
                        *self.state = PacketStructTypeSerializerState::Content__(
                            IterSerializer::new(&self.value.content[..], None, false),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "name", &self.value.name)?;
                        write_attrib(&mut bytes, "number", &self.value.number)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    PacketStructTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PacketStructTypeSerializerState::End__,
                    },
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
    impl<'ser> Iterator for PacketStructTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PacketStructTypeContentSerializerState::Done__,
                        }
                    }
                    PacketStructTypeContentSerializerState::Field(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PacketStructTypeContentSerializerState::Done__,
                        }
                    }
                    PacketStructTypeContentSerializerState::List(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PacketStructTypeContentSerializerState::Done__,
                        }
                    }
                    PacketStructTypeContentSerializerState::Fd(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PacketStructTypeContentSerializerState::Done__,
                    },
                    PacketStructTypeContentSerializerState::Done__ => return Ok(None),
                    PacketStructTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for PacketStructTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    StructTypeSerializerState::Init__ => {
                        *self.state = StructTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "name", &self.value.name)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    StructTypeSerializerState::Content__(x) => match x.next().transpose()? {
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
    impl<'ser> Iterator for StructTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                    StructTypeContentSerializerState::Pad(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = StructTypeContentSerializerState::Done__,
                    },
                    StructTypeContentSerializerState::Field(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = StructTypeContentSerializerState::Done__,
                    },
                    StructTypeContentSerializerState::List(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = StructTypeContentSerializerState::Done__,
                    },
                    StructTypeContentSerializerState::Fd(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = StructTypeContentSerializerState::Done__,
                    },
                    StructTypeContentSerializerState::Switch(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = StructTypeContentSerializerState::Done__,
                    },
                    StructTypeContentSerializerState::Done__ => return Ok(None),
                    StructTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for StructTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    XidtypeTypeSerializerState::Init__ => {
                        *self.state = XidtypeTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "name", &self.value.name)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    XidtypeTypeSerializerState::Done__ => return Ok(None),
                    XidtypeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for XidtypeTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    XidunionTypeSerializerState::Init__ => {
                        *self.state = XidunionTypeSerializerState::Type(IterSerializer::new(
                            &self.value.type_[..],
                            Some("type"),
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "name", &self.value.name)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    XidunionTypeSerializerState::Type(x) => match x.next().transpose()? {
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
    impl<'ser> Iterator for XidunionTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EnumTypeSerializerState::Init__ => {
                        *self.state = EnumTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "name", &self.value.name)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    EnumTypeSerializerState::Content__(x) => match x.next().transpose()? {
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
    impl<'ser> Iterator for EnumTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EnumTypeContentSerializerState::Init__ => {
                        *self.state = EnumTypeContentSerializerState::Item(
                            WithSerializer::serializer(&self.value.item, Some("item"), false)?,
                        );
                    }
                    EnumTypeContentSerializerState::Item(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = EnumTypeContentSerializerState::Doc(IterSerializer::new(
                                self.value.doc.as_ref(),
                                Some("doc"),
                                false,
                            ))
                        }
                    },
                    EnumTypeContentSerializerState::Doc(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = EnumTypeContentSerializerState::Done__,
                    },
                    EnumTypeContentSerializerState::Done__ => return Ok(None),
                    EnumTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for EnumTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TypedefTypeSerializerState::Init__ => {
                        *self.state = TypedefTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "oldname", &self.value.oldname)?;
                        write_attrib(&mut bytes, "newname", &self.value.newname)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    TypedefTypeSerializerState::Done__ => return Ok(None),
                    TypedefTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for TypedefTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PadTypeSerializerState::Init__ => {
                        *self.state = PadTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib_opt(&mut bytes, "bytes", &self.value.bytes)?;
                        write_attrib_opt(&mut bytes, "align", &self.value.align)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    PadTypeSerializerState::Done__ => return Ok(None),
                    PadTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for PadTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    VarTypeSerializerState::Init__ => {
                        *self.state = VarTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "name", &self.value.name)?;
                        write_attrib(&mut bytes, "type", &self.value.type_)?;
                        write_attrib_opt(&mut bytes, "enum", &self.value.enum_)?;
                        write_attrib_opt(&mut bytes, "altenum", &self.value.altenum)?;
                        write_attrib_opt(&mut bytes, "mask", &self.value.mask)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    VarTypeSerializerState::Done__ => return Ok(None),
                    VarTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for VarTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ListTypeSerializerState::Init__ => {
                        *self.state = ListTypeSerializerState::Content__(IterSerializer::new(
                            self.value.content.as_ref(),
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "name", &self.value.name)?;
                        write_attrib(&mut bytes, "type", &self.value.type_)?;
                        write_attrib_opt(&mut bytes, "enum", &self.value.enum_)?;
                        write_attrib_opt(&mut bytes, "altenum", &self.value.altenum)?;
                        write_attrib_opt(&mut bytes, "mask", &self.value.mask)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ListTypeSerializerState::Content__(x) => match x.next().transpose()? {
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
    impl<'ser> Iterator for ListTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                    ListTypeContentSerializerState::Op(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ListTypeContentSerializerState::Done__,
                    },
                    ListTypeContentSerializerState::Unop(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ListTypeContentSerializerState::Done__,
                    },
                    ListTypeContentSerializerState::Fieldref(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ListTypeContentSerializerState::Done__,
                    },
                    ListTypeContentSerializerState::Enumref(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ListTypeContentSerializerState::Done__,
                    },
                    ListTypeContentSerializerState::Popcount(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ListTypeContentSerializerState::Done__,
                    },
                    ListTypeContentSerializerState::Sumof(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ListTypeContentSerializerState::Done__,
                    },
                    ListTypeContentSerializerState::Value(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ListTypeContentSerializerState::Done__,
                    },
                    ListTypeContentSerializerState::Bit(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ListTypeContentSerializerState::Done__,
                    },
                    ListTypeContentSerializerState::Done__ => return Ok(None),
                    ListTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ListTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
    impl<'ser> Iterator for AnyTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ExprfieldTypeSerializerState::Init__ => {
                        *self.state = ExprfieldTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "name", &self.value.name)?;
                        write_attrib(&mut bytes, "type", &self.value.type_)?;
                        write_attrib_opt(&mut bytes, "enum", &self.value.enum_)?;
                        write_attrib_opt(&mut bytes, "altenum", &self.value.altenum)?;
                        write_attrib_opt(&mut bytes, "mask", &self.value.mask)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ExprfieldTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ExprfieldTypeSerializerState::End__,
                    },
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
    impl<'ser> Iterator for ExprfieldTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                    ExprfieldTypeContentSerializerState::Op(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ExprfieldTypeContentSerializerState::Done__,
                    },
                    ExprfieldTypeContentSerializerState::Unop(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ExprfieldTypeContentSerializerState::Done__,
                    },
                    ExprfieldTypeContentSerializerState::Fieldref(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ExprfieldTypeContentSerializerState::Done__,
                        }
                    }
                    ExprfieldTypeContentSerializerState::Enumref(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ExprfieldTypeContentSerializerState::Done__,
                        }
                    }
                    ExprfieldTypeContentSerializerState::Popcount(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ExprfieldTypeContentSerializerState::Done__,
                        }
                    }
                    ExprfieldTypeContentSerializerState::Sumof(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ExprfieldTypeContentSerializerState::Done__,
                    },
                    ExprfieldTypeContentSerializerState::Value(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ExprfieldTypeContentSerializerState::Done__,
                    },
                    ExprfieldTypeContentSerializerState::Bit(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ExprfieldTypeContentSerializerState::Done__,
                    },
                    ExprfieldTypeContentSerializerState::Done__ => return Ok(None),
                    ExprfieldTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ExprfieldTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ValueparamTypeSerializerState::Init__ => {
                        *self.state = ValueparamTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "value-mask-type", &self.value.value_mask_type)?;
                        write_attrib(&mut bytes, "value-mask-name", &self.value.value_mask_name)?;
                        write_attrib(&mut bytes, "value-list-name", &self.value.value_list_name)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    ValueparamTypeSerializerState::Done__ => return Ok(None),
                    ValueparamTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ValueparamTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SwitchexprTypeSerializerState::Init__ => {
                        *self.state = SwitchexprTypeSerializerState::Content__(
                            IterSerializer::new(&self.value.content[..], None, false),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "name", &self.value.name)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SwitchexprTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = SwitchexprTypeSerializerState::End__,
                    },
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
    impl<'ser> Iterator for SwitchexprTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                    SwitchexprTypeContentSerializerState::Op(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                    },
                    SwitchexprTypeContentSerializerState::Unop(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                    },
                    SwitchexprTypeContentSerializerState::Fieldref(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                        }
                    }
                    SwitchexprTypeContentSerializerState::Enumref(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                        }
                    }
                    SwitchexprTypeContentSerializerState::Popcount(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                        }
                    }
                    SwitchexprTypeContentSerializerState::Sumof(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                        }
                    }
                    SwitchexprTypeContentSerializerState::Value(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                        }
                    }
                    SwitchexprTypeContentSerializerState::Bit(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                    },
                    SwitchexprTypeContentSerializerState::Bitcase(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                        }
                    }
                    SwitchexprTypeContentSerializerState::Pad(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                    },
                    SwitchexprTypeContentSerializerState::Field(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                        }
                    }
                    SwitchexprTypeContentSerializerState::List(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                    },
                    SwitchexprTypeContentSerializerState::Fd(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = SwitchexprTypeContentSerializerState::Done__,
                    },
                    SwitchexprTypeContentSerializerState::Done__ => return Ok(None),
                    SwitchexprTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for SwitchexprTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    RequestReplyTypeSerializerState::Init__ => {
                        *self.state = RequestReplyTypeSerializerState::Content__(
                            IterSerializer::new(&self.value.content[..], None, false),
                        );
                        let bytes = BytesStart::new(self.name);
                        return Ok(Some(Event::Start(bytes)));
                    }
                    RequestReplyTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = RequestReplyTypeSerializerState::End__,
                    },
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
    impl<'ser> Iterator for RequestReplyTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestReplyTypeContentSerializerState::Done__,
                        }
                    }
                    RequestReplyTypeContentSerializerState::Field(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestReplyTypeContentSerializerState::Done__,
                        }
                    }
                    RequestReplyTypeContentSerializerState::List(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestReplyTypeContentSerializerState::Done__,
                        }
                    }
                    RequestReplyTypeContentSerializerState::Fd(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = RequestReplyTypeContentSerializerState::Done__,
                    },
                    RequestReplyTypeContentSerializerState::Valueparam(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestReplyTypeContentSerializerState::Done__,
                        }
                    }
                    RequestReplyTypeContentSerializerState::Switch(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RequestReplyTypeContentSerializerState::Done__,
                        }
                    }
                    RequestReplyTypeContentSerializerState::Doc(x) => {
                        match x.next().transpose()? {
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
    impl<'ser> Iterator for RequestReplyTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                    DocTypeSerializerState::Content__(x) => match x.next().transpose()? {
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
    impl<'ser> Iterator for DocTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                    DocTypeContentSerializerState::Brief(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DocTypeContentSerializerState::Done__,
                    },
                    DocTypeContentSerializerState::Description(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DocTypeContentSerializerState::Done__,
                    },
                    DocTypeContentSerializerState::Example(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DocTypeContentSerializerState::Done__,
                    },
                    DocTypeContentSerializerState::Field(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DocTypeContentSerializerState::Done__,
                    },
                    DocTypeContentSerializerState::Error(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DocTypeContentSerializerState::Done__,
                    },
                    DocTypeContentSerializerState::See(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DocTypeContentSerializerState::Done__,
                    },
                    DocTypeContentSerializerState::Done__ => return Ok(None),
                    DocTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DocTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EnumItemTypeSerializerState::Init__ => {
                        *self.state = EnumItemTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "name", &self.value.name)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    EnumItemTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = EnumItemTypeSerializerState::End__,
                    },
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
    impl<'ser> Iterator for EnumItemTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                    EnumItemTypeContentSerializerState::Value(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = EnumItemTypeContentSerializerState::Done__,
                    },
                    EnumItemTypeContentSerializerState::Bit(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = EnumItemTypeContentSerializerState::Done__,
                    },
                    EnumItemTypeContentSerializerState::Done__ => return Ok(None),
                    EnumItemTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for EnumItemTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    OpTypeSerializerState::Init__ => {
                        *self.state = OpTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "op", &self.value.op)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    OpTypeSerializerState::Content__(x) => match x.next().transpose()? {
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
    impl<'ser> Iterator for OpTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                                WithSerializer::serializer(&**x, Some("unop"), false)?,
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
                                WithSerializer::serializer(&**x, Some("popcount"), false)?,
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
                    OpTypeContentSerializerState::Op(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = OpTypeContentSerializerState::Done__,
                    },
                    OpTypeContentSerializerState::Unop(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = OpTypeContentSerializerState::Done__,
                    },
                    OpTypeContentSerializerState::Fieldref(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = OpTypeContentSerializerState::Done__,
                    },
                    OpTypeContentSerializerState::Enumref(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = OpTypeContentSerializerState::Done__,
                    },
                    OpTypeContentSerializerState::Popcount(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = OpTypeContentSerializerState::Done__,
                    },
                    OpTypeContentSerializerState::Sumof(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = OpTypeContentSerializerState::Done__,
                    },
                    OpTypeContentSerializerState::Value(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = OpTypeContentSerializerState::Done__,
                    },
                    OpTypeContentSerializerState::Bit(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = OpTypeContentSerializerState::Done__,
                    },
                    OpTypeContentSerializerState::Done__ => return Ok(None),
                    OpTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for OpTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        Content__(IterSerializer<'ser, &'ser [super::UnopTypeContent], super::UnopTypeContent>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> UnopTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    UnopTypeSerializerState::Init__ => {
                        *self.state = UnopTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "op", &self.value.op)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    UnopTypeSerializerState::Content__(x) => match x.next().transpose()? {
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
    impl<'ser> Iterator for UnopTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                                WithSerializer::serializer(&**x, Some("popcount"), false)?,
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
                    UnopTypeContentSerializerState::Op(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = UnopTypeContentSerializerState::Done__,
                    },
                    UnopTypeContentSerializerState::Unop(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = UnopTypeContentSerializerState::Done__,
                    },
                    UnopTypeContentSerializerState::Fieldref(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = UnopTypeContentSerializerState::Done__,
                    },
                    UnopTypeContentSerializerState::Enumref(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = UnopTypeContentSerializerState::Done__,
                    },
                    UnopTypeContentSerializerState::Popcount(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = UnopTypeContentSerializerState::Done__,
                    },
                    UnopTypeContentSerializerState::Sumof(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = UnopTypeContentSerializerState::Done__,
                    },
                    UnopTypeContentSerializerState::Value(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = UnopTypeContentSerializerState::Done__,
                    },
                    UnopTypeContentSerializerState::Bit(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = UnopTypeContentSerializerState::Done__,
                    },
                    UnopTypeContentSerializerState::Done__ => return Ok(None),
                    UnopTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for UnopTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    EnumrefTypeSerializerState::Init__ => {
                        *self.state = EnumrefTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "ref", &self.value.ref_)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    EnumrefTypeSerializerState::Content__(x) => match x.next().transpose()? {
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
    impl<'ser> Iterator for EnumrefTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PopcountTypeSerializerState::Init__ => {
                        match self.value {
                            super::PopcountType::Op(x) => {
                                *self.state = PopcountTypeSerializerState::Op(
                                    WithSerializer::serializer(&**x, Some("op"), false)?,
                                )
                            }
                            super::PopcountType::Unop(x) => {
                                *self.state = PopcountTypeSerializerState::Unop(
                                    WithSerializer::serializer(&**x, Some("unop"), false)?,
                                )
                            }
                            super::PopcountType::Fieldref(x) => {
                                *self.state = PopcountTypeSerializerState::Fieldref(
                                    WithSerializer::serializer(x, Some("fieldref"), false)?,
                                )
                            }
                            super::PopcountType::Enumref(x) => {
                                *self.state = PopcountTypeSerializerState::Enumref(
                                    WithSerializer::serializer(x, Some("enumref"), false)?,
                                )
                            }
                            super::PopcountType::Popcount(x) => {
                                *self.state = PopcountTypeSerializerState::Popcount(
                                    WithSerializer::serializer(&**x, Some("popcount"), false)?,
                                )
                            }
                            super::PopcountType::Sumof(x) => {
                                *self.state = PopcountTypeSerializerState::Sumof(
                                    WithSerializer::serializer(x, Some("sumof"), false)?,
                                )
                            }
                            super::PopcountType::Value(x) => {
                                *self.state = PopcountTypeSerializerState::Value(
                                    WithSerializer::serializer(x, Some("value"), false)?,
                                )
                            }
                            super::PopcountType::Bit(x) => {
                                *self.state = PopcountTypeSerializerState::Bit(
                                    WithSerializer::serializer(x, Some("bit"), false)?,
                                )
                            }
                        }
                        let bytes = BytesStart::new(self.name);
                        return Ok(Some(Event::Start(bytes)));
                    }
                    PopcountTypeSerializerState::Op(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PopcountTypeSerializerState::End__,
                    },
                    PopcountTypeSerializerState::Unop(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PopcountTypeSerializerState::End__,
                    },
                    PopcountTypeSerializerState::Fieldref(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PopcountTypeSerializerState::End__,
                    },
                    PopcountTypeSerializerState::Enumref(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PopcountTypeSerializerState::End__,
                    },
                    PopcountTypeSerializerState::Popcount(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PopcountTypeSerializerState::End__,
                    },
                    PopcountTypeSerializerState::Sumof(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PopcountTypeSerializerState::End__,
                    },
                    PopcountTypeSerializerState::Value(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PopcountTypeSerializerState::End__,
                    },
                    PopcountTypeSerializerState::Bit(x) => match x.next().transpose()? {
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
    impl<'ser> Iterator for PopcountTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SumofTypeSerializerState::Init__ => {
                        *self.state = SumofTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "ref", &self.value.ref_)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    SumofTypeSerializerState::Done__ => return Ok(None),
                    SumofTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for SumofTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CaseexprTypeSerializerState::Init__ => {
                        *self.state = CaseexprTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib_opt(&mut bytes, "name", &self.value.name)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    CaseexprTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CaseexprTypeSerializerState::End__,
                    },
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
    impl<'ser> Iterator for CaseexprTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                    CaseexprTypeContentSerializerState::Op(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CaseexprTypeContentSerializerState::Done__,
                    },
                    CaseexprTypeContentSerializerState::Unop(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CaseexprTypeContentSerializerState::Done__,
                    },
                    CaseexprTypeContentSerializerState::Fieldref(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CaseexprTypeContentSerializerState::Done__,
                        }
                    }
                    CaseexprTypeContentSerializerState::Enumref(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CaseexprTypeContentSerializerState::Done__,
                        }
                    }
                    CaseexprTypeContentSerializerState::Popcount(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CaseexprTypeContentSerializerState::Done__,
                        }
                    }
                    CaseexprTypeContentSerializerState::Sumof(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CaseexprTypeContentSerializerState::Done__,
                    },
                    CaseexprTypeContentSerializerState::Value(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CaseexprTypeContentSerializerState::Done__,
                    },
                    CaseexprTypeContentSerializerState::Bit(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CaseexprTypeContentSerializerState::Done__,
                    },
                    CaseexprTypeContentSerializerState::Pad(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CaseexprTypeContentSerializerState::Done__,
                    },
                    CaseexprTypeContentSerializerState::Field(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CaseexprTypeContentSerializerState::Done__,
                    },
                    CaseexprTypeContentSerializerState::List(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CaseexprTypeContentSerializerState::Done__,
                    },
                    CaseexprTypeContentSerializerState::Fd(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CaseexprTypeContentSerializerState::Done__,
                    },
                    CaseexprTypeContentSerializerState::Switch(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CaseexprTypeContentSerializerState::Done__,
                    },
                    CaseexprTypeContentSerializerState::Done__ => return Ok(None),
                    CaseexprTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for CaseexprTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FieldTypeSerializerState::Init__ => {
                        *self.state = FieldTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib_opt(&mut bytes, "name", &self.value.name)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    FieldTypeSerializerState::Content__(x) => match x.next().transpose()? {
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
    impl<'ser> Iterator for FieldTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ErrorTypeSerializerState::Init__ => {
                        *self.state = ErrorTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib_opt(&mut bytes, "type", &self.value.type_)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ErrorTypeSerializerState::Content__(x) => match x.next().transpose()? {
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
    impl<'ser> Iterator for ErrorTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SeeTypeSerializerState::Init__ => {
                        *self.state = SeeTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib_opt(&mut bytes, "name", &self.value.name)?;
                        write_attrib_opt(&mut bytes, "type", &self.value.type_)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    SeeTypeSerializerState::Done__ => return Ok(None),
                    SeeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for SeeTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
