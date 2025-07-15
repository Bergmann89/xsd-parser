use xsd_parser::{
    models::schema::Namespace,
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_DEFAULT: Namespace =
    Namespace::new_const(b"http://www.idealdesk.com/ideal/messages/mer-acq/3.3.1");
pub const NS_DS: Namespace = Namespace::new_const(b"http://www.w3.org/2000/09/xmldsig#");
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
    use xsd_parser::quick_xml::{
        filter_xmlns_attributes, BytesStart, ContentDeserializer, DeserializeReader, Deserializer,
        DeserializerArtifact, DeserializerEvent, DeserializerOutput, DeserializerResult,
        ElementHandlerOutput, Error, ErrorKind, Event, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct DirectoryReqTypeDeserializer {
        version: String,
        create_date_timestamp: Option<String>,
        merchant: Option<super::DirectoryReqMerchantType>,
        signature: Option<super::SignatureType>,
        state: Box<DirectoryReqTypeDeserializerState>,
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
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut version: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DEFAULT),
                    Some(b"version")
                ) {
                    reader.read_attrib(&mut version, b"version", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                version: version.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("version".into()))
                })?,
                create_date_timestamp: None,
                merchant: None,
                signature: None,
                state: Box::new(DirectoryReqTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DirectoryReqTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use DirectoryReqTypeDeserializerState as S;
            match state {
                S::CreateDateTimestamp(Some(deserializer)) => {
                    self.store_create_date_timestamp(deserializer.finish(reader)?)?
                }
                S::Merchant(Some(deserializer)) => {
                    self.store_merchant(deserializer.finish(reader)?)?
                }
                S::Signature(Some(deserializer)) => {
                    self.store_signature(deserializer.finish(reader)?)?
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
        fn handle_create_date_timestamp<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DirectoryReqTypeDeserializerState>,
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
                if self.create_date_timestamp.is_some() {
                    fallback.get_or_insert(DirectoryReqTypeDeserializerState::CreateDateTimestamp(
                        None,
                    ));
                    *self.state = DirectoryReqTypeDeserializerState::Merchant(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DirectoryReqTypeDeserializerState::CreateDateTimestamp(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_create_date_timestamp(data)?;
                    *self.state = DirectoryReqTypeDeserializerState::Merchant(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                DirectoryReqTypeDeserializerState::CreateDateTimestamp(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = DirectoryReqTypeDeserializerState::Merchant(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DirectoryReqTypeDeserializerState::CreateDateTimestamp(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_merchant<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DirectoryReqMerchantType>,
            fallback: &mut Option<DirectoryReqTypeDeserializerState>,
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
                if self.merchant.is_some() {
                    fallback.get_or_insert(DirectoryReqTypeDeserializerState::Merchant(None));
                    *self.state = DirectoryReqTypeDeserializerState::Signature(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DirectoryReqTypeDeserializerState::Merchant(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_merchant(data)?;
                    *self.state = DirectoryReqTypeDeserializerState::Signature(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DirectoryReqTypeDeserializerState::Merchant(
                                Some(deserializer),
                            ));
                            *self.state = DirectoryReqTypeDeserializerState::Signature(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                DirectoryReqTypeDeserializerState::Merchant(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_signature<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SignatureType>,
            fallback: &mut Option<DirectoryReqTypeDeserializerState>,
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
                if self.signature.is_some() {
                    fallback.get_or_insert(DirectoryReqTypeDeserializerState::Signature(None));
                    *self.state = DirectoryReqTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DirectoryReqTypeDeserializerState::Signature(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_signature(data)?;
                    *self.state = DirectoryReqTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DirectoryReqTypeDeserializerState::Signature(
                                Some(deserializer),
                            ));
                            *self.state = DirectoryReqTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                DirectoryReqTypeDeserializerState::Signature(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DirectoryReqType> for DirectoryReqTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DirectoryReqType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DirectoryReqType>
        where
            R: DeserializeReader,
        {
            use DirectoryReqTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::CreateDateTimestamp(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_create_date_timestamp(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_merchant(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_signature(reader, output, &mut fallback)? {
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
                        *self.state = DirectoryReqTypeDeserializerState::CreateDateTimestamp(None);
                        event
                    }
                    (S::CreateDateTimestamp(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_DEFAULT),
                            b"createDateTimestamp",
                        ) {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_create_date_timestamp(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Merchant(None);
                            event
                        }
                    }
                    (S::Merchant(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_DEFAULT),
                            b"Merchant",
                        ) {
                            let output = < super :: DirectoryReqMerchantType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_merchant(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Signature(None);
                            event
                        }
                    }
                    (S::Signature(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"Signature") {
                            let output =
                                <super::SignatureType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_signature(reader, output, &mut fallback)? {
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
                            allow_any_element = true;
                            fallback.get_or_insert(S::Signature(None));
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
        fn finish<R>(mut self, reader: &R) -> Result<super::DirectoryReqType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                DirectoryReqTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::DirectoryReqType {
                version: self.version,
                create_date_timestamp: self
                    .create_date_timestamp
                    .ok_or_else(|| ErrorKind::MissingElement("createDateTimestamp".into()))?,
                merchant: self
                    .merchant
                    .ok_or_else(|| ErrorKind::MissingElement("Merchant".into()))?,
                signature: self
                    .signature
                    .ok_or_else(|| ErrorKind::MissingElement("Signature".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DirectoryReqMerchantTypeDeserializer {
        merchant_id: Option<String>,
        sub_id: Option<usize>,
        state: Box<DirectoryReqMerchantTypeDeserializerState>,
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
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                merchant_id: None,
                sub_id: None,
                state: Box::new(DirectoryReqMerchantTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DirectoryReqMerchantTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use DirectoryReqMerchantTypeDeserializerState as S;
            match state {
                S::MerchantId(Some(deserializer)) => {
                    self.store_merchant_id(deserializer.finish(reader)?)?
                }
                S::SubId(Some(deserializer)) => self.store_sub_id(deserializer.finish(reader)?)?,
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
        fn handle_merchant_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DirectoryReqMerchantTypeDeserializerState>,
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
                if self.merchant_id.is_some() {
                    fallback
                        .get_or_insert(DirectoryReqMerchantTypeDeserializerState::MerchantId(None));
                    *self.state = DirectoryReqMerchantTypeDeserializerState::SubId(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DirectoryReqMerchantTypeDeserializerState::MerchantId(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_merchant_id(data)?;
                    *self.state = DirectoryReqMerchantTypeDeserializerState::SubId(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                DirectoryReqMerchantTypeDeserializerState::MerchantId(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = DirectoryReqMerchantTypeDeserializerState::SubId(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DirectoryReqMerchantTypeDeserializerState::MerchantId(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_sub_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, usize>,
            fallback: &mut Option<DirectoryReqMerchantTypeDeserializerState>,
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
                if self.sub_id.is_some() {
                    fallback.get_or_insert(DirectoryReqMerchantTypeDeserializerState::SubId(None));
                    *self.state = DirectoryReqMerchantTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DirectoryReqMerchantTypeDeserializerState::SubId(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_sub_id(data)?;
                    *self.state = DirectoryReqMerchantTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                DirectoryReqMerchantTypeDeserializerState::SubId(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = DirectoryReqMerchantTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DirectoryReqMerchantTypeDeserializerState::SubId(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DirectoryReqMerchantType>
        for DirectoryReqMerchantTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DirectoryReqMerchantType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DirectoryReqMerchantType>
        where
            R: DeserializeReader,
        {
            use DirectoryReqMerchantTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::MerchantId(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_merchant_id(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_sub_id(reader, output, &mut fallback)? {
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
                        *self.state = DirectoryReqMerchantTypeDeserializerState::MerchantId(None);
                        event
                    }
                    (S::MerchantId(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_DEFAULT),
                            b"merchantID",
                        ) {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_merchant_id(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SubId(None);
                            event
                        }
                    }
                    (S::SubId(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DEFAULT), b"subID") {
                            let output =
                                <usize as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_sub_id(reader, output, &mut fallback)? {
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
        fn finish<R>(mut self, reader: &R) -> Result<super::DirectoryReqMerchantType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                DirectoryReqMerchantTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::DirectoryReqMerchantType {
                merchant_id: self
                    .merchant_id
                    .ok_or_else(|| ErrorKind::MissingElement("merchantID".into()))?,
                sub_id: self
                    .sub_id
                    .ok_or_else(|| ErrorKind::MissingElement("subID".into()))?,
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
        state: Box<SignatureTypeDeserializerState>,
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
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Id")
                ) {
                    reader.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                id: id,
                signed_info: None,
                signature_value: None,
                key_info: None,
                object: Vec::new(),
                state: Box::new(SignatureTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SignatureTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use SignatureTypeDeserializerState as S;
            match state {
                S::SignedInfo(Some(deserializer)) => {
                    self.store_signed_info(deserializer.finish(reader)?)?
                }
                S::SignatureValue(Some(deserializer)) => {
                    self.store_signature_value(deserializer.finish(reader)?)?
                }
                S::KeyInfo(Some(deserializer)) => {
                    self.store_key_info(deserializer.finish(reader)?)?
                }
                S::Object(Some(deserializer)) => self.store_object(deserializer.finish(reader)?)?,
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
        fn handle_signed_info<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SignedInfoType>,
            fallback: &mut Option<SignatureTypeDeserializerState>,
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
                if self.signed_info.is_some() {
                    fallback.get_or_insert(SignatureTypeDeserializerState::SignedInfo(None));
                    *self.state = SignatureTypeDeserializerState::SignatureValue(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = SignatureTypeDeserializerState::SignedInfo(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_signed_info(data)?;
                    *self.state = SignatureTypeDeserializerState::SignatureValue(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SignatureTypeDeserializerState::SignedInfo(
                                Some(deserializer),
                            ));
                            *self.state = SignatureTypeDeserializerState::SignatureValue(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                SignatureTypeDeserializerState::SignedInfo(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_signature_value<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SignatureValueType>,
            fallback: &mut Option<SignatureTypeDeserializerState>,
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
                if self.signature_value.is_some() {
                    fallback.get_or_insert(SignatureTypeDeserializerState::SignatureValue(None));
                    *self.state = SignatureTypeDeserializerState::KeyInfo(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = SignatureTypeDeserializerState::SignatureValue(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_signature_value(data)?;
                    *self.state = SignatureTypeDeserializerState::KeyInfo(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SignatureTypeDeserializerState::SignatureValue(
                                Some(deserializer),
                            ));
                            *self.state = SignatureTypeDeserializerState::KeyInfo(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                SignatureTypeDeserializerState::SignatureValue(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_key_info<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::KeyInfoType>,
            fallback: &mut Option<SignatureTypeDeserializerState>,
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
                fallback.get_or_insert(SignatureTypeDeserializerState::KeyInfo(None));
                *self.state = SignatureTypeDeserializerState::Object(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_key_info(data)?;
                    *self.state = SignatureTypeDeserializerState::Object(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SignatureTypeDeserializerState::KeyInfo(Some(
                                deserializer,
                            )));
                            *self.state = SignatureTypeDeserializerState::Object(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                SignatureTypeDeserializerState::KeyInfo(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_object<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ObjectType>,
            fallback: &mut Option<SignatureTypeDeserializerState>,
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
                fallback.get_or_insert(SignatureTypeDeserializerState::Object(None));
                *self.state = SignatureTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_object(data)?;
                    *self.state = SignatureTypeDeserializerState::Object(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SignatureTypeDeserializerState::Object(Some(
                                deserializer,
                            )));
                            *self.state = SignatureTypeDeserializerState::Object(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                SignatureTypeDeserializerState::Object(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SignatureType> for SignatureTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::SignatureType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SignatureType>
        where
            R: DeserializeReader,
        {
            use SignatureTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::SignedInfo(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_signed_info(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_signature_value(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_key_info(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_object(reader, output, &mut fallback)? {
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
                        *self.state = SignatureTypeDeserializerState::SignedInfo(None);
                        event
                    }
                    (S::SignedInfo(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"SignedInfo") {
                            let output =
                                <super::SignedInfoType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_signed_info(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SignatureValue(None);
                            allow_any_element = true;
                            fallback.get_or_insert(S::SignedInfo(None));
                            event
                        }
                    }
                    (S::SignatureValue(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_DS),
                            b"SignatureValue",
                        ) {
                            let output = < super :: SignatureValueType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_signature_value(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::KeyInfo(None);
                            event
                        }
                    }
                    (S::KeyInfo(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"KeyInfo") {
                            let output =
                                <super::KeyInfoType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_key_info(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Object(None);
                            allow_any_element = true;
                            fallback.get_or_insert(S::KeyInfo(None));
                            event
                        }
                    }
                    (S::Object(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"Object") {
                            let output =
                                <super::ObjectType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_object(reader, output, &mut fallback)? {
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
                            allow_any_element = true;
                            fallback.get_or_insert(S::Object(None));
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
        fn finish<R>(mut self, reader: &R) -> Result<super::SignatureType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, SignatureTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::SignatureType {
                id: self.id,
                signed_info: self
                    .signed_info
                    .ok_or_else(|| ErrorKind::MissingElement("SignedInfo".into()))?,
                signature_value: self
                    .signature_value
                    .ok_or_else(|| ErrorKind::MissingElement("SignatureValue".into()))?,
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
        state: Box<SignedInfoTypeDeserializerState>,
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
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Id")
                ) {
                    reader.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                id: id,
                canonicalization_method: None,
                signature_method: None,
                reference: Vec::new(),
                state: Box::new(SignedInfoTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SignedInfoTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use SignedInfoTypeDeserializerState as S;
            match state {
                S::CanonicalizationMethod(Some(deserializer)) => {
                    self.store_canonicalization_method(deserializer.finish(reader)?)?
                }
                S::SignatureMethod(Some(deserializer)) => {
                    self.store_signature_method(deserializer.finish(reader)?)?
                }
                S::Reference(Some(deserializer)) => {
                    self.store_reference(deserializer.finish(reader)?)?
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
        fn handle_canonicalization_method<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::CanonicalizationMethodType>,
            fallback: &mut Option<SignedInfoTypeDeserializerState>,
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
                if self.canonicalization_method.is_some() {
                    fallback.get_or_insert(
                        SignedInfoTypeDeserializerState::CanonicalizationMethod(None),
                    );
                    *self.state = SignedInfoTypeDeserializerState::SignatureMethod(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = SignedInfoTypeDeserializerState::CanonicalizationMethod(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_canonicalization_method(data)?;
                    *self.state = SignedInfoTypeDeserializerState::SignatureMethod(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                SignedInfoTypeDeserializerState::CanonicalizationMethod(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = SignedInfoTypeDeserializerState::SignatureMethod(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = SignedInfoTypeDeserializerState::CanonicalizationMethod(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_signature_method<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SignatureMethodType>,
            fallback: &mut Option<SignedInfoTypeDeserializerState>,
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
                if self.signature_method.is_some() {
                    fallback.get_or_insert(SignedInfoTypeDeserializerState::SignatureMethod(None));
                    *self.state = SignedInfoTypeDeserializerState::Reference(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = SignedInfoTypeDeserializerState::SignatureMethod(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_signature_method(data)?;
                    *self.state = SignedInfoTypeDeserializerState::Reference(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                SignedInfoTypeDeserializerState::SignatureMethod(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = SignedInfoTypeDeserializerState::Reference(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = SignedInfoTypeDeserializerState::SignatureMethod(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_reference<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ReferenceType>,
            fallback: &mut Option<SignedInfoTypeDeserializerState>,
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
                if self.reference.len() < 1usize {
                    *self.state = SignedInfoTypeDeserializerState::Reference(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(SignedInfoTypeDeserializerState::Reference(None));
                    *self.state = SignedInfoTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_reference(data)?;
                    *self.state = SignedInfoTypeDeserializerState::Reference(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SignedInfoTypeDeserializerState::Reference(
                                Some(deserializer),
                            ));
                            if self.reference.len().saturating_add(1) < 1usize {
                                *self.state = SignedInfoTypeDeserializerState::Reference(None);
                            } else {
                                *self.state = SignedInfoTypeDeserializerState::Done__;
                            }
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                SignedInfoTypeDeserializerState::Reference(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SignedInfoType> for SignedInfoTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::SignedInfoType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SignedInfoType>
        where
            R: DeserializeReader,
        {
            use SignedInfoTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::CanonicalizationMethod(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_canonicalization_method(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_signature_method(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_reference(reader, output, &mut fallback)? {
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
                        *self.state = SignedInfoTypeDeserializerState::CanonicalizationMethod(None);
                        event
                    }
                    (
                        S::CanonicalizationMethod(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_DS),
                            b"CanonicalizationMethod",
                        ) {
                            let output = < super :: CanonicalizationMethodType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_canonicalization_method(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SignatureMethod(None);
                            allow_any_element = true;
                            fallback.get_or_insert(S::CanonicalizationMethod(None));
                            event
                        }
                    }
                    (S::SignatureMethod(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_DS),
                            b"SignatureMethod",
                        ) {
                            let output = < super :: SignatureMethodType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_signature_method(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Reference(None);
                            event
                        }
                    }
                    (S::Reference(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"Reference") {
                            let output =
                                <super::ReferenceType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_reference(reader, output, &mut fallback)? {
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
                            allow_any_element = true;
                            fallback.get_or_insert(S::Reference(None));
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
        fn finish<R>(mut self, reader: &R) -> Result<super::SignedInfoType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, SignedInfoTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::SignedInfoType {
                id: self.id,
                canonicalization_method: self
                    .canonicalization_method
                    .ok_or_else(|| ErrorKind::MissingElement("CanonicalizationMethod".into()))?,
                signature_method: self
                    .signature_method
                    .ok_or_else(|| ErrorKind::MissingElement("SignatureMethod".into()))?,
                reference: self.reference,
            })
        }
    }
    #[derive(Debug)]
    pub struct SignatureValueTypeDeserializer {
        id: Option<String>,
        content: Option<String>,
        state: Box<SignatureValueTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SignatureValueTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl SignatureValueTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Id")
                ) {
                    reader.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                id: id,
                content: None,
                state: Box::new(SignatureValueTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SignatureValueTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let SignatureValueTypeDeserializerState::Content__(deserializer) = state {
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
        ) -> DeserializerResult<'de, super::SignatureValueType>
        where
            R: DeserializeReader,
        {
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
    impl<'de> Deserializer<'de, super::SignatureValueType> for SignatureValueTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SignatureValueType>
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
        ) -> DeserializerResult<'de, super::SignatureValueType>
        where
            R: DeserializeReader,
        {
            use SignatureValueTypeDeserializerState as S;
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
        fn finish<R>(mut self, reader: &R) -> Result<super::SignatureValueType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                SignatureValueTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::SignatureValueType {
                id: self.id,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct KeyInfoTypeDeserializer {
        id: Option<String>,
        content: Vec<super::KeyInfoTypeContent>,
        state: Box<KeyInfoTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum KeyInfoTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::KeyInfoTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl KeyInfoTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Id")
                ) {
                    reader.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                id: id,
                content: Vec::new(),
                state: Box::new(KeyInfoTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: KeyInfoTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let KeyInfoTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::KeyInfoTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::KeyInfoTypeContent>,
            fallback: &mut Option<KeyInfoTypeDeserializerState>,
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
                    .unwrap_or(KeyInfoTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = KeyInfoTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = KeyInfoTypeDeserializerState::Content__(deserializer);
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(KeyInfoTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = KeyInfoTypeDeserializerState::Next__;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::KeyInfoType> for KeyInfoTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::KeyInfoType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeyInfoType>
        where
            R: DeserializeReader,
        {
            use KeyInfoTypeDeserializerState as S;
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
                            <super::KeyInfoTypeContent as WithDeserializer>::Deserializer::init(
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
        fn finish<R>(mut self, reader: &R) -> Result<super::KeyInfoType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, KeyInfoTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::KeyInfoType {
                id: self.id,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct KeyInfoTypeContentDeserializer {
        state: Box<KeyInfoTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum KeyInfoTypeContentDeserializerState {
        Init__,
        KeyName(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        KeyValue(
            Option<super::KeyValueType>,
            Option<<super::KeyValueType as WithDeserializer>::Deserializer>,
        ),
        RetrievalMethod(
            Option<super::RetrievalMethodType>,
            Option<<super::RetrievalMethodType as WithDeserializer>::Deserializer>,
        ),
        X509Data(
            Option<super::X509DataType>,
            Option<<super::X509DataType as WithDeserializer>::Deserializer>,
        ),
        PgpData(
            Option<super::PgpDataType>,
            Option<<super::PgpDataType as WithDeserializer>::Deserializer>,
        ),
        SpkiData(
            Option<super::SpkiDataType>,
            Option<<super::SpkiDataType as WithDeserializer>::Deserializer>,
        ),
        MgmtData(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::KeyInfoTypeContent),
        Unknown__,
    }
    impl KeyInfoTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<KeyInfoTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"KeyName")
                ) {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_key_name(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"KeyValue")
                ) {
                    let output = <super::KeyValueType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_key_value(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"RetrievalMethod")
                ) {
                    let output =
                        <super::RetrievalMethodType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    return self.handle_retrieval_method(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"X509Data")
                ) {
                    let output = <super::X509DataType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_x509_data(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"PGPData")
                ) {
                    let output = <super::PgpDataType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_pgp_data(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"SPKIData")
                ) {
                    let output = <super::SpkiDataType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_spki_data(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"MgmtData")
                ) {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_mgmt_data(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(KeyInfoTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, true))
        }
        fn finish_state<R>(
            reader: &R,
            state: KeyInfoTypeContentDeserializerState,
        ) -> Result<super::KeyInfoTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use KeyInfoTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::KeyName(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_key_name(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::KeyName(values.ok_or_else(
                        || ErrorKind::MissingElement("KeyName".into()),
                    )?))
                }
                S::KeyValue(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_key_value(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::KeyValue(values.ok_or_else(
                        || ErrorKind::MissingElement("KeyValue".into()),
                    )?))
                }
                S::RetrievalMethod(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_retrieval_method(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::RetrievalMethod(
                        values
                            .ok_or_else(|| ErrorKind::MissingElement("RetrievalMethod".into()))?,
                    ))
                }
                S::X509Data(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_x509_data(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::X509Data(values.ok_or_else(
                        || ErrorKind::MissingElement("X509Data".into()),
                    )?))
                }
                S::PgpData(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_pgp_data(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::PgpData(values.ok_or_else(
                        || ErrorKind::MissingElement("PGPData".into()),
                    )?))
                }
                S::SpkiData(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_spki_data(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::SpkiData(values.ok_or_else(
                        || ErrorKind::MissingElement("SPKIData".into()),
                    )?))
                }
                S::MgmtData(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_mgmt_data(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::MgmtData(values.ok_or_else(
                        || ErrorKind::MissingElement("MgmtData".into()),
                    )?))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_key_name<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<KeyInfoTypeContentDeserializerState>,
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
                    None => KeyInfoTypeContentDeserializerState::KeyName(values, None),
                    Some(KeyInfoTypeContentDeserializerState::KeyName(_, Some(deserializer))) => {
                        KeyInfoTypeContentDeserializerState::KeyName(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(KeyInfoTypeContentDeserializerState::KeyName(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_key_name(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_key_name(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        KeyInfoTypeContentDeserializerState::KeyName(values, None),
                    )?;
                    *self.state = KeyInfoTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        KeyInfoTypeContentDeserializerState::KeyName(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_key_value<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::KeyValueType>,
            output: DeserializerOutput<'de, super::KeyValueType>,
            fallback: &mut Option<KeyInfoTypeContentDeserializerState>,
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
                    None => KeyInfoTypeContentDeserializerState::KeyValue(values, None),
                    Some(KeyInfoTypeContentDeserializerState::KeyValue(_, Some(deserializer))) => {
                        KeyInfoTypeContentDeserializerState::KeyValue(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(KeyInfoTypeContentDeserializerState::KeyValue(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_key_value(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_key_value(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        KeyInfoTypeContentDeserializerState::KeyValue(values, None),
                    )?;
                    *self.state = KeyInfoTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        KeyInfoTypeContentDeserializerState::KeyValue(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_retrieval_method<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::RetrievalMethodType>,
            output: DeserializerOutput<'de, super::RetrievalMethodType>,
            fallback: &mut Option<KeyInfoTypeContentDeserializerState>,
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
                    None => KeyInfoTypeContentDeserializerState::RetrievalMethod(values, None),
                    Some(KeyInfoTypeContentDeserializerState::RetrievalMethod(
                        _,
                        Some(deserializer),
                    )) => KeyInfoTypeContentDeserializerState::RetrievalMethod(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(KeyInfoTypeContentDeserializerState::RetrievalMethod(
                    _,
                    Some(deserializer),
                )) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_retrieval_method(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_retrieval_method(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        KeyInfoTypeContentDeserializerState::RetrievalMethod(values, None),
                    )?;
                    *self.state = KeyInfoTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = KeyInfoTypeContentDeserializerState::RetrievalMethod(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_x509_data<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::X509DataType>,
            output: DeserializerOutput<'de, super::X509DataType>,
            fallback: &mut Option<KeyInfoTypeContentDeserializerState>,
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
                    None => KeyInfoTypeContentDeserializerState::X509Data(values, None),
                    Some(KeyInfoTypeContentDeserializerState::X509Data(_, Some(deserializer))) => {
                        KeyInfoTypeContentDeserializerState::X509Data(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(KeyInfoTypeContentDeserializerState::X509Data(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_x509_data(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x509_data(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        KeyInfoTypeContentDeserializerState::X509Data(values, None),
                    )?;
                    *self.state = KeyInfoTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        KeyInfoTypeContentDeserializerState::X509Data(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_pgp_data<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PgpDataType>,
            output: DeserializerOutput<'de, super::PgpDataType>,
            fallback: &mut Option<KeyInfoTypeContentDeserializerState>,
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
                    None => KeyInfoTypeContentDeserializerState::PgpData(values, None),
                    Some(KeyInfoTypeContentDeserializerState::PgpData(_, Some(deserializer))) => {
                        KeyInfoTypeContentDeserializerState::PgpData(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(KeyInfoTypeContentDeserializerState::PgpData(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_pgp_data(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pgp_data(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        KeyInfoTypeContentDeserializerState::PgpData(values, None),
                    )?;
                    *self.state = KeyInfoTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        KeyInfoTypeContentDeserializerState::PgpData(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_spki_data<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SpkiDataType>,
            output: DeserializerOutput<'de, super::SpkiDataType>,
            fallback: &mut Option<KeyInfoTypeContentDeserializerState>,
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
                    None => KeyInfoTypeContentDeserializerState::SpkiData(values, None),
                    Some(KeyInfoTypeContentDeserializerState::SpkiData(_, Some(deserializer))) => {
                        KeyInfoTypeContentDeserializerState::SpkiData(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(KeyInfoTypeContentDeserializerState::SpkiData(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_spki_data(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_spki_data(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        KeyInfoTypeContentDeserializerState::SpkiData(values, None),
                    )?;
                    *self.state = KeyInfoTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        KeyInfoTypeContentDeserializerState::SpkiData(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_mgmt_data<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<KeyInfoTypeContentDeserializerState>,
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
                    None => KeyInfoTypeContentDeserializerState::MgmtData(values, None),
                    Some(KeyInfoTypeContentDeserializerState::MgmtData(_, Some(deserializer))) => {
                        KeyInfoTypeContentDeserializerState::MgmtData(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(KeyInfoTypeContentDeserializerState::MgmtData(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_mgmt_data(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_mgmt_data(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        KeyInfoTypeContentDeserializerState::MgmtData(values, None),
                    )?;
                    *self.state = KeyInfoTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        KeyInfoTypeContentDeserializerState::MgmtData(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::KeyInfoTypeContent> for KeyInfoTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeyInfoTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(KeyInfoTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, KeyInfoTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::KeyInfoTypeContent>
        where
            R: DeserializeReader,
        {
            use KeyInfoTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::KeyName(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_key_name(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::KeyValue(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_key_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::RetrievalMethod(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_retrieval_method(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::X509Data(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_x509_data(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::PgpData(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pgp_data(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::SpkiData(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_spki_data(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::MgmtData(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_mgmt_data(reader, values, output, &mut fallback)? {
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
                    (S::KeyName(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_key_name(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::KeyValue(values, None), event) => {
                        let output = <super::KeyValueType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_key_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::RetrievalMethod(values, None), event) => {
                        let output =
                            <super::RetrievalMethodType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_retrieval_method(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::X509Data(values, None), event) => {
                        let output = <super::X509DataType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_x509_data(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::PgpData(values, None), event) => {
                        let output = <super::PgpDataType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_pgp_data(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::SpkiData(values, None), event) => {
                        let output = <super::SpkiDataType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_spki_data(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::MgmtData(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_mgmt_data(reader, values, output, &mut fallback)? {
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
        fn finish<R>(self, reader: &R) -> Result<super::KeyInfoTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct ObjectTypeDeserializer {
        id: Option<String>,
        mime_type: Option<String>,
        encoding: Option<String>,
        state: Box<ObjectTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ObjectTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl ObjectTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut mime_type: Option<String> = None;
            let mut encoding: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Id")
                ) {
                    reader.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"MimeType")
                ) {
                    reader.read_attrib(&mut mime_type, b"MimeType", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Encoding")
                ) {
                    reader.read_attrib(&mut encoding, b"Encoding", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                id: id,
                mime_type: mime_type,
                encoding: encoding,
                state: Box::new(ObjectTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ObjectTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::ObjectType> for ObjectTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ObjectType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ObjectType>
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
        fn finish<R>(mut self, reader: &R) -> Result<super::ObjectType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, ObjectTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
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
        state: Box<CanonicalizationMethodTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CanonicalizationMethodTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl CanonicalizationMethodTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut algorithm: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Algorithm")
                ) {
                    reader.read_attrib(&mut algorithm, b"Algorithm", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                algorithm: algorithm.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("Algorithm".into()))
                })?,
                state: Box::new(CanonicalizationMethodTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: CanonicalizationMethodTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::CanonicalizationMethodType>
        for CanonicalizationMethodTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CanonicalizationMethodType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CanonicalizationMethodType>
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
        fn finish<R>(mut self, reader: &R) -> Result<super::CanonicalizationMethodType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                CanonicalizationMethodTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::CanonicalizationMethodType {
                algorithm: self.algorithm,
            })
        }
    }
    #[derive(Debug)]
    pub struct SignatureMethodTypeDeserializer {
        algorithm: String,
        hmac_output_length: Option<i32>,
        state: Box<SignatureMethodTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SignatureMethodTypeDeserializerState {
        Init__,
        HmacOutputLength(Option<<i32 as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SignatureMethodTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut algorithm: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Algorithm")
                ) {
                    reader.read_attrib(&mut algorithm, b"Algorithm", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                algorithm: algorithm.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("Algorithm".into()))
                })?,
                hmac_output_length: None,
                state: Box::new(SignatureMethodTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SignatureMethodTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use SignatureMethodTypeDeserializerState as S;
            match state {
                S::HmacOutputLength(Some(deserializer)) => {
                    self.store_hmac_output_length(deserializer.finish(reader)?)?
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
        fn handle_hmac_output_length<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<SignatureMethodTypeDeserializerState>,
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
                fallback
                    .get_or_insert(SignatureMethodTypeDeserializerState::HmacOutputLength(None));
                *self.state = SignatureMethodTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_hmac_output_length(data)?;
                    *self.state = SignatureMethodTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                SignatureMethodTypeDeserializerState::HmacOutputLength(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = SignatureMethodTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = SignatureMethodTypeDeserializerState::HmacOutputLength(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SignatureMethodType> for SignatureMethodTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SignatureMethodType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SignatureMethodType>
        where
            R: DeserializeReader,
        {
            use SignatureMethodTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::HmacOutputLength(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_hmac_output_length(reader, output, &mut fallback)? {
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
                        allow_any_element = true;
                        fallback.get_or_insert(S::Init__);
                        *self.state = SignatureMethodTypeDeserializerState::HmacOutputLength(None);
                        event
                    }
                    (S::HmacOutputLength(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_DS),
                            b"HMACOutputLength",
                        ) {
                            let output =
                                <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_hmac_output_length(reader, output, &mut fallback)? {
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
                            allow_any_element = true;
                            fallback.get_or_insert(S::HmacOutputLength(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), true);
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
        fn finish<R>(mut self, reader: &R) -> Result<super::SignatureMethodType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                SignatureMethodTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
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
        state: Box<ReferenceTypeDeserializerState>,
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
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            let mut uri: Option<String> = None;
            let mut type_: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Id")
                ) {
                    reader.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"URI")
                ) {
                    reader.read_attrib(&mut uri, b"URI", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Type")
                ) {
                    reader.read_attrib(&mut type_, b"Type", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                id: id,
                uri: uri,
                type_: type_,
                transforms: None,
                digest_method: None,
                digest_value: None,
                state: Box::new(ReferenceTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ReferenceTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use ReferenceTypeDeserializerState as S;
            match state {
                S::Transforms(Some(deserializer)) => {
                    self.store_transforms(deserializer.finish(reader)?)?
                }
                S::DigestMethod(Some(deserializer)) => {
                    self.store_digest_method(deserializer.finish(reader)?)?
                }
                S::DigestValue(Some(deserializer)) => {
                    self.store_digest_value(deserializer.finish(reader)?)?
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
        fn handle_transforms<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TransformsType>,
            fallback: &mut Option<ReferenceTypeDeserializerState>,
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
                fallback.get_or_insert(ReferenceTypeDeserializerState::Transforms(None));
                *self.state = ReferenceTypeDeserializerState::DigestMethod(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_transforms(data)?;
                    *self.state = ReferenceTypeDeserializerState::DigestMethod(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ReferenceTypeDeserializerState::Transforms(
                                Some(deserializer),
                            ));
                            *self.state = ReferenceTypeDeserializerState::DigestMethod(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                ReferenceTypeDeserializerState::Transforms(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_digest_method<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DigestMethodType>,
            fallback: &mut Option<ReferenceTypeDeserializerState>,
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
                if self.digest_method.is_some() {
                    fallback.get_or_insert(ReferenceTypeDeserializerState::DigestMethod(None));
                    *self.state = ReferenceTypeDeserializerState::DigestValue(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = ReferenceTypeDeserializerState::DigestMethod(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_digest_method(data)?;
                    *self.state = ReferenceTypeDeserializerState::DigestValue(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ReferenceTypeDeserializerState::DigestMethod(
                                Some(deserializer),
                            ));
                            *self.state = ReferenceTypeDeserializerState::DigestValue(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                ReferenceTypeDeserializerState::DigestMethod(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_digest_value<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ReferenceTypeDeserializerState>,
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
                if self.digest_value.is_some() {
                    fallback.get_or_insert(ReferenceTypeDeserializerState::DigestValue(None));
                    *self.state = ReferenceTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = ReferenceTypeDeserializerState::DigestValue(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_digest_value(data)?;
                    *self.state = ReferenceTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ReferenceTypeDeserializerState::DigestValue(
                                Some(deserializer),
                            ));
                            *self.state = ReferenceTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                ReferenceTypeDeserializerState::DigestValue(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ReferenceType> for ReferenceTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ReferenceType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ReferenceType>
        where
            R: DeserializeReader,
        {
            use ReferenceTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Transforms(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_transforms(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_digest_method(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_digest_value(reader, output, &mut fallback)? {
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
                        *self.state = ReferenceTypeDeserializerState::Transforms(None);
                        event
                    }
                    (S::Transforms(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"Transforms") {
                            let output =
                                <super::TransformsType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_transforms(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::DigestMethod(None);
                            allow_any_element = true;
                            fallback.get_or_insert(S::Transforms(None));
                            event
                        }
                    }
                    (S::DigestMethod(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"DigestMethod")
                        {
                            let output =
                                <super::DigestMethodType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_digest_method(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::DigestValue(None);
                            allow_any_element = true;
                            fallback.get_or_insert(S::DigestMethod(None));
                            event
                        }
                    }
                    (S::DigestValue(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"DigestValue")
                        {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_digest_value(reader, output, &mut fallback)? {
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
        fn finish<R>(mut self, reader: &R) -> Result<super::ReferenceType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, ReferenceTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::ReferenceType {
                id: self.id,
                uri: self.uri,
                type_: self.type_,
                transforms: self.transforms,
                digest_method: self
                    .digest_method
                    .ok_or_else(|| ErrorKind::MissingElement("DigestMethod".into()))?,
                digest_value: self
                    .digest_value
                    .ok_or_else(|| ErrorKind::MissingElement("DigestValue".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct KeyValueTypeDeserializer {
        content: Option<super::KeyValueTypeContent>,
        state: Box<KeyValueTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum KeyValueTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::KeyValueTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl KeyValueTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(KeyValueTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: KeyValueTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let KeyValueTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
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
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::KeyValueTypeContent>,
            fallback: &mut Option<KeyValueTypeDeserializerState>,
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
                    .unwrap_or(KeyValueTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = KeyValueTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = KeyValueTypeDeserializerState::Content__(deserializer);
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::KeyValueType> for KeyValueTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::KeyValueType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeyValueType>
        where
            R: DeserializeReader,
        {
            use KeyValueTypeDeserializerState as S;
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
                            <super::KeyValueTypeContent as WithDeserializer>::Deserializer::init(
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
        fn finish<R>(mut self, reader: &R) -> Result<super::KeyValueType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, KeyValueTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::KeyValueType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct KeyValueTypeContentDeserializer {
        state: Box<KeyValueTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum KeyValueTypeContentDeserializerState {
        Init__,
        DsaKeyValue(
            Option<super::DsaKeyValueType>,
            Option<<super::DsaKeyValueType as WithDeserializer>::Deserializer>,
        ),
        RsaKeyValue(
            Option<super::RsaKeyValueType>,
            Option<<super::RsaKeyValueType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::KeyValueTypeContent),
        Unknown__,
    }
    impl KeyValueTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<KeyValueTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"DSAKeyValue")
                ) {
                    let output = <super::DsaKeyValueType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_dsa_key_value(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"RSAKeyValue")
                ) {
                    let output = <super::RsaKeyValueType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    return self.handle_rsa_key_value(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(KeyValueTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, true))
        }
        fn finish_state<R>(
            reader: &R,
            state: KeyValueTypeContentDeserializerState,
        ) -> Result<super::KeyValueTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use KeyValueTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::DsaKeyValue(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_dsa_key_value(&mut values, value)?;
                    }
                    Ok(super::KeyValueTypeContent::DsaKeyValue(values.ok_or_else(
                        || ErrorKind::MissingElement("DSAKeyValue".into()),
                    )?))
                }
                S::RsaKeyValue(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_rsa_key_value(&mut values, value)?;
                    }
                    Ok(super::KeyValueTypeContent::RsaKeyValue(values.ok_or_else(
                        || ErrorKind::MissingElement("RSAKeyValue".into()),
                    )?))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_dsa_key_value<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::DsaKeyValueType>,
            output: DeserializerOutput<'de, super::DsaKeyValueType>,
            fallback: &mut Option<KeyValueTypeContentDeserializerState>,
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
                    None => KeyValueTypeContentDeserializerState::DsaKeyValue(values, None),
                    Some(KeyValueTypeContentDeserializerState::DsaKeyValue(
                        _,
                        Some(deserializer),
                    )) => KeyValueTypeContentDeserializerState::DsaKeyValue(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(KeyValueTypeContentDeserializerState::DsaKeyValue(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_dsa_key_value(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_dsa_key_value(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        KeyValueTypeContentDeserializerState::DsaKeyValue(values, None),
                    )?;
                    *self.state = KeyValueTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = KeyValueTypeContentDeserializerState::DsaKeyValue(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_rsa_key_value<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::RsaKeyValueType>,
            output: DeserializerOutput<'de, super::RsaKeyValueType>,
            fallback: &mut Option<KeyValueTypeContentDeserializerState>,
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
                    None => KeyValueTypeContentDeserializerState::RsaKeyValue(values, None),
                    Some(KeyValueTypeContentDeserializerState::RsaKeyValue(
                        _,
                        Some(deserializer),
                    )) => KeyValueTypeContentDeserializerState::RsaKeyValue(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(KeyValueTypeContentDeserializerState::RsaKeyValue(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_rsa_key_value(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_rsa_key_value(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        KeyValueTypeContentDeserializerState::RsaKeyValue(values, None),
                    )?;
                    *self.state = KeyValueTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = KeyValueTypeContentDeserializerState::RsaKeyValue(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::KeyValueTypeContent> for KeyValueTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::KeyValueTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(KeyValueTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, KeyValueTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::KeyValueTypeContent>
        where
            R: DeserializeReader,
        {
            use KeyValueTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::DsaKeyValue(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_dsa_key_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::RsaKeyValue(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_rsa_key_value(reader, values, output, &mut fallback)? {
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
                    (S::DsaKeyValue(values, None), event) => {
                        let output =
                            <super::DsaKeyValueType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_dsa_key_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::RsaKeyValue(values, None), event) => {
                        let output =
                            <super::RsaKeyValueType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_rsa_key_value(reader, values, output, &mut fallback)? {
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
        fn finish<R>(self, reader: &R) -> Result<super::KeyValueTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct RetrievalMethodTypeDeserializer {
        uri: Option<String>,
        type_: Option<String>,
        transforms: Option<super::TransformsType>,
        state: Box<RetrievalMethodTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RetrievalMethodTypeDeserializerState {
        Init__,
        Transforms(Option<<super::TransformsType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl RetrievalMethodTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut uri: Option<String> = None;
            let mut type_: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"URI")
                ) {
                    reader.read_attrib(&mut uri, b"URI", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Type")
                ) {
                    reader.read_attrib(&mut type_, b"Type", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                uri: uri,
                type_: type_,
                transforms: None,
                state: Box::new(RetrievalMethodTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: RetrievalMethodTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use RetrievalMethodTypeDeserializerState as S;
            match state {
                S::Transforms(Some(deserializer)) => {
                    self.store_transforms(deserializer.finish(reader)?)?
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
        fn handle_transforms<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TransformsType>,
            fallback: &mut Option<RetrievalMethodTypeDeserializerState>,
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
                fallback.get_or_insert(RetrievalMethodTypeDeserializerState::Transforms(None));
                *self.state = RetrievalMethodTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_transforms(data)?;
                    *self.state = RetrievalMethodTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                RetrievalMethodTypeDeserializerState::Transforms(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = RetrievalMethodTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = RetrievalMethodTypeDeserializerState::Transforms(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RetrievalMethodType> for RetrievalMethodTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RetrievalMethodType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RetrievalMethodType>
        where
            R: DeserializeReader,
        {
            use RetrievalMethodTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Transforms(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_transforms(reader, output, &mut fallback)? {
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
                        *self.state = RetrievalMethodTypeDeserializerState::Transforms(None);
                        event
                    }
                    (S::Transforms(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"Transforms") {
                            let output =
                                <super::TransformsType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_transforms(reader, output, &mut fallback)? {
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
                            allow_any_element = true;
                            fallback.get_or_insert(S::Transforms(None));
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
        fn finish<R>(mut self, reader: &R) -> Result<super::RetrievalMethodType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                RetrievalMethodTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
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
        state: Box<X509DataTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum X509DataTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::X509DataTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl X509DataTypeDeserializer {
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
                state: Box::new(X509DataTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: X509DataTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let X509DataTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::X509DataTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::X509DataTypeContent>,
            fallback: &mut Option<X509DataTypeDeserializerState>,
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
                    .unwrap_or(X509DataTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = X509DataTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = X509DataTypeDeserializerState::Content__(deserializer);
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(X509DataTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = X509DataTypeDeserializerState::Next__;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::X509DataType> for X509DataTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::X509DataType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::X509DataType>
        where
            R: DeserializeReader,
        {
            use X509DataTypeDeserializerState as S;
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
                            <super::X509DataTypeContent as WithDeserializer>::Deserializer::init(
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
        fn finish<R>(mut self, reader: &R) -> Result<super::X509DataType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, X509DataTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::X509DataType {
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct X509DataTypeContentDeserializer {
        content_43: Option<super::X509DataContent43Type>,
        state: Box<X509DataTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    enum X509DataTypeContentDeserializerState {
        Init__,
        Content43(Option<<super::X509DataContent43Type as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl X509DataTypeContentDeserializer {
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: X509DataTypeContentDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use X509DataTypeContentDeserializerState as S;
            match state {
                S::Content43(Some(deserializer)) => {
                    self.store_content_43(deserializer.finish(reader)?)?
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
        fn handle_content_43<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::X509DataContent43Type>,
            fallback: &mut Option<X509DataTypeContentDeserializerState>,
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
                if self.content_43.is_some() {
                    fallback.get_or_insert(X509DataTypeContentDeserializerState::Content43(None));
                    *self.state = X509DataTypeContentDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = X509DataTypeContentDeserializerState::Content43(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content_43(data)?;
                    *self.state = X509DataTypeContentDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                X509DataTypeContentDeserializerState::Content43(Some(deserializer)),
                            );
                            *self.state = X509DataTypeContentDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                X509DataTypeContentDeserializerState::Content43(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::X509DataTypeContent> for X509DataTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::X509DataTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                content_43: None,
                state: Box::new(X509DataTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, X509DataTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::X509DataTypeContent>
        where
            R: DeserializeReader,
        {
            use X509DataTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content43(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content_43(reader, output, &mut fallback)? {
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
                        *self.state = X509DataTypeContentDeserializerState::Content43(None);
                        event
                    }
                    (S::Content43(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::X509DataContent43Type as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content_43(reader, output, &mut fallback)? {
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
        fn finish<R>(mut self, reader: &R) -> Result<super::X509DataTypeContent, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                X509DataTypeContentDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::X509DataTypeContent {
                content_43: self
                    .content_43
                    .ok_or_else(|| ErrorKind::MissingElement("Content43".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PgpDataTypeDeserializer {
        content: Option<super::PgpDataTypeContent>,
        state: Box<PgpDataTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PgpDataTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::PgpDataTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl PgpDataTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(PgpDataTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: PgpDataTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let PgpDataTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
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
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::PgpDataTypeContent>,
            fallback: &mut Option<PgpDataTypeDeserializerState>,
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
                    .unwrap_or(PgpDataTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = PgpDataTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = PgpDataTypeDeserializerState::Content__(deserializer);
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::PgpDataType> for PgpDataTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::PgpDataType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PgpDataType>
        where
            R: DeserializeReader,
        {
            use PgpDataTypeDeserializerState as S;
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
                            <super::PgpDataTypeContent as WithDeserializer>::Deserializer::init(
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
        fn finish<R>(mut self, reader: &R) -> Result<super::PgpDataType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, PgpDataTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::PgpDataType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PgpDataTypeContentDeserializer {
        state: Box<PgpDataTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum PgpDataTypeContentDeserializerState {
        Init__,
        Content47(
            Option<super::PgpDataContent47Type>,
            Option<<super::PgpDataContent47Type as WithDeserializer>::Deserializer>,
        ),
        Content49(
            Option<super::PgpDataContent49Type>,
            Option<<super::PgpDataContent49Type as WithDeserializer>::Deserializer>,
        ),
        Done__(super::PgpDataTypeContent),
        Unknown__,
    }
    impl PgpDataTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<PgpDataTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut allow_any_element = false;
            if let Event::Start(_) | Event::Empty(_) = &event {
                event = {
                    let output =
                        <super::PgpDataContent47Type as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_content_47(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    )? {
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
                        <super::PgpDataContent49Type as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_content_49(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    )? {
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
            *self.state = fallback
                .take()
                .unwrap_or(PgpDataTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
        }
        fn finish_state<R>(
            reader: &R,
            state: PgpDataTypeContentDeserializerState,
        ) -> Result<super::PgpDataTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use PgpDataTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Content47(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_content_47(&mut values, value)?;
                    }
                    Ok(super::PgpDataTypeContent::Content47(values.ok_or_else(
                        || ErrorKind::MissingElement("Content47".into()),
                    )?))
                }
                S::Content49(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_content_49(&mut values, value)?;
                    }
                    Ok(super::PgpDataTypeContent::Content49(values.ok_or_else(
                        || ErrorKind::MissingElement("Content49".into()),
                    )?))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_content_47<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PgpDataContent47Type>,
            output: DeserializerOutput<'de, super::PgpDataContent47Type>,
            fallback: &mut Option<PgpDataTypeContentDeserializerState>,
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
                    None => PgpDataTypeContentDeserializerState::Content47(values, None),
                    Some(PgpDataTypeContentDeserializerState::Content47(_, Some(deserializer))) => {
                        PgpDataTypeContentDeserializerState::Content47(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(PgpDataTypeContentDeserializerState::Content47(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_content_47(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_content_47(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        PgpDataTypeContentDeserializerState::Content47(values, None),
                    )?;
                    *self.state = PgpDataTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        PgpDataTypeContentDeserializerState::Content47(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_content_49<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PgpDataContent49Type>,
            output: DeserializerOutput<'de, super::PgpDataContent49Type>,
            fallback: &mut Option<PgpDataTypeContentDeserializerState>,
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
                    None => PgpDataTypeContentDeserializerState::Content49(values, None),
                    Some(PgpDataTypeContentDeserializerState::Content49(_, Some(deserializer))) => {
                        PgpDataTypeContentDeserializerState::Content49(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(PgpDataTypeContentDeserializerState::Content49(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_content_49(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_content_49(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        PgpDataTypeContentDeserializerState::Content49(values, None),
                    )?;
                    *self.state = PgpDataTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        PgpDataTypeContentDeserializerState::Content49(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::PgpDataTypeContent> for PgpDataTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PgpDataTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(PgpDataTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, PgpDataTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::PgpDataTypeContent>
        where
            R: DeserializeReader,
        {
            use PgpDataTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content47(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content_47(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Content49(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content_49(reader, values, output, &mut fallback)? {
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
                    (S::Content47(values, None), event) => {
                        let output =
                            <super::PgpDataContent47Type as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content_47(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Content49(values, None), event) => {
                        let output =
                            <super::PgpDataContent49Type as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content_49(reader, values, output, &mut fallback)? {
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
        fn finish<R>(self, reader: &R) -> Result<super::PgpDataTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct SpkiDataTypeDeserializer {
        content: Vec<super::SpkiDataTypeContent>,
        state: Box<SpkiDataTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SpkiDataTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::SpkiDataTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl SpkiDataTypeDeserializer {
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
                state: Box::new(SpkiDataTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SpkiDataTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let SpkiDataTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::SpkiDataTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SpkiDataTypeContent>,
            fallback: &mut Option<SpkiDataTypeDeserializerState>,
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
                    .unwrap_or(SpkiDataTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = SpkiDataTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = SpkiDataTypeDeserializerState::Content__(deserializer);
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SpkiDataTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = SpkiDataTypeDeserializerState::Next__;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SpkiDataType> for SpkiDataTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::SpkiDataType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SpkiDataType>
        where
            R: DeserializeReader,
        {
            use SpkiDataTypeDeserializerState as S;
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
                            <super::SpkiDataTypeContent as WithDeserializer>::Deserializer::init(
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
        fn finish<R>(mut self, reader: &R) -> Result<super::SpkiDataType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, SpkiDataTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::SpkiDataType {
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct SpkiDataTypeContentDeserializer {
        spki_sexp: Option<String>,
        state: Box<SpkiDataTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    enum SpkiDataTypeContentDeserializerState {
        Init__,
        SpkiSexp(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SpkiDataTypeContentDeserializer {
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SpkiDataTypeContentDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use SpkiDataTypeContentDeserializerState as S;
            match state {
                S::SpkiSexp(Some(deserializer)) => {
                    self.store_spki_sexp(deserializer.finish(reader)?)?
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
        fn handle_spki_sexp<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<SpkiDataTypeContentDeserializerState>,
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
                if self.spki_sexp.is_some() {
                    fallback.get_or_insert(SpkiDataTypeContentDeserializerState::SpkiSexp(None));
                    *self.state = SpkiDataTypeContentDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = SpkiDataTypeContentDeserializerState::SpkiSexp(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_spki_sexp(data)?;
                    *self.state = SpkiDataTypeContentDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SpkiDataTypeContentDeserializerState::SpkiSexp(
                                Some(deserializer),
                            ));
                            *self.state = SpkiDataTypeContentDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                SpkiDataTypeContentDeserializerState::SpkiSexp(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SpkiDataTypeContent> for SpkiDataTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SpkiDataTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                spki_sexp: None,
                state: Box::new(SpkiDataTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, SpkiDataTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::SpkiDataTypeContent>
        where
            R: DeserializeReader,
        {
            use SpkiDataTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::SpkiSexp(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_spki_sexp(reader, output, &mut fallback)? {
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
                        allow_any_element = true;
                        fallback.get_or_insert(S::Init__);
                        *self.state = SpkiDataTypeContentDeserializerState::SpkiSexp(None);
                        event
                    }
                    (S::SpkiSexp(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"SPKISexp") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_spki_sexp(reader, output, &mut fallback)? {
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
                            allow_any_element = true;
                            fallback.get_or_insert(S::SpkiSexp(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), true);
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
        fn finish<R>(mut self, reader: &R) -> Result<super::SpkiDataTypeContent, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                SpkiDataTypeContentDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::SpkiDataTypeContent {
                spki_sexp: self
                    .spki_sexp
                    .ok_or_else(|| ErrorKind::MissingElement("SPKISexp".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TransformsTypeDeserializer {
        transform: Vec<super::TransformType>,
        state: Box<TransformsTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TransformsTypeDeserializerState {
        Init__,
        Transform(Option<<super::TransformType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TransformsTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                transform: Vec::new(),
                state: Box::new(TransformsTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TransformsTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use TransformsTypeDeserializerState as S;
            match state {
                S::Transform(Some(deserializer)) => {
                    self.store_transform(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_transform(&mut self, value: super::TransformType) -> Result<(), Error> {
            self.transform.push(value);
            Ok(())
        }
        fn handle_transform<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TransformType>,
            fallback: &mut Option<TransformsTypeDeserializerState>,
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
                if self.transform.len() < 1usize {
                    *self.state = TransformsTypeDeserializerState::Transform(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(TransformsTypeDeserializerState::Transform(None));
                    *self.state = TransformsTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_transform(data)?;
                    *self.state = TransformsTypeDeserializerState::Transform(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TransformsTypeDeserializerState::Transform(
                                Some(deserializer),
                            ));
                            if self.transform.len().saturating_add(1) < 1usize {
                                *self.state = TransformsTypeDeserializerState::Transform(None);
                            } else {
                                *self.state = TransformsTypeDeserializerState::Done__;
                            }
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TransformsTypeDeserializerState::Transform(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::TransformsType> for TransformsTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::TransformsType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TransformsType>
        where
            R: DeserializeReader,
        {
            use TransformsTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Transform(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_transform(reader, output, &mut fallback)? {
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
                        *self.state = TransformsTypeDeserializerState::Transform(None);
                        event
                    }
                    (S::Transform(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"Transform") {
                            let output =
                                <super::TransformType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_transform(reader, output, &mut fallback)? {
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
                            allow_any_element = true;
                            fallback.get_or_insert(S::Transform(None));
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
        fn finish<R>(mut self, reader: &R) -> Result<super::TransformsType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, TransformsTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::TransformsType {
                transform: self.transform,
            })
        }
    }
    #[derive(Debug)]
    pub struct DigestMethodTypeDeserializer {
        algorithm: String,
        state: Box<DigestMethodTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DigestMethodTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl DigestMethodTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut algorithm: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Algorithm")
                ) {
                    reader.read_attrib(&mut algorithm, b"Algorithm", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                algorithm: algorithm.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("Algorithm".into()))
                })?,
                state: Box::new(DigestMethodTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DigestMethodTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::DigestMethodType> for DigestMethodTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DigestMethodType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DigestMethodType>
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
        fn finish<R>(mut self, reader: &R) -> Result<super::DigestMethodType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                DigestMethodTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
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
        state: Box<DsaKeyValueTypeDeserializerState>,
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
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                content_60: None,
                g: None,
                y: None,
                j: None,
                content_61: None,
                state: Box::new(DsaKeyValueTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DsaKeyValueTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use DsaKeyValueTypeDeserializerState as S;
            match state {
                S::Content60(Some(deserializer)) => {
                    self.store_content_60(deserializer.finish(reader)?)?
                }
                S::G(Some(deserializer)) => self.store_g(deserializer.finish(reader)?)?,
                S::Y(Some(deserializer)) => self.store_y(deserializer.finish(reader)?)?,
                S::J(Some(deserializer)) => self.store_j(deserializer.finish(reader)?)?,
                S::Content61(Some(deserializer)) => {
                    self.store_content_61(deserializer.finish(reader)?)?
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
        fn handle_content_60<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DsaKeyValueContent60Type>,
            fallback: &mut Option<DsaKeyValueTypeDeserializerState>,
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
                fallback.get_or_insert(DsaKeyValueTypeDeserializerState::Content60(None));
                *self.state = DsaKeyValueTypeDeserializerState::G(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content_60(data)?;
                    *self.state = DsaKeyValueTypeDeserializerState::G(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DsaKeyValueTypeDeserializerState::Content60(
                                Some(deserializer),
                            ));
                            *self.state = DsaKeyValueTypeDeserializerState::G(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                DsaKeyValueTypeDeserializerState::Content60(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_g<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DsaKeyValueTypeDeserializerState>,
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
                fallback.get_or_insert(DsaKeyValueTypeDeserializerState::G(None));
                *self.state = DsaKeyValueTypeDeserializerState::Y(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_g(data)?;
                    *self.state = DsaKeyValueTypeDeserializerState::Y(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DsaKeyValueTypeDeserializerState::G(Some(
                                deserializer,
                            )));
                            *self.state = DsaKeyValueTypeDeserializerState::Y(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DsaKeyValueTypeDeserializerState::G(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_y<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DsaKeyValueTypeDeserializerState>,
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
                if self.y.is_some() {
                    fallback.get_or_insert(DsaKeyValueTypeDeserializerState::Y(None));
                    *self.state = DsaKeyValueTypeDeserializerState::J(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DsaKeyValueTypeDeserializerState::Y(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_y(data)?;
                    *self.state = DsaKeyValueTypeDeserializerState::J(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DsaKeyValueTypeDeserializerState::Y(Some(
                                deserializer,
                            )));
                            *self.state = DsaKeyValueTypeDeserializerState::J(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DsaKeyValueTypeDeserializerState::Y(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_j<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DsaKeyValueTypeDeserializerState>,
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
                fallback.get_or_insert(DsaKeyValueTypeDeserializerState::J(None));
                *self.state = DsaKeyValueTypeDeserializerState::Content61(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_j(data)?;
                    *self.state = DsaKeyValueTypeDeserializerState::Content61(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DsaKeyValueTypeDeserializerState::J(Some(
                                deserializer,
                            )));
                            *self.state = DsaKeyValueTypeDeserializerState::Content61(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DsaKeyValueTypeDeserializerState::J(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_content_61<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DsaKeyValueContent61Type>,
            fallback: &mut Option<DsaKeyValueTypeDeserializerState>,
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
                fallback.get_or_insert(DsaKeyValueTypeDeserializerState::Content61(None));
                *self.state = DsaKeyValueTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content_61(data)?;
                    *self.state = DsaKeyValueTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DsaKeyValueTypeDeserializerState::Content61(
                                Some(deserializer),
                            ));
                            *self.state = DsaKeyValueTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                DsaKeyValueTypeDeserializerState::Content61(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DsaKeyValueType> for DsaKeyValueTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::DsaKeyValueType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DsaKeyValueType>
        where
            R: DeserializeReader,
        {
            use DsaKeyValueTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content60(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content_60(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_g(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_y(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_j(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content_61(reader, output, &mut fallback)? {
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
                        *self.state = DsaKeyValueTypeDeserializerState::Content60(None);
                        event
                    }
                    (S::Content60(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = < super :: DsaKeyValueContent60Type as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content_60(reader, output, &mut fallback)? {
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
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"G") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_g(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Y(None);
                            event
                        }
                    }
                    (S::Y(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"Y") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_y(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::J(None);
                            event
                        }
                    }
                    (S::J(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"J") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_j(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Content61(None);
                            event
                        }
                    }
                    (S::Content61(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = < super :: DsaKeyValueContent61Type as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content_61(reader, output, &mut fallback)? {
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
        fn finish<R>(mut self, reader: &R) -> Result<super::DsaKeyValueType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                DsaKeyValueTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::DsaKeyValueType {
                content_60: self.content_60,
                g: self.g,
                y: self
                    .y
                    .ok_or_else(|| ErrorKind::MissingElement("Y".into()))?,
                j: self.j,
                content_61: self.content_61,
            })
        }
    }
    #[derive(Debug)]
    pub struct RsaKeyValueTypeDeserializer {
        modulus: Option<String>,
        exponent: Option<String>,
        state: Box<RsaKeyValueTypeDeserializerState>,
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
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                modulus: None,
                exponent: None,
                state: Box::new(RsaKeyValueTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: RsaKeyValueTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use RsaKeyValueTypeDeserializerState as S;
            match state {
                S::Modulus(Some(deserializer)) => {
                    self.store_modulus(deserializer.finish(reader)?)?
                }
                S::Exponent(Some(deserializer)) => {
                    self.store_exponent(deserializer.finish(reader)?)?
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
        fn handle_modulus<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<RsaKeyValueTypeDeserializerState>,
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
                if self.modulus.is_some() {
                    fallback.get_or_insert(RsaKeyValueTypeDeserializerState::Modulus(None));
                    *self.state = RsaKeyValueTypeDeserializerState::Exponent(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = RsaKeyValueTypeDeserializerState::Modulus(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_modulus(data)?;
                    *self.state = RsaKeyValueTypeDeserializerState::Exponent(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(RsaKeyValueTypeDeserializerState::Modulus(
                                Some(deserializer),
                            ));
                            *self.state = RsaKeyValueTypeDeserializerState::Exponent(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                RsaKeyValueTypeDeserializerState::Modulus(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_exponent<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<RsaKeyValueTypeDeserializerState>,
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
                if self.exponent.is_some() {
                    fallback.get_or_insert(RsaKeyValueTypeDeserializerState::Exponent(None));
                    *self.state = RsaKeyValueTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = RsaKeyValueTypeDeserializerState::Exponent(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_exponent(data)?;
                    *self.state = RsaKeyValueTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(RsaKeyValueTypeDeserializerState::Exponent(
                                Some(deserializer),
                            ));
                            *self.state = RsaKeyValueTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                RsaKeyValueTypeDeserializerState::Exponent(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RsaKeyValueType> for RsaKeyValueTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::RsaKeyValueType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RsaKeyValueType>
        where
            R: DeserializeReader,
        {
            use RsaKeyValueTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Modulus(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_modulus(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_exponent(reader, output, &mut fallback)? {
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
                        *self.state = RsaKeyValueTypeDeserializerState::Modulus(None);
                        event
                    }
                    (S::Modulus(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"Modulus") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_modulus(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Exponent(None);
                            event
                        }
                    }
                    (S::Exponent(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"Exponent") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_exponent(reader, output, &mut fallback)? {
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
        fn finish<R>(mut self, reader: &R) -> Result<super::RsaKeyValueType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                RsaKeyValueTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::RsaKeyValueType {
                modulus: self
                    .modulus
                    .ok_or_else(|| ErrorKind::MissingElement("Modulus".into()))?,
                exponent: self
                    .exponent
                    .ok_or_else(|| ErrorKind::MissingElement("Exponent".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct X509DataContent43TypeDeserializer {
        content: Option<super::X509DataContent43TypeContent>,
        state: Box<X509DataContent43TypeDeserializerState>,
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
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: X509DataContent43TypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let X509DataContent43TypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
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
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::X509DataContent43TypeContent>,
            fallback: &mut Option<X509DataContent43TypeDeserializerState>,
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
                    .unwrap_or(X509DataContent43TypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = X509DataContent43TypeDeserializerState::Done__;
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = X509DataContent43TypeDeserializerState::Content__(deserializer);
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::X509DataContent43Type> for X509DataContent43TypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::X509DataContent43Type>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                content: None,
                state: Box::new(X509DataContent43TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, X509DataContent43TypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::X509DataContent43Type>
        where
            R: DeserializeReader,
        {
            use X509DataContent43TypeDeserializerState as S;
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
                    (_, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output = < super :: X509DataContent43TypeContent as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Done__, event) => {
                        *self.state = S::Done__;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match &*self.state {
                S::Done__ => DeserializerArtifact::Data(self.finish(reader)?),
                _ => DeserializerArtifact::Deserializer(self),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::X509DataContent43Type, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                X509DataContent43TypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::X509DataContent43Type {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct X509DataContent43TypeContentDeserializer {
        state: Box<X509DataContent43TypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum X509DataContent43TypeContentDeserializerState {
        Init__,
        X509IssuerSerial(
            Option<super::X509IssuerSerialType>,
            Option<<super::X509IssuerSerialType as WithDeserializer>::Deserializer>,
        ),
        X509Ski(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        X509SubjectName(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        X509Certificate(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        X509Crl(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::X509DataContent43TypeContent),
        Unknown__,
    }
    impl X509DataContent43TypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<X509DataContent43TypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"X509IssuerSerial")
                ) {
                    let output =
                        <super::X509IssuerSerialType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    return self.handle_x509_issuer_serial(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"X509SKI")
                ) {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_x509_ski(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"X509SubjectName")
                ) {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_x509_subject_name(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"X509Certificate")
                ) {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_x509_certificate(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"X509CRL")
                ) {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_x509_crl(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(X509DataContent43TypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, true))
        }
        fn finish_state<R>(
            reader: &R,
            state: X509DataContent43TypeContentDeserializerState,
        ) -> Result<super::X509DataContent43TypeContent, Error>
        where
            R: DeserializeReader,
        {
            use X509DataContent43TypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::X509IssuerSerial(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_x509_issuer_serial(&mut values, value)?;
                    }
                    Ok(super::X509DataContent43TypeContent::X509IssuerSerial(
                        values
                            .ok_or_else(|| ErrorKind::MissingElement("X509IssuerSerial".into()))?,
                    ))
                }
                S::X509Ski(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_x509_ski(&mut values, value)?;
                    }
                    Ok(super::X509DataContent43TypeContent::X509Ski(
                        values.ok_or_else(|| ErrorKind::MissingElement("X509SKI".into()))?,
                    ))
                }
                S::X509SubjectName(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_x509_subject_name(&mut values, value)?;
                    }
                    Ok(super::X509DataContent43TypeContent::X509SubjectName(
                        values
                            .ok_or_else(|| ErrorKind::MissingElement("X509SubjectName".into()))?,
                    ))
                }
                S::X509Certificate(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_x509_certificate(&mut values, value)?;
                    }
                    Ok(super::X509DataContent43TypeContent::X509Certificate(
                        values
                            .ok_or_else(|| ErrorKind::MissingElement("X509Certificate".into()))?,
                    ))
                }
                S::X509Crl(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_x509_crl(&mut values, value)?;
                    }
                    Ok(super::X509DataContent43TypeContent::X509Crl(
                        values.ok_or_else(|| ErrorKind::MissingElement("X509CRL".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_x509_issuer_serial<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::X509IssuerSerialType>,
            output: DeserializerOutput<'de, super::X509IssuerSerialType>,
            fallback: &mut Option<X509DataContent43TypeContentDeserializerState>,
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
                    None => X509DataContent43TypeContentDeserializerState::X509IssuerSerial(
                        values, None,
                    ),
                    Some(X509DataContent43TypeContentDeserializerState::X509IssuerSerial(
                        _,
                        Some(deserializer),
                    )) => X509DataContent43TypeContentDeserializerState::X509IssuerSerial(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(X509DataContent43TypeContentDeserializerState::X509IssuerSerial(
                    _,
                    Some(deserializer),
                )) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_x509_issuer_serial(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x509_issuer_serial(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        X509DataContent43TypeContentDeserializerState::X509IssuerSerial(
                            values, None,
                        ),
                    )?;
                    *self.state = X509DataContent43TypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = X509DataContent43TypeContentDeserializerState::X509IssuerSerial(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_x509_ski<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<X509DataContent43TypeContentDeserializerState>,
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
                    None => X509DataContent43TypeContentDeserializerState::X509Ski(values, None),
                    Some(X509DataContent43TypeContentDeserializerState::X509Ski(
                        _,
                        Some(deserializer),
                    )) => X509DataContent43TypeContentDeserializerState::X509Ski(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(X509DataContent43TypeContentDeserializerState::X509Ski(
                    _,
                    Some(deserializer),
                )) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_x509_ski(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x509_ski(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        X509DataContent43TypeContentDeserializerState::X509Ski(values, None),
                    )?;
                    *self.state = X509DataContent43TypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = X509DataContent43TypeContentDeserializerState::X509Ski(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_x509_subject_name<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<X509DataContent43TypeContentDeserializerState>,
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
                    None => {
                        X509DataContent43TypeContentDeserializerState::X509SubjectName(values, None)
                    }
                    Some(X509DataContent43TypeContentDeserializerState::X509SubjectName(
                        _,
                        Some(deserializer),
                    )) => X509DataContent43TypeContentDeserializerState::X509SubjectName(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(X509DataContent43TypeContentDeserializerState::X509SubjectName(
                    _,
                    Some(deserializer),
                )) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_x509_subject_name(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x509_subject_name(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        X509DataContent43TypeContentDeserializerState::X509SubjectName(
                            values, None,
                        ),
                    )?;
                    *self.state = X509DataContent43TypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = X509DataContent43TypeContentDeserializerState::X509SubjectName(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_x509_certificate<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<X509DataContent43TypeContentDeserializerState>,
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
                    None => {
                        X509DataContent43TypeContentDeserializerState::X509Certificate(values, None)
                    }
                    Some(X509DataContent43TypeContentDeserializerState::X509Certificate(
                        _,
                        Some(deserializer),
                    )) => X509DataContent43TypeContentDeserializerState::X509Certificate(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(X509DataContent43TypeContentDeserializerState::X509Certificate(
                    _,
                    Some(deserializer),
                )) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_x509_certificate(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x509_certificate(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        X509DataContent43TypeContentDeserializerState::X509Certificate(
                            values, None,
                        ),
                    )?;
                    *self.state = X509DataContent43TypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = X509DataContent43TypeContentDeserializerState::X509Certificate(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_x509_crl<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<X509DataContent43TypeContentDeserializerState>,
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
                    None => X509DataContent43TypeContentDeserializerState::X509Crl(values, None),
                    Some(X509DataContent43TypeContentDeserializerState::X509Crl(
                        _,
                        Some(deserializer),
                    )) => X509DataContent43TypeContentDeserializerState::X509Crl(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(X509DataContent43TypeContentDeserializerState::X509Crl(
                    _,
                    Some(deserializer),
                )) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_x509_crl(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x509_crl(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        X509DataContent43TypeContentDeserializerState::X509Crl(values, None),
                    )?;
                    *self.state = X509DataContent43TypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = X509DataContent43TypeContentDeserializerState::X509Crl(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::X509DataContent43TypeContent>
        for X509DataContent43TypeContentDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::X509DataContent43TypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(X509DataContent43TypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state,
                        X509DataContent43TypeContentDeserializerState::Init__
                    ) =>
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
        ) -> DeserializerResult<'de, super::X509DataContent43TypeContent>
        where
            R: DeserializeReader,
        {
            use X509DataContent43TypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::X509IssuerSerial(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_x509_issuer_serial(
                            reader,
                            values,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::X509Ski(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_x509_ski(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::X509SubjectName(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_x509_subject_name(
                            reader,
                            values,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::X509Certificate(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_x509_certificate(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::X509Crl(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_x509_crl(reader, values, output, &mut fallback)? {
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
                    (S::X509IssuerSerial(values, None), event) => {
                        let output =
                            <super::X509IssuerSerialType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_x509_issuer_serial(
                            reader,
                            values,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::X509Ski(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_x509_ski(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::X509SubjectName(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_x509_subject_name(
                            reader,
                            values,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::X509Certificate(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_x509_certificate(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::X509Crl(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_x509_crl(reader, values, output, &mut fallback)? {
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
        fn finish<R>(self, reader: &R) -> Result<super::X509DataContent43TypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct PgpDataContent47TypeDeserializer {
        pgp_key_id: Option<String>,
        pgp_key_packet: Option<String>,
        state: Box<PgpDataContent47TypeDeserializerState>,
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
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: PgpDataContent47TypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use PgpDataContent47TypeDeserializerState as S;
            match state {
                S::PgpKeyId(Some(deserializer)) => {
                    self.store_pgp_key_id(deserializer.finish(reader)?)?
                }
                S::PgpKeyPacket(Some(deserializer)) => {
                    self.store_pgp_key_packet(deserializer.finish(reader)?)?
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
        fn handle_pgp_key_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<PgpDataContent47TypeDeserializerState>,
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
                if self.pgp_key_id.is_some() {
                    fallback.get_or_insert(PgpDataContent47TypeDeserializerState::PgpKeyId(None));
                    *self.state = PgpDataContent47TypeDeserializerState::PgpKeyPacket(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = PgpDataContent47TypeDeserializerState::PgpKeyId(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_pgp_key_id(data)?;
                    *self.state = PgpDataContent47TypeDeserializerState::PgpKeyPacket(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                PgpDataContent47TypeDeserializerState::PgpKeyId(Some(deserializer)),
                            );
                            *self.state = PgpDataContent47TypeDeserializerState::PgpKeyPacket(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                PgpDataContent47TypeDeserializerState::PgpKeyId(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_pgp_key_packet<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<PgpDataContent47TypeDeserializerState>,
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
                fallback.get_or_insert(PgpDataContent47TypeDeserializerState::PgpKeyPacket(None));
                *self.state = PgpDataContent47TypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_pgp_key_packet(data)?;
                    *self.state = PgpDataContent47TypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                PgpDataContent47TypeDeserializerState::PgpKeyPacket(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = PgpDataContent47TypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = PgpDataContent47TypeDeserializerState::PgpKeyPacket(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::PgpDataContent47Type> for PgpDataContent47TypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PgpDataContent47Type>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                pgp_key_id: None,
                pgp_key_packet: None,
                state: Box::new(PgpDataContent47TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, PgpDataContent47TypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::PgpDataContent47Type>
        where
            R: DeserializeReader,
        {
            use PgpDataContent47TypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::PgpKeyId(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pgp_key_id(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pgp_key_packet(reader, output, &mut fallback)? {
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
                        allow_any_element = true;
                        fallback.get_or_insert(S::Init__);
                        *self.state = PgpDataContent47TypeDeserializerState::PgpKeyId(None);
                        event
                    }
                    (S::PgpKeyId(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"PGPKeyID") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_pgp_key_id(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::PgpKeyPacket(None);
                            allow_any_element = true;
                            fallback.get_or_insert(S::PgpKeyId(None));
                            event
                        }
                    }
                    (S::PgpKeyPacket(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"PGPKeyPacket")
                        {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_pgp_key_packet(reader, output, &mut fallback)? {
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
                            allow_any_element = true;
                            fallback.get_or_insert(S::PgpKeyPacket(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), true);
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
        fn finish<R>(mut self, reader: &R) -> Result<super::PgpDataContent47Type, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                PgpDataContent47TypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::PgpDataContent47Type {
                pgp_key_id: self
                    .pgp_key_id
                    .ok_or_else(|| ErrorKind::MissingElement("PGPKeyID".into()))?,
                pgp_key_packet: self.pgp_key_packet,
            })
        }
    }
    #[derive(Debug)]
    pub struct PgpDataContent49TypeDeserializer {
        pgp_key_packet: Option<String>,
        state: Box<PgpDataContent49TypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PgpDataContent49TypeDeserializerState {
        Init__,
        PgpKeyPacket(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl PgpDataContent49TypeDeserializer {
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: PgpDataContent49TypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use PgpDataContent49TypeDeserializerState as S;
            match state {
                S::PgpKeyPacket(Some(deserializer)) => {
                    self.store_pgp_key_packet(deserializer.finish(reader)?)?
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
        fn handle_pgp_key_packet<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<PgpDataContent49TypeDeserializerState>,
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
                if self.pgp_key_packet.is_some() {
                    fallback
                        .get_or_insert(PgpDataContent49TypeDeserializerState::PgpKeyPacket(None));
                    *self.state = PgpDataContent49TypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = PgpDataContent49TypeDeserializerState::PgpKeyPacket(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_pgp_key_packet(data)?;
                    *self.state = PgpDataContent49TypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                PgpDataContent49TypeDeserializerState::PgpKeyPacket(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = PgpDataContent49TypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = PgpDataContent49TypeDeserializerState::PgpKeyPacket(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::PgpDataContent49Type> for PgpDataContent49TypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PgpDataContent49Type>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                pgp_key_packet: None,
                state: Box::new(PgpDataContent49TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, PgpDataContent49TypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::PgpDataContent49Type>
        where
            R: DeserializeReader,
        {
            use PgpDataContent49TypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::PgpKeyPacket(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pgp_key_packet(reader, output, &mut fallback)? {
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
                        allow_any_element = true;
                        fallback.get_or_insert(S::Init__);
                        *self.state = PgpDataContent49TypeDeserializerState::PgpKeyPacket(None);
                        event
                    }
                    (S::PgpKeyPacket(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"PGPKeyPacket")
                        {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_pgp_key_packet(reader, output, &mut fallback)? {
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
                            allow_any_element = true;
                            fallback.get_or_insert(S::PgpKeyPacket(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), true);
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
        fn finish<R>(mut self, reader: &R) -> Result<super::PgpDataContent49Type, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                PgpDataContent49TypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::PgpDataContent49Type {
                pgp_key_packet: self
                    .pgp_key_packet
                    .ok_or_else(|| ErrorKind::MissingElement("PGPKeyPacket".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TransformTypeDeserializer {
        algorithm: String,
        content: Vec<super::TransformTypeContent>,
        state: Box<TransformTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TransformTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::TransformTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl TransformTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut algorithm: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Algorithm")
                ) {
                    reader.read_attrib(&mut algorithm, b"Algorithm", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                algorithm: algorithm.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("Algorithm".into()))
                })?,
                content: Vec::new(),
                state: Box::new(TransformTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TransformTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let TransformTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::TransformTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TransformTypeContent>,
            fallback: &mut Option<TransformTypeDeserializerState>,
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
                    .unwrap_or(TransformTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = TransformTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TransformTypeDeserializerState::Content__(deserializer);
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TransformTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = TransformTypeDeserializerState::Next__;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::TransformType> for TransformTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::TransformType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TransformType>
        where
            R: DeserializeReader,
        {
            use TransformTypeDeserializerState as S;
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
                            <super::TransformTypeContent as WithDeserializer>::Deserializer::init(
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
        fn finish<R>(mut self, reader: &R) -> Result<super::TransformType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, TransformTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::TransformType {
                algorithm: self.algorithm,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct TransformTypeContentDeserializer {
        state: Box<TransformTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum TransformTypeContentDeserializerState {
        Init__,
        XPath(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::TransformTypeContent),
        Unknown__,
    }
    impl TransformTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<TransformTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_DS),
                    Some(b"XPath")
                ) {
                    let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_x_path(reader, Default::default(), output, &mut *fallback);
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(TransformTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, true))
        }
        fn finish_state<R>(
            reader: &R,
            state: TransformTypeContentDeserializerState,
        ) -> Result<super::TransformTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use TransformTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::XPath(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_x_path(&mut values, value)?;
                    }
                    Ok(super::TransformTypeContent::XPath(values.ok_or_else(
                        || ErrorKind::MissingElement("XPath".into()),
                    )?))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
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
        fn handle_x_path<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<TransformTypeContentDeserializerState>,
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
                    None => TransformTypeContentDeserializerState::XPath(values, None),
                    Some(TransformTypeContentDeserializerState::XPath(_, Some(deserializer))) => {
                        TransformTypeContentDeserializerState::XPath(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(TransformTypeContentDeserializerState::XPath(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_x_path(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x_path(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        TransformTypeContentDeserializerState::XPath(values, None),
                    )?;
                    *self.state = TransformTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state =
                        TransformTypeContentDeserializerState::XPath(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::TransformTypeContent> for TransformTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TransformTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(TransformTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, TransformTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::TransformTypeContent>
        where
            R: DeserializeReader,
        {
            use TransformTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::XPath(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_x_path(reader, values, output, &mut fallback)? {
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
                    (S::XPath(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_x_path(reader, values, output, &mut fallback)? {
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
        fn finish<R>(self, reader: &R) -> Result<super::TransformTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct DsaKeyValueContent60TypeDeserializer {
        p: Option<String>,
        q: Option<String>,
        state: Box<DsaKeyValueContent60TypeDeserializerState>,
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
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DsaKeyValueContent60TypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use DsaKeyValueContent60TypeDeserializerState as S;
            match state {
                S::P(Some(deserializer)) => self.store_p(deserializer.finish(reader)?)?,
                S::Q(Some(deserializer)) => self.store_q(deserializer.finish(reader)?)?,
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
        fn handle_p<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DsaKeyValueContent60TypeDeserializerState>,
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
                if self.p.is_some() {
                    fallback.get_or_insert(DsaKeyValueContent60TypeDeserializerState::P(None));
                    *self.state = DsaKeyValueContent60TypeDeserializerState::Q(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DsaKeyValueContent60TypeDeserializerState::P(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_p(data)?;
                    *self.state = DsaKeyValueContent60TypeDeserializerState::Q(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DsaKeyValueContent60TypeDeserializerState::P(
                                Some(deserializer),
                            ));
                            *self.state = DsaKeyValueContent60TypeDeserializerState::Q(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                DsaKeyValueContent60TypeDeserializerState::P(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_q<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DsaKeyValueContent60TypeDeserializerState>,
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
                if self.q.is_some() {
                    fallback.get_or_insert(DsaKeyValueContent60TypeDeserializerState::Q(None));
                    *self.state = DsaKeyValueContent60TypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DsaKeyValueContent60TypeDeserializerState::Q(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_q(data)?;
                    *self.state = DsaKeyValueContent60TypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DsaKeyValueContent60TypeDeserializerState::Q(
                                Some(deserializer),
                            ));
                            *self.state = DsaKeyValueContent60TypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                DsaKeyValueContent60TypeDeserializerState::Q(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DsaKeyValueContent60Type>
        for DsaKeyValueContent60TypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DsaKeyValueContent60Type>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                p: None,
                q: None,
                state: Box::new(DsaKeyValueContent60TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, DsaKeyValueContent60TypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::DsaKeyValueContent60Type>
        where
            R: DeserializeReader,
        {
            use DsaKeyValueContent60TypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::P(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_p(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_q(reader, output, &mut fallback)? {
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
                        *self.state = DsaKeyValueContent60TypeDeserializerState::P(None);
                        event
                    }
                    (S::P(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"P") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_p(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Q(None);
                            event
                        }
                    }
                    (S::Q(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"Q") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_q(reader, output, &mut fallback)? {
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
        fn finish<R>(mut self, reader: &R) -> Result<super::DsaKeyValueContent60Type, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                DsaKeyValueContent60TypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::DsaKeyValueContent60Type {
                p: self
                    .p
                    .ok_or_else(|| ErrorKind::MissingElement("P".into()))?,
                q: self
                    .q
                    .ok_or_else(|| ErrorKind::MissingElement("Q".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DsaKeyValueContent61TypeDeserializer {
        seed: Option<String>,
        pgen_counter: Option<String>,
        state: Box<DsaKeyValueContent61TypeDeserializerState>,
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
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DsaKeyValueContent61TypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use DsaKeyValueContent61TypeDeserializerState as S;
            match state {
                S::Seed(Some(deserializer)) => self.store_seed(deserializer.finish(reader)?)?,
                S::PgenCounter(Some(deserializer)) => {
                    self.store_pgen_counter(deserializer.finish(reader)?)?
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
        fn handle_seed<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DsaKeyValueContent61TypeDeserializerState>,
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
                if self.seed.is_some() {
                    fallback.get_or_insert(DsaKeyValueContent61TypeDeserializerState::Seed(None));
                    *self.state = DsaKeyValueContent61TypeDeserializerState::PgenCounter(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DsaKeyValueContent61TypeDeserializerState::Seed(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_seed(data)?;
                    *self.state = DsaKeyValueContent61TypeDeserializerState::PgenCounter(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                DsaKeyValueContent61TypeDeserializerState::Seed(Some(deserializer)),
                            );
                            *self.state =
                                DsaKeyValueContent61TypeDeserializerState::PgenCounter(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                DsaKeyValueContent61TypeDeserializerState::Seed(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_pgen_counter<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DsaKeyValueContent61TypeDeserializerState>,
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
                if self.pgen_counter.is_some() {
                    fallback.get_or_insert(DsaKeyValueContent61TypeDeserializerState::PgenCounter(
                        None,
                    ));
                    *self.state = DsaKeyValueContent61TypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DsaKeyValueContent61TypeDeserializerState::PgenCounter(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_pgen_counter(data)?;
                    *self.state = DsaKeyValueContent61TypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                DsaKeyValueContent61TypeDeserializerState::PgenCounter(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = DsaKeyValueContent61TypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DsaKeyValueContent61TypeDeserializerState::PgenCounter(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DsaKeyValueContent61Type>
        for DsaKeyValueContent61TypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DsaKeyValueContent61Type>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                seed: None,
                pgen_counter: None,
                state: Box::new(DsaKeyValueContent61TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, DsaKeyValueContent61TypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::DsaKeyValueContent61Type>
        where
            R: DeserializeReader,
        {
            use DsaKeyValueContent61TypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Seed(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_seed(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pgen_counter(reader, output, &mut fallback)? {
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
                        *self.state = DsaKeyValueContent61TypeDeserializerState::Seed(None);
                        event
                    }
                    (S::Seed(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"Seed") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_seed(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::PgenCounter(None);
                            event
                        }
                    }
                    (S::PgenCounter(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"PgenCounter")
                        {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_pgen_counter(reader, output, &mut fallback)? {
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
        fn finish<R>(mut self, reader: &R) -> Result<super::DsaKeyValueContent61Type, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                DsaKeyValueContent61TypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::DsaKeyValueContent61Type {
                seed: self
                    .seed
                    .ok_or_else(|| ErrorKind::MissingElement("Seed".into()))?,
                pgen_counter: self
                    .pgen_counter
                    .ok_or_else(|| ErrorKind::MissingElement("PgenCounter".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct X509IssuerSerialTypeDeserializer {
        x509_issuer_name: Option<String>,
        x509_serial_number: Option<i32>,
        state: Box<X509IssuerSerialTypeDeserializerState>,
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
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                x509_issuer_name: None,
                x509_serial_number: None,
                state: Box::new(X509IssuerSerialTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: X509IssuerSerialTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use X509IssuerSerialTypeDeserializerState as S;
            match state {
                S::X509IssuerName(Some(deserializer)) => {
                    self.store_x509_issuer_name(deserializer.finish(reader)?)?
                }
                S::X509SerialNumber(Some(deserializer)) => {
                    self.store_x509_serial_number(deserializer.finish(reader)?)?
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
        fn handle_x509_issuer_name<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<X509IssuerSerialTypeDeserializerState>,
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
                if self.x509_issuer_name.is_some() {
                    fallback
                        .get_or_insert(X509IssuerSerialTypeDeserializerState::X509IssuerName(None));
                    *self.state = X509IssuerSerialTypeDeserializerState::X509SerialNumber(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = X509IssuerSerialTypeDeserializerState::X509IssuerName(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_x509_issuer_name(data)?;
                    *self.state = X509IssuerSerialTypeDeserializerState::X509SerialNumber(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                X509IssuerSerialTypeDeserializerState::X509IssuerName(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                X509IssuerSerialTypeDeserializerState::X509SerialNumber(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = X509IssuerSerialTypeDeserializerState::X509IssuerName(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_x509_serial_number<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<X509IssuerSerialTypeDeserializerState>,
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
                if self.x509_serial_number.is_some() {
                    fallback.get_or_insert(
                        X509IssuerSerialTypeDeserializerState::X509SerialNumber(None),
                    );
                    *self.state = X509IssuerSerialTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = X509IssuerSerialTypeDeserializerState::X509SerialNumber(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_x509_serial_number(data)?;
                    *self.state = X509IssuerSerialTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                X509IssuerSerialTypeDeserializerState::X509SerialNumber(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = X509IssuerSerialTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = X509IssuerSerialTypeDeserializerState::X509SerialNumber(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::X509IssuerSerialType> for X509IssuerSerialTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::X509IssuerSerialType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::X509IssuerSerialType>
        where
            R: DeserializeReader,
        {
            use X509IssuerSerialTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::X509IssuerName(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_x509_issuer_name(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_x509_serial_number(reader, output, &mut fallback)? {
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
                        *self.state = X509IssuerSerialTypeDeserializerState::X509IssuerName(None);
                        event
                    }
                    (S::X509IssuerName(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_DS),
                            b"X509IssuerName",
                        ) {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_x509_issuer_name(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::X509SerialNumber(None);
                            event
                        }
                    }
                    (S::X509SerialNumber(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_DS),
                            b"X509SerialNumber",
                        ) {
                            let output =
                                <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_x509_serial_number(reader, output, &mut fallback)? {
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
        fn finish<R>(mut self, reader: &R) -> Result<super::X509IssuerSerialType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                X509IssuerSerialTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::X509IssuerSerialType {
                x509_issuer_name: self
                    .x509_issuer_name
                    .ok_or_else(|| ErrorKind::MissingElement("X509IssuerName".into()))?,
                x509_serial_number: self
                    .x509_serial_number
                    .ok_or_else(|| ErrorKind::MissingElement("X509SerialNumber".into()))?,
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        write_attrib(&mut bytes, "version", &self.value.version)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DirectoryReqTypeSerializerState::CreateDateTimestamp(x) => {
                        match x.next().transpose()? {
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
                    DirectoryReqTypeSerializerState::Merchant(x) => match x.next().transpose()? {
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
                    },
                    DirectoryReqTypeSerializerState::Signature(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DirectoryReqTypeSerializerState::End__,
                    },
                    DirectoryReqTypeSerializerState::End__ => {
                        *self.state = DirectoryReqTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DirectoryReqTypeSerializerState::Done__ => return Ok(None),
                    DirectoryReqTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DirectoryReqTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DirectoryReqMerchantTypeSerializerState::MerchantId(x) => {
                        match x.next().transpose()? {
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
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = DirectoryReqMerchantTypeSerializerState::End__,
                        }
                    }
                    DirectoryReqMerchantTypeSerializerState::End__ => {
                        *self.state = DirectoryReqMerchantTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DirectoryReqMerchantTypeSerializerState::Done__ => return Ok(None),
                    DirectoryReqMerchantTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DirectoryReqMerchantTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        write_attrib_opt(&mut bytes, "ds:Id", &self.value.id)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SignatureTypeSerializerState::SignedInfo(x) => match x.next().transpose()? {
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
                    },
                    SignatureTypeSerializerState::SignatureValue(x) => {
                        match x.next().transpose()? {
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
                    SignatureTypeSerializerState::KeyInfo(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = SignatureTypeSerializerState::Object(IterSerializer::new(
                                &self.value.object[..],
                                Some("ds:Object"),
                                false,
                            ))
                        }
                    },
                    SignatureTypeSerializerState::Object(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = SignatureTypeSerializerState::End__,
                    },
                    SignatureTypeSerializerState::End__ => {
                        *self.state = SignatureTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SignatureTypeSerializerState::Done__ => return Ok(None),
                    SignatureTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for SignatureTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        write_attrib_opt(&mut bytes, "ds:Id", &self.value.id)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SignedInfoTypeSerializerState::CanonicalizationMethod(x) => {
                        match x.next().transpose()? {
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
                        match x.next().transpose()? {
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
                    SignedInfoTypeSerializerState::Reference(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = SignedInfoTypeSerializerState::End__,
                    },
                    SignedInfoTypeSerializerState::End__ => {
                        *self.state = SignedInfoTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SignedInfoTypeSerializerState::Done__ => return Ok(None),
                    SignedInfoTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for SignedInfoTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SignatureValueTypeSerializerState::Init__ => {
                        *self.state = SignatureValueTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        write_attrib_opt(&mut bytes, "ds:Id", &self.value.id)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SignatureValueTypeSerializerState::Content__(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SignatureValueTypeSerializerState::End__,
                        }
                    }
                    SignatureValueTypeSerializerState::End__ => {
                        *self.state = SignatureValueTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SignatureValueTypeSerializerState::Done__ => return Ok(None),
                    SignatureValueTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for SignatureValueTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    KeyInfoTypeSerializerState::Init__ => {
                        *self.state = KeyInfoTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        write_attrib_opt(&mut bytes, "ds:Id", &self.value.id)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    KeyInfoTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = KeyInfoTypeSerializerState::End__,
                    },
                    KeyInfoTypeSerializerState::End__ => {
                        *self.state = KeyInfoTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    KeyInfoTypeSerializerState::Done__ => return Ok(None),
                    KeyInfoTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for KeyInfoTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                    KeyInfoTypeContentSerializerState::KeyName(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = KeyInfoTypeContentSerializerState::Done__,
                    },
                    KeyInfoTypeContentSerializerState::KeyValue(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = KeyInfoTypeContentSerializerState::Done__,
                        }
                    }
                    KeyInfoTypeContentSerializerState::RetrievalMethod(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = KeyInfoTypeContentSerializerState::Done__,
                        }
                    }
                    KeyInfoTypeContentSerializerState::X509Data(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = KeyInfoTypeContentSerializerState::Done__,
                        }
                    }
                    KeyInfoTypeContentSerializerState::PgpData(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = KeyInfoTypeContentSerializerState::Done__,
                    },
                    KeyInfoTypeContentSerializerState::SpkiData(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = KeyInfoTypeContentSerializerState::Done__,
                        }
                    }
                    KeyInfoTypeContentSerializerState::MgmtData(x) => {
                        match x.next().transpose()? {
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
    impl<'ser> Iterator for KeyInfoTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ObjectTypeSerializerState::Init__ => {
                        *self.state = ObjectTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        write_attrib_opt(&mut bytes, "ds:Id", &self.value.id)?;
                        write_attrib_opt(&mut bytes, "ds:MimeType", &self.value.mime_type)?;
                        write_attrib_opt(&mut bytes, "ds:Encoding", &self.value.encoding)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    ObjectTypeSerializerState::Done__ => return Ok(None),
                    ObjectTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ObjectTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CanonicalizationMethodTypeSerializerState::Init__ => {
                        *self.state = CanonicalizationMethodTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        write_attrib(&mut bytes, "ds:Algorithm", &self.value.algorithm)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    CanonicalizationMethodTypeSerializerState::Done__ => return Ok(None),
                    CanonicalizationMethodTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for CanonicalizationMethodTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        write_attrib(&mut bytes, "ds:Algorithm", &self.value.algorithm)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SignatureMethodTypeSerializerState::HmacOutputLength(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SignatureMethodTypeSerializerState::End__,
                        }
                    }
                    SignatureMethodTypeSerializerState::End__ => {
                        *self.state = SignatureMethodTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SignatureMethodTypeSerializerState::Done__ => return Ok(None),
                    SignatureMethodTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for SignatureMethodTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        write_attrib_opt(&mut bytes, "ds:Id", &self.value.id)?;
                        write_attrib_opt(&mut bytes, "ds:URI", &self.value.uri)?;
                        write_attrib_opt(&mut bytes, "ds:Type", &self.value.type_)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ReferenceTypeSerializerState::Transforms(x) => match x.next().transpose()? {
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
                    },
                    ReferenceTypeSerializerState::DigestMethod(x) => match x.next().transpose()? {
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
                    },
                    ReferenceTypeSerializerState::DigestValue(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ReferenceTypeSerializerState::End__,
                    },
                    ReferenceTypeSerializerState::End__ => {
                        *self.state = ReferenceTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ReferenceTypeSerializerState::Done__ => return Ok(None),
                    ReferenceTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ReferenceTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    KeyValueTypeSerializerState::Init__ => {
                        *self.state = KeyValueTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    KeyValueTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = KeyValueTypeSerializerState::End__,
                    },
                    KeyValueTypeSerializerState::End__ => {
                        *self.state = KeyValueTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    KeyValueTypeSerializerState::Done__ => return Ok(None),
                    KeyValueTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for KeyValueTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = KeyValueTypeContentSerializerState::Done__,
                        }
                    }
                    KeyValueTypeContentSerializerState::RsaKeyValue(x) => {
                        match x.next().transpose()? {
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
    impl<'ser> Iterator for KeyValueTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        write_attrib_opt(&mut bytes, "ds:URI", &self.value.uri)?;
                        write_attrib_opt(&mut bytes, "ds:Type", &self.value.type_)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    RetrievalMethodTypeSerializerState::Transforms(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RetrievalMethodTypeSerializerState::End__,
                        }
                    }
                    RetrievalMethodTypeSerializerState::End__ => {
                        *self.state = RetrievalMethodTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    RetrievalMethodTypeSerializerState::Done__ => return Ok(None),
                    RetrievalMethodTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for RetrievalMethodTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    X509DataTypeSerializerState::Init__ => {
                        *self.state = X509DataTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    X509DataTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = X509DataTypeSerializerState::End__,
                    },
                    X509DataTypeSerializerState::End__ => {
                        *self.state = X509DataTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    X509DataTypeSerializerState::Done__ => return Ok(None),
                    X509DataTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for X509DataTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    X509DataTypeContentSerializerState::Init__ => {
                        *self.state = X509DataTypeContentSerializerState::Content43(
                            WithSerializer::serializer(
                                &self.value.content_43,
                                Some("ds:Content43"),
                                false,
                            )?,
                        );
                    }
                    X509DataTypeContentSerializerState::Content43(x) => {
                        match x.next().transpose()? {
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
    impl<'ser> Iterator for X509DataTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PgpDataTypeSerializerState::Init__ => {
                        *self.state = PgpDataTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    PgpDataTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PgpDataTypeSerializerState::End__,
                    },
                    PgpDataTypeSerializerState::End__ => {
                        *self.state = PgpDataTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    PgpDataTypeSerializerState::Done__ => return Ok(None),
                    PgpDataTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for PgpDataTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PgpDataTypeContentSerializerState::Init__ => match self.value {
                        super::PgpDataTypeContent::Content47(x) => {
                            *self.state = PgpDataTypeContentSerializerState::Content47(
                                WithSerializer::serializer(x, Some("ds:Content47"), false)?,
                            )
                        }
                        super::PgpDataTypeContent::Content49(x) => {
                            *self.state = PgpDataTypeContentSerializerState::Content49(
                                WithSerializer::serializer(x, Some("ds:Content49"), false)?,
                            )
                        }
                    },
                    PgpDataTypeContentSerializerState::Content47(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PgpDataTypeContentSerializerState::Done__,
                        }
                    }
                    PgpDataTypeContentSerializerState::Content49(x) => {
                        match x.next().transpose()? {
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
    impl<'ser> Iterator for PgpDataTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SpkiDataTypeSerializerState::Init__ => {
                        *self.state = SpkiDataTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SpkiDataTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = SpkiDataTypeSerializerState::End__,
                    },
                    SpkiDataTypeSerializerState::End__ => {
                        *self.state = SpkiDataTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SpkiDataTypeSerializerState::Done__ => return Ok(None),
                    SpkiDataTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for SpkiDataTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        match x.next().transpose()? {
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
    impl<'ser> Iterator for SpkiDataTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TransformsTypeSerializerState::Transform(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = TransformsTypeSerializerState::End__,
                    },
                    TransformsTypeSerializerState::End__ => {
                        *self.state = TransformsTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TransformsTypeSerializerState::Done__ => return Ok(None),
                    TransformsTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for TransformsTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DigestMethodTypeSerializerState::Init__ => {
                        *self.state = DigestMethodTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        write_attrib(&mut bytes, "ds:Algorithm", &self.value.algorithm)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    DigestMethodTypeSerializerState::Done__ => return Ok(None),
                    DigestMethodTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DigestMethodTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DsaKeyValueTypeSerializerState::Init__ => {
                        *self.state =
                            DsaKeyValueTypeSerializerState::Content60(IterSerializer::new(
                                self.value.content_60.as_ref(),
                                Some("ds:Content60"),
                                false,
                            ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DsaKeyValueTypeSerializerState::Content60(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = DsaKeyValueTypeSerializerState::G(
                                    IterSerializer::new(self.value.g.as_ref(), Some("ds:G"), false),
                                )
                            }
                        }
                    }
                    DsaKeyValueTypeSerializerState::G(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = DsaKeyValueTypeSerializerState::Y(
                                WithSerializer::serializer(&self.value.y, Some("ds:Y"), false)?,
                            )
                        }
                    },
                    DsaKeyValueTypeSerializerState::Y(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = DsaKeyValueTypeSerializerState::J(
                                    IterSerializer::new(self.value.j.as_ref(), Some("ds:J"), false),
                                )
                            }
                        }
                    }
                    DsaKeyValueTypeSerializerState::J(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                DsaKeyValueTypeSerializerState::Content61(IterSerializer::new(
                                    self.value.content_61.as_ref(),
                                    Some("ds:Content61"),
                                    false,
                                ))
                        }
                    },
                    DsaKeyValueTypeSerializerState::Content61(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DsaKeyValueTypeSerializerState::End__,
                    },
                    DsaKeyValueTypeSerializerState::End__ => {
                        *self.state = DsaKeyValueTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DsaKeyValueTypeSerializerState::Done__ => return Ok(None),
                    DsaKeyValueTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DsaKeyValueTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    RsaKeyValueTypeSerializerState::Modulus(x) => match x.next().transpose()? {
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
                    },
                    RsaKeyValueTypeSerializerState::Exponent(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = RsaKeyValueTypeSerializerState::End__,
                    },
                    RsaKeyValueTypeSerializerState::End__ => {
                        *self.state = RsaKeyValueTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    RsaKeyValueTypeSerializerState::Done__ => return Ok(None),
                    RsaKeyValueTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for RsaKeyValueTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    X509DataContent43TypeSerializerState::Init__ => {
                        *self.state = X509DataContent43TypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                    }
                    X509DataContent43TypeSerializerState::Content__(x) => {
                        match x.next().transpose()? {
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
    impl<'ser> Iterator for X509DataContent43TypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = X509DataContent43TypeContentSerializerState::Done__
                            }
                        }
                    }
                    X509DataContent43TypeContentSerializerState::X509Ski(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = X509DataContent43TypeContentSerializerState::Done__
                            }
                        }
                    }
                    X509DataContent43TypeContentSerializerState::X509SubjectName(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = X509DataContent43TypeContentSerializerState::Done__
                            }
                        }
                    }
                    X509DataContent43TypeContentSerializerState::X509Certificate(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = X509DataContent43TypeContentSerializerState::Done__
                            }
                        }
                    }
                    X509DataContent43TypeContentSerializerState::X509Crl(x) => {
                        match x.next().transpose()? {
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
    impl<'ser> Iterator for X509DataContent43TypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        match x.next().transpose()? {
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
                        match x.next().transpose()? {
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
    impl<'ser> Iterator for PgpDataContent47TypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        match x.next().transpose()? {
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
    impl<'ser> Iterator for PgpDataContent49TypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TransformTypeSerializerState::Init__ => {
                        *self.state = TransformTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        write_attrib(&mut bytes, "ds:Algorithm", &self.value.algorithm)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TransformTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = TransformTypeSerializerState::End__,
                    },
                    TransformTypeSerializerState::End__ => {
                        *self.state = TransformTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TransformTypeSerializerState::Done__ => return Ok(None),
                    TransformTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for TransformTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TransformTypeContentSerializerState::Init__ => match self.value {
                        super::TransformTypeContent::XPath(x) => {
                            *self.state = TransformTypeContentSerializerState::XPath(
                                WithSerializer::serializer(x, Some("ds:XPath"), false)?,
                            )
                        }
                    },
                    TransformTypeContentSerializerState::XPath(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = TransformTypeContentSerializerState::Done__,
                    },
                    TransformTypeContentSerializerState::Done__ => return Ok(None),
                    TransformTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for TransformTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DsaKeyValueContent60TypeSerializerState::Init__ => {
                        *self.state = DsaKeyValueContent60TypeSerializerState::P(
                            WithSerializer::serializer(&self.value.p, Some("ds:P"), false)?,
                        );
                    }
                    DsaKeyValueContent60TypeSerializerState::P(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = DsaKeyValueContent60TypeSerializerState::Q(
                                WithSerializer::serializer(&self.value.q, Some("ds:Q"), false)?,
                            )
                        }
                    },
                    DsaKeyValueContent60TypeSerializerState::Q(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DsaKeyValueContent60TypeSerializerState::Done__,
                    },
                    DsaKeyValueContent60TypeSerializerState::Done__ => return Ok(None),
                    DsaKeyValueContent60TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DsaKeyValueContent60TypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DsaKeyValueContent61TypeSerializerState::Init__ => {
                        *self.state = DsaKeyValueContent61TypeSerializerState::Seed(
                            WithSerializer::serializer(&self.value.seed, Some("ds:Seed"), false)?,
                        );
                    }
                    DsaKeyValueContent61TypeSerializerState::Seed(x) => {
                        match x.next().transpose()? {
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
                        match x.next().transpose()? {
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
    impl<'ser> Iterator for DsaKeyValueContent61TypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
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
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    X509IssuerSerialTypeSerializerState::X509IssuerName(x) => {
                        match x.next().transpose()? {
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
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = X509IssuerSerialTypeSerializerState::End__,
                        }
                    }
                    X509IssuerSerialTypeSerializerState::End__ => {
                        *self.state = X509IssuerSerialTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    X509IssuerSerialTypeSerializerState::Done__ => return Ok(None),
                    X509IssuerSerialTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for X509IssuerSerialTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
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
