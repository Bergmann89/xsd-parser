use serde::{Deserialize, Serialize};
pub type DirectoryReq = DirectoryReqType;
#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryReqType {
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "createDateTimestamp")]
    pub create_date_timestamp: String,
    #[serde(rename = "Merchant")]
    pub merchant: DirectoryReqMerchantType,
    #[serde(rename = "Signature")]
    pub signature: SignatureType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryReqMerchantType {
    #[serde(rename = "merchantID")]
    pub merchant_id: String,
    #[serde(rename = "subID")]
    pub sub_id: usize,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureType {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(rename = "SignedInfo")]
    pub signed_info: SignedInfoType,
    #[serde(rename = "SignatureValue")]
    pub signature_value: SignatureValueType,
    #[serde(default, rename = "KeyInfo")]
    pub key_info: Option<KeyInfoType>,
    #[serde(default, rename = "Object")]
    pub object: Vec<ObjectType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignedInfoType {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(rename = "CanonicalizationMethod")]
    pub canonicalization_method: CanonicalizationMethodType,
    #[serde(rename = "SignatureMethod")]
    pub signature_method: SignatureMethodType,
    #[serde(default, rename = "Reference")]
    pub reference: Vec<ReferenceType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureValueType {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(rename = "$text")]
    pub content: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyInfoType {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(rename = "$value")]
    pub content: Vec<KeyInfoTypeContent>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum KeyInfoTypeContent {
    #[serde(rename = "KeyName")]
    KeyName(String),
    #[serde(rename = "KeyValue")]
    KeyValue(KeyValueType),
    #[serde(rename = "RetrievalMethod")]
    RetrievalMethod(RetrievalMethodType),
    #[serde(rename = "X509Data")]
    X509Data(X509DataType),
    #[serde(rename = "PGPData")]
    PgpData(PgpDataType),
    #[serde(rename = "SPKIData")]
    SpkiData(SpkiDataType),
    #[serde(rename = "MgmtData")]
    MgmtData(String),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectType {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(default, rename = "@MimeType")]
    pub mime_type: Option<String>,
    #[serde(default, rename = "@Encoding")]
    pub encoding: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CanonicalizationMethodType {
    #[serde(rename = "@Algorithm")]
    pub algorithm: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureMethodType {
    #[serde(rename = "@Algorithm")]
    pub algorithm: String,
    #[serde(default, rename = "HMACOutputLength")]
    pub hmac_output_length: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceType {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(default, rename = "@URI")]
    pub uri: Option<String>,
    #[serde(default, rename = "@Type")]
    pub type_: Option<String>,
    #[serde(default, rename = "Transforms")]
    pub transforms: Option<TransformsType>,
    #[serde(rename = "DigestMethod")]
    pub digest_method: DigestMethodType,
    #[serde(rename = "DigestValue")]
    pub digest_value: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValueType {
    #[serde(rename = "$value")]
    pub content: KeyValueTypeContent,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum KeyValueTypeContent {
    #[serde(rename = "DSAKeyValue")]
    DsaKeyValue(DsaKeyValueType),
    #[serde(rename = "RSAKeyValue")]
    RsaKeyValue(RsaKeyValueType),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RetrievalMethodType {
    #[serde(default, rename = "@URI")]
    pub uri: Option<String>,
    #[serde(default, rename = "@Type")]
    pub type_: Option<String>,
    #[serde(default, rename = "Transforms")]
    pub transforms: Option<TransformsType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct X509DataType {
    #[serde(rename = "$value")]
    pub content: Vec<X509DataTypeContent>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum X509DataTypeContent {
    #[serde(rename = "X509IssuerSerial")]
    X509IssuerSerial(X509IssuerSerialType),
    #[serde(rename = "X509SKI")]
    X509Ski(String),
    #[serde(rename = "X509SubjectName")]
    X509SubjectName(String),
    #[serde(rename = "X509Certificate")]
    X509Certificate(String),
    #[serde(rename = "X509CRL")]
    X509Crl(String),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PgpDataType {
    #[serde(rename = "$value")]
    pub content: Vec<PgpDataTypeContent>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PgpDataTypeContent {
    #[serde(rename = "PGPKeyID")]
    PgpKeyId(String),
    #[serde(rename = "PGPKeyPacket")]
    PgpKeyPacket(String),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SpkiDataType {
    #[serde(rename = "$value")]
    pub content: Vec<SpkiDataTypeContent>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SpkiDataTypeContent {
    #[serde(rename = "SPKISexp")]
    pub spki_sexp: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransformsType {
    #[serde(default, rename = "Transform")]
    pub transform: Vec<TransformType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DigestMethodType {
    #[serde(rename = "@Algorithm")]
    pub algorithm: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DsaKeyValueType {
    #[serde(default, rename = "P")]
    pub p: Option<String>,
    #[serde(default, rename = "Q")]
    pub q: Option<String>,
    #[serde(default, rename = "G")]
    pub g: Option<String>,
    #[serde(rename = "Y")]
    pub y: String,
    #[serde(default, rename = "J")]
    pub j: Option<String>,
    #[serde(default, rename = "Seed")]
    pub seed: Option<String>,
    #[serde(default, rename = "PgenCounter")]
    pub pgen_counter: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RsaKeyValueType {
    #[serde(rename = "Modulus")]
    pub modulus: String,
    #[serde(rename = "Exponent")]
    pub exponent: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct X509IssuerSerialType {
    #[serde(rename = "X509IssuerName")]
    pub x509_issuer_name: String,
    #[serde(rename = "X509SerialNumber")]
    pub x509_serial_number: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransformType {
    #[serde(rename = "@Algorithm")]
    pub algorithm: String,
    #[serde(default, rename = "$value")]
    pub content: Vec<TransformTypeContent>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransformTypeContent {
    #[serde(rename = "XPath")]
    XPath(String),
}
