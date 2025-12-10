use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_UNNAMED_2: Namespace =
    Namespace::new_const(b"http://www.idealdesk.com/ideal/messages/mer-acq/3.3.1");
pub const NS_DS: Namespace = Namespace::new_const(b"http://www.w3.org/2000/09/xmldsig#");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_DS: NamespacePrefix = NamespacePrefix::new_const(b"ds");
pub type DirectoryReq = DirectoryReqType;
#[derive(Debug)]
pub struct DirectoryReqType {
    pub version: String,
    pub create_date_timestamp: String,
    pub merchant: DirectoryReqMerchantType,
    pub signature: SignatureType,
}
impl WithSerializer for DirectoryReqType {
    type Serializer<'x> = quick_xml_serialize::DirectoryReqTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::DirectoryReqTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DirectoryReqTypeSerializerState::Init__),
            name: name.unwrap_or("DirectoryReq"),
            is_root,
        })
    }
}
impl WithDeserializer for DirectoryReqType {
    type Deserializer = quick_xml_deserialize::DirectoryReqTypeDeserializer;
}
#[derive(Debug)]
pub struct DirectoryReqMerchantType {
    pub merchant_id: String,
    pub sub_id: usize,
}
impl WithSerializer for DirectoryReqMerchantType {
    type Serializer<'x> = quick_xml_serialize::DirectoryReqMerchantTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::DirectoryReqMerchantTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DirectoryReqMerchantTypeSerializerState::Init__),
            name: name.unwrap_or("DirectoryReqMerchant"),
            is_root,
        })
    }
}
impl WithDeserializer for DirectoryReqMerchantType {
    type Deserializer = quick_xml_deserialize::DirectoryReqMerchantTypeDeserializer;
}
#[derive(Debug)]
pub struct SignatureType {
    pub id: Option<String>,
    pub signed_info: SignedInfoType,
    pub signature_value: SignatureValueType,
    pub key_info: Option<KeyInfoType>,
    pub object: Vec<ObjectType>,
}
impl WithSerializer for SignatureType {
    type Serializer<'x> = quick_xml_serialize::SignatureTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::SignatureTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SignatureTypeSerializerState::Init__),
            name: name.unwrap_or("ds:SignatureType"),
            is_root,
        })
    }
}
impl WithDeserializer for SignatureType {
    type Deserializer = quick_xml_deserialize::SignatureTypeDeserializer;
}
#[derive(Debug)]
pub struct SignedInfoType {
    pub id: Option<String>,
    pub canonicalization_method: CanonicalizationMethodType,
    pub signature_method: SignatureMethodType,
    pub reference: Vec<ReferenceType>,
}
impl WithSerializer for SignedInfoType {
    type Serializer<'x> = quick_xml_serialize::SignedInfoTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::SignedInfoTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SignedInfoTypeSerializerState::Init__),
            name: name.unwrap_or("ds:SignedInfoType"),
            is_root,
        })
    }
}
impl WithDeserializer for SignedInfoType {
    type Deserializer = quick_xml_deserialize::SignedInfoTypeDeserializer;
}
#[derive(Debug)]
pub struct SignatureValueType {
    pub id: Option<String>,
    pub content: String,
}
impl WithSerializer for SignatureValueType {
    type Serializer<'x> = quick_xml_serialize::SignatureValueTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::SignatureValueTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SignatureValueTypeSerializerState::Init__),
            name: name.unwrap_or("ds:SignatureValueType"),
            is_root,
        })
    }
}
impl WithDeserializer for SignatureValueType {
    type Deserializer = quick_xml_deserialize::SignatureValueTypeDeserializer;
}
#[derive(Debug)]
pub struct KeyInfoType {
    pub id: Option<String>,
    pub content: Vec<KeyInfoTypeContent>,
}
#[derive(Debug)]
pub enum KeyInfoTypeContent {
    KeyName(String),
    KeyValue(KeyValueType),
    RetrievalMethod(RetrievalMethodType),
    X509Data(X509DataType),
    PgpData(PgpDataType),
    SpkiData(SpkiDataType),
    MgmtData(String),
}
impl WithSerializer for KeyInfoType {
    type Serializer<'x> = quick_xml_serialize::KeyInfoTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::KeyInfoTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::KeyInfoTypeSerializerState::Init__),
            name: name.unwrap_or("ds:KeyInfoType"),
            is_root,
        })
    }
}
impl WithSerializer for KeyInfoTypeContent {
    type Serializer<'x> = quick_xml_serialize::KeyInfoTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::KeyInfoTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::KeyInfoTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for KeyInfoType {
    type Deserializer = quick_xml_deserialize::KeyInfoTypeDeserializer;
}
impl WithDeserializer for KeyInfoTypeContent {
    type Deserializer = quick_xml_deserialize::KeyInfoTypeContentDeserializer;
}
#[derive(Debug)]
pub struct ObjectType {
    pub id: Option<String>,
    pub mime_type: Option<String>,
    pub encoding: Option<String>,
}
impl WithSerializer for ObjectType {
    type Serializer<'x> = quick_xml_serialize::ObjectTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ObjectTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ObjectTypeSerializerState::Init__),
            name: name.unwrap_or("ds:ObjectType"),
            is_root,
        })
    }
}
impl WithDeserializer for ObjectType {
    type Deserializer = quick_xml_deserialize::ObjectTypeDeserializer;
}
#[derive(Debug)]
pub struct CanonicalizationMethodType {
    pub algorithm: String,
}
impl WithSerializer for CanonicalizationMethodType {
    type Serializer<'x> = quick_xml_serialize::CanonicalizationMethodTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::CanonicalizationMethodTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::CanonicalizationMethodTypeSerializerState::Init__),
            name: name.unwrap_or("ds:CanonicalizationMethodType"),
            is_root,
        })
    }
}
impl WithDeserializer for CanonicalizationMethodType {
    type Deserializer = quick_xml_deserialize::CanonicalizationMethodTypeDeserializer;
}
#[derive(Debug)]
pub struct SignatureMethodType {
    pub algorithm: String,
    pub hmac_output_length: Option<i32>,
}
impl WithSerializer for SignatureMethodType {
    type Serializer<'x> = quick_xml_serialize::SignatureMethodTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::SignatureMethodTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SignatureMethodTypeSerializerState::Init__),
            name: name.unwrap_or("ds:SignatureMethodType"),
            is_root,
        })
    }
}
impl WithDeserializer for SignatureMethodType {
    type Deserializer = quick_xml_deserialize::SignatureMethodTypeDeserializer;
}
#[derive(Debug)]
pub struct ReferenceType {
    pub id: Option<String>,
    pub uri: Option<String>,
    pub type_: Option<String>,
    pub transforms: Option<TransformsType>,
    pub digest_method: DigestMethodType,
    pub digest_value: String,
}
impl WithSerializer for ReferenceType {
    type Serializer<'x> = quick_xml_serialize::ReferenceTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ReferenceTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ReferenceTypeSerializerState::Init__),
            name: name.unwrap_or("ds:ReferenceType"),
            is_root,
        })
    }
}
impl WithDeserializer for ReferenceType {
    type Deserializer = quick_xml_deserialize::ReferenceTypeDeserializer;
}
#[derive(Debug)]
pub struct KeyValueType {
    pub content: KeyValueTypeContent,
}
#[derive(Debug)]
pub enum KeyValueTypeContent {
    DsaKeyValue(DsaKeyValueType),
    RsaKeyValue(RsaKeyValueType),
}
impl WithSerializer for KeyValueType {
    type Serializer<'x> = quick_xml_serialize::KeyValueTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::KeyValueTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::KeyValueTypeSerializerState::Init__),
            name: name.unwrap_or("ds:KeyValueType"),
            is_root,
        })
    }
}
impl WithSerializer for KeyValueTypeContent {
    type Serializer<'x> = quick_xml_serialize::KeyValueTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::KeyValueTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::KeyValueTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for KeyValueType {
    type Deserializer = quick_xml_deserialize::KeyValueTypeDeserializer;
}
impl WithDeserializer for KeyValueTypeContent {
    type Deserializer = quick_xml_deserialize::KeyValueTypeContentDeserializer;
}
#[derive(Debug)]
pub struct RetrievalMethodType {
    pub uri: Option<String>,
    pub type_: Option<String>,
    pub transforms: Option<TransformsType>,
}
impl WithSerializer for RetrievalMethodType {
    type Serializer<'x> = quick_xml_serialize::RetrievalMethodTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::RetrievalMethodTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::RetrievalMethodTypeSerializerState::Init__),
            name: name.unwrap_or("ds:RetrievalMethodType"),
            is_root,
        })
    }
}
impl WithDeserializer for RetrievalMethodType {
    type Deserializer = quick_xml_deserialize::RetrievalMethodTypeDeserializer;
}
#[derive(Debug)]
pub struct X509DataType {
    pub content: Vec<X509DataTypeContent>,
}
#[derive(Debug)]
pub struct X509DataTypeContent {
    pub content_43: X509DataContent43Type,
}
impl WithSerializer for X509DataType {
    type Serializer<'x> = quick_xml_serialize::X509DataTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::X509DataTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::X509DataTypeSerializerState::Init__),
            name: name.unwrap_or("ds:X509DataType"),
            is_root,
        })
    }
}
impl WithSerializer for X509DataTypeContent {
    type Serializer<'x> = quick_xml_serialize::X509DataTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::X509DataTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::X509DataTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for X509DataType {
    type Deserializer = quick_xml_deserialize::X509DataTypeDeserializer;
}
impl WithDeserializer for X509DataTypeContent {
    type Deserializer = quick_xml_deserialize::X509DataTypeContentDeserializer;
}
#[derive(Debug)]
pub struct PgpDataType {
    pub content: PgpDataTypeContent,
}
#[derive(Debug)]
pub enum PgpDataTypeContent {
    Content47(PgpDataContent47Type),
    Content49(PgpDataContent49Type),
}
impl WithSerializer for PgpDataType {
    type Serializer<'x> = quick_xml_serialize::PgpDataTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::PgpDataTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::PgpDataTypeSerializerState::Init__),
            name: name.unwrap_or("ds:PGPDataType"),
            is_root,
        })
    }
}
impl WithSerializer for PgpDataTypeContent {
    type Serializer<'x> = quick_xml_serialize::PgpDataTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::PgpDataTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::PgpDataTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for PgpDataType {
    type Deserializer = quick_xml_deserialize::PgpDataTypeDeserializer;
}
impl WithDeserializer for PgpDataTypeContent {
    type Deserializer = quick_xml_deserialize::PgpDataTypeContentDeserializer;
}
#[derive(Debug)]
pub struct SpkiDataType {
    pub content: Vec<SpkiDataTypeContent>,
}
#[derive(Debug)]
pub struct SpkiDataTypeContent {
    pub spki_sexp: String,
}
impl WithSerializer for SpkiDataType {
    type Serializer<'x> = quick_xml_serialize::SpkiDataTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::SpkiDataTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SpkiDataTypeSerializerState::Init__),
            name: name.unwrap_or("ds:SPKIDataType"),
            is_root,
        })
    }
}
impl WithSerializer for SpkiDataTypeContent {
    type Serializer<'x> = quick_xml_serialize::SpkiDataTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::SpkiDataTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SpkiDataTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for SpkiDataType {
    type Deserializer = quick_xml_deserialize::SpkiDataTypeDeserializer;
}
impl WithDeserializer for SpkiDataTypeContent {
    type Deserializer = quick_xml_deserialize::SpkiDataTypeContentDeserializer;
}
#[derive(Debug)]
pub struct TransformsType {
    pub transform: Vec<TransformType>,
}
impl WithSerializer for TransformsType {
    type Serializer<'x> = quick_xml_serialize::TransformsTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TransformsTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TransformsTypeSerializerState::Init__),
            name: name.unwrap_or("ds:TransformsType"),
            is_root,
        })
    }
}
impl WithDeserializer for TransformsType {
    type Deserializer = quick_xml_deserialize::TransformsTypeDeserializer;
}
#[derive(Debug)]
pub struct DigestMethodType {
    pub algorithm: String,
}
impl WithSerializer for DigestMethodType {
    type Serializer<'x> = quick_xml_serialize::DigestMethodTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::DigestMethodTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DigestMethodTypeSerializerState::Init__),
            name: name.unwrap_or("ds:DigestMethodType"),
            is_root,
        })
    }
}
impl WithDeserializer for DigestMethodType {
    type Deserializer = quick_xml_deserialize::DigestMethodTypeDeserializer;
}
#[derive(Debug)]
pub struct DsaKeyValueType {
    pub content_60: Option<DsaKeyValueContent60Type>,
    pub g: Option<String>,
    pub y: String,
    pub j: Option<String>,
    pub content_61: Option<DsaKeyValueContent61Type>,
}
impl WithSerializer for DsaKeyValueType {
    type Serializer<'x> = quick_xml_serialize::DsaKeyValueTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::DsaKeyValueTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DsaKeyValueTypeSerializerState::Init__),
            name: name.unwrap_or("ds:DSAKeyValueType"),
            is_root,
        })
    }
}
impl WithDeserializer for DsaKeyValueType {
    type Deserializer = quick_xml_deserialize::DsaKeyValueTypeDeserializer;
}
#[derive(Debug)]
pub struct RsaKeyValueType {
    pub modulus: String,
    pub exponent: String,
}
impl WithSerializer for RsaKeyValueType {
    type Serializer<'x> = quick_xml_serialize::RsaKeyValueTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::RsaKeyValueTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::RsaKeyValueTypeSerializerState::Init__),
            name: name.unwrap_or("ds:RSAKeyValueType"),
            is_root,
        })
    }
}
impl WithDeserializer for RsaKeyValueType {
    type Deserializer = quick_xml_deserialize::RsaKeyValueTypeDeserializer;
}
#[derive(Debug)]
pub struct X509DataContent43Type {
    pub content: X509DataContent43TypeContent,
}
#[derive(Debug)]
pub enum X509DataContent43TypeContent {
    X509IssuerSerial(X509IssuerSerialType),
    X509Ski(String),
    X509SubjectName(String),
    X509Certificate(String),
    X509Crl(String),
}
impl WithSerializer for X509DataContent43Type {
    type Serializer<'x> = quick_xml_serialize::X509DataContent43TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::X509DataContent43TypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::X509DataContent43TypeSerializerState::Init__),
        })
    }
}
impl WithSerializer for X509DataContent43TypeContent {
    type Serializer<'x> = quick_xml_serialize::X509DataContent43TypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(
            quick_xml_serialize::X509DataContent43TypeContentSerializer {
                value: self,
                state: Box::new(
                    quick_xml_serialize::X509DataContent43TypeContentSerializerState::Init__,
                ),
            },
        )
    }
}
impl WithDeserializer for X509DataContent43Type {
    type Deserializer = quick_xml_deserialize::X509DataContent43TypeDeserializer;
}
impl WithDeserializer for X509DataContent43TypeContent {
    type Deserializer = quick_xml_deserialize::X509DataContent43TypeContentDeserializer;
}
#[derive(Debug)]
pub struct PgpDataContent47Type {
    pub pgp_key_id: String,
    pub pgp_key_packet: Option<String>,
}
impl WithSerializer for PgpDataContent47Type {
    type Serializer<'x> = quick_xml_serialize::PgpDataContent47TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::PgpDataContent47TypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::PgpDataContent47TypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for PgpDataContent47Type {
    type Deserializer = quick_xml_deserialize::PgpDataContent47TypeDeserializer;
}
#[derive(Debug)]
pub struct PgpDataContent49Type {
    pub pgp_key_packet: String,
}
impl WithSerializer for PgpDataContent49Type {
    type Serializer<'x> = quick_xml_serialize::PgpDataContent49TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::PgpDataContent49TypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::PgpDataContent49TypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for PgpDataContent49Type {
    type Deserializer = quick_xml_deserialize::PgpDataContent49TypeDeserializer;
}
#[derive(Debug)]
pub struct TransformType {
    pub algorithm: String,
    pub content: Vec<TransformTypeContent>,
}
#[derive(Debug)]
pub enum TransformTypeContent {
    XPath(String),
}
impl WithSerializer for TransformType {
    type Serializer<'x> = quick_xml_serialize::TransformTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TransformTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TransformTypeSerializerState::Init__),
            name: name.unwrap_or("ds:TransformType"),
            is_root,
        })
    }
}
impl WithSerializer for TransformTypeContent {
    type Serializer<'x> = quick_xml_serialize::TransformTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::TransformTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TransformTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for TransformType {
    type Deserializer = quick_xml_deserialize::TransformTypeDeserializer;
}
impl WithDeserializer for TransformTypeContent {
    type Deserializer = quick_xml_deserialize::TransformTypeContentDeserializer;
}
#[derive(Debug)]
pub struct DsaKeyValueContent60Type {
    pub p: String,
    pub q: String,
}
impl WithSerializer for DsaKeyValueContent60Type {
    type Serializer<'x> = quick_xml_serialize::DsaKeyValueContent60TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::DsaKeyValueContent60TypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DsaKeyValueContent60TypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for DsaKeyValueContent60Type {
    type Deserializer = quick_xml_deserialize::DsaKeyValueContent60TypeDeserializer;
}
#[derive(Debug)]
pub struct DsaKeyValueContent61Type {
    pub seed: String,
    pub pgen_counter: String,
}
impl WithSerializer for DsaKeyValueContent61Type {
    type Serializer<'x> = quick_xml_serialize::DsaKeyValueContent61TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::DsaKeyValueContent61TypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DsaKeyValueContent61TypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for DsaKeyValueContent61Type {
    type Deserializer = quick_xml_deserialize::DsaKeyValueContent61TypeDeserializer;
}
#[derive(Debug)]
pub struct X509IssuerSerialType {
    pub x509_issuer_name: String,
    pub x509_serial_number: i32,
}
impl WithSerializer for X509IssuerSerialType {
    type Serializer<'x> = quick_xml_serialize::X509IssuerSerialTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::X509IssuerSerialTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::X509IssuerSerialTypeSerializerState::Init__),
            name: name.unwrap_or("ds:X509IssuerSerialType"),
            is_root,
        })
    }
}
impl WithDeserializer for X509IssuerSerialType {
    type Deserializer = quick_xml_deserialize::X509IssuerSerialTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, ContentDeserializer, DeserializeHelper, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
        ErrorKind, Event, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct DirectoryReqTypeDeserializer {
        version: String,
        create_date_timestamp: Option<String>,
        merchant: Option<super::DirectoryReqMerchantType>,
        signature: Option<super::SignatureType>,
        state__: Box<DirectoryReqTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DirectoryReqTypeDeserializerState {
        Init__,
        CreateDateTimestamp(Option<<String as WithDeserializer>::Deserializer>),
        Merchant(Option<<super::DirectoryReqMerchantType as WithDeserializer>::Deserializer>),
        Signature(Option<<super::SignatureType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DirectoryReqTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut version: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"version")
                ) {
                    helper.read_attrib(&mut version, b"version", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                version: version.ok_or_else(|| ErrorKind::MissingAttribute("version".into()))?,
                create_date_timestamp: None,
                merchant: None,
                signature: None,
                state__: Box::new(DirectoryReqTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: DirectoryReqTypeDeserializerState,
        ) -> Result<(), Error> {
            use DirectoryReqTypeDeserializerState as S;
            match state {
                S::CreateDateTimestamp(Some(deserializer)) => {
                    self.store_create_date_timestamp(deserializer.finish(helper)?)?
                }
                S::Merchant(Some(deserializer)) => {
                    self.store_merchant(deserializer.finish(helper)?)?
                }
                S::Signature(Some(deserializer)) => {
                    self.store_signature(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_create_date_timestamp(&mut self, value: String) -> Result<(), Error> {
            if self.create_date_timestamp.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"createDateTimestamp",
                )))?;
            }
            self.create_date_timestamp = Some(value);
            Ok(())
        }
        fn store_merchant(&mut self, value: super::DirectoryReqMerchantType) -> Result<(), Error> {
            if self.merchant.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Merchant",
                )))?;
            }
            self.merchant = Some(value);
            Ok(())
        }
        fn store_signature(&mut self, value: super::SignatureType) -> Result<(), Error> {
            if self.signature.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Signature",
                )))?;
            }
            self.signature = Some(value);
            Ok(())
        }
        fn handle_create_date_timestamp<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DirectoryReqTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DirectoryReqTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::CreateDateTimestamp(None));
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
                    self.store_create_date_timestamp(data)?;
                    *self.state__ = S::Merchant(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::CreateDateTimestamp(Some(deserializer)));
                    *self.state__ = S::Merchant(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_merchant<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::DirectoryReqMerchantType>,
            fallback: &mut Option<DirectoryReqTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DirectoryReqTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Merchant(None));
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
                    self.store_merchant(data)?;
                    *self.state__ = S::Signature(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Merchant(Some(deserializer)));
                    *self.state__ = S::Signature(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_signature<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::SignatureType>,
            fallback: &mut Option<DirectoryReqTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DirectoryReqTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Signature(None));
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
                    self.store_signature(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Signature(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DirectoryReqType> for DirectoryReqTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DirectoryReqType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DirectoryReqType> {
            use DirectoryReqTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::CreateDateTimestamp(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_create_date_timestamp(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Merchant(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_merchant(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Signature(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_signature(helper, output, &mut fallback)? {
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
                        *self.state__ = S::CreateDateTimestamp(None);
                        event
                    }
                    (S::CreateDateTimestamp(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"createDateTimestamp",
                            false,
                        )?;
                        match self.handle_create_date_timestamp(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Merchant(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"Merchant",
                            false,
                        )?;
                        match self.handle_merchant(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Signature(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"Signature",
                            true,
                        )?;
                        match self.handle_signature(helper, output, &mut fallback)? {
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
        ) -> Result<super::DirectoryReqType, Error> {
            let state = replace(
                &mut *self.state__,
                DirectoryReqTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::DirectoryReqType {
                version: self.version,
                create_date_timestamp: helper
                    .finish_element("createDateTimestamp", self.create_date_timestamp)?,
                merchant: helper.finish_element("Merchant", self.merchant)?,
                signature: helper.finish_element("Signature", self.signature)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DirectoryReqMerchantTypeDeserializer {
        merchant_id: Option<String>,
        sub_id: Option<usize>,
        state__: Box<DirectoryReqMerchantTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DirectoryReqMerchantTypeDeserializerState {
        Init__,
        MerchantId(Option<<String as WithDeserializer>::Deserializer>),
        SubId(Option<<usize as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DirectoryReqMerchantTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                merchant_id: None,
                sub_id: None,
                state__: Box::new(DirectoryReqMerchantTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: DirectoryReqMerchantTypeDeserializerState,
        ) -> Result<(), Error> {
            use DirectoryReqMerchantTypeDeserializerState as S;
            match state {
                S::MerchantId(Some(deserializer)) => {
                    self.store_merchant_id(deserializer.finish(helper)?)?
                }
                S::SubId(Some(deserializer)) => self.store_sub_id(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_merchant_id(&mut self, value: String) -> Result<(), Error> {
            if self.merchant_id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"merchantID",
                )))?;
            }
            self.merchant_id = Some(value);
            Ok(())
        }
        fn store_sub_id(&mut self, value: usize) -> Result<(), Error> {
            if self.sub_id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"subID",
                )))?;
            }
            self.sub_id = Some(value);
            Ok(())
        }
        fn handle_merchant_id<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DirectoryReqMerchantTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DirectoryReqMerchantTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::MerchantId(None));
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
                    self.store_merchant_id(data)?;
                    *self.state__ = S::SubId(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::MerchantId(Some(deserializer)));
                    *self.state__ = S::SubId(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_sub_id<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, usize>,
            fallback: &mut Option<DirectoryReqMerchantTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DirectoryReqMerchantTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::SubId(None));
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
                    self.store_sub_id(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::SubId(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DirectoryReqMerchantType>
        for DirectoryReqMerchantTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DirectoryReqMerchantType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DirectoryReqMerchantType> {
            use DirectoryReqMerchantTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::MerchantId(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_merchant_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SubId(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_sub_id(helper, output, &mut fallback)? {
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
                        *self.state__ = S::MerchantId(None);
                        event
                    }
                    (S::MerchantId(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"merchantID",
                            false,
                        )?;
                        match self.handle_merchant_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SubId(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"subID",
                            false,
                        )?;
                        match self.handle_sub_id(helper, output, &mut fallback)? {
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
        ) -> Result<super::DirectoryReqMerchantType, Error> {
            let state = replace(
                &mut *self.state__,
                DirectoryReqMerchantTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::DirectoryReqMerchantType {
                merchant_id: helper.finish_element("merchantID", self.merchant_id)?,
                sub_id: helper.finish_element("subID", self.sub_id)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct SignatureTypeDeserializer {
        id: Option<String>,
        signed_info: Option<super::SignedInfoType>,
        signature_value: Option<super::SignatureValueType>,
        key_info: Option<super::KeyInfoType>,
        object: Vec<super::ObjectType>,
        state__: Box<SignatureTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SignatureTypeDeserializerState {
        Init__,
        SignedInfo(Option<<super::SignedInfoType as WithDeserializer>::Deserializer>),
        SignatureValue(Option<<super::SignatureValueType as WithDeserializer>::Deserializer>),
        KeyInfo(Option<<super::KeyInfoType as WithDeserializer>::Deserializer>),
        Object(Option<<super::ObjectType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SignatureTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut id: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Id")
                ) {
                    helper.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id,
                signed_info: None,
                signature_value: None,
                key_info: None,
                object: Vec::new(),
                state__: Box::new(SignatureTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SignatureTypeDeserializerState,
        ) -> Result<(), Error> {
            use SignatureTypeDeserializerState as S;
            match state {
                S::SignedInfo(Some(deserializer)) => {
                    self.store_signed_info(deserializer.finish(helper)?)?
                }
                S::SignatureValue(Some(deserializer)) => {
                    self.store_signature_value(deserializer.finish(helper)?)?
                }
                S::KeyInfo(Some(deserializer)) => {
                    self.store_key_info(deserializer.finish(helper)?)?
                }
                S::Object(Some(deserializer)) => self.store_object(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_signed_info(&mut self, value: super::SignedInfoType) -> Result<(), Error> {
            if self.signed_info.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SignedInfo",
                )))?;
            }
            self.signed_info = Some(value);
            Ok(())
        }
        fn store_signature_value(&mut self, value: super::SignatureValueType) -> Result<(), Error> {
            if self.signature_value.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SignatureValue",
                )))?;
            }
            self.signature_value = Some(value);
            Ok(())
        }
        fn store_key_info(&mut self, value: super::KeyInfoType) -> Result<(), Error> {
            if self.key_info.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"KeyInfo",
                )))?;
            }
            self.key_info = Some(value);
            Ok(())
        }
        fn store_object(&mut self, value: super::ObjectType) -> Result<(), Error> {
            self.object.push(value);
            Ok(())
        }
        fn handle_signed_info<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::SignedInfoType>,
            fallback: &mut Option<SignatureTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SignatureTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::SignedInfo(None));
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
                    self.store_signed_info(data)?;
                    *self.state__ = S::SignatureValue(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::SignedInfo(Some(deserializer)));
                    *self.state__ = S::SignatureValue(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_signature_value<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::SignatureValueType>,
            fallback: &mut Option<SignatureTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SignatureTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::SignatureValue(None));
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
                    self.store_signature_value(data)?;
                    *self.state__ = S::KeyInfo(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::SignatureValue(Some(deserializer)));
                    *self.state__ = S::KeyInfo(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_key_info<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::KeyInfoType>,
            fallback: &mut Option<SignatureTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SignatureTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::KeyInfo(None));
                *self.state__ = S::Object(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_key_info(data)?;
                    *self.state__ = S::Object(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::KeyInfo(Some(deserializer)));
                    *self.state__ = S::Object(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_object<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ObjectType>,
            fallback: &mut Option<SignatureTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SignatureTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Object(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_object(data)?;
                    *self.state__ = S::Object(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Object(Some(deserializer)));
                    *self.state__ = S::Object(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SignatureType> for SignatureTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SignatureType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SignatureType> {
            use SignatureTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::SignedInfo(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_signed_info(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SignatureValue(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_signature_value(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::KeyInfo(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_key_info(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Object(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_object(helper, output, &mut fallback)? {
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
                        *self.state__ = S::SignedInfo(None);
                        event
                    }
                    (S::SignedInfo(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"SignedInfo",
                            true,
                        )?;
                        match self.handle_signed_info(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SignatureValue(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"SignatureValue",
                            false,
                        )?;
                        match self.handle_signature_value(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::KeyInfo(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"KeyInfo",
                            true,
                        )?;
                        match self.handle_key_info(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Object(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"Object",
                            true,
                        )?;
                        match self.handle_object(helper, output, &mut fallback)? {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::SignatureType, Error> {
            let state = replace(
                &mut *self.state__,
                SignatureTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::SignatureType {
                id: self.id,
                signed_info: helper.finish_element("SignedInfo", self.signed_info)?,
                signature_value: helper.finish_element("SignatureValue", self.signature_value)?,
                key_info: self.key_info,
                object: self.object,
            })
        }
    }
    #[derive(Debug)]
    pub struct SignedInfoTypeDeserializer {
        id: Option<String>,
        canonicalization_method: Option<super::CanonicalizationMethodType>,
        signature_method: Option<super::SignatureMethodType>,
        reference: Vec<super::ReferenceType>,
        state__: Box<SignedInfoTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SignedInfoTypeDeserializerState {
        Init__,
        CanonicalizationMethod(
            Option<<super::CanonicalizationMethodType as WithDeserializer>::Deserializer>,
        ),
        SignatureMethod(Option<<super::SignatureMethodType as WithDeserializer>::Deserializer>),
        Reference(Option<<super::ReferenceType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SignedInfoTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut id: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Id")
                ) {
                    helper.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id,
                canonicalization_method: None,
                signature_method: None,
                reference: Vec::new(),
                state__: Box::new(SignedInfoTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SignedInfoTypeDeserializerState,
        ) -> Result<(), Error> {
            use SignedInfoTypeDeserializerState as S;
            match state {
                S::CanonicalizationMethod(Some(deserializer)) => {
                    self.store_canonicalization_method(deserializer.finish(helper)?)?
                }
                S::SignatureMethod(Some(deserializer)) => {
                    self.store_signature_method(deserializer.finish(helper)?)?
                }
                S::Reference(Some(deserializer)) => {
                    self.store_reference(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_canonicalization_method(
            &mut self,
            value: super::CanonicalizationMethodType,
        ) -> Result<(), Error> {
            if self.canonicalization_method.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"CanonicalizationMethod",
                )))?;
            }
            self.canonicalization_method = Some(value);
            Ok(())
        }
        fn store_signature_method(
            &mut self,
            value: super::SignatureMethodType,
        ) -> Result<(), Error> {
            if self.signature_method.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SignatureMethod",
                )))?;
            }
            self.signature_method = Some(value);
            Ok(())
        }
        fn store_reference(&mut self, value: super::ReferenceType) -> Result<(), Error> {
            self.reference.push(value);
            Ok(())
        }
        fn handle_canonicalization_method<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::CanonicalizationMethodType>,
            fallback: &mut Option<SignedInfoTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SignedInfoTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::CanonicalizationMethod(None));
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
                    self.store_canonicalization_method(data)?;
                    *self.state__ = S::SignatureMethod(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::CanonicalizationMethod(Some(deserializer)));
                    *self.state__ = S::SignatureMethod(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_signature_method<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::SignatureMethodType>,
            fallback: &mut Option<SignedInfoTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SignedInfoTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::SignatureMethod(None));
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
                    self.store_signature_method(data)?;
                    *self.state__ = S::Reference(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::SignatureMethod(Some(deserializer)));
                    *self.state__ = S::Reference(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_reference<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ReferenceType>,
            fallback: &mut Option<SignedInfoTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SignedInfoTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else if self.reference.len() < 1usize {
                    fallback.get_or_insert(S::Reference(None));
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else {
                    fallback.get_or_insert(S::Reference(None));
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
                    self.store_reference(data)?;
                    *self.state__ = S::Reference(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Reference(Some(deserializer)));
                    *self.state__ = S::Reference(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SignedInfoType> for SignedInfoTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SignedInfoType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SignedInfoType> {
            use SignedInfoTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::CanonicalizationMethod(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_canonicalization_method(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SignatureMethod(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_signature_method(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Reference(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_reference(helper, output, &mut fallback)? {
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
                        *self.state__ = S::CanonicalizationMethod(None);
                        event
                    }
                    (
                        S::CanonicalizationMethod(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"CanonicalizationMethod",
                            true,
                        )?;
                        match self.handle_canonicalization_method(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SignatureMethod(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"SignatureMethod",
                            false,
                        )?;
                        match self.handle_signature_method(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Reference(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"Reference",
                            true,
                        )?;
                        match self.handle_reference(helper, output, &mut fallback)? {
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
        ) -> Result<super::SignedInfoType, Error> {
            let state = replace(
                &mut *self.state__,
                SignedInfoTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::SignedInfoType {
                id: self.id,
                canonicalization_method: helper
                    .finish_element("CanonicalizationMethod", self.canonicalization_method)?,
                signature_method: helper
                    .finish_element("SignatureMethod", self.signature_method)?,
                reference: helper.finish_vec(1usize, None, self.reference)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct SignatureValueTypeDeserializer {
        id: Option<String>,
        content: Option<String>,
        state__: Box<SignatureValueTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SignatureValueTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl SignatureValueTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut id: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Id")
                ) {
                    helper.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id,
                content: None,
                state__: Box::new(SignatureValueTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SignatureValueTypeDeserializerState,
        ) -> Result<(), Error> {
            if let SignatureValueTypeDeserializerState::Content__(deserializer) = state {
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
        ) -> DeserializerResult<'de, super::SignatureValueType> {
            use SignatureValueTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::SignatureValueType> for SignatureValueTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SignatureValueType> {
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
        ) -> DeserializerResult<'de, super::SignatureValueType> {
            use SignatureValueTypeDeserializerState as S;
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
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::SignatureValueType, Error> {
            let state = replace(
                &mut *self.state__,
                SignatureValueTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::SignatureValueType {
                id: self.id,
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct KeyInfoTypeDeserializer {
        id: Option<String>,
        content: Vec<super::KeyInfoTypeContent>,
        state__: Box<KeyInfoTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum KeyInfoTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::KeyInfoTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl KeyInfoTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut id: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Id")
                ) {
                    helper.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id,
                content: Vec::new(),
                state__: Box::new(KeyInfoTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: KeyInfoTypeDeserializerState,
        ) -> Result<(), Error> {
            if let KeyInfoTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::KeyInfoTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::KeyInfoTypeContent>,
            fallback: &mut Option<KeyInfoTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeyInfoTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::KeyInfoType> for KeyInfoTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeyInfoType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeyInfoType> {
            use KeyInfoTypeDeserializerState as S;
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
                            <super::KeyInfoTypeContent as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::KeyInfoType, Error> {
            let state = replace(&mut *self.state__, KeyInfoTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::KeyInfoType {
                id: self.id,
                content: helper.finish_vec(1usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct KeyInfoTypeContentDeserializer {
        state__: Box<KeyInfoTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum KeyInfoTypeContentDeserializerState {
        Init__,
        KeyName(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        KeyValue(
            Option<super::KeyValueType>,
            Option<<super::KeyValueType as WithDeserializer>::Deserializer>,
            Option<<super::KeyValueType as WithDeserializer>::Deserializer>,
        ),
        RetrievalMethod(
            Option<super::RetrievalMethodType>,
            Option<<super::RetrievalMethodType as WithDeserializer>::Deserializer>,
            Option<<super::RetrievalMethodType as WithDeserializer>::Deserializer>,
        ),
        X509Data(
            Option<super::X509DataType>,
            Option<<super::X509DataType as WithDeserializer>::Deserializer>,
            Option<<super::X509DataType as WithDeserializer>::Deserializer>,
        ),
        PgpData(
            Option<super::PgpDataType>,
            Option<<super::PgpDataType as WithDeserializer>::Deserializer>,
            Option<<super::PgpDataType as WithDeserializer>::Deserializer>,
        ),
        SpkiData(
            Option<super::SpkiDataType>,
            Option<<super::SpkiDataType as WithDeserializer>::Deserializer>,
            Option<<super::SpkiDataType as WithDeserializer>::Deserializer>,
        ),
        MgmtData(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::KeyInfoTypeContent),
        Unknown__,
    }
    impl KeyInfoTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"KeyName")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_key_name(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"KeyValue")
                ) {
                    let output = <super::KeyValueType as WithDeserializer>::init(helper, event)?;
                    return self.handle_key_value(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"RetrievalMethod")
                ) {
                    let output =
                        <super::RetrievalMethodType as WithDeserializer>::init(helper, event)?;
                    return self.handle_retrieval_method(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"X509Data")
                ) {
                    let output = <super::X509DataType as WithDeserializer>::init(helper, event)?;
                    return self.handle_x509_data(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"PGPData")
                ) {
                    let output = <super::PgpDataType as WithDeserializer>::init(helper, event)?;
                    return self.handle_pgp_data(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"SPKIData")
                ) {
                    let output = <super::SpkiDataType as WithDeserializer>::init(helper, event)?;
                    return self.handle_spki_data(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"MgmtData")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_mgmt_data(helper, Default::default(), None, output);
                }
            }
            *self.state__ = KeyInfoTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, true))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: KeyInfoTypeContentDeserializerState,
        ) -> Result<super::KeyInfoTypeContent, Error> {
            use KeyInfoTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::KeyName(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_key_name(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::KeyName(
                        helper.finish_element("KeyName", values)?,
                    ))
                }
                S::KeyValue(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_key_value(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::KeyValue(
                        helper.finish_element("KeyValue", values)?,
                    ))
                }
                S::RetrievalMethod(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_retrieval_method(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::RetrievalMethod(
                        helper.finish_element("RetrievalMethod", values)?,
                    ))
                }
                S::X509Data(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_x509_data(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::X509Data(
                        helper.finish_element("X509Data", values)?,
                    ))
                }
                S::PgpData(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_pgp_data(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::PgpData(
                        helper.finish_element("PGPData", values)?,
                    ))
                }
                S::SpkiData(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_spki_data(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::SpkiData(
                        helper.finish_element("SPKIData", values)?,
                    ))
                }
                S::MgmtData(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_mgmt_data(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::MgmtData(
                        helper.finish_element("MgmtData", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_key_name(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"KeyName",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_key_value(
            values: &mut Option<super::KeyValueType>,
            value: super::KeyValueType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"KeyValue",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_retrieval_method(
            values: &mut Option<super::RetrievalMethodType>,
            value: super::RetrievalMethodType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"RetrievalMethod",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_x509_data(
            values: &mut Option<super::X509DataType>,
            value: super::X509DataType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"X509Data",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_pgp_data(
            values: &mut Option<super::PgpDataType>,
            value: super::PgpDataType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PGPData",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_spki_data(
            values: &mut Option<super::SpkiDataType>,
            value: super::SpkiDataType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SPKIData",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_mgmt_data(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"MgmtData",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_key_name<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeyInfoTypeContentDeserializerState as S;
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
                Self::store_key_name(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_key_name(&mut values, data)?;
                    let data = Self::finish_state(helper, S::KeyName(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::KeyName(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_key_value<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::KeyValueType>,
            fallback: Option<<super::KeyValueType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::KeyValueType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeyInfoTypeContentDeserializerState as S;
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
                Self::store_key_value(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_key_value(&mut values, data)?;
                    let data = Self::finish_state(helper, S::KeyValue(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::KeyValue(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_retrieval_method<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::RetrievalMethodType>,
            fallback: Option<<super::RetrievalMethodType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::RetrievalMethodType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeyInfoTypeContentDeserializerState as S;
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
                Self::store_retrieval_method(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_retrieval_method(&mut values, data)?;
                    let data = Self::finish_state(helper, S::RetrievalMethod(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::RetrievalMethod(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_x509_data<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::X509DataType>,
            fallback: Option<<super::X509DataType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::X509DataType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeyInfoTypeContentDeserializerState as S;
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
                Self::store_x509_data(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x509_data(&mut values, data)?;
                    let data = Self::finish_state(helper, S::X509Data(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::X509Data(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_pgp_data<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PgpDataType>,
            fallback: Option<<super::PgpDataType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PgpDataType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeyInfoTypeContentDeserializerState as S;
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
                Self::store_pgp_data(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pgp_data(&mut values, data)?;
                    let data = Self::finish_state(helper, S::PgpData(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::PgpData(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_spki_data<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::SpkiDataType>,
            fallback: Option<<super::SpkiDataType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::SpkiDataType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeyInfoTypeContentDeserializerState as S;
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
                Self::store_spki_data(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_spki_data(&mut values, data)?;
                    let data = Self::finish_state(helper, S::SpkiData(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::SpkiData(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_mgmt_data<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeyInfoTypeContentDeserializerState as S;
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
                Self::store_mgmt_data(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_mgmt_data(&mut values, data)?;
                    let data = Self::finish_state(helper, S::MgmtData(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::MgmtData(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::KeyInfoTypeContent> for KeyInfoTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeyInfoTypeContent> {
            let deserializer = Self {
                state__: Box::new(KeyInfoTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, KeyInfoTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::KeyInfoTypeContent> {
            use KeyInfoTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::KeyName(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_key_name(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::KeyValue(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_key_value(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::RetrievalMethod(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_retrieval_method(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::X509Data(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_x509_data(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::PgpData(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_pgp_data(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::SpkiData(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_spki_data(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::MgmtData(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_mgmt_data(helper, values, fallback, output)? {
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
                        S::KeyName(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"KeyName",
                            false,
                        )?;
                        match self.handle_key_name(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::KeyValue(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"KeyValue",
                            true,
                        )?;
                        match self.handle_key_value(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::RetrievalMethod(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"RetrievalMethod",
                            true,
                        )?;
                        match self.handle_retrieval_method(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::X509Data(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"X509Data",
                            true,
                        )?;
                        match self.handle_x509_data(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::PgpData(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"PGPData",
                            false,
                        )?;
                        match self.handle_pgp_data(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::SpkiData(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"SPKIData",
                            false,
                        )?;
                        match self.handle_spki_data(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::MgmtData(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"MgmtData",
                            false,
                        )?;
                        match self.handle_mgmt_data(helper, values, fallback, output)? {
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
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::KeyInfoTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct ObjectTypeDeserializer {
        id: Option<String>,
        mime_type: Option<String>,
        encoding: Option<String>,
        state__: Box<ObjectTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ObjectTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl ObjectTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut id: Option<String> = None;
            let mut mime_type: Option<String> = None;
            let mut encoding: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Id")
                ) {
                    helper.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"MimeType")
                ) {
                    helper.read_attrib(&mut mime_type, b"MimeType", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Encoding")
                ) {
                    helper.read_attrib(&mut encoding, b"Encoding", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id,
                mime_type: mime_type,
                encoding: encoding,
                state__: Box::new(ObjectTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ObjectTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::ObjectType> for ObjectTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ObjectType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ObjectType> {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ObjectType, Error> {
            let state = replace(&mut *self.state__, ObjectTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::ObjectType {
                id: self.id,
                mime_type: self.mime_type,
                encoding: self.encoding,
            })
        }
    }
    #[derive(Debug)]
    pub struct CanonicalizationMethodTypeDeserializer {
        algorithm: String,
        state__: Box<CanonicalizationMethodTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CanonicalizationMethodTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl CanonicalizationMethodTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut algorithm: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Algorithm")
                ) {
                    helper.read_attrib(&mut algorithm, b"Algorithm", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                algorithm: algorithm
                    .ok_or_else(|| ErrorKind::MissingAttribute("Algorithm".into()))?,
                state__: Box::new(CanonicalizationMethodTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: CanonicalizationMethodTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::CanonicalizationMethodType>
        for CanonicalizationMethodTypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CanonicalizationMethodType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CanonicalizationMethodType> {
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
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::CanonicalizationMethodType, Error> {
            let state = replace(
                &mut *self.state__,
                CanonicalizationMethodTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::CanonicalizationMethodType {
                algorithm: self.algorithm,
            })
        }
    }
    #[derive(Debug)]
    pub struct SignatureMethodTypeDeserializer {
        algorithm: String,
        hmac_output_length: Option<i32>,
        state__: Box<SignatureMethodTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SignatureMethodTypeDeserializerState {
        Init__,
        HmacOutputLength(Option<<i32 as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SignatureMethodTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut algorithm: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Algorithm")
                ) {
                    helper.read_attrib(&mut algorithm, b"Algorithm", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                algorithm: algorithm
                    .ok_or_else(|| ErrorKind::MissingAttribute("Algorithm".into()))?,
                hmac_output_length: None,
                state__: Box::new(SignatureMethodTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SignatureMethodTypeDeserializerState,
        ) -> Result<(), Error> {
            use SignatureMethodTypeDeserializerState as S;
            match state {
                S::HmacOutputLength(Some(deserializer)) => {
                    self.store_hmac_output_length(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_hmac_output_length(&mut self, value: i32) -> Result<(), Error> {
            if self.hmac_output_length.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"HMACOutputLength",
                )))?;
            }
            self.hmac_output_length = Some(value);
            Ok(())
        }
        fn handle_hmac_output_length<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<SignatureMethodTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SignatureMethodTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::HmacOutputLength(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_hmac_output_length(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::HmacOutputLength(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SignatureMethodType> for SignatureMethodTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SignatureMethodType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SignatureMethodType> {
            use SignatureMethodTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::HmacOutputLength(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_hmac_output_length(helper, output, &mut fallback)? {
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
                        allow_any_element = true;
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::HmacOutputLength(None);
                        event
                    }
                    (S::HmacOutputLength(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"HMACOutputLength",
                            true,
                        )?;
                        match self.handle_hmac_output_length(helper, output, &mut fallback)? {
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
                        break (DeserializerEvent::Continue(event), true);
                    }
                    (state, Event::Text(_) | Event::CData(_)) => {
                        *self.state__ = state;
                        break (DeserializerEvent::None, false);
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
        ) -> Result<super::SignatureMethodType, Error> {
            let state = replace(
                &mut *self.state__,
                SignatureMethodTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::SignatureMethodType {
                algorithm: self.algorithm,
                hmac_output_length: self.hmac_output_length,
            })
        }
    }
    #[derive(Debug)]
    pub struct ReferenceTypeDeserializer {
        id: Option<String>,
        uri: Option<String>,
        type_: Option<String>,
        transforms: Option<super::TransformsType>,
        digest_method: Option<super::DigestMethodType>,
        digest_value: Option<String>,
        state__: Box<ReferenceTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ReferenceTypeDeserializerState {
        Init__,
        Transforms(Option<<super::TransformsType as WithDeserializer>::Deserializer>),
        DigestMethod(Option<<super::DigestMethodType as WithDeserializer>::Deserializer>),
        DigestValue(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ReferenceTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut id: Option<String> = None;
            let mut uri: Option<String> = None;
            let mut type_: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Id")
                ) {
                    helper.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"URI")
                ) {
                    helper.read_attrib(&mut uri, b"URI", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Type")
                ) {
                    helper.read_attrib(&mut type_, b"Type", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id,
                uri: uri,
                type_: type_,
                transforms: None,
                digest_method: None,
                digest_value: None,
                state__: Box::new(ReferenceTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ReferenceTypeDeserializerState,
        ) -> Result<(), Error> {
            use ReferenceTypeDeserializerState as S;
            match state {
                S::Transforms(Some(deserializer)) => {
                    self.store_transforms(deserializer.finish(helper)?)?
                }
                S::DigestMethod(Some(deserializer)) => {
                    self.store_digest_method(deserializer.finish(helper)?)?
                }
                S::DigestValue(Some(deserializer)) => {
                    self.store_digest_value(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_transforms(&mut self, value: super::TransformsType) -> Result<(), Error> {
            if self.transforms.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Transforms",
                )))?;
            }
            self.transforms = Some(value);
            Ok(())
        }
        fn store_digest_method(&mut self, value: super::DigestMethodType) -> Result<(), Error> {
            if self.digest_method.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DigestMethod",
                )))?;
            }
            self.digest_method = Some(value);
            Ok(())
        }
        fn store_digest_value(&mut self, value: String) -> Result<(), Error> {
            if self.digest_value.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DigestValue",
                )))?;
            }
            self.digest_value = Some(value);
            Ok(())
        }
        fn handle_transforms<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TransformsType>,
            fallback: &mut Option<ReferenceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ReferenceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Transforms(None));
                *self.state__ = S::DigestMethod(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_transforms(data)?;
                    *self.state__ = S::DigestMethod(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Transforms(Some(deserializer)));
                    *self.state__ = S::DigestMethod(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_digest_method<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::DigestMethodType>,
            fallback: &mut Option<ReferenceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ReferenceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::DigestMethod(None));
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
                    self.store_digest_method(data)?;
                    *self.state__ = S::DigestValue(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::DigestMethod(Some(deserializer)));
                    *self.state__ = S::DigestValue(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_digest_value<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ReferenceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ReferenceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::DigestValue(None));
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
                    self.store_digest_value(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::DigestValue(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ReferenceType> for ReferenceTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ReferenceType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ReferenceType> {
            use ReferenceTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Transforms(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_transforms(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DigestMethod(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_digest_method(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DigestValue(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_digest_value(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Transforms(None);
                        event
                    }
                    (S::Transforms(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"Transforms",
                            true,
                        )?;
                        match self.handle_transforms(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DigestMethod(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"DigestMethod",
                            true,
                        )?;
                        match self.handle_digest_method(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DigestValue(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"DigestValue",
                            false,
                        )?;
                        match self.handle_digest_value(helper, output, &mut fallback)? {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ReferenceType, Error> {
            let state = replace(
                &mut *self.state__,
                ReferenceTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ReferenceType {
                id: self.id,
                uri: self.uri,
                type_: self.type_,
                transforms: self.transforms,
                digest_method: helper.finish_element("DigestMethod", self.digest_method)?,
                digest_value: helper.finish_element("DigestValue", self.digest_value)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct KeyValueTypeDeserializer {
        content: Option<super::KeyValueTypeContent>,
        state__: Box<KeyValueTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum KeyValueTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::KeyValueTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl KeyValueTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: None,
                state__: Box::new(KeyValueTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: KeyValueTypeDeserializerState,
        ) -> Result<(), Error> {
            if let KeyValueTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::KeyValueTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::KeyValueTypeContent>,
            fallback: &mut Option<KeyValueTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeyValueTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::KeyValueType> for KeyValueTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeyValueType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeyValueType> {
            use KeyValueTypeDeserializerState as S;
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
                            <super::KeyValueTypeContent as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::KeyValueType, Error> {
            let state = replace(&mut *self.state__, KeyValueTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::KeyValueType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct KeyValueTypeContentDeserializer {
        state__: Box<KeyValueTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum KeyValueTypeContentDeserializerState {
        Init__,
        DsaKeyValue(
            Option<super::DsaKeyValueType>,
            Option<<super::DsaKeyValueType as WithDeserializer>::Deserializer>,
            Option<<super::DsaKeyValueType as WithDeserializer>::Deserializer>,
        ),
        RsaKeyValue(
            Option<super::RsaKeyValueType>,
            Option<<super::RsaKeyValueType as WithDeserializer>::Deserializer>,
            Option<<super::RsaKeyValueType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::KeyValueTypeContent),
        Unknown__,
    }
    impl KeyValueTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"DSAKeyValue")
                ) {
                    let output = <super::DsaKeyValueType as WithDeserializer>::init(helper, event)?;
                    return self.handle_dsa_key_value(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"RSAKeyValue")
                ) {
                    let output = <super::RsaKeyValueType as WithDeserializer>::init(helper, event)?;
                    return self.handle_rsa_key_value(helper, Default::default(), None, output);
                }
            }
            *self.state__ = KeyValueTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, true))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: KeyValueTypeContentDeserializerState,
        ) -> Result<super::KeyValueTypeContent, Error> {
            use KeyValueTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::DsaKeyValue(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_dsa_key_value(&mut values, value)?;
                    }
                    Ok(super::KeyValueTypeContent::DsaKeyValue(
                        helper.finish_element("DSAKeyValue", values)?,
                    ))
                }
                S::RsaKeyValue(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_rsa_key_value(&mut values, value)?;
                    }
                    Ok(super::KeyValueTypeContent::RsaKeyValue(
                        helper.finish_element("RSAKeyValue", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_dsa_key_value(
            values: &mut Option<super::DsaKeyValueType>,
            value: super::DsaKeyValueType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DSAKeyValue",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_rsa_key_value(
            values: &mut Option<super::RsaKeyValueType>,
            value: super::RsaKeyValueType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"RSAKeyValue",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_dsa_key_value<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::DsaKeyValueType>,
            fallback: Option<<super::DsaKeyValueType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::DsaKeyValueType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeyValueTypeContentDeserializerState as S;
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
                Self::store_dsa_key_value(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_dsa_key_value(&mut values, data)?;
                    let data = Self::finish_state(helper, S::DsaKeyValue(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::DsaKeyValue(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_rsa_key_value<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::RsaKeyValueType>,
            fallback: Option<<super::RsaKeyValueType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::RsaKeyValueType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use KeyValueTypeContentDeserializerState as S;
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
                Self::store_rsa_key_value(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_rsa_key_value(&mut values, data)?;
                    let data = Self::finish_state(helper, S::RsaKeyValue(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::RsaKeyValue(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::KeyValueTypeContent> for KeyValueTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeyValueTypeContent> {
            let deserializer = Self {
                state__: Box::new(KeyValueTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, KeyValueTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::KeyValueTypeContent> {
            use KeyValueTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::DsaKeyValue(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_dsa_key_value(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::RsaKeyValue(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_rsa_key_value(helper, values, fallback, output)? {
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
                        S::DsaKeyValue(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"DSAKeyValue",
                            false,
                        )?;
                        match self.handle_dsa_key_value(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::RsaKeyValue(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"RSAKeyValue",
                            false,
                        )?;
                        match self.handle_rsa_key_value(helper, values, fallback, output)? {
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
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::KeyValueTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct RetrievalMethodTypeDeserializer {
        uri: Option<String>,
        type_: Option<String>,
        transforms: Option<super::TransformsType>,
        state__: Box<RetrievalMethodTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RetrievalMethodTypeDeserializerState {
        Init__,
        Transforms(Option<<super::TransformsType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl RetrievalMethodTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut uri: Option<String> = None;
            let mut type_: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"URI")
                ) {
                    helper.read_attrib(&mut uri, b"URI", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Type")
                ) {
                    helper.read_attrib(&mut type_, b"Type", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                uri: uri,
                type_: type_,
                transforms: None,
                state__: Box::new(RetrievalMethodTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: RetrievalMethodTypeDeserializerState,
        ) -> Result<(), Error> {
            use RetrievalMethodTypeDeserializerState as S;
            match state {
                S::Transforms(Some(deserializer)) => {
                    self.store_transforms(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_transforms(&mut self, value: super::TransformsType) -> Result<(), Error> {
            if self.transforms.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Transforms",
                )))?;
            }
            self.transforms = Some(value);
            Ok(())
        }
        fn handle_transforms<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TransformsType>,
            fallback: &mut Option<RetrievalMethodTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RetrievalMethodTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Transforms(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_transforms(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Transforms(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RetrievalMethodType> for RetrievalMethodTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RetrievalMethodType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RetrievalMethodType> {
            use RetrievalMethodTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Transforms(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_transforms(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Transforms(None);
                        event
                    }
                    (S::Transforms(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"Transforms",
                            true,
                        )?;
                        match self.handle_transforms(helper, output, &mut fallback)? {
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
        ) -> Result<super::RetrievalMethodType, Error> {
            let state = replace(
                &mut *self.state__,
                RetrievalMethodTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::RetrievalMethodType {
                uri: self.uri,
                type_: self.type_,
                transforms: self.transforms,
            })
        }
    }
    #[derive(Debug)]
    pub struct X509DataTypeDeserializer {
        content: Vec<super::X509DataTypeContent>,
        state__: Box<X509DataTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum X509DataTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::X509DataTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl X509DataTypeDeserializer {
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
                state__: Box::new(X509DataTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: X509DataTypeDeserializerState,
        ) -> Result<(), Error> {
            if let X509DataTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::X509DataTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::X509DataTypeContent>,
            fallback: &mut Option<X509DataTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use X509DataTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::X509DataType> for X509DataTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::X509DataType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::X509DataType> {
            use X509DataTypeDeserializerState as S;
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
                            <super::X509DataTypeContent as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::X509DataType, Error> {
            let state = replace(&mut *self.state__, X509DataTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::X509DataType {
                content: helper.finish_vec(1usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct X509DataTypeContentDeserializer {
        content_43: Option<super::X509DataContent43Type>,
        state__: Box<X509DataTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    enum X509DataTypeContentDeserializerState {
        Init__,
        Content43(Option<<super::X509DataContent43Type as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl X509DataTypeContentDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: X509DataTypeContentDeserializerState,
        ) -> Result<(), Error> {
            use X509DataTypeContentDeserializerState as S;
            match state {
                S::Content43(Some(deserializer)) => {
                    self.store_content_43(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_content_43(&mut self, value: super::X509DataContent43Type) -> Result<(), Error> {
            if self.content_43.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Content43",
                )))?;
            }
            self.content_43 = Some(value);
            Ok(())
        }
        fn handle_content_43<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::X509DataContent43Type>,
            fallback: &mut Option<X509DataTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use X509DataTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Content43(None));
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
                    self.store_content_43(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Content43(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::X509DataTypeContent> for X509DataTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::X509DataTypeContent> {
            let deserializer = Self {
                content_43: None,
                state__: Box::new(X509DataTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, X509DataTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::X509DataTypeContent> {
            use X509DataTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content43(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content_43(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Content43(None);
                        event
                    }
                    (S::Content43(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = <super::X509DataContent43Type as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_content_43(helper, output, &mut fallback)? {
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
        ) -> Result<super::X509DataTypeContent, Error> {
            let state = replace(
                &mut *self.state__,
                X509DataTypeContentDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::X509DataTypeContent {
                content_43: helper.finish_element("Content43", self.content_43)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PgpDataTypeDeserializer {
        content: Option<super::PgpDataTypeContent>,
        state__: Box<PgpDataTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PgpDataTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::PgpDataTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl PgpDataTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: None,
                state__: Box::new(PgpDataTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: PgpDataTypeDeserializerState,
        ) -> Result<(), Error> {
            if let PgpDataTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::PgpDataTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::PgpDataTypeContent>,
            fallback: &mut Option<PgpDataTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PgpDataTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::PgpDataType> for PgpDataTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PgpDataType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PgpDataType> {
            use PgpDataTypeDeserializerState as S;
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
                            <super::PgpDataTypeContent as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::PgpDataType, Error> {
            let state = replace(&mut *self.state__, PgpDataTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::PgpDataType {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PgpDataTypeContentDeserializer {
        state__: Box<PgpDataTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum PgpDataTypeContentDeserializerState {
        Init__,
        Content47(
            Option<super::PgpDataContent47Type>,
            Option<<super::PgpDataContent47Type as WithDeserializer>::Deserializer>,
            Option<<super::PgpDataContent47Type as WithDeserializer>::Deserializer>,
        ),
        Content49(
            Option<super::PgpDataContent49Type>,
            Option<<super::PgpDataContent49Type as WithDeserializer>::Deserializer>,
            Option<<super::PgpDataContent49Type as WithDeserializer>::Deserializer>,
        ),
        Done__(super::PgpDataTypeContent),
        Unknown__,
    }
    impl PgpDataTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let mut event = event;
            let mut allow_any_element = false;
            if let Event::Start(_) | Event::Empty(_) = &event {
                event = {
                    let output =
                        <super::PgpDataContent47Type as WithDeserializer>::init(helper, event)?;
                    match self.handle_content_47(helper, Default::default(), None, output)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        output => {
                            return Ok(output);
                        }
                    }
                };
                event = {
                    let output =
                        <super::PgpDataContent49Type as WithDeserializer>::init(helper, event)?;
                    match self.handle_content_49(helper, Default::default(), None, output)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        output => {
                            return Ok(output);
                        }
                    }
                };
            }
            *self.state__ = PgpDataTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: PgpDataTypeContentDeserializerState,
        ) -> Result<super::PgpDataTypeContent, Error> {
            use PgpDataTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Content47(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_content_47(&mut values, value)?;
                    }
                    Ok(super::PgpDataTypeContent::Content47(
                        helper.finish_element("Content47", values)?,
                    ))
                }
                S::Content49(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_content_49(&mut values, value)?;
                    }
                    Ok(super::PgpDataTypeContent::Content49(
                        helper.finish_element("Content49", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_content_47(
            values: &mut Option<super::PgpDataContent47Type>,
            value: super::PgpDataContent47Type,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Content47",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_content_49(
            values: &mut Option<super::PgpDataContent49Type>,
            value: super::PgpDataContent49Type,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Content49",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_content_47<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PgpDataContent47Type>,
            fallback: Option<<super::PgpDataContent47Type as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PgpDataContent47Type>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PgpDataTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_content_47(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_content_47(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Content47(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content47(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_content_49<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::PgpDataContent49Type>,
            fallback: Option<<super::PgpDataContent49Type as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::PgpDataContent49Type>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PgpDataTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_content_49(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_content_49(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Content49(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content49(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::PgpDataTypeContent> for PgpDataTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PgpDataTypeContent> {
            let deserializer = Self {
                state__: Box::new(PgpDataTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, PgpDataTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::PgpDataTypeContent> {
            use PgpDataTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content47(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content_47(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Content49(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content_49(helper, values, fallback, output)? {
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
                        S::Content47(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            <super::PgpDataContent47Type as WithDeserializer>::init(helper, event)?;
                        match self.handle_content_47(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Content49(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            <super::PgpDataContent49Type as WithDeserializer>::init(helper, event)?;
                        match self.handle_content_49(helper, values, fallback, output)? {
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
        ) -> Result<super::PgpDataTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct SpkiDataTypeDeserializer {
        content: Vec<super::SpkiDataTypeContent>,
        state__: Box<SpkiDataTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SpkiDataTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::SpkiDataTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl SpkiDataTypeDeserializer {
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
                state__: Box::new(SpkiDataTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SpkiDataTypeDeserializerState,
        ) -> Result<(), Error> {
            if let SpkiDataTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::SpkiDataTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::SpkiDataTypeContent>,
            fallback: &mut Option<SpkiDataTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SpkiDataTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::SpkiDataType> for SpkiDataTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SpkiDataType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SpkiDataType> {
            use SpkiDataTypeDeserializerState as S;
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
                            <super::SpkiDataTypeContent as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::SpkiDataType, Error> {
            let state = replace(&mut *self.state__, SpkiDataTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::SpkiDataType {
                content: helper.finish_vec(1usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct SpkiDataTypeContentDeserializer {
        spki_sexp: Option<String>,
        state__: Box<SpkiDataTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    enum SpkiDataTypeContentDeserializerState {
        Init__,
        SpkiSexp(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SpkiDataTypeContentDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SpkiDataTypeContentDeserializerState,
        ) -> Result<(), Error> {
            use SpkiDataTypeContentDeserializerState as S;
            match state {
                S::SpkiSexp(Some(deserializer)) => {
                    self.store_spki_sexp(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_spki_sexp(&mut self, value: String) -> Result<(), Error> {
            if self.spki_sexp.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SPKISexp",
                )))?;
            }
            self.spki_sexp = Some(value);
            Ok(())
        }
        fn handle_spki_sexp<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<SpkiDataTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SpkiDataTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::SpkiSexp(None));
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
                    self.store_spki_sexp(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::SpkiSexp(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SpkiDataTypeContent> for SpkiDataTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SpkiDataTypeContent> {
            let deserializer = Self {
                spki_sexp: None,
                state__: Box::new(SpkiDataTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, SpkiDataTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::SpkiDataTypeContent> {
            use SpkiDataTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::SpkiSexp(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_spki_sexp(helper, output, &mut fallback)? {
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
                        allow_any_element = true;
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::SpkiSexp(None);
                        event
                    }
                    (S::SpkiSexp(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"SPKISexp",
                            true,
                        )?;
                        match self.handle_spki_sexp(helper, output, &mut fallback)? {
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
                        break (DeserializerEvent::Continue(event), true);
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
        ) -> Result<super::SpkiDataTypeContent, Error> {
            let state = replace(
                &mut *self.state__,
                SpkiDataTypeContentDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::SpkiDataTypeContent {
                spki_sexp: helper.finish_element("SPKISexp", self.spki_sexp)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TransformsTypeDeserializer {
        transform: Vec<super::TransformType>,
        state__: Box<TransformsTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TransformsTypeDeserializerState {
        Init__,
        Transform(Option<<super::TransformType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TransformsTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                transform: Vec::new(),
                state__: Box::new(TransformsTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TransformsTypeDeserializerState,
        ) -> Result<(), Error> {
            use TransformsTypeDeserializerState as S;
            match state {
                S::Transform(Some(deserializer)) => {
                    self.store_transform(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_transform(&mut self, value: super::TransformType) -> Result<(), Error> {
            self.transform.push(value);
            Ok(())
        }
        fn handle_transform<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TransformType>,
            fallback: &mut Option<TransformsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TransformsTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else if self.transform.len() < 1usize {
                    fallback.get_or_insert(S::Transform(None));
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else {
                    fallback.get_or_insert(S::Transform(None));
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
                    self.store_transform(data)?;
                    *self.state__ = S::Transform(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Transform(Some(deserializer)));
                    *self.state__ = S::Transform(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TransformsType> for TransformsTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TransformsType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TransformsType> {
            use TransformsTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Transform(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_transform(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Transform(None);
                        event
                    }
                    (S::Transform(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"Transform",
                            true,
                        )?;
                        match self.handle_transform(helper, output, &mut fallback)? {
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
        ) -> Result<super::TransformsType, Error> {
            let state = replace(
                &mut *self.state__,
                TransformsTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::TransformsType {
                transform: helper.finish_vec(1usize, None, self.transform)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DigestMethodTypeDeserializer {
        algorithm: String,
        state__: Box<DigestMethodTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DigestMethodTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl DigestMethodTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut algorithm: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Algorithm")
                ) {
                    helper.read_attrib(&mut algorithm, b"Algorithm", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                algorithm: algorithm
                    .ok_or_else(|| ErrorKind::MissingAttribute("Algorithm".into()))?,
                state__: Box::new(DigestMethodTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: DigestMethodTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::DigestMethodType> for DigestMethodTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DigestMethodType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DigestMethodType> {
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
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::DigestMethodType, Error> {
            let state = replace(
                &mut *self.state__,
                DigestMethodTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::DigestMethodType {
                algorithm: self.algorithm,
            })
        }
    }
    #[derive(Debug)]
    pub struct DsaKeyValueTypeDeserializer {
        content_60: Option<super::DsaKeyValueContent60Type>,
        g: Option<String>,
        y: Option<String>,
        j: Option<String>,
        content_61: Option<super::DsaKeyValueContent61Type>,
        state__: Box<DsaKeyValueTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DsaKeyValueTypeDeserializerState {
        Init__,
        Content60(Option<<super::DsaKeyValueContent60Type as WithDeserializer>::Deserializer>),
        G(Option<<String as WithDeserializer>::Deserializer>),
        Y(Option<<String as WithDeserializer>::Deserializer>),
        J(Option<<String as WithDeserializer>::Deserializer>),
        Content61(Option<<super::DsaKeyValueContent61Type as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DsaKeyValueTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content_60: None,
                g: None,
                y: None,
                j: None,
                content_61: None,
                state__: Box::new(DsaKeyValueTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: DsaKeyValueTypeDeserializerState,
        ) -> Result<(), Error> {
            use DsaKeyValueTypeDeserializerState as S;
            match state {
                S::Content60(Some(deserializer)) => {
                    self.store_content_60(deserializer.finish(helper)?)?
                }
                S::G(Some(deserializer)) => self.store_g(deserializer.finish(helper)?)?,
                S::Y(Some(deserializer)) => self.store_y(deserializer.finish(helper)?)?,
                S::J(Some(deserializer)) => self.store_j(deserializer.finish(helper)?)?,
                S::Content61(Some(deserializer)) => {
                    self.store_content_61(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_content_60(
            &mut self,
            value: super::DsaKeyValueContent60Type,
        ) -> Result<(), Error> {
            if self.content_60.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Content60",
                )))?;
            }
            self.content_60 = Some(value);
            Ok(())
        }
        fn store_g(&mut self, value: String) -> Result<(), Error> {
            if self.g.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"G")))?;
            }
            self.g = Some(value);
            Ok(())
        }
        fn store_y(&mut self, value: String) -> Result<(), Error> {
            if self.y.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Y")))?;
            }
            self.y = Some(value);
            Ok(())
        }
        fn store_j(&mut self, value: String) -> Result<(), Error> {
            if self.j.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"J")))?;
            }
            self.j = Some(value);
            Ok(())
        }
        fn store_content_61(
            &mut self,
            value: super::DsaKeyValueContent61Type,
        ) -> Result<(), Error> {
            if self.content_61.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Content61",
                )))?;
            }
            self.content_61 = Some(value);
            Ok(())
        }
        fn handle_content_60<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::DsaKeyValueContent60Type>,
            fallback: &mut Option<DsaKeyValueTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DsaKeyValueTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Content60(None));
                *self.state__ = S::G(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content_60(data)?;
                    *self.state__ = S::G(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Content60(Some(deserializer)));
                    *self.state__ = S::G(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_g<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DsaKeyValueTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DsaKeyValueTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::G(None));
                *self.state__ = S::Y(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_g(data)?;
                    *self.state__ = S::Y(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::G(Some(deserializer)));
                    *self.state__ = S::Y(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_y<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DsaKeyValueTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DsaKeyValueTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Y(None));
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
                    self.store_y(data)?;
                    *self.state__ = S::J(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Y(Some(deserializer)));
                    *self.state__ = S::J(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_j<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DsaKeyValueTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DsaKeyValueTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::J(None));
                *self.state__ = S::Content61(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_j(data)?;
                    *self.state__ = S::Content61(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::J(Some(deserializer)));
                    *self.state__ = S::Content61(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_content_61<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::DsaKeyValueContent61Type>,
            fallback: &mut Option<DsaKeyValueTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DsaKeyValueTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Content61(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content_61(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Content61(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DsaKeyValueType> for DsaKeyValueTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DsaKeyValueType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DsaKeyValueType> {
            use DsaKeyValueTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content60(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content_60(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::G(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_g(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Y(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_y(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::J(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_j(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Content61(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content_61(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Content60(None);
                        event
                    }
                    (S::Content60(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = <super::DsaKeyValueContent60Type as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_content_60(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::G(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"G",
                            false,
                        )?;
                        match self.handle_g(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Y(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"Y",
                            false,
                        )?;
                        match self.handle_y(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::J(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"J",
                            false,
                        )?;
                        match self.handle_j(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Content61(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = <super::DsaKeyValueContent61Type as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_content_61(helper, output, &mut fallback)? {
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
        ) -> Result<super::DsaKeyValueType, Error> {
            let state = replace(
                &mut *self.state__,
                DsaKeyValueTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::DsaKeyValueType {
                content_60: self.content_60,
                g: self.g,
                y: helper.finish_element("Y", self.y)?,
                j: self.j,
                content_61: self.content_61,
            })
        }
    }
    #[derive(Debug)]
    pub struct RsaKeyValueTypeDeserializer {
        modulus: Option<String>,
        exponent: Option<String>,
        state__: Box<RsaKeyValueTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RsaKeyValueTypeDeserializerState {
        Init__,
        Modulus(Option<<String as WithDeserializer>::Deserializer>),
        Exponent(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl RsaKeyValueTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                modulus: None,
                exponent: None,
                state__: Box::new(RsaKeyValueTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: RsaKeyValueTypeDeserializerState,
        ) -> Result<(), Error> {
            use RsaKeyValueTypeDeserializerState as S;
            match state {
                S::Modulus(Some(deserializer)) => {
                    self.store_modulus(deserializer.finish(helper)?)?
                }
                S::Exponent(Some(deserializer)) => {
                    self.store_exponent(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_modulus(&mut self, value: String) -> Result<(), Error> {
            if self.modulus.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Modulus",
                )))?;
            }
            self.modulus = Some(value);
            Ok(())
        }
        fn store_exponent(&mut self, value: String) -> Result<(), Error> {
            if self.exponent.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Exponent",
                )))?;
            }
            self.exponent = Some(value);
            Ok(())
        }
        fn handle_modulus<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<RsaKeyValueTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RsaKeyValueTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Modulus(None));
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
                    self.store_modulus(data)?;
                    *self.state__ = S::Exponent(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Modulus(Some(deserializer)));
                    *self.state__ = S::Exponent(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_exponent<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<RsaKeyValueTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RsaKeyValueTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Exponent(None));
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
                    self.store_exponent(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Exponent(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RsaKeyValueType> for RsaKeyValueTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RsaKeyValueType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RsaKeyValueType> {
            use RsaKeyValueTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Modulus(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_modulus(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Exponent(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_exponent(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Modulus(None);
                        event
                    }
                    (S::Modulus(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"Modulus",
                            false,
                        )?;
                        match self.handle_modulus(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Exponent(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"Exponent",
                            false,
                        )?;
                        match self.handle_exponent(helper, output, &mut fallback)? {
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
        ) -> Result<super::RsaKeyValueType, Error> {
            let state = replace(
                &mut *self.state__,
                RsaKeyValueTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::RsaKeyValueType {
                modulus: helper.finish_element("Modulus", self.modulus)?,
                exponent: helper.finish_element("Exponent", self.exponent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct X509DataContent43TypeDeserializer {
        content: Option<super::X509DataContent43TypeContent>,
        state__: Box<X509DataContent43TypeDeserializerState>,
    }
    #[derive(Debug)]
    enum X509DataContent43TypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::X509DataContent43TypeContent as WithDeserializer>::Deserializer),
        Done__,
        Unknown__,
    }
    impl X509DataContent43TypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: X509DataContent43TypeDeserializerState,
        ) -> Result<(), Error> {
            if let X509DataContent43TypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(
            &mut self,
            value: super::X509DataContent43TypeContent,
        ) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::X509DataContent43TypeContent>,
            fallback: &mut Option<X509DataContent43TypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use X509DataContent43TypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Init__;
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::X509DataContent43Type> for X509DataContent43TypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::X509DataContent43Type> {
            let deserializer = Self {
                content: None,
                state__: Box::new(X509DataContent43TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, X509DataContent43TypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::X509DataContent43Type> {
            use X509DataContent43TypeDeserializerState as S;
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
                    (_, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::X509DataContent43TypeContent as WithDeserializer>::init(
                                helper, event,
                            )?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = match &*self.state__ {
                S::Done__ => DeserializerArtifact::Data(self.finish(helper)?),
                _ => DeserializerArtifact::Deserializer(self),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::X509DataContent43Type, Error> {
            let state = replace(
                &mut *self.state__,
                X509DataContent43TypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::X509DataContent43Type {
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct X509DataContent43TypeContentDeserializer {
        state__: Box<X509DataContent43TypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum X509DataContent43TypeContentDeserializerState {
        Init__,
        X509IssuerSerial(
            Option<super::X509IssuerSerialType>,
            Option<<super::X509IssuerSerialType as WithDeserializer>::Deserializer>,
            Option<<super::X509IssuerSerialType as WithDeserializer>::Deserializer>,
        ),
        X509Ski(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        X509SubjectName(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        X509Certificate(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        X509Crl(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::X509DataContent43TypeContent),
        Unknown__,
    }
    impl X509DataContent43TypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"X509IssuerSerial")
                ) {
                    let output =
                        <super::X509IssuerSerialType as WithDeserializer>::init(helper, event)?;
                    return self.handle_x509_issuer_serial(
                        helper,
                        Default::default(),
                        None,
                        output,
                    );
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"X509SKI")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_x509_ski(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"X509SubjectName")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_x509_subject_name(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"X509Certificate")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_x509_certificate(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"X509CRL")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_x509_crl(helper, Default::default(), None, output);
                }
            }
            *self.state__ = X509DataContent43TypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, true))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: X509DataContent43TypeContentDeserializerState,
        ) -> Result<super::X509DataContent43TypeContent, Error> {
            use X509DataContent43TypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::X509IssuerSerial(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_x509_issuer_serial(&mut values, value)?;
                    }
                    Ok(super::X509DataContent43TypeContent::X509IssuerSerial(
                        helper.finish_element("X509IssuerSerial", values)?,
                    ))
                }
                S::X509Ski(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_x509_ski(&mut values, value)?;
                    }
                    Ok(super::X509DataContent43TypeContent::X509Ski(
                        helper.finish_element("X509SKI", values)?,
                    ))
                }
                S::X509SubjectName(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_x509_subject_name(&mut values, value)?;
                    }
                    Ok(super::X509DataContent43TypeContent::X509SubjectName(
                        helper.finish_element("X509SubjectName", values)?,
                    ))
                }
                S::X509Certificate(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_x509_certificate(&mut values, value)?;
                    }
                    Ok(super::X509DataContent43TypeContent::X509Certificate(
                        helper.finish_element("X509Certificate", values)?,
                    ))
                }
                S::X509Crl(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_x509_crl(&mut values, value)?;
                    }
                    Ok(super::X509DataContent43TypeContent::X509Crl(
                        helper.finish_element("X509CRL", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_x509_issuer_serial(
            values: &mut Option<super::X509IssuerSerialType>,
            value: super::X509IssuerSerialType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"X509IssuerSerial",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_x509_ski(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"X509SKI",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_x509_subject_name(
            values: &mut Option<String>,
            value: String,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"X509SubjectName",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_x509_certificate(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"X509Certificate",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_x509_crl(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"X509CRL",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_x509_issuer_serial<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::X509IssuerSerialType>,
            fallback: Option<<super::X509IssuerSerialType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::X509IssuerSerialType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use X509DataContent43TypeContentDeserializerState as S;
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
                Self::store_x509_issuer_serial(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x509_issuer_serial(&mut values, data)?;
                    let data = Self::finish_state(helper, S::X509IssuerSerial(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::X509IssuerSerial(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_x509_ski<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use X509DataContent43TypeContentDeserializerState as S;
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
                Self::store_x509_ski(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x509_ski(&mut values, data)?;
                    let data = Self::finish_state(helper, S::X509Ski(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::X509Ski(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_x509_subject_name<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use X509DataContent43TypeContentDeserializerState as S;
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
                Self::store_x509_subject_name(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x509_subject_name(&mut values, data)?;
                    let data = Self::finish_state(helper, S::X509SubjectName(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::X509SubjectName(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_x509_certificate<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use X509DataContent43TypeContentDeserializerState as S;
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
                Self::store_x509_certificate(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x509_certificate(&mut values, data)?;
                    let data = Self::finish_state(helper, S::X509Certificate(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::X509Certificate(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_x509_crl<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use X509DataContent43TypeContentDeserializerState as S;
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
                Self::store_x509_crl(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x509_crl(&mut values, data)?;
                    let data = Self::finish_state(helper, S::X509Crl(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::X509Crl(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::X509DataContent43TypeContent>
        for X509DataContent43TypeContentDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::X509DataContent43TypeContent> {
            let deserializer = Self {
                state__: Box::new(X509DataContent43TypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        X509DataContent43TypeContentDeserializerState::Init__
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
        ) -> DeserializerResult<'de, super::X509DataContent43TypeContent> {
            use X509DataContent43TypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::X509IssuerSerial(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_x509_issuer_serial(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::X509Ski(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_x509_ski(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::X509SubjectName(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_x509_subject_name(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::X509Certificate(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_x509_certificate(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::X509Crl(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_x509_crl(helper, values, fallback, output)? {
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
                        S::X509IssuerSerial(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"X509IssuerSerial",
                            false,
                        )?;
                        match self.handle_x509_issuer_serial(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::X509Ski(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"X509SKI",
                            false,
                        )?;
                        match self.handle_x509_ski(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::X509SubjectName(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"X509SubjectName",
                            false,
                        )?;
                        match self.handle_x509_subject_name(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::X509Certificate(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"X509Certificate",
                            false,
                        )?;
                        match self.handle_x509_certificate(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::X509Crl(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"X509CRL",
                            false,
                        )?;
                        match self.handle_x509_crl(helper, values, fallback, output)? {
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
        ) -> Result<super::X509DataContent43TypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct PgpDataContent47TypeDeserializer {
        pgp_key_id: Option<String>,
        pgp_key_packet: Option<String>,
        state__: Box<PgpDataContent47TypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PgpDataContent47TypeDeserializerState {
        Init__,
        PgpKeyId(Option<<String as WithDeserializer>::Deserializer>),
        PgpKeyPacket(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl PgpDataContent47TypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: PgpDataContent47TypeDeserializerState,
        ) -> Result<(), Error> {
            use PgpDataContent47TypeDeserializerState as S;
            match state {
                S::PgpKeyId(Some(deserializer)) => {
                    self.store_pgp_key_id(deserializer.finish(helper)?)?
                }
                S::PgpKeyPacket(Some(deserializer)) => {
                    self.store_pgp_key_packet(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_pgp_key_id(&mut self, value: String) -> Result<(), Error> {
            if self.pgp_key_id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PGPKeyID",
                )))?;
            }
            self.pgp_key_id = Some(value);
            Ok(())
        }
        fn store_pgp_key_packet(&mut self, value: String) -> Result<(), Error> {
            if self.pgp_key_packet.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PGPKeyPacket",
                )))?;
            }
            self.pgp_key_packet = Some(value);
            Ok(())
        }
        fn handle_pgp_key_id<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<PgpDataContent47TypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PgpDataContent47TypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::PgpKeyId(None));
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
                    self.store_pgp_key_id(data)?;
                    *self.state__ = S::PgpKeyPacket(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::PgpKeyId(Some(deserializer)));
                    *self.state__ = S::PgpKeyPacket(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_pgp_key_packet<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<PgpDataContent47TypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PgpDataContent47TypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::PgpKeyPacket(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_pgp_key_packet(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::PgpKeyPacket(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::PgpDataContent47Type> for PgpDataContent47TypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PgpDataContent47Type> {
            let deserializer = Self {
                pgp_key_id: None,
                pgp_key_packet: None,
                state__: Box::new(PgpDataContent47TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, PgpDataContent47TypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::PgpDataContent47Type> {
            use PgpDataContent47TypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::PgpKeyId(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_pgp_key_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::PgpKeyPacket(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_pgp_key_packet(helper, output, &mut fallback)? {
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
                        allow_any_element = true;
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::PgpKeyId(None);
                        event
                    }
                    (S::PgpKeyId(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"PGPKeyID",
                            true,
                        )?;
                        match self.handle_pgp_key_id(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::PgpKeyPacket(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"PGPKeyPacket",
                            true,
                        )?;
                        match self.handle_pgp_key_packet(helper, output, &mut fallback)? {
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
                        break (DeserializerEvent::Continue(event), true);
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
        ) -> Result<super::PgpDataContent47Type, Error> {
            let state = replace(
                &mut *self.state__,
                PgpDataContent47TypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::PgpDataContent47Type {
                pgp_key_id: helper.finish_element("PGPKeyID", self.pgp_key_id)?,
                pgp_key_packet: self.pgp_key_packet,
            })
        }
    }
    #[derive(Debug)]
    pub struct PgpDataContent49TypeDeserializer {
        pgp_key_packet: Option<String>,
        state__: Box<PgpDataContent49TypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PgpDataContent49TypeDeserializerState {
        Init__,
        PgpKeyPacket(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl PgpDataContent49TypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: PgpDataContent49TypeDeserializerState,
        ) -> Result<(), Error> {
            use PgpDataContent49TypeDeserializerState as S;
            match state {
                S::PgpKeyPacket(Some(deserializer)) => {
                    self.store_pgp_key_packet(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_pgp_key_packet(&mut self, value: String) -> Result<(), Error> {
            if self.pgp_key_packet.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PGPKeyPacket",
                )))?;
            }
            self.pgp_key_packet = Some(value);
            Ok(())
        }
        fn handle_pgp_key_packet<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<PgpDataContent49TypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PgpDataContent49TypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::PgpKeyPacket(None));
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
                    self.store_pgp_key_packet(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::PgpKeyPacket(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::PgpDataContent49Type> for PgpDataContent49TypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PgpDataContent49Type> {
            let deserializer = Self {
                pgp_key_packet: None,
                state__: Box::new(PgpDataContent49TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, PgpDataContent49TypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::PgpDataContent49Type> {
            use PgpDataContent49TypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::PgpKeyPacket(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_pgp_key_packet(helper, output, &mut fallback)? {
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
                        allow_any_element = true;
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::PgpKeyPacket(None);
                        event
                    }
                    (S::PgpKeyPacket(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"PGPKeyPacket",
                            true,
                        )?;
                        match self.handle_pgp_key_packet(helper, output, &mut fallback)? {
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
                        break (DeserializerEvent::Continue(event), true);
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
        ) -> Result<super::PgpDataContent49Type, Error> {
            let state = replace(
                &mut *self.state__,
                PgpDataContent49TypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::PgpDataContent49Type {
                pgp_key_packet: helper.finish_element("PGPKeyPacket", self.pgp_key_packet)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TransformTypeDeserializer {
        algorithm: String,
        content: Vec<super::TransformTypeContent>,
        state__: Box<TransformTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TransformTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::TransformTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl TransformTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut algorithm: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Algorithm")
                ) {
                    helper.read_attrib(&mut algorithm, b"Algorithm", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                algorithm: algorithm
                    .ok_or_else(|| ErrorKind::MissingAttribute("Algorithm".into()))?,
                content: Vec::new(),
                state__: Box::new(TransformTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TransformTypeDeserializerState,
        ) -> Result<(), Error> {
            if let TransformTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::TransformTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TransformTypeContent>,
            fallback: &mut Option<TransformTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TransformTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::TransformType> for TransformTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TransformType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TransformType> {
            use TransformTypeDeserializerState as S;
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
                            <super::TransformTypeContent as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::TransformType, Error> {
            let state = replace(
                &mut *self.state__,
                TransformTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::TransformType {
                algorithm: self.algorithm,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TransformTypeContentDeserializer {
        state__: Box<TransformTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum TransformTypeContentDeserializerState {
        Init__,
        XPath(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::TransformTypeContent),
        Unknown__,
    }
    impl TransformTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"XPath")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_x_path(helper, Default::default(), None, output);
                }
            }
            *self.state__ = TransformTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, true))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: TransformTypeContentDeserializerState,
        ) -> Result<super::TransformTypeContent, Error> {
            use TransformTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::XPath(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_x_path(&mut values, value)?;
                    }
                    Ok(super::TransformTypeContent::XPath(
                        helper.finish_element("XPath", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_x_path(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"XPath",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_x_path<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TransformTypeContentDeserializerState as S;
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
                Self::store_x_path(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x_path(&mut values, data)?;
                    let data = Self::finish_state(helper, S::XPath(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::XPath(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TransformTypeContent> for TransformTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TransformTypeContent> {
            let deserializer = Self {
                state__: Box::new(TransformTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, TransformTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::TransformTypeContent> {
            use TransformTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::XPath(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_x_path(helper, values, fallback, output)? {
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
                        S::XPath(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"XPath",
                            false,
                        )?;
                        match self.handle_x_path(helper, values, fallback, output)? {
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
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::TransformTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct DsaKeyValueContent60TypeDeserializer {
        p: Option<String>,
        q: Option<String>,
        state__: Box<DsaKeyValueContent60TypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DsaKeyValueContent60TypeDeserializerState {
        Init__,
        P(Option<<String as WithDeserializer>::Deserializer>),
        Q(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DsaKeyValueContent60TypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: DsaKeyValueContent60TypeDeserializerState,
        ) -> Result<(), Error> {
            use DsaKeyValueContent60TypeDeserializerState as S;
            match state {
                S::P(Some(deserializer)) => self.store_p(deserializer.finish(helper)?)?,
                S::Q(Some(deserializer)) => self.store_q(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_p(&mut self, value: String) -> Result<(), Error> {
            if self.p.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"P")))?;
            }
            self.p = Some(value);
            Ok(())
        }
        fn store_q(&mut self, value: String) -> Result<(), Error> {
            if self.q.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Q")))?;
            }
            self.q = Some(value);
            Ok(())
        }
        fn handle_p<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DsaKeyValueContent60TypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DsaKeyValueContent60TypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::P(None));
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
                    self.store_p(data)?;
                    *self.state__ = S::Q(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::P(Some(deserializer)));
                    *self.state__ = S::Q(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_q<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DsaKeyValueContent60TypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DsaKeyValueContent60TypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Q(None));
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
                    self.store_q(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Q(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DsaKeyValueContent60Type>
        for DsaKeyValueContent60TypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DsaKeyValueContent60Type> {
            let deserializer = Self {
                p: None,
                q: None,
                state__: Box::new(DsaKeyValueContent60TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        DsaKeyValueContent60TypeDeserializerState::Init__
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
        ) -> DeserializerResult<'de, super::DsaKeyValueContent60Type> {
            use DsaKeyValueContent60TypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::P(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_p(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Q(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_q(helper, output, &mut fallback)? {
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
                        *self.state__ = S::P(None);
                        event
                    }
                    (S::P(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"P",
                            false,
                        )?;
                        match self.handle_p(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Q(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"Q",
                            false,
                        )?;
                        match self.handle_q(helper, output, &mut fallback)? {
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
        ) -> Result<super::DsaKeyValueContent60Type, Error> {
            let state = replace(
                &mut *self.state__,
                DsaKeyValueContent60TypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::DsaKeyValueContent60Type {
                p: helper.finish_element("P", self.p)?,
                q: helper.finish_element("Q", self.q)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DsaKeyValueContent61TypeDeserializer {
        seed: Option<String>,
        pgen_counter: Option<String>,
        state__: Box<DsaKeyValueContent61TypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DsaKeyValueContent61TypeDeserializerState {
        Init__,
        Seed(Option<<String as WithDeserializer>::Deserializer>),
        PgenCounter(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DsaKeyValueContent61TypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: DsaKeyValueContent61TypeDeserializerState,
        ) -> Result<(), Error> {
            use DsaKeyValueContent61TypeDeserializerState as S;
            match state {
                S::Seed(Some(deserializer)) => self.store_seed(deserializer.finish(helper)?)?,
                S::PgenCounter(Some(deserializer)) => {
                    self.store_pgen_counter(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_seed(&mut self, value: String) -> Result<(), Error> {
            if self.seed.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Seed")))?;
            }
            self.seed = Some(value);
            Ok(())
        }
        fn store_pgen_counter(&mut self, value: String) -> Result<(), Error> {
            if self.pgen_counter.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PgenCounter",
                )))?;
            }
            self.pgen_counter = Some(value);
            Ok(())
        }
        fn handle_seed<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DsaKeyValueContent61TypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DsaKeyValueContent61TypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Seed(None));
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
                    self.store_seed(data)?;
                    *self.state__ = S::PgenCounter(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Seed(Some(deserializer)));
                    *self.state__ = S::PgenCounter(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_pgen_counter<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DsaKeyValueContent61TypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DsaKeyValueContent61TypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::PgenCounter(None));
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
                    self.store_pgen_counter(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::PgenCounter(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DsaKeyValueContent61Type>
        for DsaKeyValueContent61TypeDeserializer
    {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DsaKeyValueContent61Type> {
            let deserializer = Self {
                seed: None,
                pgen_counter: None,
                state__: Box::new(DsaKeyValueContent61TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state__,
                        DsaKeyValueContent61TypeDeserializerState::Init__
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
        ) -> DeserializerResult<'de, super::DsaKeyValueContent61Type> {
            use DsaKeyValueContent61TypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Seed(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_seed(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::PgenCounter(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_pgen_counter(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Seed(None);
                        event
                    }
                    (S::Seed(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"Seed",
                            false,
                        )?;
                        match self.handle_seed(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::PgenCounter(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"PgenCounter",
                            false,
                        )?;
                        match self.handle_pgen_counter(helper, output, &mut fallback)? {
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
        ) -> Result<super::DsaKeyValueContent61Type, Error> {
            let state = replace(
                &mut *self.state__,
                DsaKeyValueContent61TypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::DsaKeyValueContent61Type {
                seed: helper.finish_element("Seed", self.seed)?,
                pgen_counter: helper.finish_element("PgenCounter", self.pgen_counter)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct X509IssuerSerialTypeDeserializer {
        x509_issuer_name: Option<String>,
        x509_serial_number: Option<i32>,
        state__: Box<X509IssuerSerialTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum X509IssuerSerialTypeDeserializerState {
        Init__,
        X509IssuerName(Option<<String as WithDeserializer>::Deserializer>),
        X509SerialNumber(Option<<i32 as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl X509IssuerSerialTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                x509_issuer_name: None,
                x509_serial_number: None,
                state__: Box::new(X509IssuerSerialTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: X509IssuerSerialTypeDeserializerState,
        ) -> Result<(), Error> {
            use X509IssuerSerialTypeDeserializerState as S;
            match state {
                S::X509IssuerName(Some(deserializer)) => {
                    self.store_x509_issuer_name(deserializer.finish(helper)?)?
                }
                S::X509SerialNumber(Some(deserializer)) => {
                    self.store_x509_serial_number(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_x509_issuer_name(&mut self, value: String) -> Result<(), Error> {
            if self.x509_issuer_name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"X509IssuerName",
                )))?;
            }
            self.x509_issuer_name = Some(value);
            Ok(())
        }
        fn store_x509_serial_number(&mut self, value: i32) -> Result<(), Error> {
            if self.x509_serial_number.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"X509SerialNumber",
                )))?;
            }
            self.x509_serial_number = Some(value);
            Ok(())
        }
        fn handle_x509_issuer_name<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<X509IssuerSerialTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use X509IssuerSerialTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::X509IssuerName(None));
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
                    self.store_x509_issuer_name(data)?;
                    *self.state__ = S::X509SerialNumber(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::X509IssuerName(Some(deserializer)));
                    *self.state__ = S::X509SerialNumber(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_x509_serial_number<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<X509IssuerSerialTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use X509IssuerSerialTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::X509SerialNumber(None));
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
                    self.store_x509_serial_number(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::X509SerialNumber(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::X509IssuerSerialType> for X509IssuerSerialTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::X509IssuerSerialType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::X509IssuerSerialType> {
            use X509IssuerSerialTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::X509IssuerName(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_x509_issuer_name(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::X509SerialNumber(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_x509_serial_number(helper, output, &mut fallback)? {
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
                        *self.state__ = S::X509IssuerName(None);
                        event
                    }
                    (S::X509IssuerName(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"X509IssuerName",
                            false,
                        )?;
                        match self.handle_x509_issuer_name(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::X509SerialNumber(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DS),
                            b"X509SerialNumber",
                            false,
                        )?;
                        match self.handle_x509_serial_number(helper, output, &mut fallback)? {
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
        ) -> Result<super::X509IssuerSerialType, Error> {
            let state = replace(
                &mut *self.state__,
                X509IssuerSerialTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::X509IssuerSerialType {
                x509_issuer_name: helper.finish_element("X509IssuerName", self.x509_issuer_name)?,
                x509_serial_number: helper
                    .finish_element("X509SerialNumber", self.x509_serial_number)?,
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
    pub struct DirectoryReqTypeSerializer<'ser> {
        pub(super) value: &'ser super::DirectoryReqType,
        pub(super) state: Box<DirectoryReqTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DirectoryReqTypeSerializerState<'ser> {
        Init__,
        CreateDateTimestamp(<String as WithSerializer>::Serializer<'ser>),
        Merchant(<super::DirectoryReqMerchantType as WithSerializer>::Serializer<'ser>),
        Signature(<super::SignatureType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DirectoryReqTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DirectoryReqTypeSerializerState::Init__ => {
                        *self.state = DirectoryReqTypeSerializerState::CreateDateTimestamp(
                            WithSerializer::serializer(
                                &self.value.create_date_timestamp,
                                Some("createDateTimestamp"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        if self.is_root {
                            helper.write_xmlns(&mut bytes, Some(&super::PREFIX_DS), &super::NS_DS);
                        }
                        helper.write_attrib(&mut bytes, "version", &self.value.version)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DirectoryReqTypeSerializerState::CreateDateTimestamp(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = DirectoryReqTypeSerializerState::Merchant(
                                    WithSerializer::serializer(
                                        &self.value.merchant,
                                        Some("Merchant"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    DirectoryReqTypeSerializerState::Merchant(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = DirectoryReqTypeSerializerState::Signature(
                                    WithSerializer::serializer(
                                        &self.value.signature,
                                        Some("ds:Signature"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    DirectoryReqTypeSerializerState::Signature(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = DirectoryReqTypeSerializerState::End__,
                        }
                    }
                    DirectoryReqTypeSerializerState::End__ => {
                        *self.state = DirectoryReqTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DirectoryReqTypeSerializerState::Done__ => return Ok(None),
                    DirectoryReqTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for DirectoryReqTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DirectoryReqTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DirectoryReqMerchantTypeSerializer<'ser> {
        pub(super) value: &'ser super::DirectoryReqMerchantType,
        pub(super) state: Box<DirectoryReqMerchantTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DirectoryReqMerchantTypeSerializerState<'ser> {
        Init__,
        MerchantId(<String as WithSerializer>::Serializer<'ser>),
        SubId(<usize as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DirectoryReqMerchantTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DirectoryReqMerchantTypeSerializerState::Init__ => {
                        *self.state = DirectoryReqMerchantTypeSerializerState::MerchantId(
                            WithSerializer::serializer(
                                &self.value.merchant_id,
                                Some("merchantID"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DirectoryReqMerchantTypeSerializerState::MerchantId(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = DirectoryReqMerchantTypeSerializerState::SubId(
                                    WithSerializer::serializer(
                                        &self.value.sub_id,
                                        Some("subID"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    DirectoryReqMerchantTypeSerializerState::SubId(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = DirectoryReqMerchantTypeSerializerState::End__,
                        }
                    }
                    DirectoryReqMerchantTypeSerializerState::End__ => {
                        *self.state = DirectoryReqMerchantTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DirectoryReqMerchantTypeSerializerState::Done__ => return Ok(None),
                    DirectoryReqMerchantTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for DirectoryReqMerchantTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DirectoryReqMerchantTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SignatureTypeSerializer<'ser> {
        pub(super) value: &'ser super::SignatureType,
        pub(super) state: Box<SignatureTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SignatureTypeSerializerState<'ser> {
        Init__,
        SignedInfo(<super::SignedInfoType as WithSerializer>::Serializer<'ser>),
        SignatureValue(<super::SignatureValueType as WithSerializer>::Serializer<'ser>),
        KeyInfo(IterSerializer<'ser, Option<&'ser super::KeyInfoType>, super::KeyInfoType>),
        Object(IterSerializer<'ser, &'ser [super::ObjectType], super::ObjectType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SignatureTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SignatureTypeSerializerState::Init__ => {
                        *self.state =
                            SignatureTypeSerializerState::SignedInfo(WithSerializer::serializer(
                                &self.value.signed_info,
                                Some("ds:SignedInfo"),
                                false,
                            )?);
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(&mut bytes, Some(&super::PREFIX_DS), &super::NS_DS);
                        }
                        helper.write_attrib_opt(&mut bytes, "Id", &self.value.id)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SignatureTypeSerializerState::SignedInfo(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = SignatureTypeSerializerState::SignatureValue(
                                    WithSerializer::serializer(
                                        &self.value.signature_value,
                                        Some("ds:SignatureValue"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    SignatureTypeSerializerState::SignatureValue(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    SignatureTypeSerializerState::KeyInfo(IterSerializer::new(
                                        self.value.key_info.as_ref(),
                                        Some("ds:KeyInfo"),
                                        false,
                                    ))
                            }
                        }
                    }
                    SignatureTypeSerializerState::KeyInfo(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = SignatureTypeSerializerState::Object(IterSerializer::new(
                                &self.value.object[..],
                                Some("ds:Object"),
                                false,
                            ))
                        }
                    },
                    SignatureTypeSerializerState::Object(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = SignatureTypeSerializerState::End__,
                    },
                    SignatureTypeSerializerState::End__ => {
                        *self.state = SignatureTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SignatureTypeSerializerState::Done__ => return Ok(None),
                    SignatureTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for SignatureTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SignatureTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SignedInfoTypeSerializer<'ser> {
        pub(super) value: &'ser super::SignedInfoType,
        pub(super) state: Box<SignedInfoTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SignedInfoTypeSerializerState<'ser> {
        Init__,
        CanonicalizationMethod(
            <super::CanonicalizationMethodType as WithSerializer>::Serializer<'ser>,
        ),
        SignatureMethod(<super::SignatureMethodType as WithSerializer>::Serializer<'ser>),
        Reference(IterSerializer<'ser, &'ser [super::ReferenceType], super::ReferenceType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SignedInfoTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SignedInfoTypeSerializerState::Init__ => {
                        *self.state = SignedInfoTypeSerializerState::CanonicalizationMethod(
                            WithSerializer::serializer(
                                &self.value.canonicalization_method,
                                Some("ds:CanonicalizationMethod"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(&mut bytes, Some(&super::PREFIX_DS), &super::NS_DS);
                        }
                        helper.write_attrib_opt(&mut bytes, "Id", &self.value.id)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SignedInfoTypeSerializerState::CanonicalizationMethod(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = SignedInfoTypeSerializerState::SignatureMethod(
                                    WithSerializer::serializer(
                                        &self.value.signature_method,
                                        Some("ds:SignatureMethod"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    SignedInfoTypeSerializerState::SignatureMethod(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    SignedInfoTypeSerializerState::Reference(IterSerializer::new(
                                        &self.value.reference[..],
                                        Some("ds:Reference"),
                                        false,
                                    ))
                            }
                        }
                    }
                    SignedInfoTypeSerializerState::Reference(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SignedInfoTypeSerializerState::End__,
                        }
                    }
                    SignedInfoTypeSerializerState::End__ => {
                        *self.state = SignedInfoTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SignedInfoTypeSerializerState::Done__ => return Ok(None),
                    SignedInfoTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for SignedInfoTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SignedInfoTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SignatureValueTypeSerializer<'ser> {
        pub(super) value: &'ser super::SignatureValueType,
        pub(super) state: Box<SignatureValueTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SignatureValueTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SignatureValueTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SignatureValueTypeSerializerState::Init__ => {
                        *self.state = SignatureValueTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(&mut bytes, Some(&super::PREFIX_DS), &super::NS_DS);
                        }
                        helper.write_attrib_opt(&mut bytes, "Id", &self.value.id)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SignatureValueTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SignatureValueTypeSerializerState::End__,
                        }
                    }
                    SignatureValueTypeSerializerState::End__ => {
                        *self.state = SignatureValueTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SignatureValueTypeSerializerState::Done__ => return Ok(None),
                    SignatureValueTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for SignatureValueTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SignatureValueTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct KeyInfoTypeSerializer<'ser> {
        pub(super) value: &'ser super::KeyInfoType,
        pub(super) state: Box<KeyInfoTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum KeyInfoTypeSerializerState<'ser> {
        Init__,
        Content__(
            IterSerializer<'ser, &'ser [super::KeyInfoTypeContent], super::KeyInfoTypeContent>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> KeyInfoTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    KeyInfoTypeSerializerState::Init__ => {
                        *self.state = KeyInfoTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(&mut bytes, Some(&super::PREFIX_DS), &super::NS_DS);
                        }
                        helper.write_attrib_opt(&mut bytes, "Id", &self.value.id)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    KeyInfoTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = KeyInfoTypeSerializerState::End__,
                    },
                    KeyInfoTypeSerializerState::End__ => {
                        *self.state = KeyInfoTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    KeyInfoTypeSerializerState::Done__ => return Ok(None),
                    KeyInfoTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for KeyInfoTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = KeyInfoTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct KeyInfoTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::KeyInfoTypeContent,
        pub(super) state: Box<KeyInfoTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum KeyInfoTypeContentSerializerState<'ser> {
        Init__,
        KeyName(<String as WithSerializer>::Serializer<'ser>),
        KeyValue(<super::KeyValueType as WithSerializer>::Serializer<'ser>),
        RetrievalMethod(<super::RetrievalMethodType as WithSerializer>::Serializer<'ser>),
        X509Data(<super::X509DataType as WithSerializer>::Serializer<'ser>),
        PgpData(<super::PgpDataType as WithSerializer>::Serializer<'ser>),
        SpkiData(<super::SpkiDataType as WithSerializer>::Serializer<'ser>),
        MgmtData(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> KeyInfoTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    KeyInfoTypeContentSerializerState::Init__ => match self.value {
                        super::KeyInfoTypeContent::KeyName(x) => {
                            *self.state = KeyInfoTypeContentSerializerState::KeyName(
                                WithSerializer::serializer(x, Some("ds:KeyName"), false)?,
                            )
                        }
                        super::KeyInfoTypeContent::KeyValue(x) => {
                            *self.state = KeyInfoTypeContentSerializerState::KeyValue(
                                WithSerializer::serializer(x, Some("ds:KeyValue"), false)?,
                            )
                        }
                        super::KeyInfoTypeContent::RetrievalMethod(x) => {
                            *self.state = KeyInfoTypeContentSerializerState::RetrievalMethod(
                                WithSerializer::serializer(x, Some("ds:RetrievalMethod"), false)?,
                            )
                        }
                        super::KeyInfoTypeContent::X509Data(x) => {
                            *self.state = KeyInfoTypeContentSerializerState::X509Data(
                                WithSerializer::serializer(x, Some("ds:X509Data"), false)?,
                            )
                        }
                        super::KeyInfoTypeContent::PgpData(x) => {
                            *self.state = KeyInfoTypeContentSerializerState::PgpData(
                                WithSerializer::serializer(x, Some("ds:PGPData"), false)?,
                            )
                        }
                        super::KeyInfoTypeContent::SpkiData(x) => {
                            *self.state = KeyInfoTypeContentSerializerState::SpkiData(
                                WithSerializer::serializer(x, Some("ds:SPKIData"), false)?,
                            )
                        }
                        super::KeyInfoTypeContent::MgmtData(x) => {
                            *self.state = KeyInfoTypeContentSerializerState::MgmtData(
                                WithSerializer::serializer(x, Some("ds:MgmtData"), false)?,
                            )
                        }
                    },
                    KeyInfoTypeContentSerializerState::KeyName(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = KeyInfoTypeContentSerializerState::Done__,
                        }
                    }
                    KeyInfoTypeContentSerializerState::KeyValue(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = KeyInfoTypeContentSerializerState::Done__,
                        }
                    }
                    KeyInfoTypeContentSerializerState::RetrievalMethod(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = KeyInfoTypeContentSerializerState::Done__,
                        }
                    }
                    KeyInfoTypeContentSerializerState::X509Data(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = KeyInfoTypeContentSerializerState::Done__,
                        }
                    }
                    KeyInfoTypeContentSerializerState::PgpData(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = KeyInfoTypeContentSerializerState::Done__,
                        }
                    }
                    KeyInfoTypeContentSerializerState::SpkiData(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = KeyInfoTypeContentSerializerState::Done__,
                        }
                    }
                    KeyInfoTypeContentSerializerState::MgmtData(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = KeyInfoTypeContentSerializerState::Done__,
                        }
                    }
                    KeyInfoTypeContentSerializerState::Done__ => return Ok(None),
                    KeyInfoTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for KeyInfoTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = KeyInfoTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ObjectTypeSerializer<'ser> {
        pub(super) value: &'ser super::ObjectType,
        pub(super) state: Box<ObjectTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ObjectTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ObjectTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ObjectTypeSerializerState::Init__ => {
                        *self.state = ObjectTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(&mut bytes, Some(&super::PREFIX_DS), &super::NS_DS);
                        }
                        helper.write_attrib_opt(&mut bytes, "Id", &self.value.id)?;
                        helper.write_attrib_opt(&mut bytes, "MimeType", &self.value.mime_type)?;
                        helper.write_attrib_opt(&mut bytes, "Encoding", &self.value.encoding)?;
                        helper.end_ns_scope();
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    ObjectTypeSerializerState::Done__ => return Ok(None),
                    ObjectTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ObjectTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ObjectTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct CanonicalizationMethodTypeSerializer<'ser> {
        pub(super) value: &'ser super::CanonicalizationMethodType,
        pub(super) state: Box<CanonicalizationMethodTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum CanonicalizationMethodTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CanonicalizationMethodTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CanonicalizationMethodTypeSerializerState::Init__ => {
                        *self.state = CanonicalizationMethodTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(&mut bytes, Some(&super::PREFIX_DS), &super::NS_DS);
                        }
                        helper.write_attrib(&mut bytes, "Algorithm", &self.value.algorithm)?;
                        helper.end_ns_scope();
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    CanonicalizationMethodTypeSerializerState::Done__ => return Ok(None),
                    CanonicalizationMethodTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for CanonicalizationMethodTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CanonicalizationMethodTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SignatureMethodTypeSerializer<'ser> {
        pub(super) value: &'ser super::SignatureMethodType,
        pub(super) state: Box<SignatureMethodTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SignatureMethodTypeSerializerState<'ser> {
        Init__,
        HmacOutputLength(IterSerializer<'ser, Option<&'ser i32>, i32>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SignatureMethodTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SignatureMethodTypeSerializerState::Init__ => {
                        *self.state = SignatureMethodTypeSerializerState::HmacOutputLength(
                            IterSerializer::new(
                                self.value.hmac_output_length.as_ref(),
                                Some("ds:HMACOutputLength"),
                                false,
                            ),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(&mut bytes, Some(&super::PREFIX_DS), &super::NS_DS);
                        }
                        helper.write_attrib(&mut bytes, "Algorithm", &self.value.algorithm)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SignatureMethodTypeSerializerState::HmacOutputLength(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SignatureMethodTypeSerializerState::End__,
                        }
                    }
                    SignatureMethodTypeSerializerState::End__ => {
                        *self.state = SignatureMethodTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SignatureMethodTypeSerializerState::Done__ => return Ok(None),
                    SignatureMethodTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for SignatureMethodTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SignatureMethodTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ReferenceTypeSerializer<'ser> {
        pub(super) value: &'ser super::ReferenceType,
        pub(super) state: Box<ReferenceTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ReferenceTypeSerializerState<'ser> {
        Init__,
        Transforms(
            IterSerializer<'ser, Option<&'ser super::TransformsType>, super::TransformsType>,
        ),
        DigestMethod(<super::DigestMethodType as WithSerializer>::Serializer<'ser>),
        DigestValue(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ReferenceTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ReferenceTypeSerializerState::Init__ => {
                        *self.state =
                            ReferenceTypeSerializerState::Transforms(IterSerializer::new(
                                self.value.transforms.as_ref(),
                                Some("ds:Transforms"),
                                false,
                            ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(&mut bytes, Some(&super::PREFIX_DS), &super::NS_DS);
                        }
                        helper.write_attrib_opt(&mut bytes, "Id", &self.value.id)?;
                        helper.write_attrib_opt(&mut bytes, "URI", &self.value.uri)?;
                        helper.write_attrib_opt(&mut bytes, "Type", &self.value.type_)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ReferenceTypeSerializerState::Transforms(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = ReferenceTypeSerializerState::DigestMethod(
                                    WithSerializer::serializer(
                                        &self.value.digest_method,
                                        Some("ds:DigestMethod"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    ReferenceTypeSerializerState::DigestMethod(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = ReferenceTypeSerializerState::DigestValue(
                                    WithSerializer::serializer(
                                        &self.value.digest_value,
                                        Some("ds:DigestValue"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    ReferenceTypeSerializerState::DigestValue(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ReferenceTypeSerializerState::End__,
                        }
                    }
                    ReferenceTypeSerializerState::End__ => {
                        *self.state = ReferenceTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ReferenceTypeSerializerState::Done__ => return Ok(None),
                    ReferenceTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ReferenceTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ReferenceTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct KeyValueTypeSerializer<'ser> {
        pub(super) value: &'ser super::KeyValueType,
        pub(super) state: Box<KeyValueTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum KeyValueTypeSerializerState<'ser> {
        Init__,
        Content__(<super::KeyValueTypeContent as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> KeyValueTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    KeyValueTypeSerializerState::Init__ => {
                        *self.state = KeyValueTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(&mut bytes, Some(&super::PREFIX_DS), &super::NS_DS);
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    KeyValueTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = KeyValueTypeSerializerState::End__,
                        }
                    }
                    KeyValueTypeSerializerState::End__ => {
                        *self.state = KeyValueTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    KeyValueTypeSerializerState::Done__ => return Ok(None),
                    KeyValueTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for KeyValueTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = KeyValueTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct KeyValueTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::KeyValueTypeContent,
        pub(super) state: Box<KeyValueTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum KeyValueTypeContentSerializerState<'ser> {
        Init__,
        DsaKeyValue(<super::DsaKeyValueType as WithSerializer>::Serializer<'ser>),
        RsaKeyValue(<super::RsaKeyValueType as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> KeyValueTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    KeyValueTypeContentSerializerState::Init__ => match self.value {
                        super::KeyValueTypeContent::DsaKeyValue(x) => {
                            *self.state = KeyValueTypeContentSerializerState::DsaKeyValue(
                                WithSerializer::serializer(x, Some("ds:DSAKeyValue"), false)?,
                            )
                        }
                        super::KeyValueTypeContent::RsaKeyValue(x) => {
                            *self.state = KeyValueTypeContentSerializerState::RsaKeyValue(
                                WithSerializer::serializer(x, Some("ds:RSAKeyValue"), false)?,
                            )
                        }
                    },
                    KeyValueTypeContentSerializerState::DsaKeyValue(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = KeyValueTypeContentSerializerState::Done__,
                        }
                    }
                    KeyValueTypeContentSerializerState::RsaKeyValue(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = KeyValueTypeContentSerializerState::Done__,
                        }
                    }
                    KeyValueTypeContentSerializerState::Done__ => return Ok(None),
                    KeyValueTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for KeyValueTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = KeyValueTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct RetrievalMethodTypeSerializer<'ser> {
        pub(super) value: &'ser super::RetrievalMethodType,
        pub(super) state: Box<RetrievalMethodTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum RetrievalMethodTypeSerializerState<'ser> {
        Init__,
        Transforms(
            IterSerializer<'ser, Option<&'ser super::TransformsType>, super::TransformsType>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RetrievalMethodTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    RetrievalMethodTypeSerializerState::Init__ => {
                        *self.state =
                            RetrievalMethodTypeSerializerState::Transforms(IterSerializer::new(
                                self.value.transforms.as_ref(),
                                Some("ds:Transforms"),
                                false,
                            ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(&mut bytes, Some(&super::PREFIX_DS), &super::NS_DS);
                        }
                        helper.write_attrib_opt(&mut bytes, "URI", &self.value.uri)?;
                        helper.write_attrib_opt(&mut bytes, "Type", &self.value.type_)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    RetrievalMethodTypeSerializerState::Transforms(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RetrievalMethodTypeSerializerState::End__,
                        }
                    }
                    RetrievalMethodTypeSerializerState::End__ => {
                        *self.state = RetrievalMethodTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    RetrievalMethodTypeSerializerState::Done__ => return Ok(None),
                    RetrievalMethodTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for RetrievalMethodTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = RetrievalMethodTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct X509DataTypeSerializer<'ser> {
        pub(super) value: &'ser super::X509DataType,
        pub(super) state: Box<X509DataTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum X509DataTypeSerializerState<'ser> {
        Init__,
        Content__(
            IterSerializer<'ser, &'ser [super::X509DataTypeContent], super::X509DataTypeContent>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> X509DataTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    X509DataTypeSerializerState::Init__ => {
                        *self.state = X509DataTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(&mut bytes, Some(&super::PREFIX_DS), &super::NS_DS);
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    X509DataTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = X509DataTypeSerializerState::End__,
                        }
                    }
                    X509DataTypeSerializerState::End__ => {
                        *self.state = X509DataTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    X509DataTypeSerializerState::Done__ => return Ok(None),
                    X509DataTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for X509DataTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = X509DataTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct X509DataTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::X509DataTypeContent,
        pub(super) state: Box<X509DataTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum X509DataTypeContentSerializerState<'ser> {
        Init__,
        Content43(<super::X509DataContent43Type as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> X509DataTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    X509DataTypeContentSerializerState::Init__ => {
                        *self.state = X509DataTypeContentSerializerState::Content43(
                            WithSerializer::serializer(
                                &self.value.content_43,
                                Some("Content43"),
                                false,
                            )?,
                        );
                    }
                    X509DataTypeContentSerializerState::Content43(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = X509DataTypeContentSerializerState::Done__,
                        }
                    }
                    X509DataTypeContentSerializerState::Done__ => return Ok(None),
                    X509DataTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for X509DataTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = X509DataTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PgpDataTypeSerializer<'ser> {
        pub(super) value: &'ser super::PgpDataType,
        pub(super) state: Box<PgpDataTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum PgpDataTypeSerializerState<'ser> {
        Init__,
        Content__(<super::PgpDataTypeContent as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PgpDataTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PgpDataTypeSerializerState::Init__ => {
                        *self.state = PgpDataTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(&mut bytes, Some(&super::PREFIX_DS), &super::NS_DS);
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    PgpDataTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PgpDataTypeSerializerState::End__,
                    },
                    PgpDataTypeSerializerState::End__ => {
                        *self.state = PgpDataTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    PgpDataTypeSerializerState::Done__ => return Ok(None),
                    PgpDataTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for PgpDataTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PgpDataTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PgpDataTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::PgpDataTypeContent,
        pub(super) state: Box<PgpDataTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum PgpDataTypeContentSerializerState<'ser> {
        Init__,
        Content47(<super::PgpDataContent47Type as WithSerializer>::Serializer<'ser>),
        Content49(<super::PgpDataContent49Type as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PgpDataTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PgpDataTypeContentSerializerState::Init__ => match self.value {
                        super::PgpDataTypeContent::Content47(x) => {
                            *self.state = PgpDataTypeContentSerializerState::Content47(
                                WithSerializer::serializer(x, Some("Content47"), false)?,
                            )
                        }
                        super::PgpDataTypeContent::Content49(x) => {
                            *self.state = PgpDataTypeContentSerializerState::Content49(
                                WithSerializer::serializer(x, Some("Content49"), false)?,
                            )
                        }
                    },
                    PgpDataTypeContentSerializerState::Content47(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PgpDataTypeContentSerializerState::Done__,
                        }
                    }
                    PgpDataTypeContentSerializerState::Content49(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PgpDataTypeContentSerializerState::Done__,
                        }
                    }
                    PgpDataTypeContentSerializerState::Done__ => return Ok(None),
                    PgpDataTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for PgpDataTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PgpDataTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SpkiDataTypeSerializer<'ser> {
        pub(super) value: &'ser super::SpkiDataType,
        pub(super) state: Box<SpkiDataTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SpkiDataTypeSerializerState<'ser> {
        Init__,
        Content__(
            IterSerializer<'ser, &'ser [super::SpkiDataTypeContent], super::SpkiDataTypeContent>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SpkiDataTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SpkiDataTypeSerializerState::Init__ => {
                        *self.state = SpkiDataTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(&mut bytes, Some(&super::PREFIX_DS), &super::NS_DS);
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SpkiDataTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SpkiDataTypeSerializerState::End__,
                        }
                    }
                    SpkiDataTypeSerializerState::End__ => {
                        *self.state = SpkiDataTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SpkiDataTypeSerializerState::Done__ => return Ok(None),
                    SpkiDataTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for SpkiDataTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SpkiDataTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SpkiDataTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::SpkiDataTypeContent,
        pub(super) state: Box<SpkiDataTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum SpkiDataTypeContentSerializerState<'ser> {
        Init__,
        SpkiSexp(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SpkiDataTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SpkiDataTypeContentSerializerState::Init__ => {
                        *self.state = SpkiDataTypeContentSerializerState::SpkiSexp(
                            WithSerializer::serializer(
                                &self.value.spki_sexp,
                                Some("ds:SPKISexp"),
                                false,
                            )?,
                        );
                    }
                    SpkiDataTypeContentSerializerState::SpkiSexp(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SpkiDataTypeContentSerializerState::Done__,
                        }
                    }
                    SpkiDataTypeContentSerializerState::Done__ => return Ok(None),
                    SpkiDataTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for SpkiDataTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SpkiDataTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TransformsTypeSerializer<'ser> {
        pub(super) value: &'ser super::TransformsType,
        pub(super) state: Box<TransformsTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TransformsTypeSerializerState<'ser> {
        Init__,
        Transform(IterSerializer<'ser, &'ser [super::TransformType], super::TransformType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TransformsTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TransformsTypeSerializerState::Init__ => {
                        *self.state =
                            TransformsTypeSerializerState::Transform(IterSerializer::new(
                                &self.value.transform[..],
                                Some("ds:Transform"),
                                false,
                            ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(&mut bytes, Some(&super::PREFIX_DS), &super::NS_DS);
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TransformsTypeSerializerState::Transform(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TransformsTypeSerializerState::End__,
                        }
                    }
                    TransformsTypeSerializerState::End__ => {
                        *self.state = TransformsTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TransformsTypeSerializerState::Done__ => return Ok(None),
                    TransformsTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TransformsTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TransformsTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DigestMethodTypeSerializer<'ser> {
        pub(super) value: &'ser super::DigestMethodType,
        pub(super) state: Box<DigestMethodTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DigestMethodTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DigestMethodTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DigestMethodTypeSerializerState::Init__ => {
                        *self.state = DigestMethodTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(&mut bytes, Some(&super::PREFIX_DS), &super::NS_DS);
                        }
                        helper.write_attrib(&mut bytes, "Algorithm", &self.value.algorithm)?;
                        helper.end_ns_scope();
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    DigestMethodTypeSerializerState::Done__ => return Ok(None),
                    DigestMethodTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for DigestMethodTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DigestMethodTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DsaKeyValueTypeSerializer<'ser> {
        pub(super) value: &'ser super::DsaKeyValueType,
        pub(super) state: Box<DsaKeyValueTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DsaKeyValueTypeSerializerState<'ser> {
        Init__,
        Content60(
            IterSerializer<
                'ser,
                Option<&'ser super::DsaKeyValueContent60Type>,
                super::DsaKeyValueContent60Type,
            >,
        ),
        G(IterSerializer<'ser, Option<&'ser String>, String>),
        Y(<String as WithSerializer>::Serializer<'ser>),
        J(IterSerializer<'ser, Option<&'ser String>, String>),
        Content61(
            IterSerializer<
                'ser,
                Option<&'ser super::DsaKeyValueContent61Type>,
                super::DsaKeyValueContent61Type,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DsaKeyValueTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DsaKeyValueTypeSerializerState::Init__ => {
                        *self.state =
                            DsaKeyValueTypeSerializerState::Content60(IterSerializer::new(
                                self.value.content_60.as_ref(),
                                Some("Content60"),
                                false,
                            ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(&mut bytes, Some(&super::PREFIX_DS), &super::NS_DS);
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DsaKeyValueTypeSerializerState::Content60(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = DsaKeyValueTypeSerializerState::G(
                                    IterSerializer::new(self.value.g.as_ref(), Some("ds:G"), false),
                                )
                            }
                        }
                    }
                    DsaKeyValueTypeSerializerState::G(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = DsaKeyValueTypeSerializerState::Y(
                                WithSerializer::serializer(&self.value.y, Some("ds:Y"), false)?,
                            )
                        }
                    },
                    DsaKeyValueTypeSerializerState::Y(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = DsaKeyValueTypeSerializerState::J(
                                    IterSerializer::new(self.value.j.as_ref(), Some("ds:J"), false),
                                )
                            }
                        }
                    }
                    DsaKeyValueTypeSerializerState::J(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                DsaKeyValueTypeSerializerState::Content61(IterSerializer::new(
                                    self.value.content_61.as_ref(),
                                    Some("Content61"),
                                    false,
                                ))
                        }
                    },
                    DsaKeyValueTypeSerializerState::Content61(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = DsaKeyValueTypeSerializerState::End__,
                        }
                    }
                    DsaKeyValueTypeSerializerState::End__ => {
                        *self.state = DsaKeyValueTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DsaKeyValueTypeSerializerState::Done__ => return Ok(None),
                    DsaKeyValueTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for DsaKeyValueTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DsaKeyValueTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct RsaKeyValueTypeSerializer<'ser> {
        pub(super) value: &'ser super::RsaKeyValueType,
        pub(super) state: Box<RsaKeyValueTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum RsaKeyValueTypeSerializerState<'ser> {
        Init__,
        Modulus(<String as WithSerializer>::Serializer<'ser>),
        Exponent(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RsaKeyValueTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    RsaKeyValueTypeSerializerState::Init__ => {
                        *self.state =
                            RsaKeyValueTypeSerializerState::Modulus(WithSerializer::serializer(
                                &self.value.modulus,
                                Some("ds:Modulus"),
                                false,
                            )?);
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(&mut bytes, Some(&super::PREFIX_DS), &super::NS_DS);
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    RsaKeyValueTypeSerializerState::Modulus(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = RsaKeyValueTypeSerializerState::Exponent(
                                    WithSerializer::serializer(
                                        &self.value.exponent,
                                        Some("ds:Exponent"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    RsaKeyValueTypeSerializerState::Exponent(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RsaKeyValueTypeSerializerState::End__,
                        }
                    }
                    RsaKeyValueTypeSerializerState::End__ => {
                        *self.state = RsaKeyValueTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    RsaKeyValueTypeSerializerState::Done__ => return Ok(None),
                    RsaKeyValueTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for RsaKeyValueTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = RsaKeyValueTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct X509DataContent43TypeSerializer<'ser> {
        pub(super) value: &'ser super::X509DataContent43Type,
        pub(super) state: Box<X509DataContent43TypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum X509DataContent43TypeSerializerState<'ser> {
        Init__,
        Content__(<super::X509DataContent43TypeContent as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> X509DataContent43TypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    X509DataContent43TypeSerializerState::Init__ => {
                        *self.state = X509DataContent43TypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                    }
                    X509DataContent43TypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = X509DataContent43TypeSerializerState::Done__,
                        }
                    }
                    X509DataContent43TypeSerializerState::Done__ => return Ok(None),
                    X509DataContent43TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for X509DataContent43TypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = X509DataContent43TypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct X509DataContent43TypeContentSerializer<'ser> {
        pub(super) value: &'ser super::X509DataContent43TypeContent,
        pub(super) state: Box<X509DataContent43TypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum X509DataContent43TypeContentSerializerState<'ser> {
        Init__,
        X509IssuerSerial(<super::X509IssuerSerialType as WithSerializer>::Serializer<'ser>),
        X509Ski(<String as WithSerializer>::Serializer<'ser>),
        X509SubjectName(<String as WithSerializer>::Serializer<'ser>),
        X509Certificate(<String as WithSerializer>::Serializer<'ser>),
        X509Crl(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> X509DataContent43TypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    X509DataContent43TypeContentSerializerState::Init__ => match self.value {
                        super::X509DataContent43TypeContent::X509IssuerSerial(x) => {
                            *self.state =
                                X509DataContent43TypeContentSerializerState::X509IssuerSerial(
                                    WithSerializer::serializer(
                                        x,
                                        Some("ds:X509IssuerSerial"),
                                        false,
                                    )?,
                                )
                        }
                        super::X509DataContent43TypeContent::X509Ski(x) => {
                            *self.state = X509DataContent43TypeContentSerializerState::X509Ski(
                                WithSerializer::serializer(x, Some("ds:X509SKI"), false)?,
                            )
                        }
                        super::X509DataContent43TypeContent::X509SubjectName(x) => {
                            *self.state =
                                X509DataContent43TypeContentSerializerState::X509SubjectName(
                                    WithSerializer::serializer(
                                        x,
                                        Some("ds:X509SubjectName"),
                                        false,
                                    )?,
                                )
                        }
                        super::X509DataContent43TypeContent::X509Certificate(x) => {
                            *self.state =
                                X509DataContent43TypeContentSerializerState::X509Certificate(
                                    WithSerializer::serializer(
                                        x,
                                        Some("ds:X509Certificate"),
                                        false,
                                    )?,
                                )
                        }
                        super::X509DataContent43TypeContent::X509Crl(x) => {
                            *self.state = X509DataContent43TypeContentSerializerState::X509Crl(
                                WithSerializer::serializer(x, Some("ds:X509CRL"), false)?,
                            )
                        }
                    },
                    X509DataContent43TypeContentSerializerState::X509IssuerSerial(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = X509DataContent43TypeContentSerializerState::Done__
                            }
                        }
                    }
                    X509DataContent43TypeContentSerializerState::X509Ski(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = X509DataContent43TypeContentSerializerState::Done__
                            }
                        }
                    }
                    X509DataContent43TypeContentSerializerState::X509SubjectName(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = X509DataContent43TypeContentSerializerState::Done__
                            }
                        }
                    }
                    X509DataContent43TypeContentSerializerState::X509Certificate(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = X509DataContent43TypeContentSerializerState::Done__
                            }
                        }
                    }
                    X509DataContent43TypeContentSerializerState::X509Crl(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = X509DataContent43TypeContentSerializerState::Done__
                            }
                        }
                    }
                    X509DataContent43TypeContentSerializerState::Done__ => return Ok(None),
                    X509DataContent43TypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for X509DataContent43TypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = X509DataContent43TypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PgpDataContent47TypeSerializer<'ser> {
        pub(super) value: &'ser super::PgpDataContent47Type,
        pub(super) state: Box<PgpDataContent47TypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum PgpDataContent47TypeSerializerState<'ser> {
        Init__,
        PgpKeyId(<String as WithSerializer>::Serializer<'ser>),
        PgpKeyPacket(IterSerializer<'ser, Option<&'ser String>, String>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PgpDataContent47TypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PgpDataContent47TypeSerializerState::Init__ => {
                        *self.state = PgpDataContent47TypeSerializerState::PgpKeyId(
                            WithSerializer::serializer(
                                &self.value.pgp_key_id,
                                Some("ds:PGPKeyID"),
                                false,
                            )?,
                        );
                    }
                    PgpDataContent47TypeSerializerState::PgpKeyId(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = PgpDataContent47TypeSerializerState::PgpKeyPacket(
                                    IterSerializer::new(
                                        self.value.pgp_key_packet.as_ref(),
                                        Some("ds:PGPKeyPacket"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    PgpDataContent47TypeSerializerState::PgpKeyPacket(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PgpDataContent47TypeSerializerState::Done__,
                        }
                    }
                    PgpDataContent47TypeSerializerState::Done__ => return Ok(None),
                    PgpDataContent47TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for PgpDataContent47TypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PgpDataContent47TypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PgpDataContent49TypeSerializer<'ser> {
        pub(super) value: &'ser super::PgpDataContent49Type,
        pub(super) state: Box<PgpDataContent49TypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum PgpDataContent49TypeSerializerState<'ser> {
        Init__,
        PgpKeyPacket(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PgpDataContent49TypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PgpDataContent49TypeSerializerState::Init__ => {
                        *self.state = PgpDataContent49TypeSerializerState::PgpKeyPacket(
                            WithSerializer::serializer(
                                &self.value.pgp_key_packet,
                                Some("ds:PGPKeyPacket"),
                                false,
                            )?,
                        );
                    }
                    PgpDataContent49TypeSerializerState::PgpKeyPacket(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PgpDataContent49TypeSerializerState::Done__,
                        }
                    }
                    PgpDataContent49TypeSerializerState::Done__ => return Ok(None),
                    PgpDataContent49TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for PgpDataContent49TypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PgpDataContent49TypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TransformTypeSerializer<'ser> {
        pub(super) value: &'ser super::TransformType,
        pub(super) state: Box<TransformTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TransformTypeSerializerState<'ser> {
        Init__,
        Content__(
            IterSerializer<'ser, &'ser [super::TransformTypeContent], super::TransformTypeContent>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TransformTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TransformTypeSerializerState::Init__ => {
                        *self.state = TransformTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(&mut bytes, Some(&super::PREFIX_DS), &super::NS_DS);
                        }
                        helper.write_attrib(&mut bytes, "Algorithm", &self.value.algorithm)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TransformTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TransformTypeSerializerState::End__,
                        }
                    }
                    TransformTypeSerializerState::End__ => {
                        *self.state = TransformTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TransformTypeSerializerState::Done__ => return Ok(None),
                    TransformTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TransformTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TransformTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TransformTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::TransformTypeContent,
        pub(super) state: Box<TransformTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum TransformTypeContentSerializerState<'ser> {
        Init__,
        XPath(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TransformTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TransformTypeContentSerializerState::Init__ => match self.value {
                        super::TransformTypeContent::XPath(x) => {
                            *self.state = TransformTypeContentSerializerState::XPath(
                                WithSerializer::serializer(x, Some("ds:XPath"), false)?,
                            )
                        }
                    },
                    TransformTypeContentSerializerState::XPath(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TransformTypeContentSerializerState::Done__,
                        }
                    }
                    TransformTypeContentSerializerState::Done__ => return Ok(None),
                    TransformTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TransformTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TransformTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DsaKeyValueContent60TypeSerializer<'ser> {
        pub(super) value: &'ser super::DsaKeyValueContent60Type,
        pub(super) state: Box<DsaKeyValueContent60TypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum DsaKeyValueContent60TypeSerializerState<'ser> {
        Init__,
        P(<String as WithSerializer>::Serializer<'ser>),
        Q(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DsaKeyValueContent60TypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DsaKeyValueContent60TypeSerializerState::Init__ => {
                        *self.state = DsaKeyValueContent60TypeSerializerState::P(
                            WithSerializer::serializer(&self.value.p, Some("ds:P"), false)?,
                        );
                    }
                    DsaKeyValueContent60TypeSerializerState::P(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = DsaKeyValueContent60TypeSerializerState::Q(
                                    WithSerializer::serializer(&self.value.q, Some("ds:Q"), false)?,
                                )
                            }
                        }
                    }
                    DsaKeyValueContent60TypeSerializerState::Q(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = DsaKeyValueContent60TypeSerializerState::Done__,
                        }
                    }
                    DsaKeyValueContent60TypeSerializerState::Done__ => return Ok(None),
                    DsaKeyValueContent60TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for DsaKeyValueContent60TypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DsaKeyValueContent60TypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DsaKeyValueContent61TypeSerializer<'ser> {
        pub(super) value: &'ser super::DsaKeyValueContent61Type,
        pub(super) state: Box<DsaKeyValueContent61TypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum DsaKeyValueContent61TypeSerializerState<'ser> {
        Init__,
        Seed(<String as WithSerializer>::Serializer<'ser>),
        PgenCounter(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DsaKeyValueContent61TypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DsaKeyValueContent61TypeSerializerState::Init__ => {
                        *self.state = DsaKeyValueContent61TypeSerializerState::Seed(
                            WithSerializer::serializer(&self.value.seed, Some("ds:Seed"), false)?,
                        );
                    }
                    DsaKeyValueContent61TypeSerializerState::Seed(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = DsaKeyValueContent61TypeSerializerState::PgenCounter(
                                    WithSerializer::serializer(
                                        &self.value.pgen_counter,
                                        Some("ds:PgenCounter"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    DsaKeyValueContent61TypeSerializerState::PgenCounter(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = DsaKeyValueContent61TypeSerializerState::Done__,
                        }
                    }
                    DsaKeyValueContent61TypeSerializerState::Done__ => return Ok(None),
                    DsaKeyValueContent61TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for DsaKeyValueContent61TypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DsaKeyValueContent61TypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct X509IssuerSerialTypeSerializer<'ser> {
        pub(super) value: &'ser super::X509IssuerSerialType,
        pub(super) state: Box<X509IssuerSerialTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum X509IssuerSerialTypeSerializerState<'ser> {
        Init__,
        X509IssuerName(<String as WithSerializer>::Serializer<'ser>),
        X509SerialNumber(<i32 as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> X509IssuerSerialTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    X509IssuerSerialTypeSerializerState::Init__ => {
                        *self.state = X509IssuerSerialTypeSerializerState::X509IssuerName(
                            WithSerializer::serializer(
                                &self.value.x509_issuer_name,
                                Some("ds:X509IssuerName"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(&mut bytes, Some(&super::PREFIX_DS), &super::NS_DS);
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    X509IssuerSerialTypeSerializerState::X509IssuerName(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = X509IssuerSerialTypeSerializerState::X509SerialNumber(
                                    WithSerializer::serializer(
                                        &self.value.x509_serial_number,
                                        Some("ds:X509SerialNumber"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    X509IssuerSerialTypeSerializerState::X509SerialNumber(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = X509IssuerSerialTypeSerializerState::End__,
                        }
                    }
                    X509IssuerSerialTypeSerializerState::End__ => {
                        *self.state = X509IssuerSerialTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    X509IssuerSerialTypeSerializerState::Done__ => return Ok(None),
                    X509IssuerSerialTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for X509IssuerSerialTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = X509IssuerSerialTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
