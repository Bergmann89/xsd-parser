pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_DEFAULT: Namespace =
    Namespace::new_const(b"http://www.idealdesk.com/ideal/messages/mer-acq/3.3.1");
pub const NS_DS: Namespace = Namespace::new_const(b"http://www.w3.org/2000/09/xmldsig#");
use xsd_parser::{
    quick_xml::{deserialize_new::WithDeserializer, Error, WithSerializer},
    schema::Namespace,
};
pub type DirectoryReq = DirectoryReqType;
#[derive(Debug, Clone)]
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
            state: quick_xml_serialize::DirectoryReqTypeSerializerState::Init__,
            name: name.unwrap_or("DirectoryReq"),
            is_root,
        })
    }
}
impl WithDeserializer for DirectoryReqType {
    type Deserializer = quick_xml_deserialize::DirectoryReqTypeDeserializer;
}
#[derive(Debug, Clone)]
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
            state: quick_xml_serialize::DirectoryReqMerchantTypeSerializerState::Init__,
            name: name.unwrap_or("DirectoryReqMerchant"),
            is_root,
        })
    }
}
impl WithDeserializer for DirectoryReqMerchantType {
    type Deserializer = quick_xml_deserialize::DirectoryReqMerchantTypeDeserializer;
}
#[derive(Debug, Clone)]
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
            state: quick_xml_serialize::SignatureTypeSerializerState::Init__,
            name: name.unwrap_or("ds:SignatureType"),
            is_root,
        })
    }
}
impl WithDeserializer for SignatureType {
    type Deserializer = quick_xml_deserialize::SignatureTypeDeserializer;
}
#[derive(Debug, Clone)]
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
            state: quick_xml_serialize::SignedInfoTypeSerializerState::Init__,
            name: name.unwrap_or("ds:SignedInfoType"),
            is_root,
        })
    }
}
impl WithDeserializer for SignedInfoType {
    type Deserializer = quick_xml_deserialize::SignedInfoTypeDeserializer;
}
#[derive(Debug, Clone)]
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
            state: quick_xml_serialize::SignatureValueTypeSerializerState::Init__,
            name: name.unwrap_or("ds:SignatureValueType"),
            is_root,
        })
    }
}
impl WithDeserializer for SignatureValueType {
    type Deserializer = quick_xml_deserialize::SignatureValueTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct KeyInfoType {
    pub id: Option<String>,
    pub content: Vec<KeyInfoTypeContent>,
}
#[derive(Debug, Clone)]
pub enum KeyInfoTypeContent {
    KeyName(String),
    KeyValue(KeyValueType),
    RetrievalMethod(RetrievalMethodType),
    X509Data(X509DataType),
    Pgpdata(PgpdataType),
    Spkidata(SpkidataType),
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
            state: quick_xml_serialize::KeyInfoTypeSerializerState::Init__,
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
            state: quick_xml_serialize::KeyInfoTypeContentSerializerState::Init__,
        })
    }
}
impl WithDeserializer for KeyInfoType {
    type Deserializer = quick_xml_deserialize::KeyInfoTypeDeserializer;
}
impl WithDeserializer for KeyInfoTypeContent {
    type Deserializer = quick_xml_deserialize::KeyInfoTypeContentDeserializer;
}
#[derive(Debug, Clone)]
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
            state: quick_xml_serialize::ObjectTypeSerializerState::Init__,
            name: name.unwrap_or("ds:ObjectType"),
            is_root,
        })
    }
}
impl WithDeserializer for ObjectType {
    type Deserializer = quick_xml_deserialize::ObjectTypeDeserializer;
}
#[derive(Debug, Clone)]
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
            state: quick_xml_serialize::CanonicalizationMethodTypeSerializerState::Init__,
            name: name.unwrap_or("ds:CanonicalizationMethodType"),
            is_root,
        })
    }
}
impl WithDeserializer for CanonicalizationMethodType {
    type Deserializer = quick_xml_deserialize::CanonicalizationMethodTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct SignatureMethodType {
    pub algorithm: String,
    pub hmacoutput_length: Option<i32>,
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
            state: quick_xml_serialize::SignatureMethodTypeSerializerState::Init__,
            name: name.unwrap_or("ds:SignatureMethodType"),
            is_root,
        })
    }
}
impl WithDeserializer for SignatureMethodType {
    type Deserializer = quick_xml_deserialize::SignatureMethodTypeDeserializer;
}
#[derive(Debug, Clone)]
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
            state: quick_xml_serialize::ReferenceTypeSerializerState::Init__,
            name: name.unwrap_or("ds:ReferenceType"),
            is_root,
        })
    }
}
impl WithDeserializer for ReferenceType {
    type Deserializer = quick_xml_deserialize::ReferenceTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct KeyValueType {
    pub content: KeyValueTypeContent,
}
#[derive(Debug, Clone)]
pub enum KeyValueTypeContent {
    DsakeyValue(DsakeyValueType),
    RsakeyValue(RsakeyValueType),
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
            state: quick_xml_serialize::KeyValueTypeSerializerState::Init__,
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
            state: quick_xml_serialize::KeyValueTypeContentSerializerState::Init__,
        })
    }
}
impl WithDeserializer for KeyValueType {
    type Deserializer = quick_xml_deserialize::KeyValueTypeDeserializer;
}
impl WithDeserializer for KeyValueTypeContent {
    type Deserializer = quick_xml_deserialize::KeyValueTypeContentDeserializer;
}
#[derive(Debug, Clone)]
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
            state: quick_xml_serialize::RetrievalMethodTypeSerializerState::Init__,
            name: name.unwrap_or("ds:RetrievalMethodType"),
            is_root,
        })
    }
}
impl WithDeserializer for RetrievalMethodType {
    type Deserializer = quick_xml_deserialize::RetrievalMethodTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct X509DataType {
    pub content: Vec<X509DataTypeContent>,
}
#[derive(Debug, Clone)]
pub struct X509DataTypeContent {
    pub content_103: X509DataContent103Type,
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
            state: quick_xml_serialize::X509DataTypeSerializerState::Init__,
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
            state: quick_xml_serialize::X509DataTypeContentSerializerState::Init__,
        })
    }
}
impl WithDeserializer for X509DataType {
    type Deserializer = quick_xml_deserialize::X509DataTypeDeserializer;
}
impl WithDeserializer for X509DataTypeContent {
    type Deserializer = quick_xml_deserialize::X509DataTypeContentDeserializer;
}
#[derive(Debug, Clone)]
pub struct PgpdataType {
    pub content: PgpdataTypeContent,
}
#[derive(Debug, Clone)]
pub enum PgpdataTypeContent {
    Content113(PgpdataContent113Type),
    Content116(PgpdataContent116Type),
}
impl WithSerializer for PgpdataType {
    type Serializer<'x> = quick_xml_serialize::PgpdataTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::PgpdataTypeSerializer {
            value: self,
            state: quick_xml_serialize::PgpdataTypeSerializerState::Init__,
            name: name.unwrap_or("ds:PGPDataType"),
            is_root,
        })
    }
}
impl WithSerializer for PgpdataTypeContent {
    type Serializer<'x> = quick_xml_serialize::PgpdataTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::PgpdataTypeContentSerializer {
            value: self,
            state: quick_xml_serialize::PgpdataTypeContentSerializerState::Init__,
        })
    }
}
impl WithDeserializer for PgpdataType {
    type Deserializer = quick_xml_deserialize::PgpdataTypeDeserializer;
}
impl WithDeserializer for PgpdataTypeContent {
    type Deserializer = quick_xml_deserialize::PgpdataTypeContentDeserializer;
}
#[derive(Debug, Clone)]
pub struct SpkidataType {
    pub content: Vec<SpkidataTypeContent>,
}
#[derive(Debug, Clone)]
pub struct SpkidataTypeContent {
    pub spkisexp: String,
}
impl WithSerializer for SpkidataType {
    type Serializer<'x> = quick_xml_serialize::SpkidataTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::SpkidataTypeSerializer {
            value: self,
            state: quick_xml_serialize::SpkidataTypeSerializerState::Init__,
            name: name.unwrap_or("ds:SPKIDataType"),
            is_root,
        })
    }
}
impl WithSerializer for SpkidataTypeContent {
    type Serializer<'x> = quick_xml_serialize::SpkidataTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::SpkidataTypeContentSerializer {
            value: self,
            state: quick_xml_serialize::SpkidataTypeContentSerializerState::Init__,
        })
    }
}
impl WithDeserializer for SpkidataType {
    type Deserializer = quick_xml_deserialize::SpkidataTypeDeserializer;
}
impl WithDeserializer for SpkidataTypeContent {
    type Deserializer = quick_xml_deserialize::SpkidataTypeContentDeserializer;
}
#[derive(Debug, Clone)]
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
            state: quick_xml_serialize::TransformsTypeSerializerState::Init__,
            name: name.unwrap_or("ds:TransformsType"),
            is_root,
        })
    }
}
impl WithDeserializer for TransformsType {
    type Deserializer = quick_xml_deserialize::TransformsTypeDeserializer;
}
#[derive(Debug, Clone)]
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
            state: quick_xml_serialize::DigestMethodTypeSerializerState::Init__,
            name: name.unwrap_or("ds:DigestMethodType"),
            is_root,
        })
    }
}
impl WithDeserializer for DigestMethodType {
    type Deserializer = quick_xml_deserialize::DigestMethodTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct DsakeyValueType {
    pub content_125: Option<DsakeyValueContent125Type>,
    pub g: Option<String>,
    pub y: String,
    pub j: Option<String>,
    pub content_131: Option<DsakeyValueContent131Type>,
}
impl WithSerializer for DsakeyValueType {
    type Serializer<'x> = quick_xml_serialize::DsakeyValueTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::DsakeyValueTypeSerializer {
            value: self,
            state: quick_xml_serialize::DsakeyValueTypeSerializerState::Init__,
            name: name.unwrap_or("ds:DSAKeyValueType"),
            is_root,
        })
    }
}
impl WithDeserializer for DsakeyValueType {
    type Deserializer = quick_xml_deserialize::DsakeyValueTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct RsakeyValueType {
    pub modulus: String,
    pub exponent: String,
}
impl WithSerializer for RsakeyValueType {
    type Serializer<'x> = quick_xml_serialize::RsakeyValueTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::RsakeyValueTypeSerializer {
            value: self,
            state: quick_xml_serialize::RsakeyValueTypeSerializerState::Init__,
            name: name.unwrap_or("ds:RSAKeyValueType"),
            is_root,
        })
    }
}
impl WithDeserializer for RsakeyValueType {
    type Deserializer = quick_xml_deserialize::RsakeyValueTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct X509DataContent103Type {
    pub content: X509DataContent103TypeContent,
}
#[derive(Debug, Clone)]
pub enum X509DataContent103TypeContent {
    X509IssuerSerial(X509IssuerSerialType),
    X509Ski(String),
    X509SubjectName(String),
    X509Certificate(String),
    X509Crl(String),
}
impl WithSerializer for X509DataContent103Type {
    type Serializer<'x> = quick_xml_serialize::X509DataContent103TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::X509DataContent103TypeSerializer {
            value: self,
            state: quick_xml_serialize::X509DataContent103TypeSerializerState::Init__,
        })
    }
}
impl WithSerializer for X509DataContent103TypeContent {
    type Serializer<'x> = quick_xml_serialize::X509DataContent103TypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(
            quick_xml_serialize::X509DataContent103TypeContentSerializer {
                value: self,
                state: quick_xml_serialize::X509DataContent103TypeContentSerializerState::Init__,
            },
        )
    }
}
impl WithDeserializer for X509DataContent103Type {
    type Deserializer = quick_xml_deserialize::X509DataContent103TypeDeserializer;
}
impl WithDeserializer for X509DataContent103TypeContent {
    type Deserializer = quick_xml_deserialize::X509DataContent103TypeContentDeserializer;
}
#[derive(Debug, Clone)]
pub struct PgpdataContent113Type {
    pub pgpkey_id: String,
    pub pgpkey_packet: Option<String>,
}
impl WithSerializer for PgpdataContent113Type {
    type Serializer<'x> = quick_xml_serialize::PgpdataContent113TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::PgpdataContent113TypeSerializer {
            value: self,
            state: quick_xml_serialize::PgpdataContent113TypeSerializerState::Init__,
        })
    }
}
impl WithDeserializer for PgpdataContent113Type {
    type Deserializer = quick_xml_deserialize::PgpdataContent113TypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct PgpdataContent116Type {
    pub pgpkey_packet: String,
}
impl WithSerializer for PgpdataContent116Type {
    type Serializer<'x> = quick_xml_serialize::PgpdataContent116TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::PgpdataContent116TypeSerializer {
            value: self,
            state: quick_xml_serialize::PgpdataContent116TypeSerializerState::Init__,
        })
    }
}
impl WithDeserializer for PgpdataContent116Type {
    type Deserializer = quick_xml_deserialize::PgpdataContent116TypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct TransformType {
    pub algorithm: String,
    pub content: Vec<TransformTypeContent>,
}
#[derive(Debug, Clone)]
pub enum TransformTypeContent {
    Xpath(String),
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
            state: quick_xml_serialize::TransformTypeSerializerState::Init__,
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
            state: quick_xml_serialize::TransformTypeContentSerializerState::Init__,
        })
    }
}
impl WithDeserializer for TransformType {
    type Deserializer = quick_xml_deserialize::TransformTypeDeserializer;
}
impl WithDeserializer for TransformTypeContent {
    type Deserializer = quick_xml_deserialize::TransformTypeContentDeserializer;
}
#[derive(Debug, Clone)]
pub struct DsakeyValueContent125Type {
    pub p: String,
    pub q: String,
}
impl WithSerializer for DsakeyValueContent125Type {
    type Serializer<'x> = quick_xml_serialize::DsakeyValueContent125TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::DsakeyValueContent125TypeSerializer {
            value: self,
            state: quick_xml_serialize::DsakeyValueContent125TypeSerializerState::Init__,
        })
    }
}
impl WithDeserializer for DsakeyValueContent125Type {
    type Deserializer = quick_xml_deserialize::DsakeyValueContent125TypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct DsakeyValueContent131Type {
    pub seed: String,
    pub pgen_counter: String,
}
impl WithSerializer for DsakeyValueContent131Type {
    type Serializer<'x> = quick_xml_serialize::DsakeyValueContent131TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::DsakeyValueContent131TypeSerializer {
            value: self,
            state: quick_xml_serialize::DsakeyValueContent131TypeSerializerState::Init__,
        })
    }
}
impl WithDeserializer for DsakeyValueContent131Type {
    type Deserializer = quick_xml_deserialize::DsakeyValueContent131TypeDeserializer;
}
#[derive(Debug, Clone)]
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
            state: quick_xml_serialize::X509IssuerSerialTypeSerializerState::Init__,
            name: name.unwrap_or("ds:X509IssuerSerialType"),
            is_root,
        })
    }
}
impl WithDeserializer for X509IssuerSerialType {
    type Deserializer = quick_xml_deserialize::X509IssuerSerialTypeDeserializer;
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
        pub(super) state: DirectoryReqTypeSerializerState<'ser>,
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
                match &mut self.state {
                    DirectoryReqTypeSerializerState::Init__ => {
                        self.state = DirectoryReqTypeSerializerState::CreateDateTimestamp(
                            WithSerializer::serializer(
                                &self.value.create_date_timestamp,
                                Some("createDateTimestamp"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        write_attrib(&mut bytes, "version", &self.value.version)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DirectoryReqTypeSerializerState::CreateDateTimestamp(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                self.state = DirectoryReqTypeSerializerState::Merchant(
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
                            self.state = DirectoryReqTypeSerializerState::Signature(
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
                        None => self.state = DirectoryReqTypeSerializerState::End__,
                    },
                    DirectoryReqTypeSerializerState::End__ => {
                        self.state = DirectoryReqTypeSerializerState::Done__;
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
                    self.state = DirectoryReqTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DirectoryReqMerchantTypeSerializer<'ser> {
        pub(super) value: &'ser super::DirectoryReqMerchantType,
        pub(super) state: DirectoryReqMerchantTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DirectoryReqMerchantTypeSerializerState<'ser> {
        Init__,
        MerchantID(<String as WithSerializer>::Serializer<'ser>),
        SubID(<usize as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DirectoryReqMerchantTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    DirectoryReqMerchantTypeSerializerState::Init__ => {
                        self.state = DirectoryReqMerchantTypeSerializerState::MerchantID(
                            WithSerializer::serializer(
                                &self.value.merchant_id,
                                Some("merchantID"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DirectoryReqMerchantTypeSerializerState::MerchantID(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                self.state = DirectoryReqMerchantTypeSerializerState::SubID(
                                    WithSerializer::serializer(
                                        &self.value.sub_id,
                                        Some("subID"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    DirectoryReqMerchantTypeSerializerState::SubID(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => self.state = DirectoryReqMerchantTypeSerializerState::End__,
                        }
                    }
                    DirectoryReqMerchantTypeSerializerState::End__ => {
                        self.state = DirectoryReqMerchantTypeSerializerState::Done__;
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
                    self.state = DirectoryReqMerchantTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SignatureTypeSerializer<'ser> {
        pub(super) value: &'ser super::SignatureType,
        pub(super) state: SignatureTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SignatureTypeSerializerState<'ser> {
        Init__,
        SignedInfo(<super::SignedInfoType as WithSerializer>::Serializer<'ser>),
        SignatureValue(<super::SignatureValueType as WithSerializer>::Serializer<'ser>),
        KeyInfo(IterSerializer<'ser, Option<super::KeyInfoType>, super::KeyInfoType>),
        Object(IterSerializer<'ser, Vec<super::ObjectType>, super::ObjectType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SignatureTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    SignatureTypeSerializerState::Init__ => {
                        self.state =
                            SignatureTypeSerializerState::SignedInfo(WithSerializer::serializer(
                                &self.value.signed_info,
                                Some("ds:SignedInfo"),
                                false,
                            )?);
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        write_attrib_opt(&mut bytes, "ds:Id", &self.value.id)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SignatureTypeSerializerState::SignedInfo(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = SignatureTypeSerializerState::SignatureValue(
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
                                self.state =
                                    SignatureTypeSerializerState::KeyInfo(IterSerializer::new(
                                        &self.value.key_info,
                                        Some("ds:KeyInfo"),
                                        false,
                                    ))
                            }
                        }
                    }
                    SignatureTypeSerializerState::KeyInfo(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = SignatureTypeSerializerState::Object(IterSerializer::new(
                                &self.value.object,
                                Some("ds:Object"),
                                false,
                            ))
                        }
                    },
                    SignatureTypeSerializerState::Object(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = SignatureTypeSerializerState::End__,
                    },
                    SignatureTypeSerializerState::End__ => {
                        self.state = SignatureTypeSerializerState::Done__;
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
                    self.state = SignatureTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SignedInfoTypeSerializer<'ser> {
        pub(super) value: &'ser super::SignedInfoType,
        pub(super) state: SignedInfoTypeSerializerState<'ser>,
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
        Reference(IterSerializer<'ser, Vec<super::ReferenceType>, super::ReferenceType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SignedInfoTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    SignedInfoTypeSerializerState::Init__ => {
                        self.state = SignedInfoTypeSerializerState::CanonicalizationMethod(
                            WithSerializer::serializer(
                                &self.value.canonicalization_method,
                                Some("ds:CanonicalizationMethod"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        write_attrib_opt(&mut bytes, "ds:Id", &self.value.id)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SignedInfoTypeSerializerState::CanonicalizationMethod(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                self.state = SignedInfoTypeSerializerState::SignatureMethod(
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
                                self.state =
                                    SignedInfoTypeSerializerState::Reference(IterSerializer::new(
                                        &self.value.reference,
                                        Some("ds:Reference"),
                                        false,
                                    ))
                            }
                        }
                    }
                    SignedInfoTypeSerializerState::Reference(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = SignedInfoTypeSerializerState::End__,
                    },
                    SignedInfoTypeSerializerState::End__ => {
                        self.state = SignedInfoTypeSerializerState::Done__;
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
                    self.state = SignedInfoTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SignatureValueTypeSerializer<'ser> {
        pub(super) value: &'ser super::SignatureValueType,
        pub(super) state: SignatureValueTypeSerializerState<'ser>,
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
                match &mut self.state {
                    SignatureValueTypeSerializerState::Init__ => {
                        self.state = SignatureValueTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        write_attrib_opt(&mut bytes, "ds:Id", &self.value.id)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SignatureValueTypeSerializerState::Content__(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => self.state = SignatureValueTypeSerializerState::End__,
                        }
                    }
                    SignatureValueTypeSerializerState::End__ => {
                        self.state = SignatureValueTypeSerializerState::Done__;
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
                    self.state = SignatureValueTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct KeyInfoTypeSerializer<'ser> {
        pub(super) value: &'ser super::KeyInfoType,
        pub(super) state: KeyInfoTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum KeyInfoTypeSerializerState<'ser> {
        Init__,
        Content__(IterSerializer<'ser, Vec<super::KeyInfoTypeContent>, super::KeyInfoTypeContent>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> KeyInfoTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    KeyInfoTypeSerializerState::Init__ => {
                        self.state = KeyInfoTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content,
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        write_attrib_opt(&mut bytes, "ds:Id", &self.value.id)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    KeyInfoTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = KeyInfoTypeSerializerState::End__,
                    },
                    KeyInfoTypeSerializerState::End__ => {
                        self.state = KeyInfoTypeSerializerState::Done__;
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
                    self.state = KeyInfoTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct KeyInfoTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::KeyInfoTypeContent,
        pub(super) state: KeyInfoTypeContentSerializerState<'ser>,
    }
    #[derive(Debug)]
    pub(super) enum KeyInfoTypeContentSerializerState<'ser> {
        Init__,
        KeyName(<String as WithSerializer>::Serializer<'ser>),
        KeyValue(<super::KeyValueType as WithSerializer>::Serializer<'ser>),
        RetrievalMethod(<super::RetrievalMethodType as WithSerializer>::Serializer<'ser>),
        X509Data(<super::X509DataType as WithSerializer>::Serializer<'ser>),
        Pgpdata(<super::PgpdataType as WithSerializer>::Serializer<'ser>),
        Spkidata(<super::SpkidataType as WithSerializer>::Serializer<'ser>),
        MgmtData(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> KeyInfoTypeContentSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    KeyInfoTypeContentSerializerState::Init__ => match self.value {
                        super::KeyInfoTypeContent::KeyName(x) => {
                            self.state = KeyInfoTypeContentSerializerState::KeyName(
                                WithSerializer::serializer(x, Some("ds:KeyName"), false)?,
                            )
                        }
                        super::KeyInfoTypeContent::KeyValue(x) => {
                            self.state = KeyInfoTypeContentSerializerState::KeyValue(
                                WithSerializer::serializer(x, Some("ds:KeyValue"), false)?,
                            )
                        }
                        super::KeyInfoTypeContent::RetrievalMethod(x) => {
                            self.state = KeyInfoTypeContentSerializerState::RetrievalMethod(
                                WithSerializer::serializer(x, Some("ds:RetrievalMethod"), false)?,
                            )
                        }
                        super::KeyInfoTypeContent::X509Data(x) => {
                            self.state = KeyInfoTypeContentSerializerState::X509Data(
                                WithSerializer::serializer(x, Some("ds:X509Data"), false)?,
                            )
                        }
                        super::KeyInfoTypeContent::Pgpdata(x) => {
                            self.state = KeyInfoTypeContentSerializerState::Pgpdata(
                                WithSerializer::serializer(x, Some("ds:PGPData"), false)?,
                            )
                        }
                        super::KeyInfoTypeContent::Spkidata(x) => {
                            self.state = KeyInfoTypeContentSerializerState::Spkidata(
                                WithSerializer::serializer(x, Some("ds:SPKIData"), false)?,
                            )
                        }
                        super::KeyInfoTypeContent::MgmtData(x) => {
                            self.state = KeyInfoTypeContentSerializerState::MgmtData(
                                WithSerializer::serializer(x, Some("ds:MgmtData"), false)?,
                            )
                        }
                    },
                    KeyInfoTypeContentSerializerState::KeyName(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = KeyInfoTypeContentSerializerState::Done__,
                    },
                    KeyInfoTypeContentSerializerState::KeyValue(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => self.state = KeyInfoTypeContentSerializerState::Done__,
                        }
                    }
                    KeyInfoTypeContentSerializerState::RetrievalMethod(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => self.state = KeyInfoTypeContentSerializerState::Done__,
                        }
                    }
                    KeyInfoTypeContentSerializerState::X509Data(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => self.state = KeyInfoTypeContentSerializerState::Done__,
                        }
                    }
                    KeyInfoTypeContentSerializerState::Pgpdata(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = KeyInfoTypeContentSerializerState::Done__,
                    },
                    KeyInfoTypeContentSerializerState::Spkidata(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => self.state = KeyInfoTypeContentSerializerState::Done__,
                        }
                    }
                    KeyInfoTypeContentSerializerState::MgmtData(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => self.state = KeyInfoTypeContentSerializerState::Done__,
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
                    self.state = KeyInfoTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ObjectTypeSerializer<'ser> {
        pub(super) value: &'ser super::ObjectType,
        pub(super) state: ObjectTypeSerializerState<'ser>,
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
                match &mut self.state {
                    ObjectTypeSerializerState::Init__ => {
                        self.state = ObjectTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
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
                    self.state = ObjectTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct CanonicalizationMethodTypeSerializer<'ser> {
        pub(super) value: &'ser super::CanonicalizationMethodType,
        pub(super) state: CanonicalizationMethodTypeSerializerState<'ser>,
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
                match &mut self.state {
                    CanonicalizationMethodTypeSerializerState::Init__ => {
                        self.state = CanonicalizationMethodTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
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
                    self.state = CanonicalizationMethodTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SignatureMethodTypeSerializer<'ser> {
        pub(super) value: &'ser super::SignatureMethodType,
        pub(super) state: SignatureMethodTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SignatureMethodTypeSerializerState<'ser> {
        Init__,
        HmacoutputLength(IterSerializer<'ser, Option<i32>, i32>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SignatureMethodTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    SignatureMethodTypeSerializerState::Init__ => {
                        self.state = SignatureMethodTypeSerializerState::HmacoutputLength(
                            IterSerializer::new(
                                &self.value.hmacoutput_length,
                                Some("ds:HMACOutputLength"),
                                false,
                            ),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        write_attrib(&mut bytes, "ds:Algorithm", &self.value.algorithm)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SignatureMethodTypeSerializerState::HmacoutputLength(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => self.state = SignatureMethodTypeSerializerState::End__,
                        }
                    }
                    SignatureMethodTypeSerializerState::End__ => {
                        self.state = SignatureMethodTypeSerializerState::Done__;
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
                    self.state = SignatureMethodTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ReferenceTypeSerializer<'ser> {
        pub(super) value: &'ser super::ReferenceType,
        pub(super) state: ReferenceTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ReferenceTypeSerializerState<'ser> {
        Init__,
        Transforms(IterSerializer<'ser, Option<super::TransformsType>, super::TransformsType>),
        DigestMethod(<super::DigestMethodType as WithSerializer>::Serializer<'ser>),
        DigestValue(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ReferenceTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    ReferenceTypeSerializerState::Init__ => {
                        self.state = ReferenceTypeSerializerState::Transforms(IterSerializer::new(
                            &self.value.transforms,
                            Some("ds:Transforms"),
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
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
                            self.state = ReferenceTypeSerializerState::DigestMethod(
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
                            self.state = ReferenceTypeSerializerState::DigestValue(
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
                        None => self.state = ReferenceTypeSerializerState::End__,
                    },
                    ReferenceTypeSerializerState::End__ => {
                        self.state = ReferenceTypeSerializerState::Done__;
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
                    self.state = ReferenceTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct KeyValueTypeSerializer<'ser> {
        pub(super) value: &'ser super::KeyValueType,
        pub(super) state: KeyValueTypeSerializerState<'ser>,
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
                match &mut self.state {
                    KeyValueTypeSerializerState::Init__ => {
                        self.state = KeyValueTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    KeyValueTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = KeyValueTypeSerializerState::End__,
                    },
                    KeyValueTypeSerializerState::End__ => {
                        self.state = KeyValueTypeSerializerState::Done__;
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
                    self.state = KeyValueTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct KeyValueTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::KeyValueTypeContent,
        pub(super) state: KeyValueTypeContentSerializerState<'ser>,
    }
    #[derive(Debug)]
    pub(super) enum KeyValueTypeContentSerializerState<'ser> {
        Init__,
        DsakeyValue(<super::DsakeyValueType as WithSerializer>::Serializer<'ser>),
        RsakeyValue(<super::RsakeyValueType as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> KeyValueTypeContentSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    KeyValueTypeContentSerializerState::Init__ => match self.value {
                        super::KeyValueTypeContent::DsakeyValue(x) => {
                            self.state = KeyValueTypeContentSerializerState::DsakeyValue(
                                WithSerializer::serializer(x, Some("ds:DSAKeyValue"), false)?,
                            )
                        }
                        super::KeyValueTypeContent::RsakeyValue(x) => {
                            self.state = KeyValueTypeContentSerializerState::RsakeyValue(
                                WithSerializer::serializer(x, Some("ds:RSAKeyValue"), false)?,
                            )
                        }
                    },
                    KeyValueTypeContentSerializerState::DsakeyValue(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => self.state = KeyValueTypeContentSerializerState::Done__,
                        }
                    }
                    KeyValueTypeContentSerializerState::RsakeyValue(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => self.state = KeyValueTypeContentSerializerState::Done__,
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
                    self.state = KeyValueTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct RetrievalMethodTypeSerializer<'ser> {
        pub(super) value: &'ser super::RetrievalMethodType,
        pub(super) state: RetrievalMethodTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum RetrievalMethodTypeSerializerState<'ser> {
        Init__,
        Transforms(IterSerializer<'ser, Option<super::TransformsType>, super::TransformsType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RetrievalMethodTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    RetrievalMethodTypeSerializerState::Init__ => {
                        self.state =
                            RetrievalMethodTypeSerializerState::Transforms(IterSerializer::new(
                                &self.value.transforms,
                                Some("ds:Transforms"),
                                false,
                            ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        write_attrib_opt(&mut bytes, "ds:URI", &self.value.uri)?;
                        write_attrib_opt(&mut bytes, "ds:Type", &self.value.type_)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    RetrievalMethodTypeSerializerState::Transforms(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => self.state = RetrievalMethodTypeSerializerState::End__,
                        }
                    }
                    RetrievalMethodTypeSerializerState::End__ => {
                        self.state = RetrievalMethodTypeSerializerState::Done__;
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
                    self.state = RetrievalMethodTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct X509DataTypeSerializer<'ser> {
        pub(super) value: &'ser super::X509DataType,
        pub(super) state: X509DataTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum X509DataTypeSerializerState<'ser> {
        Init__,
        Content__(
            IterSerializer<'ser, Vec<super::X509DataTypeContent>, super::X509DataTypeContent>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> X509DataTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    X509DataTypeSerializerState::Init__ => {
                        self.state = X509DataTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content,
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    X509DataTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = X509DataTypeSerializerState::End__,
                    },
                    X509DataTypeSerializerState::End__ => {
                        self.state = X509DataTypeSerializerState::Done__;
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
                    self.state = X509DataTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct X509DataTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::X509DataTypeContent,
        pub(super) state: X509DataTypeContentSerializerState<'ser>,
    }
    #[derive(Debug)]
    pub(super) enum X509DataTypeContentSerializerState<'ser> {
        Init__,
        Content103(<super::X509DataContent103Type as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> X509DataTypeContentSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    X509DataTypeContentSerializerState::Init__ => {
                        self.state = X509DataTypeContentSerializerState::Content103(
                            WithSerializer::serializer(
                                &self.value.content_103,
                                Some("ds:Content103"),
                                false,
                            )?,
                        );
                    }
                    X509DataTypeContentSerializerState::Content103(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => self.state = X509DataTypeContentSerializerState::Done__,
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
                    self.state = X509DataTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PgpdataTypeSerializer<'ser> {
        pub(super) value: &'ser super::PgpdataType,
        pub(super) state: PgpdataTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum PgpdataTypeSerializerState<'ser> {
        Init__,
        Content__(<super::PgpdataTypeContent as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PgpdataTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    PgpdataTypeSerializerState::Init__ => {
                        self.state = PgpdataTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    PgpdataTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = PgpdataTypeSerializerState::End__,
                    },
                    PgpdataTypeSerializerState::End__ => {
                        self.state = PgpdataTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    PgpdataTypeSerializerState::Done__ => return Ok(None),
                    PgpdataTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for PgpdataTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = PgpdataTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PgpdataTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::PgpdataTypeContent,
        pub(super) state: PgpdataTypeContentSerializerState<'ser>,
    }
    #[derive(Debug)]
    pub(super) enum PgpdataTypeContentSerializerState<'ser> {
        Init__,
        Content113(<super::PgpdataContent113Type as WithSerializer>::Serializer<'ser>),
        Content116(<super::PgpdataContent116Type as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PgpdataTypeContentSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    PgpdataTypeContentSerializerState::Init__ => match self.value {
                        super::PgpdataTypeContent::Content113(x) => {
                            self.state = PgpdataTypeContentSerializerState::Content113(
                                WithSerializer::serializer(x, Some("ds:Content113"), false)?,
                            )
                        }
                        super::PgpdataTypeContent::Content116(x) => {
                            self.state = PgpdataTypeContentSerializerState::Content116(
                                WithSerializer::serializer(x, Some("ds:Content116"), false)?,
                            )
                        }
                    },
                    PgpdataTypeContentSerializerState::Content113(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => self.state = PgpdataTypeContentSerializerState::Done__,
                        }
                    }
                    PgpdataTypeContentSerializerState::Content116(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => self.state = PgpdataTypeContentSerializerState::Done__,
                        }
                    }
                    PgpdataTypeContentSerializerState::Done__ => return Ok(None),
                    PgpdataTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for PgpdataTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = PgpdataTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SpkidataTypeSerializer<'ser> {
        pub(super) value: &'ser super::SpkidataType,
        pub(super) state: SpkidataTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SpkidataTypeSerializerState<'ser> {
        Init__,
        Content__(
            IterSerializer<'ser, Vec<super::SpkidataTypeContent>, super::SpkidataTypeContent>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SpkidataTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    SpkidataTypeSerializerState::Init__ => {
                        self.state = SpkidataTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content,
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SpkidataTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = SpkidataTypeSerializerState::End__,
                    },
                    SpkidataTypeSerializerState::End__ => {
                        self.state = SpkidataTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SpkidataTypeSerializerState::Done__ => return Ok(None),
                    SpkidataTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for SpkidataTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = SpkidataTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SpkidataTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::SpkidataTypeContent,
        pub(super) state: SpkidataTypeContentSerializerState<'ser>,
    }
    #[derive(Debug)]
    pub(super) enum SpkidataTypeContentSerializerState<'ser> {
        Init__,
        Spkisexp(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SpkidataTypeContentSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    SpkidataTypeContentSerializerState::Init__ => {
                        self.state = SpkidataTypeContentSerializerState::Spkisexp(
                            WithSerializer::serializer(
                                &self.value.spkisexp,
                                Some("ds:SPKISexp"),
                                false,
                            )?,
                        );
                    }
                    SpkidataTypeContentSerializerState::Spkisexp(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => self.state = SpkidataTypeContentSerializerState::Done__,
                        }
                    }
                    SpkidataTypeContentSerializerState::Done__ => return Ok(None),
                    SpkidataTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for SpkidataTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = SpkidataTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TransformsTypeSerializer<'ser> {
        pub(super) value: &'ser super::TransformsType,
        pub(super) state: TransformsTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TransformsTypeSerializerState<'ser> {
        Init__,
        Transform(IterSerializer<'ser, Vec<super::TransformType>, super::TransformType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TransformsTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    TransformsTypeSerializerState::Init__ => {
                        self.state = TransformsTypeSerializerState::Transform(IterSerializer::new(
                            &self.value.transform,
                            Some("ds:Transform"),
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TransformsTypeSerializerState::Transform(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = TransformsTypeSerializerState::End__,
                    },
                    TransformsTypeSerializerState::End__ => {
                        self.state = TransformsTypeSerializerState::Done__;
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
                    self.state = TransformsTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DigestMethodTypeSerializer<'ser> {
        pub(super) value: &'ser super::DigestMethodType,
        pub(super) state: DigestMethodTypeSerializerState<'ser>,
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
                match &mut self.state {
                    DigestMethodTypeSerializerState::Init__ => {
                        self.state = DigestMethodTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
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
                    self.state = DigestMethodTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DsakeyValueTypeSerializer<'ser> {
        pub(super) value: &'ser super::DsakeyValueType,
        pub(super) state: DsakeyValueTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DsakeyValueTypeSerializerState<'ser> {
        Init__,
        Content125(
            IterSerializer<
                'ser,
                Option<super::DsakeyValueContent125Type>,
                super::DsakeyValueContent125Type,
            >,
        ),
        G(IterSerializer<'ser, Option<String>, String>),
        Y(<String as WithSerializer>::Serializer<'ser>),
        J(IterSerializer<'ser, Option<String>, String>),
        Content131(
            IterSerializer<
                'ser,
                Option<super::DsakeyValueContent131Type>,
                super::DsakeyValueContent131Type,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DsakeyValueTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    DsakeyValueTypeSerializerState::Init__ => {
                        self.state =
                            DsakeyValueTypeSerializerState::Content125(IterSerializer::new(
                                &self.value.content_125,
                                Some("ds:Content125"),
                                false,
                            ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DsakeyValueTypeSerializerState::Content125(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = DsakeyValueTypeSerializerState::G(IterSerializer::new(
                                &self.value.g,
                                Some("ds:G"),
                                false,
                            ))
                        }
                    },
                    DsakeyValueTypeSerializerState::G(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = DsakeyValueTypeSerializerState::Y(
                                WithSerializer::serializer(&self.value.y, Some("ds:Y"), false)?,
                            )
                        }
                    },
                    DsakeyValueTypeSerializerState::Y(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = DsakeyValueTypeSerializerState::J(IterSerializer::new(
                                &self.value.j,
                                Some("ds:J"),
                                false,
                            ))
                        }
                    },
                    DsakeyValueTypeSerializerState::J(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state =
                                DsakeyValueTypeSerializerState::Content131(IterSerializer::new(
                                    &self.value.content_131,
                                    Some("ds:Content131"),
                                    false,
                                ))
                        }
                    },
                    DsakeyValueTypeSerializerState::Content131(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = DsakeyValueTypeSerializerState::End__,
                    },
                    DsakeyValueTypeSerializerState::End__ => {
                        self.state = DsakeyValueTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DsakeyValueTypeSerializerState::Done__ => return Ok(None),
                    DsakeyValueTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DsakeyValueTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = DsakeyValueTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct RsakeyValueTypeSerializer<'ser> {
        pub(super) value: &'ser super::RsakeyValueType,
        pub(super) state: RsakeyValueTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum RsakeyValueTypeSerializerState<'ser> {
        Init__,
        Modulus(<String as WithSerializer>::Serializer<'ser>),
        Exponent(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RsakeyValueTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    RsakeyValueTypeSerializerState::Init__ => {
                        self.state =
                            RsakeyValueTypeSerializerState::Modulus(WithSerializer::serializer(
                                &self.value.modulus,
                                Some("ds:Modulus"),
                                false,
                            )?);
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    RsakeyValueTypeSerializerState::Modulus(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = RsakeyValueTypeSerializerState::Exponent(
                                WithSerializer::serializer(
                                    &self.value.exponent,
                                    Some("ds:Exponent"),
                                    false,
                                )?,
                            )
                        }
                    },
                    RsakeyValueTypeSerializerState::Exponent(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = RsakeyValueTypeSerializerState::End__,
                    },
                    RsakeyValueTypeSerializerState::End__ => {
                        self.state = RsakeyValueTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    RsakeyValueTypeSerializerState::Done__ => return Ok(None),
                    RsakeyValueTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for RsakeyValueTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = RsakeyValueTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct X509DataContent103TypeSerializer<'ser> {
        pub(super) value: &'ser super::X509DataContent103Type,
        pub(super) state: X509DataContent103TypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    pub(super) enum X509DataContent103TypeSerializerState<'ser> {
        Init__,
        Content__(<super::X509DataContent103TypeContent as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> X509DataContent103TypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    X509DataContent103TypeSerializerState::Init__ => {
                        self.state = X509DataContent103TypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                    }
                    X509DataContent103TypeSerializerState::Content__(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => self.state = X509DataContent103TypeSerializerState::Done__,
                        }
                    }
                    X509DataContent103TypeSerializerState::Done__ => return Ok(None),
                    X509DataContent103TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for X509DataContent103TypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = X509DataContent103TypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct X509DataContent103TypeContentSerializer<'ser> {
        pub(super) value: &'ser super::X509DataContent103TypeContent,
        pub(super) state: X509DataContent103TypeContentSerializerState<'ser>,
    }
    #[derive(Debug)]
    pub(super) enum X509DataContent103TypeContentSerializerState<'ser> {
        Init__,
        X509IssuerSerial(<super::X509IssuerSerialType as WithSerializer>::Serializer<'ser>),
        X509Ski(<String as WithSerializer>::Serializer<'ser>),
        X509SubjectName(<String as WithSerializer>::Serializer<'ser>),
        X509Certificate(<String as WithSerializer>::Serializer<'ser>),
        X509Crl(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> X509DataContent103TypeContentSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    X509DataContent103TypeContentSerializerState::Init__ => match self.value {
                        super::X509DataContent103TypeContent::X509IssuerSerial(x) => {
                            self.state =
                                X509DataContent103TypeContentSerializerState::X509IssuerSerial(
                                    WithSerializer::serializer(
                                        x,
                                        Some("ds:X509IssuerSerial"),
                                        false,
                                    )?,
                                )
                        }
                        super::X509DataContent103TypeContent::X509Ski(x) => {
                            self.state = X509DataContent103TypeContentSerializerState::X509Ski(
                                WithSerializer::serializer(x, Some("ds:X509SKI"), false)?,
                            )
                        }
                        super::X509DataContent103TypeContent::X509SubjectName(x) => {
                            self.state =
                                X509DataContent103TypeContentSerializerState::X509SubjectName(
                                    WithSerializer::serializer(
                                        x,
                                        Some("ds:X509SubjectName"),
                                        false,
                                    )?,
                                )
                        }
                        super::X509DataContent103TypeContent::X509Certificate(x) => {
                            self.state =
                                X509DataContent103TypeContentSerializerState::X509Certificate(
                                    WithSerializer::serializer(
                                        x,
                                        Some("ds:X509Certificate"),
                                        false,
                                    )?,
                                )
                        }
                        super::X509DataContent103TypeContent::X509Crl(x) => {
                            self.state = X509DataContent103TypeContentSerializerState::X509Crl(
                                WithSerializer::serializer(x, Some("ds:X509CRL"), false)?,
                            )
                        }
                    },
                    X509DataContent103TypeContentSerializerState::X509IssuerSerial(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                self.state = X509DataContent103TypeContentSerializerState::Done__
                            }
                        }
                    }
                    X509DataContent103TypeContentSerializerState::X509Ski(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                self.state = X509DataContent103TypeContentSerializerState::Done__
                            }
                        }
                    }
                    X509DataContent103TypeContentSerializerState::X509SubjectName(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                self.state = X509DataContent103TypeContentSerializerState::Done__
                            }
                        }
                    }
                    X509DataContent103TypeContentSerializerState::X509Certificate(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                self.state = X509DataContent103TypeContentSerializerState::Done__
                            }
                        }
                    }
                    X509DataContent103TypeContentSerializerState::X509Crl(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                self.state = X509DataContent103TypeContentSerializerState::Done__
                            }
                        }
                    }
                    X509DataContent103TypeContentSerializerState::Done__ => return Ok(None),
                    X509DataContent103TypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for X509DataContent103TypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = X509DataContent103TypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PgpdataContent113TypeSerializer<'ser> {
        pub(super) value: &'ser super::PgpdataContent113Type,
        pub(super) state: PgpdataContent113TypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    pub(super) enum PgpdataContent113TypeSerializerState<'ser> {
        Init__,
        PgpkeyID(<String as WithSerializer>::Serializer<'ser>),
        PgpkeyPacket(IterSerializer<'ser, Option<String>, String>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PgpdataContent113TypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    PgpdataContent113TypeSerializerState::Init__ => {
                        self.state = PgpdataContent113TypeSerializerState::PgpkeyID(
                            WithSerializer::serializer(
                                &self.value.pgpkey_id,
                                Some("ds:PGPKeyID"),
                                false,
                            )?,
                        );
                    }
                    PgpdataContent113TypeSerializerState::PgpkeyID(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                self.state = PgpdataContent113TypeSerializerState::PgpkeyPacket(
                                    IterSerializer::new(
                                        &self.value.pgpkey_packet,
                                        Some("ds:PGPKeyPacket"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    PgpdataContent113TypeSerializerState::PgpkeyPacket(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => self.state = PgpdataContent113TypeSerializerState::Done__,
                        }
                    }
                    PgpdataContent113TypeSerializerState::Done__ => return Ok(None),
                    PgpdataContent113TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for PgpdataContent113TypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = PgpdataContent113TypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PgpdataContent116TypeSerializer<'ser> {
        pub(super) value: &'ser super::PgpdataContent116Type,
        pub(super) state: PgpdataContent116TypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    pub(super) enum PgpdataContent116TypeSerializerState<'ser> {
        Init__,
        PgpkeyPacket(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PgpdataContent116TypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    PgpdataContent116TypeSerializerState::Init__ => {
                        self.state = PgpdataContent116TypeSerializerState::PgpkeyPacket(
                            WithSerializer::serializer(
                                &self.value.pgpkey_packet,
                                Some("ds:PGPKeyPacket"),
                                false,
                            )?,
                        );
                    }
                    PgpdataContent116TypeSerializerState::PgpkeyPacket(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => self.state = PgpdataContent116TypeSerializerState::Done__,
                        }
                    }
                    PgpdataContent116TypeSerializerState::Done__ => return Ok(None),
                    PgpdataContent116TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for PgpdataContent116TypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = PgpdataContent116TypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TransformTypeSerializer<'ser> {
        pub(super) value: &'ser super::TransformType,
        pub(super) state: TransformTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TransformTypeSerializerState<'ser> {
        Init__,
        Content__(
            IterSerializer<'ser, Vec<super::TransformTypeContent>, super::TransformTypeContent>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TransformTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    TransformTypeSerializerState::Init__ => {
                        self.state = TransformTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content,
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        write_attrib(&mut bytes, "ds:Algorithm", &self.value.algorithm)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TransformTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = TransformTypeSerializerState::End__,
                    },
                    TransformTypeSerializerState::End__ => {
                        self.state = TransformTypeSerializerState::Done__;
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
                    self.state = TransformTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TransformTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::TransformTypeContent,
        pub(super) state: TransformTypeContentSerializerState<'ser>,
    }
    #[derive(Debug)]
    pub(super) enum TransformTypeContentSerializerState<'ser> {
        Init__,
        Xpath(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TransformTypeContentSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    TransformTypeContentSerializerState::Init__ => match self.value {
                        super::TransformTypeContent::Xpath(x) => {
                            self.state = TransformTypeContentSerializerState::Xpath(
                                WithSerializer::serializer(x, Some("ds:XPath"), false)?,
                            )
                        }
                    },
                    TransformTypeContentSerializerState::Xpath(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = TransformTypeContentSerializerState::Done__,
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
                    self.state = TransformTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DsakeyValueContent125TypeSerializer<'ser> {
        pub(super) value: &'ser super::DsakeyValueContent125Type,
        pub(super) state: DsakeyValueContent125TypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    pub(super) enum DsakeyValueContent125TypeSerializerState<'ser> {
        Init__,
        P(<String as WithSerializer>::Serializer<'ser>),
        Q(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DsakeyValueContent125TypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    DsakeyValueContent125TypeSerializerState::Init__ => {
                        self.state = DsakeyValueContent125TypeSerializerState::P(
                            WithSerializer::serializer(&self.value.p, Some("ds:P"), false)?,
                        );
                    }
                    DsakeyValueContent125TypeSerializerState::P(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = DsakeyValueContent125TypeSerializerState::Q(
                                WithSerializer::serializer(&self.value.q, Some("ds:Q"), false)?,
                            )
                        }
                    },
                    DsakeyValueContent125TypeSerializerState::Q(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = DsakeyValueContent125TypeSerializerState::Done__,
                    },
                    DsakeyValueContent125TypeSerializerState::Done__ => return Ok(None),
                    DsakeyValueContent125TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DsakeyValueContent125TypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = DsakeyValueContent125TypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DsakeyValueContent131TypeSerializer<'ser> {
        pub(super) value: &'ser super::DsakeyValueContent131Type,
        pub(super) state: DsakeyValueContent131TypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    pub(super) enum DsakeyValueContent131TypeSerializerState<'ser> {
        Init__,
        Seed(<String as WithSerializer>::Serializer<'ser>),
        PgenCounter(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DsakeyValueContent131TypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    DsakeyValueContent131TypeSerializerState::Init__ => {
                        self.state = DsakeyValueContent131TypeSerializerState::Seed(
                            WithSerializer::serializer(&self.value.seed, Some("ds:Seed"), false)?,
                        );
                    }
                    DsakeyValueContent131TypeSerializerState::Seed(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                self.state = DsakeyValueContent131TypeSerializerState::PgenCounter(
                                    WithSerializer::serializer(
                                        &self.value.pgen_counter,
                                        Some("ds:PgenCounter"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    DsakeyValueContent131TypeSerializerState::PgenCounter(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => self.state = DsakeyValueContent131TypeSerializerState::Done__,
                        }
                    }
                    DsakeyValueContent131TypeSerializerState::Done__ => return Ok(None),
                    DsakeyValueContent131TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DsakeyValueContent131TypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = DsakeyValueContent131TypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct X509IssuerSerialTypeSerializer<'ser> {
        pub(super) value: &'ser super::X509IssuerSerialType,
        pub(super) state: X509IssuerSerialTypeSerializerState<'ser>,
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
                match &mut self.state {
                    X509IssuerSerialTypeSerializerState::Init__ => {
                        self.state = X509IssuerSerialTypeSerializerState::X509IssuerName(
                            WithSerializer::serializer(
                                &self.value.x509_issuer_name,
                                Some("ds:X509IssuerName"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:ds"[..], &super::NS_DS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    X509IssuerSerialTypeSerializerState::X509IssuerName(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                self.state = X509IssuerSerialTypeSerializerState::X509SerialNumber(
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
                            None => self.state = X509IssuerSerialTypeSerializerState::End__,
                        }
                    }
                    X509IssuerSerialTypeSerializerState::End__ => {
                        self.state = X509IssuerSerialTypeSerializerState::Done__;
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
                    self.state = X509IssuerSerialTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        deserialize_new::{
            ContentDeserializer, DeserializeReader, Deserializer, DeserializerArtifact,
            DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput,
            WithDeserializer,
        },
        filter_xmlns_attributes, BytesStart, Error, ErrorKind, Event, RawByteStr,
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
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DEFAULT),
                    Some(b"version")
                ) {
                    reader.read_attrib(&mut version, b"version", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
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
        MerchantID(Option<<String as WithDeserializer>::Deserializer>),
        SubID(Option<<usize as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DirectoryReqMerchantTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
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
                S::MerchantID(Some(deserializer)) => {
                    self.store_merchant_id(deserializer.finish(reader)?)?
                }
                S::SubID(Some(deserializer)) => self.store_sub_id(deserializer.finish(reader)?)?,
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
                        .get_or_insert(DirectoryReqMerchantTypeDeserializerState::MerchantID(None));
                    *self.state = DirectoryReqMerchantTypeDeserializerState::SubID(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DirectoryReqMerchantTypeDeserializerState::MerchantID(None);
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
                    *self.state = DirectoryReqMerchantTypeDeserializerState::SubID(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                DirectoryReqMerchantTypeDeserializerState::MerchantID(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = DirectoryReqMerchantTypeDeserializerState::SubID(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DirectoryReqMerchantTypeDeserializerState::MerchantID(
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
                    fallback.get_or_insert(DirectoryReqMerchantTypeDeserializerState::SubID(None));
                    *self.state = DirectoryReqMerchantTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DirectoryReqMerchantTypeDeserializerState::SubID(None);
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
                                DirectoryReqMerchantTypeDeserializerState::SubID(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = DirectoryReqMerchantTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DirectoryReqMerchantTypeDeserializerState::SubID(Some(
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
                    (S::MerchantID(Some(deserializer)), event) => {
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
                    (S::SubID(Some(deserializer)), event) => {
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
                        *self.state = DirectoryReqMerchantTypeDeserializerState::MerchantID(None);
                        event
                    }
                    (S::MerchantID(None), event @ (Event::Start(_) | Event::Empty(_))) => {
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
                            *self.state = S::SubID(None);
                            event
                        }
                    }
                    (S::SubID(None), event @ (Event::Start(_) | Event::Empty(_))) => {
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
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Id")
                ) {
                    reader.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
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
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Id")
                ) {
                    reader.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
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
                            allow_any_element = true;
                            fallback.get_or_insert(S::SignatureMethod(None));
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
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Id")
                ) {
                    reader.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
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
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Id")
                ) {
                    reader.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
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
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(KeyInfoTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = KeyInfoTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = KeyInfoTypeDeserializerState::Content__(deserializer);
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
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output =
                            <super::KeyInfoTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
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
    pub enum KeyInfoTypeContentDeserializer {
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
        Pgpdata(
            Option<super::PgpdataType>,
            Option<<super::PgpdataType as WithDeserializer>::Deserializer>,
        ),
        Spkidata(
            Option<super::SpkidataType>,
            Option<<super::SpkidataType as WithDeserializer>::Deserializer>,
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
            fallback: &mut Option<KeyInfoTypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, true));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_DS),
                Some(b"KeyName")
            ) {
                let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_key_name(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_DS),
                Some(b"KeyValue")
            ) {
                let output =
                    <super::KeyValueType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_key_value(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_DS),
                Some(b"RetrievalMethod")
            ) {
                let output = <super::RetrievalMethodType as WithDeserializer>::Deserializer::init(
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
                let output =
                    <super::X509DataType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_x509_data(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_DS),
                Some(b"PGPData")
            ) {
                let output =
                    <super::PgpdataType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_pgpdata(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_DS),
                Some(b"SPKIData")
            ) {
                let output =
                    <super::SpkidataType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_spkidata(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_DS),
                Some(b"MgmtData")
            ) {
                let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_mgmt_data(reader, Default::default(), output, &mut *fallback);
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, true))
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
        fn store_pgpdata(
            values: &mut Option<super::PgpdataType>,
            value: super::PgpdataType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PGPData",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_spkidata(
            values: &mut Option<super::SpkidataType>,
            value: super::SpkidataType,
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
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::KeyName(_, Some(deserializer))) => {
                        Self::KeyName(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::KeyName(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_key_name(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_key_name(&mut values, data)?;
                    let data = Self::KeyName(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::KeyName(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_key_value<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::KeyValueType>,
            output: DeserializerOutput<'de, super::KeyValueType>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::KeyValue(_, Some(deserializer))) => {
                        Self::KeyValue(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::KeyValue(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_key_value(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_key_value(&mut values, data)?;
                    let data = Self::KeyValue(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::KeyValue(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_retrieval_method<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::RetrievalMethodType>,
            output: DeserializerOutput<'de, super::RetrievalMethodType>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::RetrievalMethod(_, Some(deserializer))) => {
                        Self::RetrievalMethod(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::RetrievalMethod(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_retrieval_method(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_retrieval_method(&mut values, data)?;
                    let data = Self::RetrievalMethod(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::RetrievalMethod(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_x509_data<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::X509DataType>,
            output: DeserializerOutput<'de, super::X509DataType>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::X509Data(_, Some(deserializer))) => {
                        Self::X509Data(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::X509Data(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_x509_data(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x509_data(&mut values, data)?;
                    let data = Self::X509Data(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::X509Data(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_pgpdata<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PgpdataType>,
            output: DeserializerOutput<'de, super::PgpdataType>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Pgpdata(_, Some(deserializer))) => {
                        Self::Pgpdata(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Pgpdata(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_pgpdata(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pgpdata(&mut values, data)?;
                    let data = Self::Pgpdata(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Pgpdata(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_spkidata<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SpkidataType>,
            output: DeserializerOutput<'de, super::SpkidataType>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Spkidata(_, Some(deserializer))) => {
                        Self::Spkidata(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Spkidata(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_spkidata(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_spkidata(&mut values, data)?;
                    let data = Self::Spkidata(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Spkidata(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_mgmt_data<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::MgmtData(_, Some(deserializer))) => {
                        Self::MgmtData(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::MgmtData(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_mgmt_data(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_mgmt_data(&mut values, data)?;
                    let data = Self::MgmtData(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::MgmtData(values, Some(deserializer));
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
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
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
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::KeyName(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_key_name(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::KeyValue(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_key_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::RetrievalMethod(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_retrieval_method(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::X509Data(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_x509_data(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Pgpdata(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pgpdata(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Spkidata(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_spkidata(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::MgmtData(values, Some(deserializer)), event) => {
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
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::KeyName(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_key_name(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::KeyValue(values, None), event) => {
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
                    (Self::RetrievalMethod(values, None), event) => {
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
                    (Self::X509Data(values, None), event) => {
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
                    (Self::Pgpdata(values, None), event) => {
                        let output = <super::PgpdataType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_pgpdata(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Spkidata(values, None), event) => {
                        let output = <super::SpkidataType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_spkidata(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::MgmtData(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_mgmt_data(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
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
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::KeyName(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_key_name(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::KeyName(values.ok_or_else(
                        || ErrorKind::MissingElement("KeyName".into()),
                    )?))
                }
                Self::KeyValue(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_key_value(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::KeyValue(values.ok_or_else(
                        || ErrorKind::MissingElement("KeyValue".into()),
                    )?))
                }
                Self::RetrievalMethod(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_retrieval_method(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::RetrievalMethod(
                        values
                            .ok_or_else(|| ErrorKind::MissingElement("RetrievalMethod".into()))?,
                    ))
                }
                Self::X509Data(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_x509_data(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::X509Data(values.ok_or_else(
                        || ErrorKind::MissingElement("X509Data".into()),
                    )?))
                }
                Self::Pgpdata(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_pgpdata(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::Pgpdata(values.ok_or_else(
                        || ErrorKind::MissingElement("PGPData".into()),
                    )?))
                }
                Self::Spkidata(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_spkidata(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::Spkidata(values.ok_or_else(
                        || ErrorKind::MissingElement("SPKIData".into()),
                    )?))
                }
                Self::MgmtData(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_mgmt_data(&mut values, value)?;
                    }
                    Ok(super::KeyInfoTypeContent::MgmtData(values.ok_or_else(
                        || ErrorKind::MissingElement("MgmtData".into()),
                    )?))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
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
            for attrib in filter_xmlns_attributes(&bytes_start) {
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
                    reader.raise_unexpected_attrib(attrib)?;
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
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Algorithm")
                ) {
                    reader.read_attrib(&mut algorithm, b"Algorithm", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
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
        hmacoutput_length: Option<i32>,
        state: Box<SignatureMethodTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SignatureMethodTypeDeserializerState {
        Init__,
        HmacoutputLength(Option<<i32 as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SignatureMethodTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut algorithm: Option<String> = None;
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Algorithm")
                ) {
                    reader.read_attrib(&mut algorithm, b"Algorithm", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
            Ok(Self {
                algorithm: algorithm.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("Algorithm".into()))
                })?,
                hmacoutput_length: None,
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
                S::HmacoutputLength(Some(deserializer)) => {
                    self.store_hmacoutput_length(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_hmacoutput_length(&mut self, value: i32) -> Result<(), Error> {
            if self.hmacoutput_length.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"HMACOutputLength",
                )))?;
            }
            self.hmacoutput_length = Some(value);
            Ok(())
        }
        fn handle_hmacoutput_length<'de, R>(
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
                    .get_or_insert(SignatureMethodTypeDeserializerState::HmacoutputLength(None));
                *self.state = SignatureMethodTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_hmacoutput_length(data)?;
                    *self.state = SignatureMethodTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                SignatureMethodTypeDeserializerState::HmacoutputLength(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = SignatureMethodTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = SignatureMethodTypeDeserializerState::HmacoutputLength(
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
                    (S::HmacoutputLength(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_hmacoutput_length(reader, output, &mut fallback)? {
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
                        *self.state = SignatureMethodTypeDeserializerState::HmacoutputLength(None);
                        event
                    }
                    (S::HmacoutputLength(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_DS),
                            b"HMACOutputLength",
                        ) {
                            let output =
                                <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_hmacoutput_length(reader, output, &mut fallback)? {
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
                            fallback.get_or_insert(S::HmacoutputLength(None));
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
                hmacoutput_length: self.hmacoutput_length,
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
            for attrib in filter_xmlns_attributes(&bytes_start) {
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
                    reader.raise_unexpected_attrib(attrib)?;
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
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
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
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(KeyValueTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = KeyValueTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = KeyValueTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
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
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output =
                            <super::KeyValueTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
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
    pub enum KeyValueTypeContentDeserializer {
        Init__,
        DsakeyValue(
            Option<super::DsakeyValueType>,
            Option<<super::DsakeyValueType as WithDeserializer>::Deserializer>,
        ),
        RsakeyValue(
            Option<super::RsakeyValueType>,
            Option<<super::RsakeyValueType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::KeyValueTypeContent),
        Unknown__,
    }
    impl KeyValueTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<KeyValueTypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, true));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_DS),
                Some(b"DSAKeyValue")
            ) {
                let output = <super::DsakeyValueType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_dsakey_value(
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
                let output = <super::RsakeyValueType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_rsakey_value(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, true))
        }
        fn store_dsakey_value(
            values: &mut Option<super::DsakeyValueType>,
            value: super::DsakeyValueType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DSAKeyValue",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_rsakey_value(
            values: &mut Option<super::RsakeyValueType>,
            value: super::RsakeyValueType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"RSAKeyValue",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_dsakey_value<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::DsakeyValueType>,
            output: DeserializerOutput<'de, super::DsakeyValueType>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::DsakeyValue(_, Some(deserializer))) => {
                        Self::DsakeyValue(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::DsakeyValue(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_dsakey_value(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_dsakey_value(&mut values, data)?;
                    let data = Self::DsakeyValue(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::DsakeyValue(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_rsakey_value<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::RsakeyValueType>,
            output: DeserializerOutput<'de, super::RsakeyValueType>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::RsakeyValue(_, Some(deserializer))) => {
                        Self::RsakeyValue(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::RsakeyValue(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_rsakey_value(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_rsakey_value(&mut values, data)?;
                    let data = Self::RsakeyValue(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::RsakeyValue(values, Some(deserializer));
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
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
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
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::DsakeyValue(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_dsakey_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::RsakeyValue(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_rsakey_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::DsakeyValue(values, None), event) => {
                        let output =
                            <super::DsakeyValueType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_dsakey_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::RsakeyValue(values, None), event) => {
                        let output =
                            <super::RsakeyValueType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_rsakey_value(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
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
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::DsakeyValue(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_dsakey_value(&mut values, value)?;
                    }
                    Ok(super::KeyValueTypeContent::DsakeyValue(values.ok_or_else(
                        || ErrorKind::MissingElement("DSAKeyValue".into()),
                    )?))
                }
                Self::RsakeyValue(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_rsakey_value(&mut values, value)?;
                    }
                    Ok(super::KeyValueTypeContent::RsakeyValue(values.ok_or_else(
                        || ErrorKind::MissingElement("RSAKeyValue".into()),
                    )?))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
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
            for attrib in filter_xmlns_attributes(&bytes_start) {
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
                    reader.raise_unexpected_attrib(attrib)?;
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
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
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
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(X509DataTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = X509DataTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = X509DataTypeDeserializerState::Content__(deserializer);
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
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output =
                            <super::X509DataTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
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
        content_103: Option<super::X509DataContent103Type>,
        state: Box<X509DataTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    enum X509DataTypeContentDeserializerState {
        Init__,
        Content103(Option<<super::X509DataContent103Type as WithDeserializer>::Deserializer>),
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
                S::Content103(Some(deserializer)) => {
                    self.store_content_103(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_content_103(&mut self, value: super::X509DataContent103Type) -> Result<(), Error> {
            if self.content_103.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Content103",
                )))?;
            }
            self.content_103 = Some(value);
            Ok(())
        }
        fn handle_content_103<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::X509DataContent103Type>,
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
                if self.content_103.is_some() {
                    fallback.get_or_insert(X509DataTypeContentDeserializerState::Content103(None));
                    *self.state = X509DataTypeContentDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = X509DataTypeContentDeserializerState::Content103(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content_103(data)?;
                    *self.state = X509DataTypeContentDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                X509DataTypeContentDeserializerState::Content103(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = X509DataTypeContentDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = X509DataTypeContentDeserializerState::Content103(Some(
                                deserializer,
                            ));
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
                content_103: None,
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
                    (S::Content103(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content_103(reader, output, &mut fallback)? {
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
                        *self.state = X509DataTypeContentDeserializerState::Content103(None);
                        event
                    }
                    (S::Content103(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = < super :: X509DataContent103Type as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content_103(reader, output, &mut fallback)? {
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
                content_103: self
                    .content_103
                    .ok_or_else(|| ErrorKind::MissingElement("Content103".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PgpdataTypeDeserializer {
        content: Option<super::PgpdataTypeContent>,
        state: Box<PgpdataTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PgpdataTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::PgpdataTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl PgpdataTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(PgpdataTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: PgpdataTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let PgpdataTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::PgpdataTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::PgpdataTypeContent>,
            fallback: &mut Option<PgpdataTypeDeserializerState>,
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
                    .unwrap_or(PgpdataTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = PgpdataTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(PgpdataTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = PgpdataTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = PgpdataTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::PgpdataType> for PgpdataTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::PgpdataType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PgpdataType>
        where
            R: DeserializeReader,
        {
            use PgpdataTypeDeserializerState as S;
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
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output =
                            <super::PgpdataTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::PgpdataType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, PgpdataTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::PgpdataType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub enum PgpdataTypeContentDeserializer {
        Init__,
        Content113(
            Option<super::PgpdataContent113Type>,
            Option<<super::PgpdataContent113Type as WithDeserializer>::Deserializer>,
        ),
        Content116(
            Option<super::PgpdataContent116Type>,
            Option<<super::PgpdataContent116Type as WithDeserializer>::Deserializer>,
        ),
        Done__(super::PgpdataTypeContent),
        Unknown__,
    }
    impl PgpdataTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<PgpdataTypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(_) | Event::Empty(_)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            let mut allow_any_element = false;
            let event = {
                let output =
                    <super::PgpdataContent113Type as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                match self.handle_content_113(reader, Default::default(), output, &mut *fallback)? {
                    ElementHandlerOutput::Continue { event, allow_any } => {
                        allow_any_element = allow_any_element || allow_any;
                        event
                    }
                    output => {
                        return Ok(output);
                    }
                }
            };
            let event = {
                let output =
                    <super::PgpdataContent116Type as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                match self.handle_content_116(reader, Default::default(), output, &mut *fallback)? {
                    ElementHandlerOutput::Continue { event, allow_any } => {
                        allow_any_element = allow_any_element || allow_any;
                        event
                    }
                    output => {
                        return Ok(output);
                    }
                }
            };
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
        }
        fn store_content_113(
            values: &mut Option<super::PgpdataContent113Type>,
            value: super::PgpdataContent113Type,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Content113",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_content_116(
            values: &mut Option<super::PgpdataContent116Type>,
            value: super::PgpdataContent116Type,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Content116",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_content_113<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PgpdataContent113Type>,
            output: DeserializerOutput<'de, super::PgpdataContent113Type>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Content113(_, Some(deserializer))) => {
                        Self::Content113(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Content113(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_content_113(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_content_113(&mut values, data)?;
                    let data = Self::Content113(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Content113(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_content_116<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::PgpdataContent116Type>,
            output: DeserializerOutput<'de, super::PgpdataContent116Type>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Content116(_, Some(deserializer))) => {
                        Self::Content116(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Content116(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_content_116(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_content_116(&mut values, data)?;
                    let data = Self::Content116(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Content116(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::PgpdataTypeContent> for PgpdataTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PgpdataTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PgpdataTypeContent>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::Content113(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content_113(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Content116(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content_116(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Content113(values, None), event) => {
                        let output =
                            <super::PgpdataContent113Type as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content_113(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Content116(values, None), event) => {
                        let output =
                            <super::PgpdataContent116Type as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content_116(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::PgpdataTypeContent, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::Content113(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_content_113(&mut values, value)?;
                    }
                    Ok(super::PgpdataTypeContent::Content113(values.ok_or_else(
                        || ErrorKind::MissingElement("Content113".into()),
                    )?))
                }
                Self::Content116(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_content_116(&mut values, value)?;
                    }
                    Ok(super::PgpdataTypeContent::Content116(values.ok_or_else(
                        || ErrorKind::MissingElement("Content116".into()),
                    )?))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
        }
    }
    #[derive(Debug)]
    pub struct SpkidataTypeDeserializer {
        content: Vec<super::SpkidataTypeContent>,
        state: Box<SpkidataTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SpkidataTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::SpkidataTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl SpkidataTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                content: Vec::new(),
                state: Box::new(SpkidataTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SpkidataTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let SpkidataTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::SpkidataTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SpkidataTypeContent>,
            fallback: &mut Option<SpkidataTypeDeserializerState>,
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
                    .unwrap_or(SpkidataTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = SpkidataTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SpkidataTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = SpkidataTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = SpkidataTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SpkidataType> for SpkidataTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::SpkidataType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SpkidataType>
        where
            R: DeserializeReader,
        {
            use SpkidataTypeDeserializerState as S;
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
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output =
                            <super::SpkidataTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::SpkidataType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, SpkidataTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::SpkidataType {
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct SpkidataTypeContentDeserializer {
        spkisexp: Option<String>,
        state: Box<SpkidataTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    enum SpkidataTypeContentDeserializerState {
        Init__,
        Spkisexp(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SpkidataTypeContentDeserializer {
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SpkidataTypeContentDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use SpkidataTypeContentDeserializerState as S;
            match state {
                S::Spkisexp(Some(deserializer)) => {
                    self.store_spkisexp(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_spkisexp(&mut self, value: String) -> Result<(), Error> {
            if self.spkisexp.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SPKISexp",
                )))?;
            }
            self.spkisexp = Some(value);
            Ok(())
        }
        fn handle_spkisexp<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<SpkidataTypeContentDeserializerState>,
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
                if self.spkisexp.is_some() {
                    fallback.get_or_insert(SpkidataTypeContentDeserializerState::Spkisexp(None));
                    *self.state = SpkidataTypeContentDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = SpkidataTypeContentDeserializerState::Spkisexp(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_spkisexp(data)?;
                    *self.state = SpkidataTypeContentDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(SpkidataTypeContentDeserializerState::Spkisexp(
                                Some(deserializer),
                            ));
                            *self.state = SpkidataTypeContentDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                SpkidataTypeContentDeserializerState::Spkisexp(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SpkidataTypeContent> for SpkidataTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SpkidataTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                spkisexp: None,
                state: Box::new(SpkidataTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, SpkidataTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::SpkidataTypeContent>
        where
            R: DeserializeReader,
        {
            use SpkidataTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Spkisexp(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_spkisexp(reader, output, &mut fallback)? {
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
                        *self.state = SpkidataTypeContentDeserializerState::Spkisexp(None);
                        event
                    }
                    (S::Spkisexp(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"SPKISexp") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_spkisexp(reader, output, &mut fallback)? {
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
                            fallback.get_or_insert(S::Spkisexp(None));
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
        fn finish<R>(mut self, reader: &R) -> Result<super::SpkidataTypeContent, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                SpkidataTypeContentDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::SpkidataTypeContent {
                spkisexp: self
                    .spkisexp
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
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
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
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Algorithm")
                ) {
                    reader.read_attrib(&mut algorithm, b"Algorithm", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
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
    pub struct DsakeyValueTypeDeserializer {
        content_125: Option<super::DsakeyValueContent125Type>,
        g: Option<String>,
        y: Option<String>,
        j: Option<String>,
        content_131: Option<super::DsakeyValueContent131Type>,
        state: Box<DsakeyValueTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DsakeyValueTypeDeserializerState {
        Init__,
        Content125(Option<<super::DsakeyValueContent125Type as WithDeserializer>::Deserializer>),
        G(Option<<String as WithDeserializer>::Deserializer>),
        Y(Option<<String as WithDeserializer>::Deserializer>),
        J(Option<<String as WithDeserializer>::Deserializer>),
        Content131(Option<<super::DsakeyValueContent131Type as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DsakeyValueTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                content_125: None,
                g: None,
                y: None,
                j: None,
                content_131: None,
                state: Box::new(DsakeyValueTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DsakeyValueTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use DsakeyValueTypeDeserializerState as S;
            match state {
                S::Content125(Some(deserializer)) => {
                    self.store_content_125(deserializer.finish(reader)?)?
                }
                S::G(Some(deserializer)) => self.store_g(deserializer.finish(reader)?)?,
                S::Y(Some(deserializer)) => self.store_y(deserializer.finish(reader)?)?,
                S::J(Some(deserializer)) => self.store_j(deserializer.finish(reader)?)?,
                S::Content131(Some(deserializer)) => {
                    self.store_content_131(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_content_125(
            &mut self,
            value: super::DsakeyValueContent125Type,
        ) -> Result<(), Error> {
            if self.content_125.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Content125",
                )))?;
            }
            self.content_125 = Some(value);
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
        fn store_content_131(
            &mut self,
            value: super::DsakeyValueContent131Type,
        ) -> Result<(), Error> {
            if self.content_131.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Content131",
                )))?;
            }
            self.content_131 = Some(value);
            Ok(())
        }
        fn handle_content_125<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DsakeyValueContent125Type>,
            fallback: &mut Option<DsakeyValueTypeDeserializerState>,
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
                fallback.get_or_insert(DsakeyValueTypeDeserializerState::Content125(None));
                *self.state = DsakeyValueTypeDeserializerState::G(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content_125(data)?;
                    *self.state = DsakeyValueTypeDeserializerState::G(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DsakeyValueTypeDeserializerState::Content125(
                                Some(deserializer),
                            ));
                            *self.state = DsakeyValueTypeDeserializerState::G(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                DsakeyValueTypeDeserializerState::Content125(Some(deserializer));
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
            fallback: &mut Option<DsakeyValueTypeDeserializerState>,
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
                fallback.get_or_insert(DsakeyValueTypeDeserializerState::G(None));
                *self.state = DsakeyValueTypeDeserializerState::Y(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_g(data)?;
                    *self.state = DsakeyValueTypeDeserializerState::Y(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DsakeyValueTypeDeserializerState::G(Some(
                                deserializer,
                            )));
                            *self.state = DsakeyValueTypeDeserializerState::Y(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DsakeyValueTypeDeserializerState::G(Some(deserializer));
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
            fallback: &mut Option<DsakeyValueTypeDeserializerState>,
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
                    fallback.get_or_insert(DsakeyValueTypeDeserializerState::Y(None));
                    *self.state = DsakeyValueTypeDeserializerState::J(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DsakeyValueTypeDeserializerState::Y(None);
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
                    *self.state = DsakeyValueTypeDeserializerState::J(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DsakeyValueTypeDeserializerState::Y(Some(
                                deserializer,
                            )));
                            *self.state = DsakeyValueTypeDeserializerState::J(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DsakeyValueTypeDeserializerState::Y(Some(deserializer));
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
            fallback: &mut Option<DsakeyValueTypeDeserializerState>,
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
                fallback.get_or_insert(DsakeyValueTypeDeserializerState::J(None));
                *self.state = DsakeyValueTypeDeserializerState::Content131(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_j(data)?;
                    *self.state = DsakeyValueTypeDeserializerState::Content131(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DsakeyValueTypeDeserializerState::J(Some(
                                deserializer,
                            )));
                            *self.state = DsakeyValueTypeDeserializerState::Content131(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DsakeyValueTypeDeserializerState::J(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_content_131<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DsakeyValueContent131Type>,
            fallback: &mut Option<DsakeyValueTypeDeserializerState>,
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
                fallback.get_or_insert(DsakeyValueTypeDeserializerState::Content131(None));
                *self.state = DsakeyValueTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content_131(data)?;
                    *self.state = DsakeyValueTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DsakeyValueTypeDeserializerState::Content131(
                                Some(deserializer),
                            ));
                            *self.state = DsakeyValueTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                DsakeyValueTypeDeserializerState::Content131(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DsakeyValueType> for DsakeyValueTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::DsakeyValueType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DsakeyValueType>
        where
            R: DeserializeReader,
        {
            use DsakeyValueTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content125(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content_125(reader, output, &mut fallback)? {
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
                    (S::Content131(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content_131(reader, output, &mut fallback)? {
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
                        *self.state = DsakeyValueTypeDeserializerState::Content125(None);
                        event
                    }
                    (S::Content125(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = < super :: DsakeyValueContent125Type as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content_125(reader, output, &mut fallback)? {
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
                            *self.state = S::Content131(None);
                            event
                        }
                    }
                    (S::Content131(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = < super :: DsakeyValueContent131Type as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content_131(reader, output, &mut fallback)? {
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
        fn finish<R>(mut self, reader: &R) -> Result<super::DsakeyValueType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                DsakeyValueTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::DsakeyValueType {
                content_125: self.content_125,
                g: self.g,
                y: self
                    .y
                    .ok_or_else(|| ErrorKind::MissingElement("Y".into()))?,
                j: self.j,
                content_131: self.content_131,
            })
        }
    }
    #[derive(Debug)]
    pub struct RsakeyValueTypeDeserializer {
        modulus: Option<String>,
        exponent: Option<String>,
        state: Box<RsakeyValueTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RsakeyValueTypeDeserializerState {
        Init__,
        Modulus(Option<<String as WithDeserializer>::Deserializer>),
        Exponent(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl RsakeyValueTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                modulus: None,
                exponent: None,
                state: Box::new(RsakeyValueTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: RsakeyValueTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use RsakeyValueTypeDeserializerState as S;
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
            fallback: &mut Option<RsakeyValueTypeDeserializerState>,
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
                    fallback.get_or_insert(RsakeyValueTypeDeserializerState::Modulus(None));
                    *self.state = RsakeyValueTypeDeserializerState::Exponent(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = RsakeyValueTypeDeserializerState::Modulus(None);
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
                    *self.state = RsakeyValueTypeDeserializerState::Exponent(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(RsakeyValueTypeDeserializerState::Modulus(
                                Some(deserializer),
                            ));
                            *self.state = RsakeyValueTypeDeserializerState::Exponent(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                RsakeyValueTypeDeserializerState::Modulus(Some(deserializer));
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
            fallback: &mut Option<RsakeyValueTypeDeserializerState>,
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
                    fallback.get_or_insert(RsakeyValueTypeDeserializerState::Exponent(None));
                    *self.state = RsakeyValueTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = RsakeyValueTypeDeserializerState::Exponent(None);
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
                    *self.state = RsakeyValueTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(RsakeyValueTypeDeserializerState::Exponent(
                                Some(deserializer),
                            ));
                            *self.state = RsakeyValueTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                RsakeyValueTypeDeserializerState::Exponent(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RsakeyValueType> for RsakeyValueTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::RsakeyValueType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RsakeyValueType>
        where
            R: DeserializeReader,
        {
            use RsakeyValueTypeDeserializerState as S;
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
                        *self.state = RsakeyValueTypeDeserializerState::Modulus(None);
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
        fn finish<R>(mut self, reader: &R) -> Result<super::RsakeyValueType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                RsakeyValueTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::RsakeyValueType {
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
    pub struct X509DataContent103TypeDeserializer {
        content: Option<super::X509DataContent103TypeContent>,
        state: Box<X509DataContent103TypeDeserializerState>,
    }
    #[derive(Debug)]
    enum X509DataContent103TypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::X509DataContent103TypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl X509DataContent103TypeDeserializer {
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: X509DataContent103TypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let X509DataContent103TypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(
            &mut self,
            value: super::X509DataContent103TypeContent,
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
            output: DeserializerOutput<'de, super::X509DataContent103TypeContent>,
            fallback: &mut Option<X509DataContent103TypeDeserializerState>,
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
                    .unwrap_or(X509DataContent103TypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = X509DataContent103TypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                X509DataContent103TypeDeserializerState::Content__(deserializer),
                            );
                            *self.state = X509DataContent103TypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                X509DataContent103TypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::X509DataContent103Type> for X509DataContent103TypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::X509DataContent103Type>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                content: None,
                state: Box::new(X509DataContent103TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, X509DataContent103TypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::X509DataContent103Type>
        where
            R: DeserializeReader,
        {
            use X509DataContent103TypeDeserializerState as S;
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
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output = < super :: X509DataContent103TypeContent as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::X509DataContent103Type, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                X509DataContent103TypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::X509DataContent103Type {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub enum X509DataContent103TypeContentDeserializer {
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
        Done__(super::X509DataContent103TypeContent),
        Unknown__,
    }
    impl X509DataContent103TypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<X509DataContent103TypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, true));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_DS),
                Some(b"X509IssuerSerial")
            ) {
                let output = <super::X509IssuerSerialType as WithDeserializer>::Deserializer::init(
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
                return self.handle_x509_ski(reader, Default::default(), output, &mut *fallback);
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
                return self.handle_x509_crl(reader, Default::default(), output, &mut *fallback);
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, true))
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
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::X509IssuerSerial(_, Some(deserializer))) => {
                        Self::X509IssuerSerial(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::X509IssuerSerial(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_x509_issuer_serial(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x509_issuer_serial(&mut values, data)?;
                    let data = Self::X509IssuerSerial(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::X509IssuerSerial(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_x509_ski<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::X509Ski(_, Some(deserializer))) => {
                        Self::X509Ski(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::X509Ski(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_x509_ski(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x509_ski(&mut values, data)?;
                    let data = Self::X509Ski(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::X509Ski(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_x509_subject_name<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::X509SubjectName(_, Some(deserializer))) => {
                        Self::X509SubjectName(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::X509SubjectName(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_x509_subject_name(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x509_subject_name(&mut values, data)?;
                    let data = Self::X509SubjectName(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::X509SubjectName(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_x509_certificate<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::X509Certificate(_, Some(deserializer))) => {
                        Self::X509Certificate(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::X509Certificate(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_x509_certificate(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x509_certificate(&mut values, data)?;
                    let data = Self::X509Certificate(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::X509Certificate(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_x509_crl<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::X509Crl(_, Some(deserializer))) => {
                        Self::X509Crl(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::X509Crl(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_x509_crl(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x509_crl(&mut values, data)?;
                    let data = Self::X509Crl(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::X509Crl(values, Some(deserializer));
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::X509DataContent103TypeContent>
        for X509DataContent103TypeContentDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::X509DataContent103TypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::X509DataContent103TypeContent>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::X509IssuerSerial(values, Some(deserializer)), event) => {
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
                    (Self::X509Ski(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_x509_ski(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::X509SubjectName(values, Some(deserializer)), event) => {
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
                    (Self::X509Certificate(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_x509_certificate(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::X509Crl(values, Some(deserializer)), event) => {
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
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::X509IssuerSerial(values, None), event) => {
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
                    (Self::X509Ski(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_x509_ski(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::X509SubjectName(values, None), event) => {
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
                    (Self::X509Certificate(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_x509_certificate(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::X509Crl(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_x509_crl(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::X509DataContent103TypeContent, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::X509IssuerSerial(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_x509_issuer_serial(&mut values, value)?;
                    }
                    Ok(super::X509DataContent103TypeContent::X509IssuerSerial(
                        values
                            .ok_or_else(|| ErrorKind::MissingElement("X509IssuerSerial".into()))?,
                    ))
                }
                Self::X509Ski(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_x509_ski(&mut values, value)?;
                    }
                    Ok(super::X509DataContent103TypeContent::X509Ski(
                        values.ok_or_else(|| ErrorKind::MissingElement("X509SKI".into()))?,
                    ))
                }
                Self::X509SubjectName(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_x509_subject_name(&mut values, value)?;
                    }
                    Ok(super::X509DataContent103TypeContent::X509SubjectName(
                        values
                            .ok_or_else(|| ErrorKind::MissingElement("X509SubjectName".into()))?,
                    ))
                }
                Self::X509Certificate(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_x509_certificate(&mut values, value)?;
                    }
                    Ok(super::X509DataContent103TypeContent::X509Certificate(
                        values
                            .ok_or_else(|| ErrorKind::MissingElement("X509Certificate".into()))?,
                    ))
                }
                Self::X509Crl(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_x509_crl(&mut values, value)?;
                    }
                    Ok(super::X509DataContent103TypeContent::X509Crl(
                        values.ok_or_else(|| ErrorKind::MissingElement("X509CRL".into()))?,
                    ))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
        }
    }
    #[derive(Debug)]
    pub struct PgpdataContent113TypeDeserializer {
        pgpkey_id: Option<String>,
        pgpkey_packet: Option<String>,
        state: Box<PgpdataContent113TypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PgpdataContent113TypeDeserializerState {
        Init__,
        PgpkeyID(Option<<String as WithDeserializer>::Deserializer>),
        PgpkeyPacket(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl PgpdataContent113TypeDeserializer {
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: PgpdataContent113TypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use PgpdataContent113TypeDeserializerState as S;
            match state {
                S::PgpkeyID(Some(deserializer)) => {
                    self.store_pgpkey_id(deserializer.finish(reader)?)?
                }
                S::PgpkeyPacket(Some(deserializer)) => {
                    self.store_pgpkey_packet(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_pgpkey_id(&mut self, value: String) -> Result<(), Error> {
            if self.pgpkey_id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PGPKeyID",
                )))?;
            }
            self.pgpkey_id = Some(value);
            Ok(())
        }
        fn store_pgpkey_packet(&mut self, value: String) -> Result<(), Error> {
            if self.pgpkey_packet.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PGPKeyPacket",
                )))?;
            }
            self.pgpkey_packet = Some(value);
            Ok(())
        }
        fn handle_pgpkey_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<PgpdataContent113TypeDeserializerState>,
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
                if self.pgpkey_id.is_some() {
                    fallback.get_or_insert(PgpdataContent113TypeDeserializerState::PgpkeyID(None));
                    *self.state = PgpdataContent113TypeDeserializerState::PgpkeyPacket(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = PgpdataContent113TypeDeserializerState::PgpkeyID(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_pgpkey_id(data)?;
                    *self.state = PgpdataContent113TypeDeserializerState::PgpkeyPacket(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                PgpdataContent113TypeDeserializerState::PgpkeyID(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                PgpdataContent113TypeDeserializerState::PgpkeyPacket(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = PgpdataContent113TypeDeserializerState::PgpkeyID(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_pgpkey_packet<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<PgpdataContent113TypeDeserializerState>,
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
                fallback.get_or_insert(PgpdataContent113TypeDeserializerState::PgpkeyPacket(None));
                *self.state = PgpdataContent113TypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_pgpkey_packet(data)?;
                    *self.state = PgpdataContent113TypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                PgpdataContent113TypeDeserializerState::PgpkeyPacket(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = PgpdataContent113TypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = PgpdataContent113TypeDeserializerState::PgpkeyPacket(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::PgpdataContent113Type> for PgpdataContent113TypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PgpdataContent113Type>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                pgpkey_id: None,
                pgpkey_packet: None,
                state: Box::new(PgpdataContent113TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, PgpdataContent113TypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::PgpdataContent113Type>
        where
            R: DeserializeReader,
        {
            use PgpdataContent113TypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::PgpkeyID(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pgpkey_id(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::PgpkeyPacket(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pgpkey_packet(reader, output, &mut fallback)? {
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
                        *self.state = PgpdataContent113TypeDeserializerState::PgpkeyID(None);
                        event
                    }
                    (S::PgpkeyID(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"PGPKeyID") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_pgpkey_id(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::PgpkeyPacket(None);
                            allow_any_element = true;
                            fallback.get_or_insert(S::PgpkeyID(None));
                            event
                        }
                    }
                    (S::PgpkeyPacket(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"PGPKeyPacket")
                        {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_pgpkey_packet(reader, output, &mut fallback)? {
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
                            fallback.get_or_insert(S::PgpkeyPacket(None));
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
        fn finish<R>(mut self, reader: &R) -> Result<super::PgpdataContent113Type, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                PgpdataContent113TypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::PgpdataContent113Type {
                pgpkey_id: self
                    .pgpkey_id
                    .ok_or_else(|| ErrorKind::MissingElement("PGPKeyID".into()))?,
                pgpkey_packet: self.pgpkey_packet,
            })
        }
    }
    #[derive(Debug)]
    pub struct PgpdataContent116TypeDeserializer {
        pgpkey_packet: Option<String>,
        state: Box<PgpdataContent116TypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PgpdataContent116TypeDeserializerState {
        Init__,
        PgpkeyPacket(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl PgpdataContent116TypeDeserializer {
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: PgpdataContent116TypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use PgpdataContent116TypeDeserializerState as S;
            match state {
                S::PgpkeyPacket(Some(deserializer)) => {
                    self.store_pgpkey_packet(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_pgpkey_packet(&mut self, value: String) -> Result<(), Error> {
            if self.pgpkey_packet.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PGPKeyPacket",
                )))?;
            }
            self.pgpkey_packet = Some(value);
            Ok(())
        }
        fn handle_pgpkey_packet<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<PgpdataContent116TypeDeserializerState>,
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
                if self.pgpkey_packet.is_some() {
                    fallback
                        .get_or_insert(PgpdataContent116TypeDeserializerState::PgpkeyPacket(None));
                    *self.state = PgpdataContent116TypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = PgpdataContent116TypeDeserializerState::PgpkeyPacket(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_pgpkey_packet(data)?;
                    *self.state = PgpdataContent116TypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                PgpdataContent116TypeDeserializerState::PgpkeyPacket(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = PgpdataContent116TypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = PgpdataContent116TypeDeserializerState::PgpkeyPacket(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::PgpdataContent116Type> for PgpdataContent116TypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PgpdataContent116Type>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                pgpkey_packet: None,
                state: Box::new(PgpdataContent116TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, PgpdataContent116TypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::PgpdataContent116Type>
        where
            R: DeserializeReader,
        {
            use PgpdataContent116TypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::PgpkeyPacket(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pgpkey_packet(reader, output, &mut fallback)? {
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
                        *self.state = PgpdataContent116TypeDeserializerState::PgpkeyPacket(None);
                        event
                    }
                    (S::PgpkeyPacket(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_DS), b"PGPKeyPacket")
                        {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_pgpkey_packet(reader, output, &mut fallback)? {
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
                            fallback.get_or_insert(S::PgpkeyPacket(None));
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
        fn finish<R>(mut self, reader: &R) -> Result<super::PgpdataContent116Type, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                PgpdataContent116TypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::PgpdataContent116Type {
                pgpkey_packet: self
                    .pgpkey_packet
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
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DS),
                    Some(b"Algorithm")
                ) {
                    reader.read_attrib(&mut algorithm, b"Algorithm", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
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
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TransformTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = TransformTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TransformTypeDeserializerState::Content__(deserializer);
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
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output =
                            <super::TransformTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
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
    pub enum TransformTypeContentDeserializer {
        Init__,
        Xpath(
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
            fallback: &mut Option<TransformTypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, true));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_DS),
                Some(b"XPath")
            ) {
                let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_xpath(reader, Default::default(), output, &mut *fallback);
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, true))
        }
        fn store_xpath(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"XPath",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_xpath<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Xpath(_, Some(deserializer))) => {
                        Self::Xpath(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Xpath(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_xpath(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_xpath(&mut values, data)?;
                    let data = Self::Xpath(values, None).finish(reader)?;
                    *self = Self::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self = Self::Xpath(values, Some(deserializer));
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
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
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
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::Xpath(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_xpath(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Xpath(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_xpath(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ Self::Done__(_), event) => {
                        self = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match self {
                Self::Done__(data) => DeserializerArtifact::Data(data),
                deserializer => DeserializerArtifact::Deserializer(deserializer),
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
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::Xpath(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_xpath(&mut values, value)?;
                    }
                    Ok(super::TransformTypeContent::Xpath(values.ok_or_else(
                        || ErrorKind::MissingElement("XPath".into()),
                    )?))
                }
                Self::Done__(data) => Ok(data),
                Self::Unknown__ => unreachable!(),
            }
        }
    }
    #[derive(Debug)]
    pub struct DsakeyValueContent125TypeDeserializer {
        p: Option<String>,
        q: Option<String>,
        state: Box<DsakeyValueContent125TypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DsakeyValueContent125TypeDeserializerState {
        Init__,
        P(Option<<String as WithDeserializer>::Deserializer>),
        Q(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DsakeyValueContent125TypeDeserializer {
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DsakeyValueContent125TypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use DsakeyValueContent125TypeDeserializerState as S;
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
            fallback: &mut Option<DsakeyValueContent125TypeDeserializerState>,
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
                    fallback.get_or_insert(DsakeyValueContent125TypeDeserializerState::P(None));
                    *self.state = DsakeyValueContent125TypeDeserializerState::Q(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DsakeyValueContent125TypeDeserializerState::P(None);
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
                    *self.state = DsakeyValueContent125TypeDeserializerState::Q(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DsakeyValueContent125TypeDeserializerState::P(
                                Some(deserializer),
                            ));
                            *self.state = DsakeyValueContent125TypeDeserializerState::Q(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                DsakeyValueContent125TypeDeserializerState::P(Some(deserializer));
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
            fallback: &mut Option<DsakeyValueContent125TypeDeserializerState>,
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
                    fallback.get_or_insert(DsakeyValueContent125TypeDeserializerState::Q(None));
                    *self.state = DsakeyValueContent125TypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DsakeyValueContent125TypeDeserializerState::Q(None);
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
                    *self.state = DsakeyValueContent125TypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DsakeyValueContent125TypeDeserializerState::Q(
                                Some(deserializer),
                            ));
                            *self.state = DsakeyValueContent125TypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                DsakeyValueContent125TypeDeserializerState::Q(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DsakeyValueContent125Type>
        for DsakeyValueContent125TypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DsakeyValueContent125Type>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                p: None,
                q: None,
                state: Box::new(DsakeyValueContent125TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state,
                        DsakeyValueContent125TypeDeserializerState::Init__
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
        ) -> DeserializerResult<'de, super::DsakeyValueContent125Type>
        where
            R: DeserializeReader,
        {
            use DsakeyValueContent125TypeDeserializerState as S;
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
                        *self.state = DsakeyValueContent125TypeDeserializerState::P(None);
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
        fn finish<R>(mut self, reader: &R) -> Result<super::DsakeyValueContent125Type, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                DsakeyValueContent125TypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::DsakeyValueContent125Type {
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
    pub struct DsakeyValueContent131TypeDeserializer {
        seed: Option<String>,
        pgen_counter: Option<String>,
        state: Box<DsakeyValueContent131TypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DsakeyValueContent131TypeDeserializerState {
        Init__,
        Seed(Option<<String as WithDeserializer>::Deserializer>),
        PgenCounter(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DsakeyValueContent131TypeDeserializer {
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DsakeyValueContent131TypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use DsakeyValueContent131TypeDeserializerState as S;
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
            fallback: &mut Option<DsakeyValueContent131TypeDeserializerState>,
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
                    fallback.get_or_insert(DsakeyValueContent131TypeDeserializerState::Seed(None));
                    *self.state = DsakeyValueContent131TypeDeserializerState::PgenCounter(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DsakeyValueContent131TypeDeserializerState::Seed(None);
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
                    *self.state = DsakeyValueContent131TypeDeserializerState::PgenCounter(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                DsakeyValueContent131TypeDeserializerState::Seed(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                DsakeyValueContent131TypeDeserializerState::PgenCounter(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DsakeyValueContent131TypeDeserializerState::Seed(Some(
                                deserializer,
                            ));
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
            fallback: &mut Option<DsakeyValueContent131TypeDeserializerState>,
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
                    fallback.get_or_insert(
                        DsakeyValueContent131TypeDeserializerState::PgenCounter(None),
                    );
                    *self.state = DsakeyValueContent131TypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DsakeyValueContent131TypeDeserializerState::PgenCounter(None);
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
                    *self.state = DsakeyValueContent131TypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                DsakeyValueContent131TypeDeserializerState::PgenCounter(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = DsakeyValueContent131TypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DsakeyValueContent131TypeDeserializerState::PgenCounter(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DsakeyValueContent131Type>
        for DsakeyValueContent131TypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DsakeyValueContent131Type>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                seed: None,
                pgen_counter: None,
                state: Box::new(DsakeyValueContent131TypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state,
                        DsakeyValueContent131TypeDeserializerState::Init__
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
        ) -> DeserializerResult<'de, super::DsakeyValueContent131Type>
        where
            R: DeserializeReader,
        {
            use DsakeyValueContent131TypeDeserializerState as S;
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
                        *self.state = DsakeyValueContent131TypeDeserializerState::Seed(None);
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
        fn finish<R>(mut self, reader: &R) -> Result<super::DsakeyValueContent131Type, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                DsakeyValueContent131TypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::DsakeyValueContent131Type {
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
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
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
