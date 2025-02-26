pub type DirectoryReq = DirectoryReqType;
#[derive(Debug, Clone)]
pub struct DirectoryReqType {
    pub version: String,
    pub create_date_timestamp: String,
    pub merchant: DirectoryReqMerchantType,
    pub signature: SignatureType,
}
#[derive(Debug, Clone)]
pub struct DirectoryReqMerchantType {
    pub merchant_id: String,
    pub sub_id: usize,
}
#[derive(Debug, Clone)]
pub struct SignatureType {
    pub id: Option<String>,
    pub signed_info: SignedInfoType,
    pub signature_value: SignatureValueType,
    pub key_info: Option<KeyInfoType>,
    pub object: Vec<ObjectType>,
}
#[derive(Debug, Clone)]
pub struct SignedInfoType {
    pub id: Option<String>,
    pub canonicalization_method: CanonicalizationMethodType,
    pub signature_method: SignatureMethodType,
    pub reference: Vec<ReferenceType>,
}
#[derive(Debug, Clone)]
pub struct SignatureValueType {
    pub id: Option<String>,
    pub content: String,
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
#[derive(Debug, Clone)]
pub struct ObjectType {
    pub id: Option<String>,
    pub mime_type: Option<String>,
    pub encoding: Option<String>,
}
#[derive(Debug, Clone)]
pub struct CanonicalizationMethodType {
    pub algorithm: String,
}
#[derive(Debug, Clone)]
pub struct SignatureMethodType {
    pub algorithm: String,
    pub hmacoutput_length: Option<i32>,
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
#[derive(Debug, Clone)]
pub struct KeyValueType {
    pub content: KeyValueTypeContent,
}
#[derive(Debug, Clone)]
pub enum KeyValueTypeContent {
    DsakeyValue(DsakeyValueType),
    RsakeyValue(RsakeyValueType),
}
#[derive(Debug, Clone)]
pub struct RetrievalMethodType {
    pub uri: Option<String>,
    pub type_: Option<String>,
    pub transforms: Option<TransformsType>,
}
#[derive(Debug, Clone)]
pub struct X509DataType {
    pub content_103: X509DataContent103Type,
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
#[derive(Debug, Clone)]
pub struct SpkidataType {
    pub spkisexp: String,
}
#[derive(Debug, Clone)]
pub struct TransformsType {
    pub transform: Vec<TransformType>,
}
#[derive(Debug, Clone)]
pub struct DigestMethodType {
    pub algorithm: String,
}
#[derive(Debug, Clone)]
pub struct DsakeyValueType {
    pub content_125: Option<DsakeyValueContent125Type>,
    pub g: Option<String>,
    pub y: String,
    pub j: Option<String>,
    pub content_131: Option<DsakeyValueContent131Type>,
}
#[derive(Debug, Clone)]
pub struct RsakeyValueType {
    pub modulus: String,
    pub exponent: String,
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
#[derive(Debug, Clone)]
pub struct PgpdataContent113Type {
    pub pgpkey_id: String,
    pub pgpkey_packet: Option<String>,
}
#[derive(Debug, Clone)]
pub struct PgpdataContent116Type {
    pub pgpkey_packet: String,
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
#[derive(Debug, Clone)]
pub struct DsakeyValueContent125Type {
    pub p: String,
    pub q: String,
}
#[derive(Debug, Clone)]
pub struct DsakeyValueContent131Type {
    pub seed: String,
    pub pgen_counter: String,
}
#[derive(Debug, Clone)]
pub struct X509IssuerSerialType {
    pub x509_issuer_name: String,
    pub x509_serial_number: i32,
}
