use xsd_parser_types::xml::Base64String;
pub type DirectoryReq = DirectoryReqType;
#[derive(Debug)]
pub struct DirectoryReqType {
    pub version: String,
    pub create_date_timestamp: String,
    pub merchant: DirectoryReqMerchantType,
    pub signature: SignatureType,
}
#[derive(Debug)]
pub struct DirectoryReqMerchantType {
    pub merchant_id: String,
    pub sub_id: usize,
}
#[derive(Debug)]
pub struct SignatureType {
    pub id: Option<String>,
    pub signed_info: SignedInfoType,
    pub signature_value: SignatureValueType,
    pub key_info: Option<KeyInfoType>,
    pub object: Vec<ObjectType>,
}
#[derive(Debug)]
pub struct SignedInfoType {
    pub id: Option<String>,
    pub canonicalization_method: CanonicalizationMethodType,
    pub signature_method: SignatureMethodType,
    pub reference: Vec<ReferenceType>,
}
#[derive(Debug)]
pub struct SignatureValueType {
    pub id: Option<String>,
    pub content: Base64String,
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
#[derive(Debug)]
pub struct ObjectType {
    pub id: Option<String>,
    pub mime_type: Option<String>,
    pub encoding: Option<String>,
}
#[derive(Debug)]
pub struct CanonicalizationMethodType {
    pub algorithm: String,
}
#[derive(Debug)]
pub struct SignatureMethodType {
    pub algorithm: String,
    pub hmac_output_length: Option<i32>,
}
#[derive(Debug)]
pub struct ReferenceType {
    pub id: Option<String>,
    pub uri: Option<String>,
    pub type_: Option<String>,
    pub transforms: Option<TransformsType>,
    pub digest_method: DigestMethodType,
    pub digest_value: Base64String,
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
#[derive(Debug)]
pub struct RetrievalMethodType {
    pub uri: Option<String>,
    pub type_: Option<String>,
    pub transforms: Option<TransformsType>,
}
#[derive(Debug)]
pub struct X509DataType {
    pub content: Vec<X509DataTypeContent>,
}
#[derive(Debug)]
pub struct X509DataTypeContent {
    pub content_19: X509DataContent19Type,
}
#[derive(Debug)]
pub struct PgpDataType {
    pub content: PgpDataTypeContent,
}
#[derive(Debug)]
pub enum PgpDataTypeContent {
    Content23(PgpDataContent23Type),
    Content25(PgpDataContent25Type),
}
#[derive(Debug)]
pub struct SpkiDataType {
    pub content: Vec<SpkiDataTypeContent>,
}
#[derive(Debug)]
pub struct SpkiDataTypeContent {
    pub spki_sexp: Base64String,
}
#[derive(Debug)]
pub struct TransformsType {
    pub transform: Vec<TransformType>,
}
#[derive(Debug)]
pub struct DigestMethodType {
    pub algorithm: String,
}
#[derive(Debug)]
pub struct DsaKeyValueType {
    pub content_36: Option<DsaKeyValueContent36Type>,
    pub g: Option<Base64String>,
    pub y: Base64String,
    pub j: Option<Base64String>,
    pub content_37: Option<DsaKeyValueContent37Type>,
}
#[derive(Debug)]
pub struct RsaKeyValueType {
    pub modulus: Base64String,
    pub exponent: Base64String,
}
#[derive(Debug)]
pub struct X509DataContent19Type {
    pub content: X509DataContent19TypeContent,
}
#[derive(Debug)]
pub enum X509DataContent19TypeContent {
    X509IssuerSerial(X509IssuerSerialType),
    X509Ski(Base64String),
    X509SubjectName(String),
    X509Certificate(Base64String),
    X509Crl(Base64String),
}
#[derive(Debug)]
pub struct PgpDataContent23Type {
    pub pgp_key_id: Base64String,
    pub pgp_key_packet: Option<Base64String>,
}
#[derive(Debug)]
pub struct PgpDataContent25Type {
    pub pgp_key_packet: Base64String,
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
#[derive(Debug)]
pub struct DsaKeyValueContent36Type {
    pub p: Base64String,
    pub q: Base64String,
}
#[derive(Debug)]
pub struct DsaKeyValueContent37Type {
    pub seed: Base64String,
    pub pgen_counter: Base64String,
}
#[derive(Debug)]
pub struct X509IssuerSerialType {
    pub x509_issuer_name: String,
    pub x509_serial_number: i32,
}
