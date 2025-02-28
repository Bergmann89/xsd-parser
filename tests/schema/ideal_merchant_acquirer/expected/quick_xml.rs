pub type DirectoryReq = DirectoryReqType;
#[derive(Debug, Clone)]
pub struct DirectoryReqType {
    pub version: String,
    pub create_date_timestamp: String,
    pub merchant: DirectoryReqMerchantType,
    pub signature: SignatureType,
}
impl xsd_parser::quick_xml::WithSerializer for DirectoryReqType {
    type Serializer<'x> = quick_xml_serialize::DirectoryReqTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::DirectoryReqTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for DirectoryReqType {
    type Deserializer = quick_xml_deserialize::DirectoryReqTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct DirectoryReqMerchantType {
    pub merchant_id: String,
    pub sub_id: usize,
}
impl xsd_parser::quick_xml::WithSerializer for DirectoryReqMerchantType {
    type Serializer<'x> = quick_xml_serialize::DirectoryReqMerchantTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::DirectoryReqMerchantTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for DirectoryReqMerchantType {
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
impl xsd_parser::quick_xml::WithSerializer for SignatureType {
    type Serializer<'x> = quick_xml_serialize::SignatureTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::SignatureTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for SignatureType {
    type Deserializer = quick_xml_deserialize::SignatureTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct SignedInfoType {
    pub id: Option<String>,
    pub canonicalization_method: CanonicalizationMethodType,
    pub signature_method: SignatureMethodType,
    pub reference: Vec<ReferenceType>,
}
impl xsd_parser::quick_xml::WithSerializer for SignedInfoType {
    type Serializer<'x> = quick_xml_serialize::SignedInfoTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::SignedInfoTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for SignedInfoType {
    type Deserializer = quick_xml_deserialize::SignedInfoTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct SignatureValueType {
    pub id: Option<String>,
    pub content: String,
}
impl xsd_parser::quick_xml::WithSerializer for SignatureValueType {
    type Serializer<'x> = quick_xml_serialize::SignatureValueTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::SignatureValueTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for SignatureValueType {
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
impl xsd_parser::quick_xml::WithSerializer for KeyInfoType {
    type Serializer<'x> = quick_xml_serialize::KeyInfoTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::KeyInfoTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for KeyInfoType {
    type Deserializer = quick_xml_deserialize::KeyInfoTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct ObjectType {
    pub id: Option<String>,
    pub mime_type: Option<String>,
    pub encoding: Option<String>,
}
impl xsd_parser::quick_xml::WithSerializer for ObjectType {
    type Serializer<'x> = quick_xml_serialize::ObjectTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::ObjectTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for ObjectType {
    type Deserializer = quick_xml_deserialize::ObjectTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct CanonicalizationMethodType {
    pub algorithm: String,
}
impl xsd_parser::quick_xml::WithSerializer for CanonicalizationMethodType {
    type Serializer<'x> = quick_xml_serialize::CanonicalizationMethodTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::CanonicalizationMethodTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for CanonicalizationMethodType {
    type Deserializer = quick_xml_deserialize::CanonicalizationMethodTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct SignatureMethodType {
    pub algorithm: String,
    pub hmacoutput_length: Option<i32>,
}
impl xsd_parser::quick_xml::WithSerializer for SignatureMethodType {
    type Serializer<'x> = quick_xml_serialize::SignatureMethodTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::SignatureMethodTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for SignatureMethodType {
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
impl xsd_parser::quick_xml::WithSerializer for ReferenceType {
    type Serializer<'x> = quick_xml_serialize::ReferenceTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::ReferenceTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for ReferenceType {
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
impl xsd_parser::quick_xml::WithSerializer for KeyValueType {
    type Serializer<'x> = quick_xml_serialize::KeyValueTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::KeyValueTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for KeyValueType {
    type Deserializer = quick_xml_deserialize::KeyValueTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct RetrievalMethodType {
    pub uri: Option<String>,
    pub type_: Option<String>,
    pub transforms: Option<TransformsType>,
}
impl xsd_parser::quick_xml::WithSerializer for RetrievalMethodType {
    type Serializer<'x> = quick_xml_serialize::RetrievalMethodTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::RetrievalMethodTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for RetrievalMethodType {
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
impl xsd_parser::quick_xml::WithSerializer for X509DataType {
    type Serializer<'x> = quick_xml_serialize::X509DataTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::X509DataTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for X509DataType {
    type Deserializer = quick_xml_deserialize::X509DataTypeDeserializer;
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
impl xsd_parser::quick_xml::WithSerializer for PgpdataType {
    type Serializer<'x> = quick_xml_serialize::PgpdataTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::PgpdataTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for PgpdataType {
    type Deserializer = quick_xml_deserialize::PgpdataTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct SpkidataType {
    pub content: Vec<SpkidataTypeContent>,
}
#[derive(Debug, Clone)]
pub struct SpkidataTypeContent {
    pub spkisexp: String,
}
impl xsd_parser::quick_xml::WithSerializer for SpkidataType {
    type Serializer<'x> = quick_xml_serialize::SpkidataTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::SpkidataTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for SpkidataType {
    type Deserializer = quick_xml_deserialize::SpkidataTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct TransformsType {
    pub transform: Vec<TransformType>,
}
impl xsd_parser::quick_xml::WithSerializer for TransformsType {
    type Serializer<'x> = quick_xml_serialize::TransformsTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::TransformsTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for TransformsType {
    type Deserializer = quick_xml_deserialize::TransformsTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct DigestMethodType {
    pub algorithm: String,
}
impl xsd_parser::quick_xml::WithSerializer for DigestMethodType {
    type Serializer<'x> = quick_xml_serialize::DigestMethodTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::DigestMethodTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for DigestMethodType {
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
impl xsd_parser::quick_xml::WithSerializer for DsakeyValueType {
    type Serializer<'x> = quick_xml_serialize::DsakeyValueTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::DsakeyValueTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for DsakeyValueType {
    type Deserializer = quick_xml_deserialize::DsakeyValueTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct RsakeyValueType {
    pub modulus: String,
    pub exponent: String,
}
impl xsd_parser::quick_xml::WithSerializer for RsakeyValueType {
    type Serializer<'x> = quick_xml_serialize::RsakeyValueTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::RsakeyValueTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for RsakeyValueType {
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
impl xsd_parser::quick_xml::WithSerializer for X509DataContent103Type {
    type Serializer<'x> = quick_xml_serialize::X509DataContent103TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::X509DataContent103TypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for X509DataContent103Type {
    type Deserializer = quick_xml_deserialize::X509DataContent103TypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct PgpdataContent113Type {
    pub pgpkey_id: String,
    pub pgpkey_packet: Option<String>,
}
impl xsd_parser::quick_xml::WithSerializer for PgpdataContent113Type {
    type Serializer<'x> = quick_xml_serialize::PgpdataContent113TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::PgpdataContent113TypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for PgpdataContent113Type {
    type Deserializer = quick_xml_deserialize::PgpdataContent113TypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct PgpdataContent116Type {
    pub pgpkey_packet: String,
}
impl xsd_parser::quick_xml::WithSerializer for PgpdataContent116Type {
    type Serializer<'x> = quick_xml_serialize::PgpdataContent116TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::PgpdataContent116TypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for PgpdataContent116Type {
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
impl xsd_parser::quick_xml::WithSerializer for TransformType {
    type Serializer<'x> = quick_xml_serialize::TransformTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::TransformTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for TransformType {
    type Deserializer = quick_xml_deserialize::TransformTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct DsakeyValueContent125Type {
    pub p: String,
    pub q: String,
}
impl xsd_parser::quick_xml::WithSerializer for DsakeyValueContent125Type {
    type Serializer<'x> = quick_xml_serialize::DsakeyValueContent125TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::DsakeyValueContent125TypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for DsakeyValueContent125Type {
    type Deserializer = quick_xml_deserialize::DsakeyValueContent125TypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct DsakeyValueContent131Type {
    pub seed: String,
    pub pgen_counter: String,
}
impl xsd_parser::quick_xml::WithSerializer for DsakeyValueContent131Type {
    type Serializer<'x> = quick_xml_serialize::DsakeyValueContent131TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::DsakeyValueContent131TypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for DsakeyValueContent131Type {
    type Deserializer = quick_xml_deserialize::DsakeyValueContent131TypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct X509IssuerSerialType {
    pub x509_issuer_name: String,
    pub x509_serial_number: i32,
}
impl xsd_parser::quick_xml::WithSerializer for X509IssuerSerialType {
    type Serializer<'x> = quick_xml_serialize::X509IssuerSerialTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::X509IssuerSerialTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for X509IssuerSerialType {
    type Deserializer = quick_xml_deserialize::X509IssuerSerialTypeDeserializer;
}
pub mod quick_xml_serialize {
    use super::*;
    #[derive(Debug)]
    pub struct DirectoryReqTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::DirectoryReqType,
        is_root: bool,
        state: DirectoryReqTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum DirectoryReqTypeSerializerState<'ser> {
        Init__,
        CreateDateTimestamp(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        Merchant(
            <DirectoryReqMerchantType as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>,
        ),
        Signature(<SignatureType as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DirectoryReqTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::DirectoryReqType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("DirectoryReq");
            Ok(Self {
                name,
                value,
                is_root,
                state: DirectoryReqTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for DirectoryReqTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            fn build_attributes<'a>(
                mut bytes: BytesStart<'a>,
                value: &'a super::DirectoryReqType,
            ) -> Result<BytesStart<'a>, Error> {
                use xsd_parser::quick_xml::SerializeBytes;
                if let Some(val) = SerializeBytes::serialize_bytes(&value.version)? {
                    bytes.push_attribute(("version", val));
                }
                Ok(bytes)
            }
            loop {
                match &mut self.state {
                    DirectoryReqTypeSerializerState::Init__ => {
                        self.state = DirectoryReqTypeSerializerState::CreateDateTimestamp(
                            xsd_parser::quick_xml::ContentSerializer::new(
                                &self.value.create_date_timestamp,
                                Some("createDateTimestamp"),
                                false,
                            ),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        match build_attributes(bytes, &self.value) {
                            Ok(bytes) => return Some(Ok(Event::Start(bytes))),
                            Err(error) => {
                                self.state = DirectoryReqTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                    }
                    DirectoryReqTypeSerializerState::CreateDateTimestamp(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = DirectoryReqTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => match WithSerializer::serializer(
                            &self.value.merchant,
                            Some("Merchant"),
                            false,
                        ) {
                            Ok(serializer) => {
                                self.state = DirectoryReqTypeSerializerState::Merchant(serializer)
                            }
                            Err(error) => {
                                self.state = DirectoryReqTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        },
                    },
                    DirectoryReqTypeSerializerState::Merchant(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = DirectoryReqTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => match WithSerializer::serializer(
                            &self.value.signature,
                            Some("ds:Signature"),
                            false,
                        ) {
                            Ok(serializer) => {
                                self.state = DirectoryReqTypeSerializerState::Signature(serializer)
                            }
                            Err(error) => {
                                self.state = DirectoryReqTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        },
                    },
                    DirectoryReqTypeSerializerState::Signature(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = DirectoryReqTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = DirectoryReqTypeSerializerState::End__,
                    },
                    DirectoryReqTypeSerializerState::End__ => {
                        self.state = DirectoryReqTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    DirectoryReqTypeSerializerState::Done__ => return None,
                    DirectoryReqTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DirectoryReqMerchantTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::DirectoryReqMerchantType,
        is_root: bool,
        state: DirectoryReqMerchantTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum DirectoryReqMerchantTypeSerializerState<'ser> {
        Init__,
        MerchantID(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        SubID(xsd_parser::quick_xml::ContentSerializer<'ser, usize>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DirectoryReqMerchantTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::DirectoryReqMerchantType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("DirectoryReqMerchant");
            Ok(Self {
                name,
                value,
                is_root,
                state: DirectoryReqMerchantTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for DirectoryReqMerchantTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    DirectoryReqMerchantTypeSerializerState::Init__ => {
                        self.state = DirectoryReqMerchantTypeSerializerState::MerchantID(
                            xsd_parser::quick_xml::ContentSerializer::new(
                                &self.value.merchant_id,
                                Some("merchantID"),
                                false,
                            ),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        return Some(Ok(Event::Start(bytes)));
                    }
                    DirectoryReqMerchantTypeSerializerState::MerchantID(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = DirectoryReqMerchantTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = DirectoryReqMerchantTypeSerializerState::SubID(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    &self.value.sub_id,
                                    Some("subID"),
                                    false,
                                ),
                            )
                        }
                    },
                    DirectoryReqMerchantTypeSerializerState::SubID(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = DirectoryReqMerchantTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = DirectoryReqMerchantTypeSerializerState::End__,
                    },
                    DirectoryReqMerchantTypeSerializerState::End__ => {
                        self.state = DirectoryReqMerchantTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    DirectoryReqMerchantTypeSerializerState::Done__ => return None,
                    DirectoryReqMerchantTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SignatureTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::SignatureType,
        is_root: bool,
        state: SignatureTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum SignatureTypeSerializerState<'ser> {
        Init__,
        SignedInfo(<SignedInfoType as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        SignatureValue(
            <SignatureValueType as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>,
        ),
        KeyInfo(xsd_parser::quick_xml::IterSerializer<'ser, Option<KeyInfoType>, KeyInfoType>),
        Object(xsd_parser::quick_xml::IterSerializer<'ser, Vec<ObjectType>, ObjectType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SignatureTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::SignatureType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:SignatureType");
            Ok(Self {
                name,
                value,
                is_root,
                state: SignatureTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for SignatureTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            fn build_attributes<'a>(
                mut bytes: BytesStart<'a>,
                value: &'a super::SignatureType,
            ) -> Result<BytesStart<'a>, Error> {
                use xsd_parser::quick_xml::SerializeBytes;
                if let Some(val) = value
                    .id
                    .as_ref()
                    .map(SerializeBytes::serialize_bytes)
                    .transpose()?
                    .flatten()
                {
                    bytes.push_attribute(("ds:Id", val));
                }
                Ok(bytes)
            }
            loop {
                match &mut self.state {
                    SignatureTypeSerializerState::Init__ => {
                        match WithSerializer::serializer(
                            &self.value.signed_info,
                            Some("ds:SignedInfo"),
                            false,
                        ) {
                            Ok(serializer) => {
                                self.state = SignatureTypeSerializerState::SignedInfo(serializer)
                            }
                            Err(error) => {
                                self.state = SignatureTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        };
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        match build_attributes(bytes, &self.value) {
                            Ok(bytes) => return Some(Ok(Event::Start(bytes))),
                            Err(error) => {
                                self.state = SignatureTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                    }
                    SignatureTypeSerializerState::SignedInfo(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = SignatureTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => match WithSerializer::serializer(
                            &self.value.signature_value,
                            Some("ds:SignatureValue"),
                            false,
                        ) {
                            Ok(serializer) => {
                                self.state =
                                    SignatureTypeSerializerState::SignatureValue(serializer)
                            }
                            Err(error) => {
                                self.state = SignatureTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        },
                    },
                    SignatureTypeSerializerState::SignatureValue(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = SignatureTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = SignatureTypeSerializerState::KeyInfo(
                                xsd_parser::quick_xml::IterSerializer::new(
                                    &self.value.key_info,
                                    Some("ds:KeyInfo"),
                                    false,
                                ),
                            )
                        }
                    },
                    SignatureTypeSerializerState::KeyInfo(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = SignatureTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = SignatureTypeSerializerState::Object(
                                xsd_parser::quick_xml::IterSerializer::new(
                                    &self.value.object,
                                    Some("ds:Object"),
                                    false,
                                ),
                            )
                        }
                    },
                    SignatureTypeSerializerState::Object(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = SignatureTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = SignatureTypeSerializerState::End__,
                    },
                    SignatureTypeSerializerState::End__ => {
                        self.state = SignatureTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    SignatureTypeSerializerState::Done__ => return None,
                    SignatureTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SignedInfoTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::SignedInfoType,
        is_root: bool,
        state: SignedInfoTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum SignedInfoTypeSerializerState<'ser> {
        Init__,
        CanonicalizationMethod(
            <CanonicalizationMethodType as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>,
        ),
        SignatureMethod(
            <SignatureMethodType as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>,
        ),
        Reference(xsd_parser::quick_xml::IterSerializer<'ser, Vec<ReferenceType>, ReferenceType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SignedInfoTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::SignedInfoType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:SignedInfoType");
            Ok(Self {
                name,
                value,
                is_root,
                state: SignedInfoTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for SignedInfoTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            fn build_attributes<'a>(
                mut bytes: BytesStart<'a>,
                value: &'a super::SignedInfoType,
            ) -> Result<BytesStart<'a>, Error> {
                use xsd_parser::quick_xml::SerializeBytes;
                if let Some(val) = value
                    .id
                    .as_ref()
                    .map(SerializeBytes::serialize_bytes)
                    .transpose()?
                    .flatten()
                {
                    bytes.push_attribute(("ds:Id", val));
                }
                Ok(bytes)
            }
            loop {
                match &mut self.state {
                    SignedInfoTypeSerializerState::Init__ => {
                        match WithSerializer::serializer(
                            &self.value.canonicalization_method,
                            Some("ds:CanonicalizationMethod"),
                            false,
                        ) {
                            Ok(serializer) => {
                                self.state = SignedInfoTypeSerializerState::CanonicalizationMethod(
                                    serializer,
                                )
                            }
                            Err(error) => {
                                self.state = SignedInfoTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        };
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        match build_attributes(bytes, &self.value) {
                            Ok(bytes) => return Some(Ok(Event::Start(bytes))),
                            Err(error) => {
                                self.state = SignedInfoTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                    }
                    SignedInfoTypeSerializerState::CanonicalizationMethod(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = SignedInfoTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => match WithSerializer::serializer(
                            &self.value.signature_method,
                            Some("ds:SignatureMethod"),
                            false,
                        ) {
                            Ok(serializer) => {
                                self.state =
                                    SignedInfoTypeSerializerState::SignatureMethod(serializer)
                            }
                            Err(error) => {
                                self.state = SignedInfoTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        },
                    },
                    SignedInfoTypeSerializerState::SignatureMethod(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = SignedInfoTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = SignedInfoTypeSerializerState::Reference(
                                xsd_parser::quick_xml::IterSerializer::new(
                                    &self.value.reference,
                                    Some("ds:Reference"),
                                    false,
                                ),
                            )
                        }
                    },
                    SignedInfoTypeSerializerState::Reference(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = SignedInfoTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = SignedInfoTypeSerializerState::End__,
                    },
                    SignedInfoTypeSerializerState::End__ => {
                        self.state = SignedInfoTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    SignedInfoTypeSerializerState::Done__ => return None,
                    SignedInfoTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SignatureValueTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::SignatureValueType,
        is_root: bool,
        state: SignatureValueTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum SignatureValueTypeSerializerState<'ser> {
        Init__,
        Content(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SignatureValueTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::SignatureValueType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:SignatureValueType");
            Ok(Self {
                name,
                value,
                is_root,
                state: SignatureValueTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for SignatureValueTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            fn build_attributes<'a>(
                mut bytes: BytesStart<'a>,
                value: &'a super::SignatureValueType,
            ) -> Result<BytesStart<'a>, Error> {
                use xsd_parser::quick_xml::SerializeBytes;
                if let Some(val) = value
                    .id
                    .as_ref()
                    .map(SerializeBytes::serialize_bytes)
                    .transpose()?
                    .flatten()
                {
                    bytes.push_attribute(("ds:Id", val));
                }
                Ok(bytes)
            }
            loop {
                match &mut self.state {
                    SignatureValueTypeSerializerState::Init__ => {
                        self.state = SignatureValueTypeSerializerState::Content(
                            xsd_parser::quick_xml::ContentSerializer::new(
                                &self.value.content,
                                None,
                                false,
                            ),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        match build_attributes(bytes, &self.value) {
                            Ok(bytes) => return Some(Ok(Event::Start(bytes))),
                            Err(error) => {
                                self.state = SignatureValueTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                    }
                    SignatureValueTypeSerializerState::Content(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = SignatureValueTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = SignatureValueTypeSerializerState::End__,
                    },
                    SignatureValueTypeSerializerState::End__ => {
                        self.state = SignatureValueTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    SignatureValueTypeSerializerState::Done__ => return None,
                    SignatureValueTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct KeyInfoTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::KeyInfoType,
        is_root: bool,
        state: KeyInfoTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum KeyInfoTypeSerializerState<'ser> {
        Init__,
        KeyName(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        KeyValue(<KeyValueType as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        RetrievalMethod(
            <RetrievalMethodType as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>,
        ),
        X509Data(<X509DataType as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        Pgpdata(<PgpdataType as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        Spkidata(<SpkidataType as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        MgmtData(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> KeyInfoTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::KeyInfoType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:KeyInfoType");
            Ok(Self {
                name,
                value,
                is_root,
                state: KeyInfoTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for KeyInfoTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            fn build_attributes<'a>(
                mut bytes: BytesStart<'a>,
                value: &'a super::KeyInfoType,
            ) -> Result<BytesStart<'a>, Error> {
                use xsd_parser::quick_xml::SerializeBytes;
                if let Some(val) = value
                    .id
                    .as_ref()
                    .map(SerializeBytes::serialize_bytes)
                    .transpose()?
                    .flatten()
                {
                    bytes.push_attribute(("ds:Id", val));
                }
                Ok(bytes)
            }
            loop {
                match &mut self.state {
                    KeyInfoTypeSerializerState::Init__ => {
                        match &self.value.content {
                            KeyInfoTypeContent::KeyName(x) => {
                                self.state = KeyInfoTypeSerializerState::KeyName(
                                    xsd_parser::quick_xml::ContentSerializer::new(
                                        x,
                                        Some("ds:KeyName"),
                                        false,
                                    ),
                                )
                            }
                            KeyInfoTypeContent::KeyValue(x) => {
                                match WithSerializer::serializer(x, Some("ds:KeyValue"), false) {
                                    Ok(serializer) => {
                                        self.state =
                                            KeyInfoTypeSerializerState::KeyValue(serializer)
                                    }
                                    Err(error) => {
                                        self.state = KeyInfoTypeSerializerState::Done__;
                                        return Some(Err(error));
                                    }
                                }
                            }
                            KeyInfoTypeContent::RetrievalMethod(x) => {
                                match WithSerializer::serializer(
                                    x,
                                    Some("ds:RetrievalMethod"),
                                    false,
                                ) {
                                    Ok(serializer) => {
                                        self.state =
                                            KeyInfoTypeSerializerState::RetrievalMethod(serializer)
                                    }
                                    Err(error) => {
                                        self.state = KeyInfoTypeSerializerState::Done__;
                                        return Some(Err(error));
                                    }
                                }
                            }
                            KeyInfoTypeContent::X509Data(x) => {
                                match WithSerializer::serializer(x, Some("ds:X509Data"), false) {
                                    Ok(serializer) => {
                                        self.state =
                                            KeyInfoTypeSerializerState::X509Data(serializer)
                                    }
                                    Err(error) => {
                                        self.state = KeyInfoTypeSerializerState::Done__;
                                        return Some(Err(error));
                                    }
                                }
                            }
                            KeyInfoTypeContent::Pgpdata(x) => {
                                match WithSerializer::serializer(x, Some("ds:PGPData"), false) {
                                    Ok(serializer) => {
                                        self.state = KeyInfoTypeSerializerState::Pgpdata(serializer)
                                    }
                                    Err(error) => {
                                        self.state = KeyInfoTypeSerializerState::Done__;
                                        return Some(Err(error));
                                    }
                                }
                            }
                            KeyInfoTypeContent::Spkidata(x) => {
                                match WithSerializer::serializer(x, Some("ds:SPKIData"), false) {
                                    Ok(serializer) => {
                                        self.state =
                                            KeyInfoTypeSerializerState::Spkidata(serializer)
                                    }
                                    Err(error) => {
                                        self.state = KeyInfoTypeSerializerState::Done__;
                                        return Some(Err(error));
                                    }
                                }
                            }
                            KeyInfoTypeContent::MgmtData(x) => {
                                self.state = KeyInfoTypeSerializerState::MgmtData(
                                    xsd_parser::quick_xml::ContentSerializer::new(
                                        x,
                                        Some("ds:MgmtData"),
                                        false,
                                    ),
                                )
                            }
                        }
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        match build_attributes(bytes, &self.value) {
                            Ok(bytes) => return Some(Ok(Event::Start(bytes))),
                            Err(error) => {
                                self.state = KeyInfoTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                    }
                    KeyInfoTypeSerializerState::KeyName(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = KeyInfoTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = KeyInfoTypeSerializerState::End__,
                    },
                    KeyInfoTypeSerializerState::KeyValue(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = KeyInfoTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = KeyInfoTypeSerializerState::End__,
                    },
                    KeyInfoTypeSerializerState::RetrievalMethod(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = KeyInfoTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = KeyInfoTypeSerializerState::End__,
                    },
                    KeyInfoTypeSerializerState::X509Data(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = KeyInfoTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = KeyInfoTypeSerializerState::End__,
                    },
                    KeyInfoTypeSerializerState::Pgpdata(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = KeyInfoTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = KeyInfoTypeSerializerState::End__,
                    },
                    KeyInfoTypeSerializerState::Spkidata(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = KeyInfoTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = KeyInfoTypeSerializerState::End__,
                    },
                    KeyInfoTypeSerializerState::MgmtData(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = KeyInfoTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = KeyInfoTypeSerializerState::End__,
                    },
                    KeyInfoTypeSerializerState::End__ => {
                        self.state = KeyInfoTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    KeyInfoTypeSerializerState::Done__ => return None,
                    KeyInfoTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ObjectTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::ObjectType,
        is_root: bool,
        state: ObjectTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum ObjectTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ObjectTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::ObjectType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:ObjectType");
            Ok(Self {
                name,
                value,
                is_root,
                state: ObjectTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for ObjectTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            fn build_attributes<'a>(
                mut bytes: BytesStart<'a>,
                value: &'a super::ObjectType,
            ) -> Result<BytesStart<'a>, Error> {
                use xsd_parser::quick_xml::SerializeBytes;
                if let Some(val) = value
                    .id
                    .as_ref()
                    .map(SerializeBytes::serialize_bytes)
                    .transpose()?
                    .flatten()
                {
                    bytes.push_attribute(("ds:Id", val));
                }
                if let Some(val) = value
                    .mime_type
                    .as_ref()
                    .map(SerializeBytes::serialize_bytes)
                    .transpose()?
                    .flatten()
                {
                    bytes.push_attribute(("ds:MimeType", val));
                }
                if let Some(val) = value
                    .encoding
                    .as_ref()
                    .map(SerializeBytes::serialize_bytes)
                    .transpose()?
                    .flatten()
                {
                    bytes.push_attribute(("ds:Encoding", val));
                }
                Ok(bytes)
            }
            loop {
                match &mut self.state {
                    ObjectTypeSerializerState::Init__ => {
                        self.state = ObjectTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        match build_attributes(bytes, &self.value) {
                            Ok(bytes) => return Some(Ok(Event::Empty(bytes))),
                            Err(error) => {
                                self.state = ObjectTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                    }
                    ObjectTypeSerializerState::Done__ => return None,
                    ObjectTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct CanonicalizationMethodTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::CanonicalizationMethodType,
        is_root: bool,
        state: CanonicalizationMethodTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum CanonicalizationMethodTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CanonicalizationMethodTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::CanonicalizationMethodType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:CanonicalizationMethodType");
            Ok(Self {
                name,
                value,
                is_root,
                state: CanonicalizationMethodTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for CanonicalizationMethodTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            fn build_attributes<'a>(
                mut bytes: BytesStart<'a>,
                value: &'a super::CanonicalizationMethodType,
            ) -> Result<BytesStart<'a>, Error> {
                use xsd_parser::quick_xml::SerializeBytes;
                if let Some(val) = SerializeBytes::serialize_bytes(&value.algorithm)? {
                    bytes.push_attribute(("ds:Algorithm", val));
                }
                Ok(bytes)
            }
            loop {
                match &mut self.state {
                    CanonicalizationMethodTypeSerializerState::Init__ => {
                        self.state = CanonicalizationMethodTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        match build_attributes(bytes, &self.value) {
                            Ok(bytes) => return Some(Ok(Event::Empty(bytes))),
                            Err(error) => {
                                self.state = CanonicalizationMethodTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                    }
                    CanonicalizationMethodTypeSerializerState::Done__ => return None,
                    CanonicalizationMethodTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SignatureMethodTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::SignatureMethodType,
        is_root: bool,
        state: SignatureMethodTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum SignatureMethodTypeSerializerState<'ser> {
        Init__,
        HmacoutputLength(xsd_parser::quick_xml::IterSerializer<'ser, Option<i32>, i32>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SignatureMethodTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::SignatureMethodType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:SignatureMethodType");
            Ok(Self {
                name,
                value,
                is_root,
                state: SignatureMethodTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for SignatureMethodTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            fn build_attributes<'a>(
                mut bytes: BytesStart<'a>,
                value: &'a super::SignatureMethodType,
            ) -> Result<BytesStart<'a>, Error> {
                use xsd_parser::quick_xml::SerializeBytes;
                if let Some(val) = SerializeBytes::serialize_bytes(&value.algorithm)? {
                    bytes.push_attribute(("ds:Algorithm", val));
                }
                Ok(bytes)
            }
            loop {
                match &mut self.state {
                    SignatureMethodTypeSerializerState::Init__ => {
                        self.state = SignatureMethodTypeSerializerState::HmacoutputLength(
                            xsd_parser::quick_xml::IterSerializer::new(
                                &self.value.hmacoutput_length,
                                Some("ds:HMACOutputLength"),
                                false,
                            ),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        match build_attributes(bytes, &self.value) {
                            Ok(bytes) => return Some(Ok(Event::Start(bytes))),
                            Err(error) => {
                                self.state = SignatureMethodTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                    }
                    SignatureMethodTypeSerializerState::HmacoutputLength(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = SignatureMethodTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = SignatureMethodTypeSerializerState::End__,
                    },
                    SignatureMethodTypeSerializerState::End__ => {
                        self.state = SignatureMethodTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    SignatureMethodTypeSerializerState::Done__ => return None,
                    SignatureMethodTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ReferenceTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::ReferenceType,
        is_root: bool,
        state: ReferenceTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum ReferenceTypeSerializerState<'ser> {
        Init__,
        Transforms(
            xsd_parser::quick_xml::IterSerializer<'ser, Option<TransformsType>, TransformsType>,
        ),
        DigestMethod(<DigestMethodType as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        DigestValue(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ReferenceTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::ReferenceType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:ReferenceType");
            Ok(Self {
                name,
                value,
                is_root,
                state: ReferenceTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for ReferenceTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            fn build_attributes<'a>(
                mut bytes: BytesStart<'a>,
                value: &'a super::ReferenceType,
            ) -> Result<BytesStart<'a>, Error> {
                use xsd_parser::quick_xml::SerializeBytes;
                if let Some(val) = value
                    .id
                    .as_ref()
                    .map(SerializeBytes::serialize_bytes)
                    .transpose()?
                    .flatten()
                {
                    bytes.push_attribute(("ds:Id", val));
                }
                if let Some(val) = value
                    .uri
                    .as_ref()
                    .map(SerializeBytes::serialize_bytes)
                    .transpose()?
                    .flatten()
                {
                    bytes.push_attribute(("ds:URI", val));
                }
                if let Some(val) = value
                    .type_
                    .as_ref()
                    .map(SerializeBytes::serialize_bytes)
                    .transpose()?
                    .flatten()
                {
                    bytes.push_attribute(("ds:Type", val));
                }
                Ok(bytes)
            }
            loop {
                match &mut self.state {
                    ReferenceTypeSerializerState::Init__ => {
                        self.state = ReferenceTypeSerializerState::Transforms(
                            xsd_parser::quick_xml::IterSerializer::new(
                                &self.value.transforms,
                                Some("ds:Transforms"),
                                false,
                            ),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        match build_attributes(bytes, &self.value) {
                            Ok(bytes) => return Some(Ok(Event::Start(bytes))),
                            Err(error) => {
                                self.state = ReferenceTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                    }
                    ReferenceTypeSerializerState::Transforms(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ReferenceTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => match WithSerializer::serializer(
                            &self.value.digest_method,
                            Some("ds:DigestMethod"),
                            false,
                        ) {
                            Ok(serializer) => {
                                self.state = ReferenceTypeSerializerState::DigestMethod(serializer)
                            }
                            Err(error) => {
                                self.state = ReferenceTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        },
                    },
                    ReferenceTypeSerializerState::DigestMethod(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ReferenceTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = ReferenceTypeSerializerState::DigestValue(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    &self.value.digest_value,
                                    Some("ds:DigestValue"),
                                    false,
                                ),
                            )
                        }
                    },
                    ReferenceTypeSerializerState::DigestValue(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ReferenceTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = ReferenceTypeSerializerState::End__,
                    },
                    ReferenceTypeSerializerState::End__ => {
                        self.state = ReferenceTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    ReferenceTypeSerializerState::Done__ => return None,
                    ReferenceTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct KeyValueTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::KeyValueType,
        is_root: bool,
        state: KeyValueTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum KeyValueTypeSerializerState<'ser> {
        Init__,
        DsakeyValue(<DsakeyValueType as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        RsakeyValue(<RsakeyValueType as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> KeyValueTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::KeyValueType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:KeyValueType");
            Ok(Self {
                name,
                value,
                is_root,
                state: KeyValueTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for KeyValueTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    KeyValueTypeSerializerState::Init__ => {
                        match &self.value.content {
                            KeyValueTypeContent::DsakeyValue(x) => {
                                match WithSerializer::serializer(x, Some("ds:DSAKeyValue"), false) {
                                    Ok(serializer) => {
                                        self.state =
                                            KeyValueTypeSerializerState::DsakeyValue(serializer)
                                    }
                                    Err(error) => {
                                        self.state = KeyValueTypeSerializerState::Done__;
                                        return Some(Err(error));
                                    }
                                }
                            }
                            KeyValueTypeContent::RsakeyValue(x) => {
                                match WithSerializer::serializer(x, Some("ds:RSAKeyValue"), false) {
                                    Ok(serializer) => {
                                        self.state =
                                            KeyValueTypeSerializerState::RsakeyValue(serializer)
                                    }
                                    Err(error) => {
                                        self.state = KeyValueTypeSerializerState::Done__;
                                        return Some(Err(error));
                                    }
                                }
                            }
                        }
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        return Some(Ok(Event::Start(bytes)));
                    }
                    KeyValueTypeSerializerState::DsakeyValue(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = KeyValueTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = KeyValueTypeSerializerState::End__,
                    },
                    KeyValueTypeSerializerState::RsakeyValue(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = KeyValueTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = KeyValueTypeSerializerState::End__,
                    },
                    KeyValueTypeSerializerState::End__ => {
                        self.state = KeyValueTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    KeyValueTypeSerializerState::Done__ => return None,
                    KeyValueTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct RetrievalMethodTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::RetrievalMethodType,
        is_root: bool,
        state: RetrievalMethodTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum RetrievalMethodTypeSerializerState<'ser> {
        Init__,
        Transforms(
            xsd_parser::quick_xml::IterSerializer<'ser, Option<TransformsType>, TransformsType>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RetrievalMethodTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::RetrievalMethodType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:RetrievalMethodType");
            Ok(Self {
                name,
                value,
                is_root,
                state: RetrievalMethodTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for RetrievalMethodTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            fn build_attributes<'a>(
                mut bytes: BytesStart<'a>,
                value: &'a super::RetrievalMethodType,
            ) -> Result<BytesStart<'a>, Error> {
                use xsd_parser::quick_xml::SerializeBytes;
                if let Some(val) = value
                    .uri
                    .as_ref()
                    .map(SerializeBytes::serialize_bytes)
                    .transpose()?
                    .flatten()
                {
                    bytes.push_attribute(("ds:URI", val));
                }
                if let Some(val) = value
                    .type_
                    .as_ref()
                    .map(SerializeBytes::serialize_bytes)
                    .transpose()?
                    .flatten()
                {
                    bytes.push_attribute(("ds:Type", val));
                }
                Ok(bytes)
            }
            loop {
                match &mut self.state {
                    RetrievalMethodTypeSerializerState::Init__ => {
                        self.state = RetrievalMethodTypeSerializerState::Transforms(
                            xsd_parser::quick_xml::IterSerializer::new(
                                &self.value.transforms,
                                Some("ds:Transforms"),
                                false,
                            ),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        match build_attributes(bytes, &self.value) {
                            Ok(bytes) => return Some(Ok(Event::Start(bytes))),
                            Err(error) => {
                                self.state = RetrievalMethodTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                    }
                    RetrievalMethodTypeSerializerState::Transforms(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = RetrievalMethodTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = RetrievalMethodTypeSerializerState::End__,
                    },
                    RetrievalMethodTypeSerializerState::End__ => {
                        self.state = RetrievalMethodTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    RetrievalMethodTypeSerializerState::Done__ => return None,
                    RetrievalMethodTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct X509DataTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::X509DataType,
        is_root: bool,
        state: X509DataTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum X509DataTypeSerializerState<'ser> {
        Init__,
        Content103(
            <X509DataContent103Type as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> X509DataTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::X509DataType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:X509DataType");
            Ok(Self {
                name,
                value,
                is_root,
                state: X509DataTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for X509DataTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    X509DataTypeSerializerState::Init__ => {
                        match WithSerializer::serializer(
                            &self.value.content_103,
                            Some("ds:Content103"),
                            false,
                        ) {
                            Ok(serializer) => {
                                self.state = X509DataTypeSerializerState::Content103(serializer)
                            }
                            Err(error) => {
                                self.state = X509DataTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        };
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        return Some(Ok(Event::Start(bytes)));
                    }
                    X509DataTypeSerializerState::Content103(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = X509DataTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = X509DataTypeSerializerState::End__,
                    },
                    X509DataTypeSerializerState::End__ => {
                        self.state = X509DataTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    X509DataTypeSerializerState::Done__ => return None,
                    X509DataTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PgpdataTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::PgpdataType,
        is_root: bool,
        state: PgpdataTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum PgpdataTypeSerializerState<'ser> {
        Init__,
        Content113(
            <PgpdataContent113Type as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>,
        ),
        Content116(
            <PgpdataContent116Type as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PgpdataTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::PgpdataType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:PGPDataType");
            Ok(Self {
                name,
                value,
                is_root,
                state: PgpdataTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for PgpdataTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    PgpdataTypeSerializerState::Init__ => {
                        match &self.value.content {
                            PgpdataTypeContent::Content113(x) => {
                                match WithSerializer::serializer(x, Some("ds:Content113"), false) {
                                    Ok(serializer) => {
                                        self.state =
                                            PgpdataTypeSerializerState::Content113(serializer)
                                    }
                                    Err(error) => {
                                        self.state = PgpdataTypeSerializerState::Done__;
                                        return Some(Err(error));
                                    }
                                }
                            }
                            PgpdataTypeContent::Content116(x) => {
                                match WithSerializer::serializer(x, Some("ds:Content116"), false) {
                                    Ok(serializer) => {
                                        self.state =
                                            PgpdataTypeSerializerState::Content116(serializer)
                                    }
                                    Err(error) => {
                                        self.state = PgpdataTypeSerializerState::Done__;
                                        return Some(Err(error));
                                    }
                                }
                            }
                        }
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        return Some(Ok(Event::Start(bytes)));
                    }
                    PgpdataTypeSerializerState::Content113(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = PgpdataTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = PgpdataTypeSerializerState::End__,
                    },
                    PgpdataTypeSerializerState::Content116(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = PgpdataTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = PgpdataTypeSerializerState::End__,
                    },
                    PgpdataTypeSerializerState::End__ => {
                        self.state = PgpdataTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    PgpdataTypeSerializerState::Done__ => return None,
                    PgpdataTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SpkidataTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::SpkidataType,
        is_root: bool,
        state: SpkidataTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum SpkidataTypeSerializerState<'ser> {
        Init__,
        Spkisexp(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SpkidataTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::SpkidataType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:SPKIDataType");
            Ok(Self {
                name,
                value,
                is_root,
                state: SpkidataTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for SpkidataTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    SpkidataTypeSerializerState::Init__ => {
                        self.state = SpkidataTypeSerializerState::Spkisexp(
                            xsd_parser::quick_xml::ContentSerializer::new(
                                &self.value.spkisexp,
                                Some("ds:SPKISexp"),
                                false,
                            ),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        return Some(Ok(Event::Start(bytes)));
                    }
                    SpkidataTypeSerializerState::Spkisexp(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = SpkidataTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = SpkidataTypeSerializerState::End__,
                    },
                    SpkidataTypeSerializerState::End__ => {
                        self.state = SpkidataTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    SpkidataTypeSerializerState::Done__ => return None,
                    SpkidataTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TransformsTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::TransformsType,
        is_root: bool,
        state: TransformsTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum TransformsTypeSerializerState<'ser> {
        Init__,
        Transform(xsd_parser::quick_xml::IterSerializer<'ser, Vec<TransformType>, TransformType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TransformsTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::TransformsType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:TransformsType");
            Ok(Self {
                name,
                value,
                is_root,
                state: TransformsTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for TransformsTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    TransformsTypeSerializerState::Init__ => {
                        self.state = TransformsTypeSerializerState::Transform(
                            xsd_parser::quick_xml::IterSerializer::new(
                                &self.value.transform,
                                Some("ds:Transform"),
                                false,
                            ),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        return Some(Ok(Event::Start(bytes)));
                    }
                    TransformsTypeSerializerState::Transform(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = TransformsTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = TransformsTypeSerializerState::End__,
                    },
                    TransformsTypeSerializerState::End__ => {
                        self.state = TransformsTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    TransformsTypeSerializerState::Done__ => return None,
                    TransformsTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DigestMethodTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::DigestMethodType,
        is_root: bool,
        state: DigestMethodTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum DigestMethodTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DigestMethodTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::DigestMethodType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:DigestMethodType");
            Ok(Self {
                name,
                value,
                is_root,
                state: DigestMethodTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for DigestMethodTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            fn build_attributes<'a>(
                mut bytes: BytesStart<'a>,
                value: &'a super::DigestMethodType,
            ) -> Result<BytesStart<'a>, Error> {
                use xsd_parser::quick_xml::SerializeBytes;
                if let Some(val) = SerializeBytes::serialize_bytes(&value.algorithm)? {
                    bytes.push_attribute(("ds:Algorithm", val));
                }
                Ok(bytes)
            }
            loop {
                match &mut self.state {
                    DigestMethodTypeSerializerState::Init__ => {
                        self.state = DigestMethodTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        match build_attributes(bytes, &self.value) {
                            Ok(bytes) => return Some(Ok(Event::Empty(bytes))),
                            Err(error) => {
                                self.state = DigestMethodTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                    }
                    DigestMethodTypeSerializerState::Done__ => return None,
                    DigestMethodTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DsakeyValueTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::DsakeyValueType,
        is_root: bool,
        state: DsakeyValueTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum DsakeyValueTypeSerializerState<'ser> {
        Init__,
        Content125(
            xsd_parser::quick_xml::IterSerializer<
                'ser,
                Option<DsakeyValueContent125Type>,
                DsakeyValueContent125Type,
            >,
        ),
        G(xsd_parser::quick_xml::IterSerializer<'ser, Option<String>, String>),
        Y(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        J(xsd_parser::quick_xml::IterSerializer<'ser, Option<String>, String>),
        Content131(
            xsd_parser::quick_xml::IterSerializer<
                'ser,
                Option<DsakeyValueContent131Type>,
                DsakeyValueContent131Type,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DsakeyValueTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::DsakeyValueType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:DSAKeyValueType");
            Ok(Self {
                name,
                value,
                is_root,
                state: DsakeyValueTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for DsakeyValueTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    DsakeyValueTypeSerializerState::Init__ => {
                        self.state = DsakeyValueTypeSerializerState::Content125(
                            xsd_parser::quick_xml::IterSerializer::new(
                                &self.value.content_125,
                                Some("ds:Content125"),
                                false,
                            ),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        return Some(Ok(Event::Start(bytes)));
                    }
                    DsakeyValueTypeSerializerState::Content125(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = DsakeyValueTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = DsakeyValueTypeSerializerState::G(
                                xsd_parser::quick_xml::IterSerializer::new(
                                    &self.value.g,
                                    Some("ds:G"),
                                    false,
                                ),
                            )
                        }
                    },
                    DsakeyValueTypeSerializerState::G(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = DsakeyValueTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = DsakeyValueTypeSerializerState::Y(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    &self.value.y,
                                    Some("ds:Y"),
                                    false,
                                ),
                            )
                        }
                    },
                    DsakeyValueTypeSerializerState::Y(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = DsakeyValueTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = DsakeyValueTypeSerializerState::J(
                                xsd_parser::quick_xml::IterSerializer::new(
                                    &self.value.j,
                                    Some("ds:J"),
                                    false,
                                ),
                            )
                        }
                    },
                    DsakeyValueTypeSerializerState::J(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = DsakeyValueTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = DsakeyValueTypeSerializerState::Content131(
                                xsd_parser::quick_xml::IterSerializer::new(
                                    &self.value.content_131,
                                    Some("ds:Content131"),
                                    false,
                                ),
                            )
                        }
                    },
                    DsakeyValueTypeSerializerState::Content131(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = DsakeyValueTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = DsakeyValueTypeSerializerState::End__,
                    },
                    DsakeyValueTypeSerializerState::End__ => {
                        self.state = DsakeyValueTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    DsakeyValueTypeSerializerState::Done__ => return None,
                    DsakeyValueTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct RsakeyValueTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::RsakeyValueType,
        is_root: bool,
        state: RsakeyValueTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum RsakeyValueTypeSerializerState<'ser> {
        Init__,
        Modulus(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        Exponent(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RsakeyValueTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::RsakeyValueType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:RSAKeyValueType");
            Ok(Self {
                name,
                value,
                is_root,
                state: RsakeyValueTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for RsakeyValueTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    RsakeyValueTypeSerializerState::Init__ => {
                        self.state = RsakeyValueTypeSerializerState::Modulus(
                            xsd_parser::quick_xml::ContentSerializer::new(
                                &self.value.modulus,
                                Some("ds:Modulus"),
                                false,
                            ),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        return Some(Ok(Event::Start(bytes)));
                    }
                    RsakeyValueTypeSerializerState::Modulus(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = RsakeyValueTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = RsakeyValueTypeSerializerState::Exponent(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    &self.value.exponent,
                                    Some("ds:Exponent"),
                                    false,
                                ),
                            )
                        }
                    },
                    RsakeyValueTypeSerializerState::Exponent(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = RsakeyValueTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = RsakeyValueTypeSerializerState::End__,
                    },
                    RsakeyValueTypeSerializerState::End__ => {
                        self.state = RsakeyValueTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    RsakeyValueTypeSerializerState::Done__ => return None,
                    RsakeyValueTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct X509DataContent103TypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::X509DataContent103Type,
        is_root: bool,
        state: X509DataContent103TypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum X509DataContent103TypeSerializerState<'ser> {
        Init__,
        X509IssuerSerial(
            <X509IssuerSerialType as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>,
        ),
        X509Ski(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        X509SubjectName(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        X509Certificate(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        X509Crl(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> X509DataContent103TypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::X509DataContent103Type,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:X509DataContent103");
            Ok(Self {
                name,
                value,
                is_root,
                state: X509DataContent103TypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for X509DataContent103TypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    X509DataContent103TypeSerializerState::Init__ => match &self.value.content {
                        X509DataContent103TypeContent::X509IssuerSerial(x) => {
                            match WithSerializer::serializer(x, Some("ds:X509IssuerSerial"), false)
                            {
                                Ok(serializer) => {
                                    self.state =
                                        X509DataContent103TypeSerializerState::X509IssuerSerial(
                                            serializer,
                                        )
                                }
                                Err(error) => {
                                    self.state = X509DataContent103TypeSerializerState::Done__;
                                    return Some(Err(error));
                                }
                            }
                        }
                        X509DataContent103TypeContent::X509Ski(x) => {
                            self.state = X509DataContent103TypeSerializerState::X509Ski(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    x,
                                    Some("ds:X509SKI"),
                                    false,
                                ),
                            )
                        }
                        X509DataContent103TypeContent::X509SubjectName(x) => {
                            self.state = X509DataContent103TypeSerializerState::X509SubjectName(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    x,
                                    Some("ds:X509SubjectName"),
                                    false,
                                ),
                            )
                        }
                        X509DataContent103TypeContent::X509Certificate(x) => {
                            self.state = X509DataContent103TypeSerializerState::X509Certificate(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    x,
                                    Some("ds:X509Certificate"),
                                    false,
                                ),
                            )
                        }
                        X509DataContent103TypeContent::X509Crl(x) => {
                            self.state = X509DataContent103TypeSerializerState::X509Crl(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    x,
                                    Some("ds:X509CRL"),
                                    false,
                                ),
                            )
                        }
                    },
                    X509DataContent103TypeSerializerState::X509IssuerSerial(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = X509DataContent103TypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = X509DataContent103TypeSerializerState::Done__,
                    },
                    X509DataContent103TypeSerializerState::X509Ski(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = X509DataContent103TypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = X509DataContent103TypeSerializerState::Done__,
                    },
                    X509DataContent103TypeSerializerState::X509SubjectName(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = X509DataContent103TypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = X509DataContent103TypeSerializerState::Done__,
                    },
                    X509DataContent103TypeSerializerState::X509Certificate(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = X509DataContent103TypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = X509DataContent103TypeSerializerState::Done__,
                    },
                    X509DataContent103TypeSerializerState::X509Crl(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = X509DataContent103TypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = X509DataContent103TypeSerializerState::Done__,
                    },
                    X509DataContent103TypeSerializerState::Done__ => return None,
                    X509DataContent103TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PgpdataContent113TypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::PgpdataContent113Type,
        is_root: bool,
        state: PgpdataContent113TypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum PgpdataContent113TypeSerializerState<'ser> {
        Init__,
        PgpkeyID(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        PgpkeyPacket(xsd_parser::quick_xml::IterSerializer<'ser, Option<String>, String>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PgpdataContent113TypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::PgpdataContent113Type,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:PgpdataContent113");
            Ok(Self {
                name,
                value,
                is_root,
                state: PgpdataContent113TypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for PgpdataContent113TypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    PgpdataContent113TypeSerializerState::Init__ => {
                        self.state = PgpdataContent113TypeSerializerState::PgpkeyID(
                            xsd_parser::quick_xml::ContentSerializer::new(
                                &self.value.pgpkey_id,
                                Some("ds:PGPKeyID"),
                                false,
                            ),
                        );
                    }
                    PgpdataContent113TypeSerializerState::PgpkeyID(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = PgpdataContent113TypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = PgpdataContent113TypeSerializerState::PgpkeyPacket(
                                xsd_parser::quick_xml::IterSerializer::new(
                                    &self.value.pgpkey_packet,
                                    Some("ds:PGPKeyPacket"),
                                    false,
                                ),
                            )
                        }
                    },
                    PgpdataContent113TypeSerializerState::PgpkeyPacket(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = PgpdataContent113TypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = PgpdataContent113TypeSerializerState::Done__,
                    },
                    PgpdataContent113TypeSerializerState::Done__ => return None,
                    PgpdataContent113TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PgpdataContent116TypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::PgpdataContent116Type,
        is_root: bool,
        state: PgpdataContent116TypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum PgpdataContent116TypeSerializerState<'ser> {
        Init__,
        PgpkeyPacket(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PgpdataContent116TypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::PgpdataContent116Type,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:PgpdataContent116");
            Ok(Self {
                name,
                value,
                is_root,
                state: PgpdataContent116TypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for PgpdataContent116TypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    PgpdataContent116TypeSerializerState::Init__ => {
                        self.state = PgpdataContent116TypeSerializerState::PgpkeyPacket(
                            xsd_parser::quick_xml::ContentSerializer::new(
                                &self.value.pgpkey_packet,
                                Some("ds:PGPKeyPacket"),
                                false,
                            ),
                        );
                    }
                    PgpdataContent116TypeSerializerState::PgpkeyPacket(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = PgpdataContent116TypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = PgpdataContent116TypeSerializerState::Done__,
                    },
                    PgpdataContent116TypeSerializerState::Done__ => return None,
                    PgpdataContent116TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TransformTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::TransformType,
        is_root: bool,
        state: TransformTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum TransformTypeSerializerState<'ser> {
        Init__,
        Xpath(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TransformTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::TransformType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:TransformType");
            Ok(Self {
                name,
                value,
                is_root,
                state: TransformTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for TransformTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            fn build_attributes<'a>(
                mut bytes: BytesStart<'a>,
                value: &'a super::TransformType,
            ) -> Result<BytesStart<'a>, Error> {
                use xsd_parser::quick_xml::SerializeBytes;
                if let Some(val) = SerializeBytes::serialize_bytes(&value.algorithm)? {
                    bytes.push_attribute(("ds:Algorithm", val));
                }
                Ok(bytes)
            }
            loop {
                match &mut self.state {
                    TransformTypeSerializerState::Init__ => {
                        match &self.value.content {
                            TransformTypeContent::Xpath(x) => {
                                self.state = TransformTypeSerializerState::Xpath(
                                    xsd_parser::quick_xml::ContentSerializer::new(
                                        x,
                                        Some("ds:XPath"),
                                        false,
                                    ),
                                )
                            }
                        }
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        match build_attributes(bytes, &self.value) {
                            Ok(bytes) => return Some(Ok(Event::Start(bytes))),
                            Err(error) => {
                                self.state = TransformTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                    }
                    TransformTypeSerializerState::Xpath(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = TransformTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = TransformTypeSerializerState::End__,
                    },
                    TransformTypeSerializerState::End__ => {
                        self.state = TransformTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    TransformTypeSerializerState::Done__ => return None,
                    TransformTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DsakeyValueContent125TypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::DsakeyValueContent125Type,
        is_root: bool,
        state: DsakeyValueContent125TypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum DsakeyValueContent125TypeSerializerState<'ser> {
        Init__,
        P(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        Q(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DsakeyValueContent125TypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::DsakeyValueContent125Type,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:DsakeyValueContent125");
            Ok(Self {
                name,
                value,
                is_root,
                state: DsakeyValueContent125TypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for DsakeyValueContent125TypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    DsakeyValueContent125TypeSerializerState::Init__ => {
                        self.state = DsakeyValueContent125TypeSerializerState::P(
                            xsd_parser::quick_xml::ContentSerializer::new(
                                &self.value.p,
                                Some("ds:P"),
                                false,
                            ),
                        );
                    }
                    DsakeyValueContent125TypeSerializerState::P(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = DsakeyValueContent125TypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = DsakeyValueContent125TypeSerializerState::Q(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    &self.value.q,
                                    Some("ds:Q"),
                                    false,
                                ),
                            )
                        }
                    },
                    DsakeyValueContent125TypeSerializerState::Q(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = DsakeyValueContent125TypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = DsakeyValueContent125TypeSerializerState::Done__,
                    },
                    DsakeyValueContent125TypeSerializerState::Done__ => return None,
                    DsakeyValueContent125TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DsakeyValueContent131TypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::DsakeyValueContent131Type,
        is_root: bool,
        state: DsakeyValueContent131TypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum DsakeyValueContent131TypeSerializerState<'ser> {
        Init__,
        Seed(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        PgenCounter(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DsakeyValueContent131TypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::DsakeyValueContent131Type,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:DsakeyValueContent131");
            Ok(Self {
                name,
                value,
                is_root,
                state: DsakeyValueContent131TypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for DsakeyValueContent131TypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    DsakeyValueContent131TypeSerializerState::Init__ => {
                        self.state = DsakeyValueContent131TypeSerializerState::Seed(
                            xsd_parser::quick_xml::ContentSerializer::new(
                                &self.value.seed,
                                Some("ds:Seed"),
                                false,
                            ),
                        );
                    }
                    DsakeyValueContent131TypeSerializerState::Seed(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = DsakeyValueContent131TypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = DsakeyValueContent131TypeSerializerState::PgenCounter(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    &self.value.pgen_counter,
                                    Some("ds:PgenCounter"),
                                    false,
                                ),
                            )
                        }
                    },
                    DsakeyValueContent131TypeSerializerState::PgenCounter(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = DsakeyValueContent131TypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = DsakeyValueContent131TypeSerializerState::Done__,
                    },
                    DsakeyValueContent131TypeSerializerState::Done__ => return None,
                    DsakeyValueContent131TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct X509IssuerSerialTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::X509IssuerSerialType,
        is_root: bool,
        state: X509IssuerSerialTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum X509IssuerSerialTypeSerializerState<'ser> {
        Init__,
        X509IssuerName(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        X509SerialNumber(xsd_parser::quick_xml::ContentSerializer<'ser, i32>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> X509IssuerSerialTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::X509IssuerSerialType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ds:X509IssuerSerialType");
            Ok(Self {
                name,
                value,
                is_root,
                state: X509IssuerSerialTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for X509IssuerSerialTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    X509IssuerSerialTypeSerializerState::Init__ => {
                        self.state = X509IssuerSerialTypeSerializerState::X509IssuerName(
                            xsd_parser::quick_xml::ContentSerializer::new(
                                &self.value.x509_issuer_name,
                                Some("ds:X509IssuerName"),
                                false,
                            ),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes
                                .push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
                        }
                        return Some(Ok(Event::Start(bytes)));
                    }
                    X509IssuerSerialTypeSerializerState::X509IssuerName(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = X509IssuerSerialTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = X509IssuerSerialTypeSerializerState::X509SerialNumber(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    &self.value.x509_serial_number,
                                    Some("ds:X509SerialNumber"),
                                    false,
                                ),
                            )
                        }
                    },
                    X509IssuerSerialTypeSerializerState::X509SerialNumber(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = X509IssuerSerialTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = X509IssuerSerialTypeSerializerState::End__,
                    },
                    X509IssuerSerialTypeSerializerState::End__ => {
                        self.state = X509IssuerSerialTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    X509IssuerSerialTypeSerializerState::Done__ => return None,
                    X509IssuerSerialTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
}
pub mod quick_xml_deserialize {
    use super::*;
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
        CreateDateTimestamp(
            Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>,
        ),
        Merchant(
            Option<
                <DirectoryReqMerchantType as xsd_parser::quick_xml::WithDeserializer>::Deserializer,
            >,
        ),
        Signature(Option<<SignatureType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl DirectoryReqTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_DEFAULT: &[u8] = b"http://www.idealdesk.com/ideal/messages/mer-acq/3.3.1";
            let mut version: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(
                    reader.resolve_local_name(attrib.key, NS_DEFAULT),
                    Some(b"version")
                ) {
                    reader.read_attrib(&mut version, b"version", &attrib.value)?;
                } else {
                    reader.err(ErrorKind::UnexpectedAttribute(
                        xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                    ))?;
                }
            }
            Ok(Self {
                version: version.ok_or(ErrorKind::MissingAttribute("version".into()))?,
                create_date_timestamp: None,
                merchant: None,
                signature: None,
                state: Box::new(DirectoryReqTypeDeserializerState::CreateDateTimestamp(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::DirectoryReqType>
        for DirectoryReqTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::DirectoryReqType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::DirectoryReqType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DEFAULT: &[u8] = b"http://www.idealdesk.com/ideal/messages/mer-acq/3.3.1";
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, DirectoryReqTypeDeserializerState::Done__),
                    event,
                ) {
                    (
                        DirectoryReqTypeDeserializerState::CreateDateTimestamp(Some(deserializer)),
                        event,
                    ) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.create_date_timestamp.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"createDateTimestamp",
                                )))?;
                            }
                            self.create_date_timestamp = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    DirectoryReqTypeDeserializerState::CreateDateTimestamp(
                                        deserializer,
                                    );
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                DirectoryReqTypeDeserializerState::CreateDateTimestamp(
                                    deserializer,
                                ),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.create_date_timestamp.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"createDateTimestamp",
                                )))?;
                            }
                            self.create_date_timestamp = Some(data);
                        }
                        *self.state = DirectoryReqTypeDeserializerState::CreateDateTimestamp(None);
                        event
                    }
                    (DirectoryReqTypeDeserializerState::CreateDateTimestamp(None), event) => {
                        match &event {
                            Event::Start(x) | Event::Empty(x)
                                if matches!(
                                    reader.resolve_local_name(x.name(), NS_DEFAULT),
                                    Some(b"createDateTimestamp")
                                ) =>
                            {
                                let DeserializerOutput {
                                    data,
                                    deserializer,
                                    event,
                                    allow_any,
                                } = <String as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                                if let Some(data) = data {
                                    if self.create_date_timestamp.is_some() {
                                        Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                            b"createDateTimestamp",
                                        )))?;
                                    }
                                    self.create_date_timestamp = Some(data);
                                }
                                *self.state =
                                    DirectoryReqTypeDeserializerState::CreateDateTimestamp(
                                        deserializer,
                                    );
                                match event {
                                    Some(event @ (Event::Start(_) | Event::End(_))) => {
                                        *self.state =
                                            DirectoryReqTypeDeserializerState::Merchant(None);
                                        if allow_any {
                                            allow_any_fallback . get_or_insert (DirectoryReqTypeDeserializerState :: CreateDateTimestamp (None)) ;
                                        }
                                        event
                                    }
                                    event @ (None | Some(_)) => {
                                        return Ok(DeserializerOutput {
                                            data: None,
                                            deserializer: Some(self),
                                            event,
                                            allow_any: false,
                                        })
                                    }
                                }
                            }
                            Event::Start(_) | Event::Empty(_) => {
                                *self.state = DirectoryReqTypeDeserializerState::Merchant(None);
                                event
                            }
                            Event::End(_) => {
                                let data = self.finish(reader)?;
                                return Ok(DeserializerOutput {
                                    data: Some(data),
                                    deserializer: None,
                                    event: None,
                                    allow_any: false,
                                });
                            }
                            _ => {
                                *self.state =
                                    DirectoryReqTypeDeserializerState::CreateDateTimestamp(None);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: Some(event),
                                    allow_any: false,
                                });
                            }
                        }
                    }
                    (DirectoryReqTypeDeserializerState::Merchant(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.merchant.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Merchant",
                                )))?;
                            }
                            self.merchant = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    DirectoryReqTypeDeserializerState::Merchant(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                DirectoryReqTypeDeserializerState::Merchant(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.merchant.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Merchant",
                                )))?;
                            }
                            self.merchant = Some(data);
                        }
                        *self.state = DirectoryReqTypeDeserializerState::Merchant(None);
                        event
                    }
                    (DirectoryReqTypeDeserializerState::Merchant(None), event) => {
                        match &event {
                            Event::Start(x) | Event::Empty(x)
                                if matches!(
                                    reader.resolve_local_name(x.name(), NS_DEFAULT),
                                    Some(b"Merchant")
                                ) =>
                            {
                                let DeserializerOutput { data , deserializer , event , allow_any } = < DirectoryReqMerchantType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                                if let Some(data) = data {
                                    if self.merchant.is_some() {
                                        Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                            b"Merchant",
                                        )))?;
                                    }
                                    self.merchant = Some(data);
                                }
                                *self.state =
                                    DirectoryReqTypeDeserializerState::Merchant(deserializer);
                                match event {
                                    Some(event @ (Event::Start(_) | Event::End(_))) => {
                                        *self.state =
                                            DirectoryReqTypeDeserializerState::Signature(None);
                                        if allow_any {
                                            allow_any_fallback.get_or_insert(
                                                DirectoryReqTypeDeserializerState::Merchant(None),
                                            );
                                        }
                                        event
                                    }
                                    event @ (None | Some(_)) => {
                                        return Ok(DeserializerOutput {
                                            data: None,
                                            deserializer: Some(self),
                                            event,
                                            allow_any: false,
                                        })
                                    }
                                }
                            }
                            Event::Start(_) | Event::Empty(_) => {
                                *self.state = DirectoryReqTypeDeserializerState::Signature(None);
                                event
                            }
                            Event::End(_) => {
                                let data = self.finish(reader)?;
                                return Ok(DeserializerOutput {
                                    data: Some(data),
                                    deserializer: None,
                                    event: None,
                                    allow_any: false,
                                });
                            }
                            _ => {
                                *self.state = DirectoryReqTypeDeserializerState::Merchant(None);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: Some(event),
                                    allow_any: false,
                                });
                            }
                        }
                    }
                    (DirectoryReqTypeDeserializerState::Signature(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.signature.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Signature",
                                )))?;
                            }
                            self.signature = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    DirectoryReqTypeDeserializerState::Signature(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                DirectoryReqTypeDeserializerState::Signature(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.signature.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Signature",
                                )))?;
                            }
                            self.signature = Some(data);
                        }
                        *self.state = DirectoryReqTypeDeserializerState::Signature(None);
                        event
                    }
                    (DirectoryReqTypeDeserializerState::Signature(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DS),
                                Some(b"Signature")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <SignatureType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.signature.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"Signature",
                                    )))?;
                                }
                                self.signature = Some(data);
                            }
                            *self.state =
                                DirectoryReqTypeDeserializerState::Signature(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = DirectoryReqTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            DirectoryReqTypeDeserializerState::Signature(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = DirectoryReqTypeDeserializerState::Done__;
                            allow_any_fallback
                                .get_or_insert(DirectoryReqTypeDeserializerState::Signature(None));
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = DirectoryReqTypeDeserializerState::Signature(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (DirectoryReqTypeDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::DirectoryReqType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
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
        MerchantID(Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        SubID(Option<<usize as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl DirectoryReqMerchantTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                reader.err(ErrorKind::UnexpectedAttribute(
                    xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                ))?;
            }
            Ok(Self {
                merchant_id: None,
                sub_id: None,
                state: Box::new(DirectoryReqMerchantTypeDeserializerState::MerchantID(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::DirectoryReqMerchantType>
        for DirectoryReqMerchantTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::DirectoryReqMerchantType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::DirectoryReqMerchantType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DEFAULT: &[u8] = b"http://www.idealdesk.com/ideal/messages/mer-acq/3.3.1";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(
                        &mut *self.state,
                        DirectoryReqMerchantTypeDeserializerState::Done__,
                    ),
                    event,
                ) {
                    (
                        DirectoryReqMerchantTypeDeserializerState::MerchantID(Some(deserializer)),
                        event,
                    ) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.merchant_id.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"merchantID",
                                )))?;
                            }
                            self.merchant_id = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = DirectoryReqMerchantTypeDeserializerState::MerchantID(
                                    deserializer,
                                );
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                DirectoryReqMerchantTypeDeserializerState::MerchantID(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.merchant_id.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"merchantID",
                                )))?;
                            }
                            self.merchant_id = Some(data);
                        }
                        *self.state = DirectoryReqMerchantTypeDeserializerState::MerchantID(None);
                        event
                    }
                    (DirectoryReqMerchantTypeDeserializerState::MerchantID(None), event) => {
                        match &event {
                            Event::Start(x) | Event::Empty(x)
                                if matches!(
                                    reader.resolve_local_name(x.name(), NS_DEFAULT),
                                    Some(b"merchantID")
                                ) =>
                            {
                                let DeserializerOutput {
                                    data,
                                    deserializer,
                                    event,
                                    allow_any,
                                } = <String as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                                if let Some(data) = data {
                                    if self.merchant_id.is_some() {
                                        Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                            b"merchantID",
                                        )))?;
                                    }
                                    self.merchant_id = Some(data);
                                }
                                *self.state = DirectoryReqMerchantTypeDeserializerState::MerchantID(
                                    deserializer,
                                );
                                match event {
                                    Some(event @ (Event::Start(_) | Event::End(_))) => {
                                        *self.state =
                                            DirectoryReqMerchantTypeDeserializerState::SubID(None);
                                        if allow_any {
                                            allow_any_fallback . get_or_insert (DirectoryReqMerchantTypeDeserializerState :: MerchantID (None)) ;
                                        }
                                        event
                                    }
                                    event @ (None | Some(_)) => {
                                        return Ok(DeserializerOutput {
                                            data: None,
                                            deserializer: Some(self),
                                            event,
                                            allow_any: false,
                                        })
                                    }
                                }
                            }
                            Event::Start(_) | Event::Empty(_) => {
                                *self.state =
                                    DirectoryReqMerchantTypeDeserializerState::SubID(None);
                                event
                            }
                            Event::End(_) => {
                                let data = self.finish(reader)?;
                                return Ok(DeserializerOutput {
                                    data: Some(data),
                                    deserializer: None,
                                    event: None,
                                    allow_any: false,
                                });
                            }
                            _ => {
                                *self.state =
                                    DirectoryReqMerchantTypeDeserializerState::MerchantID(None);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: Some(event),
                                    allow_any: false,
                                });
                            }
                        }
                    }
                    (
                        DirectoryReqMerchantTypeDeserializerState::SubID(Some(deserializer)),
                        event,
                    ) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.sub_id.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"subID",
                                )))?;
                            }
                            self.sub_id = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    DirectoryReqMerchantTypeDeserializerState::SubID(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                DirectoryReqMerchantTypeDeserializerState::SubID(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.sub_id.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"subID",
                                )))?;
                            }
                            self.sub_id = Some(data);
                        }
                        *self.state = DirectoryReqMerchantTypeDeserializerState::SubID(None);
                        event
                    }
                    (DirectoryReqMerchantTypeDeserializerState::SubID(None), event) => match &event
                    {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DEFAULT),
                                Some(b"subID")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <usize as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.sub_id.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"subID",
                                    )))?;
                                }
                                self.sub_id = Some(data);
                            }
                            *self.state =
                                DirectoryReqMerchantTypeDeserializerState::SubID(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = DirectoryReqMerchantTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            DirectoryReqMerchantTypeDeserializerState::SubID(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = DirectoryReqMerchantTypeDeserializerState::Done__;
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = DirectoryReqMerchantTypeDeserializerState::SubID(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (DirectoryReqMerchantTypeDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::DirectoryReqMerchantType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
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
        SignedInfo(
            Option<<SignedInfoType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>,
        ),
        SignatureValue(
            Option<<SignatureValueType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>,
        ),
        KeyInfo(Option<<KeyInfoType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Object(Option<<ObjectType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl SignatureTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut id: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_DS), Some(b"Id")) {
                    reader.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else {
                    reader.err(ErrorKind::UnexpectedAttribute(
                        xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                    ))?;
                }
            }
            Ok(Self {
                id: id,
                signed_info: None,
                signature_value: None,
                key_info: None,
                object: Vec::new(),
                state: Box::new(SignatureTypeDeserializerState::SignedInfo(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::SignatureType>
        for SignatureTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::SignatureType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::SignatureType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, SignatureTypeDeserializerState::Done__),
                    event,
                ) {
                    (SignatureTypeDeserializerState::SignedInfo(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.signed_info.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"SignedInfo",
                                )))?;
                            }
                            self.signed_info = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    SignatureTypeDeserializerState::SignedInfo(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                SignatureTypeDeserializerState::SignedInfo(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.signed_info.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"SignedInfo",
                                )))?;
                            }
                            self.signed_info = Some(data);
                        }
                        *self.state = SignatureTypeDeserializerState::SignedInfo(None);
                        event
                    }
                    (SignatureTypeDeserializerState::SignedInfo(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DS),
                                Some(b"SignedInfo")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <SignedInfoType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.signed_info.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"SignedInfo",
                                    )))?;
                                }
                                self.signed_info = Some(data);
                            }
                            *self.state = SignatureTypeDeserializerState::SignedInfo(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state =
                                        SignatureTypeDeserializerState::SignatureValue(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            SignatureTypeDeserializerState::SignedInfo(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = SignatureTypeDeserializerState::SignatureValue(None);
                            allow_any_fallback
                                .get_or_insert(SignatureTypeDeserializerState::SignedInfo(None));
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = SignatureTypeDeserializerState::SignedInfo(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (SignatureTypeDeserializerState::SignatureValue(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.signature_value.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"SignatureValue",
                                )))?;
                            }
                            self.signature_value = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    SignatureTypeDeserializerState::SignatureValue(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                SignatureTypeDeserializerState::SignatureValue(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.signature_value.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"SignatureValue",
                                )))?;
                            }
                            self.signature_value = Some(data);
                        }
                        *self.state = SignatureTypeDeserializerState::SignatureValue(None);
                        event
                    }
                    (SignatureTypeDeserializerState::SignatureValue(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DS),
                                Some(b"SignatureValue")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <SignatureValueType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.signature_value.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"SignatureValue",
                                    )))?;
                                }
                                self.signature_value = Some(data);
                            }
                            *self.state =
                                SignatureTypeDeserializerState::SignatureValue(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = SignatureTypeDeserializerState::KeyInfo(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            SignatureTypeDeserializerState::SignatureValue(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = SignatureTypeDeserializerState::KeyInfo(None);
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = SignatureTypeDeserializerState::SignatureValue(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (SignatureTypeDeserializerState::KeyInfo(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.key_info.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"KeyInfo",
                                )))?;
                            }
                            self.key_info = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = SignatureTypeDeserializerState::KeyInfo(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                SignatureTypeDeserializerState::KeyInfo(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.key_info.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"KeyInfo",
                                )))?;
                            }
                            self.key_info = Some(data);
                        }
                        *self.state = SignatureTypeDeserializerState::KeyInfo(None);
                        event
                    }
                    (SignatureTypeDeserializerState::KeyInfo(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DS),
                                Some(b"KeyInfo")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <KeyInfoType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.key_info.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"KeyInfo",
                                    )))?;
                                }
                                self.key_info = Some(data);
                            }
                            *self.state = SignatureTypeDeserializerState::KeyInfo(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = SignatureTypeDeserializerState::Object(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            SignatureTypeDeserializerState::KeyInfo(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = SignatureTypeDeserializerState::Object(None);
                            allow_any_fallback
                                .get_or_insert(SignatureTypeDeserializerState::KeyInfo(None));
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = SignatureTypeDeserializerState::KeyInfo(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (SignatureTypeDeserializerState::Object(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            self.object.push(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = SignatureTypeDeserializerState::Object(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                SignatureTypeDeserializerState::Object(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            self.object.push(data);
                        }
                        *self.state = SignatureTypeDeserializerState::Object(None);
                        event
                    }
                    (SignatureTypeDeserializerState::Object(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DS),
                                Some(b"Object")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <ObjectType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                self.object.push(data);
                            }
                            *self.state = SignatureTypeDeserializerState::Object(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = SignatureTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            SignatureTypeDeserializerState::Object(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = SignatureTypeDeserializerState::Done__;
                            allow_any_fallback
                                .get_or_insert(SignatureTypeDeserializerState::Object(None));
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = SignatureTypeDeserializerState::Object(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (SignatureTypeDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::SignatureType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
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
        CanonicalizationMethod (Option << CanonicalizationMethodType as xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer >) , SignatureMethod (Option << SignatureMethodType as xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer >) , Reference (Option << ReferenceType as xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , }
    impl SignedInfoTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut id: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_DS), Some(b"Id")) {
                    reader.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else {
                    reader.err(ErrorKind::UnexpectedAttribute(
                        xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                    ))?;
                }
            }
            Ok(Self {
                id: id,
                canonicalization_method: None,
                signature_method: None,
                reference: Vec::new(),
                state: Box::new(SignedInfoTypeDeserializerState::CanonicalizationMethod(
                    None,
                )),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::SignedInfoType>
        for SignedInfoTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::SignedInfoType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::SignedInfoType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, SignedInfoTypeDeserializerState::Done__),
                    event,
                ) {
                    (
                        SignedInfoTypeDeserializerState::CanonicalizationMethod(Some(deserializer)),
                        event,
                    ) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.canonicalization_method.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"CanonicalizationMethod",
                                )))?;
                            }
                            self.canonicalization_method = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    SignedInfoTypeDeserializerState::CanonicalizationMethod(
                                        deserializer,
                                    );
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                SignedInfoTypeDeserializerState::CanonicalizationMethod(
                                    deserializer,
                                ),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.canonicalization_method.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"CanonicalizationMethod",
                                )))?;
                            }
                            self.canonicalization_method = Some(data);
                        }
                        *self.state = SignedInfoTypeDeserializerState::CanonicalizationMethod(None);
                        event
                    }
                    (SignedInfoTypeDeserializerState::CanonicalizationMethod(None), event) => {
                        match &event {
                            Event::Start(x) | Event::Empty(x)
                                if matches!(
                                    reader.resolve_local_name(x.name(), NS_DS),
                                    Some(b"CanonicalizationMethod")
                                ) =>
                            {
                                let DeserializerOutput { data , deserializer , event , allow_any } = < CanonicalizationMethodType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                                if let Some(data) = data {
                                    if self.canonicalization_method.is_some() {
                                        Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                            b"CanonicalizationMethod",
                                        )))?;
                                    }
                                    self.canonicalization_method = Some(data);
                                }
                                *self.state =
                                    SignedInfoTypeDeserializerState::CanonicalizationMethod(
                                        deserializer,
                                    );
                                match event {
                                    Some(event @ (Event::Start(_) | Event::End(_))) => {
                                        *self.state =
                                            SignedInfoTypeDeserializerState::SignatureMethod(None);
                                        if allow_any {
                                            allow_any_fallback . get_or_insert (SignedInfoTypeDeserializerState :: CanonicalizationMethod (None)) ;
                                        }
                                        event
                                    }
                                    event @ (None | Some(_)) => {
                                        return Ok(DeserializerOutput {
                                            data: None,
                                            deserializer: Some(self),
                                            event,
                                            allow_any: false,
                                        })
                                    }
                                }
                            }
                            Event::Start(_) | Event::Empty(_) => {
                                *self.state =
                                    SignedInfoTypeDeserializerState::SignatureMethod(None);
                                allow_any_fallback.get_or_insert(
                                    SignedInfoTypeDeserializerState::CanonicalizationMethod(None),
                                );
                                event
                            }
                            Event::End(_) => {
                                let data = self.finish(reader)?;
                                return Ok(DeserializerOutput {
                                    data: Some(data),
                                    deserializer: None,
                                    event: None,
                                    allow_any: false,
                                });
                            }
                            _ => {
                                *self.state =
                                    SignedInfoTypeDeserializerState::CanonicalizationMethod(None);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: Some(event),
                                    allow_any: false,
                                });
                            }
                        }
                    }
                    (
                        SignedInfoTypeDeserializerState::SignatureMethod(Some(deserializer)),
                        event,
                    ) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.signature_method.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"SignatureMethod",
                                )))?;
                            }
                            self.signature_method = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    SignedInfoTypeDeserializerState::SignatureMethod(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                SignedInfoTypeDeserializerState::SignatureMethod(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.signature_method.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"SignatureMethod",
                                )))?;
                            }
                            self.signature_method = Some(data);
                        }
                        *self.state = SignedInfoTypeDeserializerState::SignatureMethod(None);
                        event
                    }
                    (SignedInfoTypeDeserializerState::SignatureMethod(None), event) => match &event
                    {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DS),
                                Some(b"SignatureMethod")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <SignatureMethodType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.signature_method.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"SignatureMethod",
                                    )))?;
                                }
                                self.signature_method = Some(data);
                            }
                            *self.state =
                                SignedInfoTypeDeserializerState::SignatureMethod(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = SignedInfoTypeDeserializerState::Reference(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            SignedInfoTypeDeserializerState::SignatureMethod(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = SignedInfoTypeDeserializerState::Reference(None);
                            allow_any_fallback.get_or_insert(
                                SignedInfoTypeDeserializerState::SignatureMethod(None),
                            );
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = SignedInfoTypeDeserializerState::SignatureMethod(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (SignedInfoTypeDeserializerState::Reference(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            self.reference.push(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    SignedInfoTypeDeserializerState::Reference(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                SignedInfoTypeDeserializerState::Reference(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            self.reference.push(data);
                        }
                        *self.state = SignedInfoTypeDeserializerState::Reference(None);
                        event
                    }
                    (SignedInfoTypeDeserializerState::Reference(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DS),
                                Some(b"Reference")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <ReferenceType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                self.reference.push(data);
                            }
                            *self.state = SignedInfoTypeDeserializerState::Reference(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = SignedInfoTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            SignedInfoTypeDeserializerState::Reference(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = SignedInfoTypeDeserializerState::Done__;
                            allow_any_fallback
                                .get_or_insert(SignedInfoTypeDeserializerState::Reference(None));
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = SignedInfoTypeDeserializerState::Reference(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (SignedInfoTypeDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::SignedInfoType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
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
        state: Option<xsd_parser::quick_xml::ContentDeserializer<String>>,
    }
    impl SignatureValueTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut id: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_DS), Some(b"Id")) {
                    reader.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else {
                    reader.err(ErrorKind::UnexpectedAttribute(
                        xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                    ))?;
                }
            }
            Ok(Self {
                id: id,
                content: None,
                state: None,
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::SignatureValueType>
        for SignatureValueTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::SignatureValueType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{ContentDeserializer, DeserializerOutput, Event};
            let (Event::Start(start) | Event::Empty(start)) = &event else {
                return Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                });
            };
            let mut this = Self::from_bytes_start(reader, &start)?;
            let DeserializerOutput {
                data,
                deserializer,
                event,
                allow_any,
            } = ContentDeserializer::init(reader, event)?;
            if let Some(data) = data {
                this.content = Some(data);
                let data = this.finish(reader)?;
                Ok(DeserializerOutput {
                    data: Some(data),
                    deserializer: None,
                    event,
                    allow_any,
                })
            } else if let Some(state) = deserializer {
                this.state = Some(state);
                Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(this),
                    event,
                    allow_any,
                })
            } else {
                Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event,
                    allow_any,
                })
            }
        }
        fn next<R>(
            self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::SignatureValueType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::DeserializerOutput;
            let mut this = self;
            let DeserializerOutput {
                data,
                deserializer,
                event,
                allow_any,
            } = this.state.take().unwrap().next(reader, event)?;
            if let Some(data) = data {
                this.content = Some(data);
                let data = this.finish(reader)?;
                Ok(DeserializerOutput {
                    data: Some(data),
                    deserializer: None,
                    event,
                    allow_any,
                })
            } else if let Some(state) = deserializer {
                this.state = Some(state);
                Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(this),
                    event,
                    allow_any,
                })
            } else {
                Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event,
                    allow_any,
                })
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::SignatureValueType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
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
        Next__,
        KeyName(<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        KeyValue(<KeyValueType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        RetrievalMethod(
            <RetrievalMethodType as xsd_parser::quick_xml::WithDeserializer>::Deserializer,
        ),
        X509Data(<X509DataType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Pgpdata(<PgpdataType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        Spkidata(<SpkidataType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        MgmtData(<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl KeyInfoTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut id: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_DS), Some(b"Id")) {
                    reader.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else {
                    reader.err(ErrorKind::UnexpectedAttribute(
                        xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                    ))?;
                }
            }
            Ok(Self {
                id: id,
                content: Vec::new(),
                state: Box::new(KeyInfoTypeDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::KeyInfoType> for KeyInfoTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::KeyInfoType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::KeyInfoType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            match (
                core::mem::replace(&mut *self.state, KeyInfoTypeDeserializerState::Next__),
                &event,
            ) {
                (KeyInfoTypeDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(reader.resolve_local_name(x.name(), NS_DS), Some(b"KeyName")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(KeyInfoTypeContent::KeyName(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = KeyInfoTypeDeserializerState::KeyName(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_DS),
                        Some(b"KeyValue")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <KeyValueType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(KeyInfoTypeContent::KeyValue(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = KeyInfoTypeDeserializerState::KeyValue(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_DS),
                        Some(b"RetrievalMethod")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <RetrievalMethodType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            self.content.push(KeyInfoTypeContent::RetrievalMethod(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                KeyInfoTypeDeserializerState::RetrievalMethod(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_DS),
                        Some(b"X509Data")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <X509DataType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(KeyInfoTypeContent::X509Data(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = KeyInfoTypeDeserializerState::X509Data(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_DS), Some(b"PGPData"))
                    {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <PgpdataType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(KeyInfoTypeContent::Pgpdata(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = KeyInfoTypeDeserializerState::Pgpdata(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_DS),
                        Some(b"SPKIData")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <SpkidataType as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(KeyInfoTypeContent::Spkidata(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = KeyInfoTypeDeserializerState::Spkidata(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_DS),
                        Some(b"MgmtData")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(KeyInfoTypeContent::MgmtData(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = KeyInfoTypeDeserializerState::MgmtData(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else {
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any: false,
                        })
                    }
                }
                (KeyInfoTypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (KeyInfoTypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (KeyInfoTypeDeserializerState::KeyName(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(KeyInfoTypeContent::KeyName(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = KeyInfoTypeDeserializerState::KeyName(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (KeyInfoTypeDeserializerState::KeyValue(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(KeyInfoTypeContent::KeyValue(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = KeyInfoTypeDeserializerState::KeyValue(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (KeyInfoTypeDeserializerState::RetrievalMethod(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(KeyInfoTypeContent::RetrievalMethod(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = KeyInfoTypeDeserializerState::RetrievalMethod(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (KeyInfoTypeDeserializerState::X509Data(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(KeyInfoTypeContent::X509Data(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = KeyInfoTypeDeserializerState::X509Data(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (KeyInfoTypeDeserializerState::Pgpdata(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(KeyInfoTypeContent::Pgpdata(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = KeyInfoTypeDeserializerState::Pgpdata(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (KeyInfoTypeDeserializerState::Spkidata(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(KeyInfoTypeContent::Spkidata(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = KeyInfoTypeDeserializerState::Spkidata(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (KeyInfoTypeDeserializerState::MgmtData(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(KeyInfoTypeContent::MgmtData(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = KeyInfoTypeDeserializerState::MgmtData(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
        fn finish<R>(self, _reader: &R) -> Result<super::KeyInfoType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::KeyInfoType {
                id: self.id,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct ObjectTypeDeserializer {
        id: Option<String>,
        mime_type: Option<String>,
        encoding: Option<String>,
    }
    impl ObjectTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut id: Option<String> = None;
            let mut mime_type: Option<String> = None;
            let mut encoding: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_DS), Some(b"Id")) {
                    reader.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_DS),
                    Some(b"MimeType")
                ) {
                    reader.read_attrib(&mut mime_type, b"MimeType", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_DS),
                    Some(b"Encoding")
                ) {
                    reader.read_attrib(&mut encoding, b"Encoding", &attrib.value)?;
                } else {
                    reader.err(ErrorKind::UnexpectedAttribute(
                        xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                    ))?;
                }
            }
            Ok(Self {
                id: id,
                mime_type: mime_type,
                encoding: encoding,
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::ObjectType> for ObjectTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ObjectType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ObjectType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::End(_) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                _ => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: Some(event),
                    allow_any: true,
                }),
            }
        }
        fn finish<R>(self, _reader: &R) -> Result<super::ObjectType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
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
    }
    impl CanonicalizationMethodTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut algorithm: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(
                    reader.resolve_local_name(attrib.key, NS_DS),
                    Some(b"Algorithm")
                ) {
                    reader.read_attrib(&mut algorithm, b"Algorithm", &attrib.value)?;
                } else {
                    reader.err(ErrorKind::UnexpectedAttribute(
                        xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                    ))?;
                }
            }
            Ok(Self {
                algorithm: algorithm.ok_or(ErrorKind::MissingAttribute("Algorithm".into()))?,
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::CanonicalizationMethodType>
        for CanonicalizationMethodTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::CanonicalizationMethodType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::CanonicalizationMethodType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::End(_) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                _ => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: Some(event),
                    allow_any: true,
                }),
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::CanonicalizationMethodType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
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
        HmacoutputLength(Option<<i32 as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl SignatureMethodTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut algorithm: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(
                    reader.resolve_local_name(attrib.key, NS_DS),
                    Some(b"Algorithm")
                ) {
                    reader.read_attrib(&mut algorithm, b"Algorithm", &attrib.value)?;
                } else {
                    reader.err(ErrorKind::UnexpectedAttribute(
                        xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                    ))?;
                }
            }
            Ok(Self {
                algorithm: algorithm.ok_or(ErrorKind::MissingAttribute("Algorithm".into()))?,
                hmacoutput_length: None,
                state: Box::new(SignatureMethodTypeDeserializerState::HmacoutputLength(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::SignatureMethodType>
        for SignatureMethodTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::SignatureMethodType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::SignatureMethodType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(
                        &mut *self.state,
                        SignatureMethodTypeDeserializerState::Done__,
                    ),
                    event,
                ) {
                    (
                        SignatureMethodTypeDeserializerState::HmacoutputLength(Some(deserializer)),
                        event,
                    ) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.hmacoutput_length.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"HMACOutputLength",
                                )))?;
                            }
                            self.hmacoutput_length = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    SignatureMethodTypeDeserializerState::HmacoutputLength(
                                        deserializer,
                                    );
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                SignatureMethodTypeDeserializerState::HmacoutputLength(
                                    deserializer,
                                ),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.hmacoutput_length.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"HMACOutputLength",
                                )))?;
                            }
                            self.hmacoutput_length = Some(data);
                        }
                        *self.state = SignatureMethodTypeDeserializerState::HmacoutputLength(None);
                        event
                    }
                    (SignatureMethodTypeDeserializerState::HmacoutputLength(None), event) => {
                        match &event {
                            Event::Start(x) | Event::Empty(x)
                                if matches!(
                                    reader.resolve_local_name(x.name(), NS_DS),
                                    Some(b"HMACOutputLength")
                                ) =>
                            {
                                let DeserializerOutput {
                                    data,
                                    deserializer,
                                    event,
                                    allow_any,
                                } = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                                if let Some(data) = data {
                                    if self.hmacoutput_length.is_some() {
                                        Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                            b"HMACOutputLength",
                                        )))?;
                                    }
                                    self.hmacoutput_length = Some(data);
                                }
                                *self.state =
                                    SignatureMethodTypeDeserializerState::HmacoutputLength(
                                        deserializer,
                                    );
                                match event {
                                    Some(event @ (Event::Start(_) | Event::End(_))) => {
                                        *self.state = SignatureMethodTypeDeserializerState::Done__;
                                        if allow_any {
                                            allow_any_fallback . get_or_insert (SignatureMethodTypeDeserializerState :: HmacoutputLength (None)) ;
                                        }
                                        event
                                    }
                                    event @ (None | Some(_)) => {
                                        return Ok(DeserializerOutput {
                                            data: None,
                                            deserializer: Some(self),
                                            event,
                                            allow_any: false,
                                        })
                                    }
                                }
                            }
                            Event::Start(_) | Event::Empty(_) => {
                                *self.state = SignatureMethodTypeDeserializerState::Done__;
                                allow_any_fallback.get_or_insert(
                                    SignatureMethodTypeDeserializerState::HmacoutputLength(None),
                                );
                                event
                            }
                            Event::End(_) => {
                                let data = self.finish(reader)?;
                                return Ok(DeserializerOutput {
                                    data: Some(data),
                                    deserializer: None,
                                    event: None,
                                    allow_any: false,
                                });
                            }
                            _ => {
                                *self.state =
                                    SignatureMethodTypeDeserializerState::HmacoutputLength(None);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: Some(event),
                                    allow_any: false,
                                });
                            }
                        }
                    }
                    (SignatureMethodTypeDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::SignatureMethodType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
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
        Transforms(
            Option<<TransformsType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>,
        ),
        DigestMethod(
            Option<<DigestMethodType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>,
        ),
        DigestValue(Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl ReferenceTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut id: Option<String> = None;
            let mut uri: Option<String> = None;
            let mut type_: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_DS), Some(b"Id")) {
                    reader.read_attrib(&mut id, b"Id", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_DS), Some(b"URI")) {
                    reader.read_attrib(&mut uri, b"URI", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_DS), Some(b"Type")) {
                    reader.read_attrib(&mut type_, b"Type", &attrib.value)?;
                } else {
                    reader.err(ErrorKind::UnexpectedAttribute(
                        xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                    ))?;
                }
            }
            Ok(Self {
                id: id,
                uri: uri,
                type_: type_,
                transforms: None,
                digest_method: None,
                digest_value: None,
                state: Box::new(ReferenceTypeDeserializerState::Transforms(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::ReferenceType>
        for ReferenceTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ReferenceType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ReferenceType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, ReferenceTypeDeserializerState::Done__),
                    event,
                ) {
                    (ReferenceTypeDeserializerState::Transforms(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.transforms.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Transforms",
                                )))?;
                            }
                            self.transforms = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    ReferenceTypeDeserializerState::Transforms(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                ReferenceTypeDeserializerState::Transforms(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.transforms.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Transforms",
                                )))?;
                            }
                            self.transforms = Some(data);
                        }
                        *self.state = ReferenceTypeDeserializerState::Transforms(None);
                        event
                    }
                    (ReferenceTypeDeserializerState::Transforms(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DS),
                                Some(b"Transforms")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <TransformsType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.transforms.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"Transforms",
                                    )))?;
                                }
                                self.transforms = Some(data);
                            }
                            *self.state = ReferenceTypeDeserializerState::Transforms(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state =
                                        ReferenceTypeDeserializerState::DigestMethod(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ReferenceTypeDeserializerState::Transforms(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = ReferenceTypeDeserializerState::DigestMethod(None);
                            allow_any_fallback
                                .get_or_insert(ReferenceTypeDeserializerState::Transforms(None));
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = ReferenceTypeDeserializerState::Transforms(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ReferenceTypeDeserializerState::DigestMethod(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.digest_method.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"DigestMethod",
                                )))?;
                            }
                            self.digest_method = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    ReferenceTypeDeserializerState::DigestMethod(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                ReferenceTypeDeserializerState::DigestMethod(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.digest_method.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"DigestMethod",
                                )))?;
                            }
                            self.digest_method = Some(data);
                        }
                        *self.state = ReferenceTypeDeserializerState::DigestMethod(None);
                        event
                    }
                    (ReferenceTypeDeserializerState::DigestMethod(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DS),
                                Some(b"DigestMethod")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <DigestMethodType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.digest_method.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"DigestMethod",
                                    )))?;
                                }
                                self.digest_method = Some(data);
                            }
                            *self.state =
                                ReferenceTypeDeserializerState::DigestMethod(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ReferenceTypeDeserializerState::DigestValue(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ReferenceTypeDeserializerState::DigestMethod(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = ReferenceTypeDeserializerState::DigestValue(None);
                            allow_any_fallback
                                .get_or_insert(ReferenceTypeDeserializerState::DigestMethod(None));
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = ReferenceTypeDeserializerState::DigestMethod(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ReferenceTypeDeserializerState::DigestValue(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.digest_value.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"DigestValue",
                                )))?;
                            }
                            self.digest_value = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    ReferenceTypeDeserializerState::DigestValue(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                ReferenceTypeDeserializerState::DigestValue(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.digest_value.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"DigestValue",
                                )))?;
                            }
                            self.digest_value = Some(data);
                        }
                        *self.state = ReferenceTypeDeserializerState::DigestValue(None);
                        event
                    }
                    (ReferenceTypeDeserializerState::DigestValue(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DS),
                                Some(b"DigestValue")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.digest_value.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"DigestValue",
                                    )))?;
                                }
                                self.digest_value = Some(data);
                            }
                            *self.state = ReferenceTypeDeserializerState::DigestValue(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ReferenceTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ReferenceTypeDeserializerState::DigestValue(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = ReferenceTypeDeserializerState::Done__;
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = ReferenceTypeDeserializerState::DigestValue(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ReferenceTypeDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::ReferenceType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
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
        Next__,
        DsakeyValue(<DsakeyValueType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        RsakeyValue(<RsakeyValueType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl KeyValueTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                reader.err(ErrorKind::UnexpectedAttribute(
                    xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                ))?;
            }
            Ok(Self {
                content: None,
                state: Box::new(KeyValueTypeDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::KeyValueType>
        for KeyValueTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::KeyValueType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::KeyValueType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            match (
                core::mem::replace(&mut *self.state, KeyValueTypeDeserializerState::Next__),
                &event,
            ) {
                (KeyValueTypeDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(
                        reader.resolve_local_name(x.name(), NS_DS),
                        Some(b"DSAKeyValue")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <DsakeyValueType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"DSAKeyValue",
                                )))?;
                            }
                            self.content = Some(KeyValueTypeContent::DsakeyValue(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = KeyValueTypeDeserializerState::DsakeyValue(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_DS),
                        Some(b"RSAKeyValue")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <RsakeyValueType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"RSAKeyValue",
                                )))?;
                            }
                            self.content = Some(KeyValueTypeContent::RsakeyValue(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = KeyValueTypeDeserializerState::RsakeyValue(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else {
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any: false,
                        })
                    }
                }
                (KeyValueTypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (KeyValueTypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (KeyValueTypeDeserializerState::DsakeyValue(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"DSAKeyValue",
                            )))?;
                        }
                        self.content = Some(KeyValueTypeContent::DsakeyValue(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = KeyValueTypeDeserializerState::DsakeyValue(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (KeyValueTypeDeserializerState::RsakeyValue(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"RSAKeyValue",
                            )))?;
                        }
                        self.content = Some(KeyValueTypeContent::RsakeyValue(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = KeyValueTypeDeserializerState::RsakeyValue(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
        fn finish<R>(self, _reader: &R) -> Result<super::KeyValueType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::KeyValueType {
                content: self
                    .content
                    .ok_or(xsd_parser::quick_xml::ErrorKind::MissingContent)?,
            })
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
        Transforms(
            Option<<TransformsType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>,
        ),
        Done__,
    }
    impl RetrievalMethodTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut uri: Option<String> = None;
            let mut type_: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(reader.resolve_local_name(attrib.key, NS_DS), Some(b"URI")) {
                    reader.read_attrib(&mut uri, b"URI", &attrib.value)?;
                } else if matches!(reader.resolve_local_name(attrib.key, NS_DS), Some(b"Type")) {
                    reader.read_attrib(&mut type_, b"Type", &attrib.value)?;
                } else {
                    reader.err(ErrorKind::UnexpectedAttribute(
                        xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                    ))?;
                }
            }
            Ok(Self {
                uri: uri,
                type_: type_,
                transforms: None,
                state: Box::new(RetrievalMethodTypeDeserializerState::Transforms(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::RetrievalMethodType>
        for RetrievalMethodTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::RetrievalMethodType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::RetrievalMethodType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(
                        &mut *self.state,
                        RetrievalMethodTypeDeserializerState::Done__,
                    ),
                    event,
                ) {
                    (
                        RetrievalMethodTypeDeserializerState::Transforms(Some(deserializer)),
                        event,
                    ) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.transforms.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Transforms",
                                )))?;
                            }
                            self.transforms = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    RetrievalMethodTypeDeserializerState::Transforms(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                RetrievalMethodTypeDeserializerState::Transforms(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.transforms.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Transforms",
                                )))?;
                            }
                            self.transforms = Some(data);
                        }
                        *self.state = RetrievalMethodTypeDeserializerState::Transforms(None);
                        event
                    }
                    (RetrievalMethodTypeDeserializerState::Transforms(None), event) => match &event
                    {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DS),
                                Some(b"Transforms")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <TransformsType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.transforms.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"Transforms",
                                    )))?;
                                }
                                self.transforms = Some(data);
                            }
                            *self.state =
                                RetrievalMethodTypeDeserializerState::Transforms(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = RetrievalMethodTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            RetrievalMethodTypeDeserializerState::Transforms(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = RetrievalMethodTypeDeserializerState::Done__;
                            allow_any_fallback.get_or_insert(
                                RetrievalMethodTypeDeserializerState::Transforms(None),
                            );
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = RetrievalMethodTypeDeserializerState::Transforms(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (RetrievalMethodTypeDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::RetrievalMethodType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::RetrievalMethodType {
                uri: self.uri,
                type_: self.type_,
                transforms: self.transforms,
            })
        }
    }
    #[derive(Debug)]
    pub struct X509DataTypeDeserializer {
        content_103: Option<super::X509DataContent103Type>,
        state: Box<X509DataTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum X509DataTypeDeserializerState {
        Content103(
            Option<
                <X509DataContent103Type as xsd_parser::quick_xml::WithDeserializer>::Deserializer,
            >,
        ),
        Done__,
    }
    impl X509DataTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                reader.err(ErrorKind::UnexpectedAttribute(
                    xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                ))?;
            }
            Ok(Self {
                content_103: None,
                state: Box::new(X509DataTypeDeserializerState::Content103(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::X509DataType>
        for X509DataTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::X509DataType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::X509DataType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, X509DataTypeDeserializerState::Done__),
                    event,
                ) {
                    (X509DataTypeDeserializerState::Content103(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.content_103.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Content103",
                                )))?;
                            }
                            self.content_103 = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    X509DataTypeDeserializerState::Content103(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                X509DataTypeDeserializerState::Content103(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.content_103.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Content103",
                                )))?;
                            }
                            self.content_103 = Some(data);
                        }
                        *self.state = X509DataTypeDeserializerState::Content103(None);
                        event
                    }
                    (X509DataTypeDeserializerState::Content103(None), event) => {
                        match &event {
                            Event::Start(_) | Event::Empty(_) => {
                                let DeserializerOutput { data , deserializer , event , allow_any } = < X509DataContent103Type as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                                if let Some(data) = data {
                                    if self.content_103.is_some() {
                                        Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                            b"Content103",
                                        )))?;
                                    }
                                    self.content_103 = Some(data);
                                }
                                *self.state =
                                    X509DataTypeDeserializerState::Content103(deserializer);
                                match event {
                                    Some(event @ (Event::Start(_) | Event::End(_))) => {
                                        *self.state = X509DataTypeDeserializerState::Done__;
                                        if allow_any {
                                            allow_any_fallback.get_or_insert(
                                                X509DataTypeDeserializerState::Content103(None),
                                            );
                                        }
                                        event
                                    }
                                    event @ (None | Some(_)) => {
                                        return Ok(DeserializerOutput {
                                            data: None,
                                            deserializer: Some(self),
                                            event,
                                            allow_any: false,
                                        })
                                    }
                                }
                            }
                            Event::End(_) => {
                                let data = self.finish(reader)?;
                                return Ok(DeserializerOutput {
                                    data: Some(data),
                                    deserializer: None,
                                    event: None,
                                    allow_any: false,
                                });
                            }
                            _ => {
                                *self.state = X509DataTypeDeserializerState::Content103(None);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: Some(event),
                                    allow_any: false,
                                });
                            }
                        }
                    }
                    (X509DataTypeDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(self, _reader: &R) -> Result<super::X509DataType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::X509DataType {
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
        Next__,
        Content113(
            <PgpdataContent113Type as xsd_parser::quick_xml::WithDeserializer>::Deserializer,
        ),
        Content116(
            <PgpdataContent116Type as xsd_parser::quick_xml::WithDeserializer>::Deserializer,
        ),
    }
    impl PgpdataTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                reader.err(ErrorKind::UnexpectedAttribute(
                    xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                ))?;
            }
            Ok(Self {
                content: None,
                state: Box::new(PgpdataTypeDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::PgpdataType> for PgpdataTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::PgpdataType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::PgpdataType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            match (
                core::mem::replace(&mut *self.state, PgpdataTypeDeserializerState::Next__),
                &event,
            ) {
                (PgpdataTypeDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    let mut allow_any_element = false;
                    let event = {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <PgpdataContent113Type as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Content113",
                                )))?;
                            }
                            self.content = Some(PgpdataTypeContent::Content113(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = PgpdataTypeDeserializerState::Content113(deserializer);
                        }
                        let Some(event) = event else {
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: None,
                                allow_any,
                            });
                        };
                        if allow_any {
                            allow_any_element = true;
                        }
                        event
                    };
                    let event = {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <PgpdataContent116Type as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Content116",
                                )))?;
                            }
                            self.content = Some(PgpdataTypeContent::Content116(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = PgpdataTypeDeserializerState::Content116(deserializer);
                        }
                        let Some(event) = event else {
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: None,
                                allow_any,
                            });
                        };
                        if allow_any {
                            allow_any_element = true;
                        }
                        event
                    };
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event: Some(event),
                        allow_any: allow_any_element,
                    })
                }
                (PgpdataTypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (PgpdataTypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (PgpdataTypeDeserializerState::Content113(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"Content113",
                            )))?;
                        }
                        self.content = Some(PgpdataTypeContent::Content113(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = PgpdataTypeDeserializerState::Content113(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (PgpdataTypeDeserializerState::Content116(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"Content116",
                            )))?;
                        }
                        self.content = Some(PgpdataTypeContent::Content116(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = PgpdataTypeDeserializerState::Content116(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
        fn finish<R>(self, _reader: &R) -> Result<super::PgpdataType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::PgpdataType {
                content: self
                    .content
                    .ok_or(xsd_parser::quick_xml::ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct SpkidataTypeDeserializer {
        spkisexp: Option<String>,
        state: Box<SpkidataTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SpkidataTypeDeserializerState {
        Spkisexp(Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl SpkidataTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                reader.err(ErrorKind::UnexpectedAttribute(
                    xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                ))?;
            }
            Ok(Self {
                spkisexp: None,
                state: Box::new(SpkidataTypeDeserializerState::Spkisexp(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::SpkidataType>
        for SpkidataTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::SpkidataType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::SpkidataType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, SpkidataTypeDeserializerState::Done__),
                    event,
                ) {
                    (SpkidataTypeDeserializerState::Spkisexp(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.spkisexp.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"SPKISexp",
                                )))?;
                            }
                            self.spkisexp = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = SpkidataTypeDeserializerState::Spkisexp(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                SpkidataTypeDeserializerState::Spkisexp(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.spkisexp.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"SPKISexp",
                                )))?;
                            }
                            self.spkisexp = Some(data);
                        }
                        *self.state = SpkidataTypeDeserializerState::Spkisexp(None);
                        event
                    }
                    (SpkidataTypeDeserializerState::Spkisexp(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DS),
                                Some(b"SPKISexp")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.spkisexp.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"SPKISexp",
                                    )))?;
                                }
                                self.spkisexp = Some(data);
                            }
                            *self.state = SpkidataTypeDeserializerState::Spkisexp(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = SpkidataTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            SpkidataTypeDeserializerState::Spkisexp(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = SpkidataTypeDeserializerState::Done__;
                            allow_any_fallback
                                .get_or_insert(SpkidataTypeDeserializerState::Spkisexp(None));
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = SpkidataTypeDeserializerState::Spkisexp(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (SpkidataTypeDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(self, _reader: &R) -> Result<super::SpkidataType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::SpkidataType {
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
        Transform(Option<<TransformType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl TransformsTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                reader.err(ErrorKind::UnexpectedAttribute(
                    xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                ))?;
            }
            Ok(Self {
                transform: Vec::new(),
                state: Box::new(TransformsTypeDeserializerState::Transform(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::TransformsType>
        for TransformsTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::TransformsType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::TransformsType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, TransformsTypeDeserializerState::Done__),
                    event,
                ) {
                    (TransformsTypeDeserializerState::Transform(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            self.transform.push(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    TransformsTypeDeserializerState::Transform(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                TransformsTypeDeserializerState::Transform(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            self.transform.push(data);
                        }
                        *self.state = TransformsTypeDeserializerState::Transform(None);
                        event
                    }
                    (TransformsTypeDeserializerState::Transform(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DS),
                                Some(b"Transform")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <TransformType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                self.transform.push(data);
                            }
                            *self.state = TransformsTypeDeserializerState::Transform(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = TransformsTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            TransformsTypeDeserializerState::Transform(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = TransformsTypeDeserializerState::Done__;
                            allow_any_fallback
                                .get_or_insert(TransformsTypeDeserializerState::Transform(None));
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = TransformsTypeDeserializerState::Transform(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (TransformsTypeDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::TransformsType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::TransformsType {
                transform: self.transform,
            })
        }
    }
    #[derive(Debug)]
    pub struct DigestMethodTypeDeserializer {
        algorithm: String,
    }
    impl DigestMethodTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut algorithm: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(
                    reader.resolve_local_name(attrib.key, NS_DS),
                    Some(b"Algorithm")
                ) {
                    reader.read_attrib(&mut algorithm, b"Algorithm", &attrib.value)?;
                } else {
                    reader.err(ErrorKind::UnexpectedAttribute(
                        xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                    ))?;
                }
            }
            Ok(Self {
                algorithm: algorithm.ok_or(ErrorKind::MissingAttribute("Algorithm".into()))?,
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::DigestMethodType>
        for DigestMethodTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::DigestMethodType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::DigestMethodType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::End(_) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                _ => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: Some(event),
                    allow_any: true,
                }),
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::DigestMethodType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
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
        Content125 (Option << DsakeyValueContent125Type as xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer >) , G (Option << String as xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer >) , Y (Option << String as xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer >) , J (Option << String as xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer >) , Content131 (Option << DsakeyValueContent131Type as xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , }
    impl DsakeyValueTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                reader.err(ErrorKind::UnexpectedAttribute(
                    xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                ))?;
            }
            Ok(Self {
                content_125: None,
                g: None,
                y: None,
                j: None,
                content_131: None,
                state: Box::new(DsakeyValueTypeDeserializerState::Content125(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::DsakeyValueType>
        for DsakeyValueTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::DsakeyValueType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::DsakeyValueType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, DsakeyValueTypeDeserializerState::Done__),
                    event,
                ) {
                    (DsakeyValueTypeDeserializerState::Content125(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.content_125.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Content125",
                                )))?;
                            }
                            self.content_125 = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    DsakeyValueTypeDeserializerState::Content125(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                DsakeyValueTypeDeserializerState::Content125(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.content_125.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Content125",
                                )))?;
                            }
                            self.content_125 = Some(data);
                        }
                        *self.state = DsakeyValueTypeDeserializerState::Content125(None);
                        event
                    }
                    (DsakeyValueTypeDeserializerState::Content125(None), event) => match &event {
                        Event::Start(_) | Event::Empty(_) => {
                            let DeserializerOutput { data , deserializer , event , allow_any } = < DsakeyValueContent125Type as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            if let Some(data) = data {
                                if self.content_125.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"Content125",
                                    )))?;
                                }
                                self.content_125 = Some(data);
                            }
                            *self.state =
                                DsakeyValueTypeDeserializerState::Content125(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = DsakeyValueTypeDeserializerState::G(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            DsakeyValueTypeDeserializerState::Content125(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = DsakeyValueTypeDeserializerState::Content125(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (DsakeyValueTypeDeserializerState::G(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.g.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"G")))?;
                            }
                            self.g = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = DsakeyValueTypeDeserializerState::G(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback
                                .get_or_insert(DsakeyValueTypeDeserializerState::G(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.g.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"G")))?;
                            }
                            self.g = Some(data);
                        }
                        *self.state = DsakeyValueTypeDeserializerState::G(None);
                        event
                    }
                    (DsakeyValueTypeDeserializerState::G(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(reader.resolve_local_name(x.name(), NS_DS), Some(b"G")) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.g.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"G")))?;
                                }
                                self.g = Some(data);
                            }
                            *self.state = DsakeyValueTypeDeserializerState::G(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = DsakeyValueTypeDeserializerState::Y(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            DsakeyValueTypeDeserializerState::G(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = DsakeyValueTypeDeserializerState::Y(None);
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = DsakeyValueTypeDeserializerState::G(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (DsakeyValueTypeDeserializerState::Y(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.y.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Y")))?;
                            }
                            self.y = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = DsakeyValueTypeDeserializerState::Y(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback
                                .get_or_insert(DsakeyValueTypeDeserializerState::Y(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.y.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Y")))?;
                            }
                            self.y = Some(data);
                        }
                        *self.state = DsakeyValueTypeDeserializerState::Y(None);
                        event
                    }
                    (DsakeyValueTypeDeserializerState::Y(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(reader.resolve_local_name(x.name(), NS_DS), Some(b"Y")) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.y.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Y")))?;
                                }
                                self.y = Some(data);
                            }
                            *self.state = DsakeyValueTypeDeserializerState::Y(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = DsakeyValueTypeDeserializerState::J(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            DsakeyValueTypeDeserializerState::Y(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = DsakeyValueTypeDeserializerState::J(None);
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = DsakeyValueTypeDeserializerState::Y(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (DsakeyValueTypeDeserializerState::J(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.j.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"J")))?;
                            }
                            self.j = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = DsakeyValueTypeDeserializerState::J(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback
                                .get_or_insert(DsakeyValueTypeDeserializerState::J(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.j.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"J")))?;
                            }
                            self.j = Some(data);
                        }
                        *self.state = DsakeyValueTypeDeserializerState::J(None);
                        event
                    }
                    (DsakeyValueTypeDeserializerState::J(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(reader.resolve_local_name(x.name(), NS_DS), Some(b"J")) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.j.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"J")))?;
                                }
                                self.j = Some(data);
                            }
                            *self.state = DsakeyValueTypeDeserializerState::J(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state =
                                        DsakeyValueTypeDeserializerState::Content131(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            DsakeyValueTypeDeserializerState::J(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = DsakeyValueTypeDeserializerState::Content131(None);
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = DsakeyValueTypeDeserializerState::J(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (DsakeyValueTypeDeserializerState::Content131(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.content_131.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Content131",
                                )))?;
                            }
                            self.content_131 = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    DsakeyValueTypeDeserializerState::Content131(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                DsakeyValueTypeDeserializerState::Content131(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.content_131.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Content131",
                                )))?;
                            }
                            self.content_131 = Some(data);
                        }
                        *self.state = DsakeyValueTypeDeserializerState::Content131(None);
                        event
                    }
                    (DsakeyValueTypeDeserializerState::Content131(None), event) => match &event {
                        Event::Start(_) | Event::Empty(_) => {
                            let DeserializerOutput { data , deserializer , event , allow_any } = < DsakeyValueContent131Type as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            if let Some(data) = data {
                                if self.content_131.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"Content131",
                                    )))?;
                                }
                                self.content_131 = Some(data);
                            }
                            *self.state =
                                DsakeyValueTypeDeserializerState::Content131(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = DsakeyValueTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            DsakeyValueTypeDeserializerState::Content131(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = DsakeyValueTypeDeserializerState::Content131(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (DsakeyValueTypeDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::DsakeyValueType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
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
        Modulus(Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Exponent(Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl RsakeyValueTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                reader.err(ErrorKind::UnexpectedAttribute(
                    xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                ))?;
            }
            Ok(Self {
                modulus: None,
                exponent: None,
                state: Box::new(RsakeyValueTypeDeserializerState::Modulus(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::RsakeyValueType>
        for RsakeyValueTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::RsakeyValueType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::RsakeyValueType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, RsakeyValueTypeDeserializerState::Done__),
                    event,
                ) {
                    (RsakeyValueTypeDeserializerState::Modulus(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.modulus.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Modulus",
                                )))?;
                            }
                            self.modulus = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    RsakeyValueTypeDeserializerState::Modulus(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                RsakeyValueTypeDeserializerState::Modulus(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.modulus.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Modulus",
                                )))?;
                            }
                            self.modulus = Some(data);
                        }
                        *self.state = RsakeyValueTypeDeserializerState::Modulus(None);
                        event
                    }
                    (RsakeyValueTypeDeserializerState::Modulus(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DS),
                                Some(b"Modulus")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.modulus.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"Modulus",
                                    )))?;
                                }
                                self.modulus = Some(data);
                            }
                            *self.state = RsakeyValueTypeDeserializerState::Modulus(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = RsakeyValueTypeDeserializerState::Exponent(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            RsakeyValueTypeDeserializerState::Modulus(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = RsakeyValueTypeDeserializerState::Exponent(None);
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = RsakeyValueTypeDeserializerState::Modulus(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (RsakeyValueTypeDeserializerState::Exponent(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.exponent.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Exponent",
                                )))?;
                            }
                            self.exponent = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    RsakeyValueTypeDeserializerState::Exponent(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                RsakeyValueTypeDeserializerState::Exponent(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.exponent.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"Exponent",
                                )))?;
                            }
                            self.exponent = Some(data);
                        }
                        *self.state = RsakeyValueTypeDeserializerState::Exponent(None);
                        event
                    }
                    (RsakeyValueTypeDeserializerState::Exponent(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DS),
                                Some(b"Exponent")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.exponent.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"Exponent",
                                    )))?;
                                }
                                self.exponent = Some(data);
                            }
                            *self.state = RsakeyValueTypeDeserializerState::Exponent(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = RsakeyValueTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            RsakeyValueTypeDeserializerState::Exponent(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = RsakeyValueTypeDeserializerState::Done__;
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = RsakeyValueTypeDeserializerState::Exponent(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (RsakeyValueTypeDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::RsakeyValueType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
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
        Next__,
        X509IssuerSerial(
            <X509IssuerSerialType as xsd_parser::quick_xml::WithDeserializer>::Deserializer,
        ),
        X509Ski(<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        X509SubjectName(<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        X509Certificate(<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
        X509Crl(<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl X509DataContent103TypeDeserializer {}
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::X509DataContent103Type>
        for X509DataContent103TypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::X509DataContent103Type, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            let deserializer = Self {
                content: None,
                state: Box::new(X509DataContent103TypeDeserializerState::Next__),
            };
            let is_empty = matches!(event, Event::Empty(_));
            let mut out = deserializer.next(reader, event)?;
            if out.event.is_some() {
                out.deserializer = None;
            } else if is_empty && out.data.is_none() {
                if let Some(deserializer) = out.deserializer.take() {
                    out.data = Some(deserializer.finish(reader)?);
                }
            }
            Ok(out)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::X509DataContent103Type, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            match (
                core::mem::replace(
                    &mut *self.state,
                    X509DataContent103TypeDeserializerState::Next__,
                ),
                &event,
            ) {
                (
                    X509DataContent103TypeDeserializerState::Next__,
                    Event::Start(x) | Event::Empty(x),
                ) => {
                    if matches!(
                        reader.resolve_local_name(x.name(), NS_DS),
                        Some(b"X509IssuerSerial")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <X509IssuerSerialType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"X509IssuerSerial",
                                )))?;
                            }
                            self.content =
                                Some(X509DataContent103TypeContent::X509IssuerSerial(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = X509DataContent103TypeDeserializerState::X509IssuerSerial(
                                deserializer,
                            );
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_DS), Some(b"X509SKI"))
                    {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"X509SKI",
                                )))?;
                            }
                            self.content = Some(X509DataContent103TypeContent::X509Ski(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                X509DataContent103TypeDeserializerState::X509Ski(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_DS),
                        Some(b"X509SubjectName")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"X509SubjectName",
                                )))?;
                            }
                            self.content =
                                Some(X509DataContent103TypeContent::X509SubjectName(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = X509DataContent103TypeDeserializerState::X509SubjectName(
                                deserializer,
                            );
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(
                        reader.resolve_local_name(x.name(), NS_DS),
                        Some(b"X509Certificate")
                    ) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"X509Certificate",
                                )))?;
                            }
                            self.content =
                                Some(X509DataContent103TypeContent::X509Certificate(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = X509DataContent103TypeDeserializerState::X509Certificate(
                                deserializer,
                            );
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else if matches!(reader.resolve_local_name(x.name(), NS_DS), Some(b"X509CRL"))
                    {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            if self.content.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"X509CRL",
                                )))?;
                            }
                            self.content = Some(X509DataContent103TypeContent::X509Crl(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state =
                                X509DataContent103TypeDeserializerState::X509Crl(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else {
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any: false,
                        })
                    }
                }
                (X509DataContent103TypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: Some(event),
                        allow_any: false,
                    })
                }
                (X509DataContent103TypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (X509DataContent103TypeDeserializerState::X509IssuerSerial(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"X509IssuerSerial",
                            )))?;
                        }
                        self.content = Some(X509DataContent103TypeContent::X509IssuerSerial(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state =
                            X509DataContent103TypeDeserializerState::X509IssuerSerial(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (X509DataContent103TypeDeserializerState::X509Ski(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"X509SKI",
                            )))?;
                        }
                        self.content = Some(X509DataContent103TypeContent::X509Ski(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state =
                            X509DataContent103TypeDeserializerState::X509Ski(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (X509DataContent103TypeDeserializerState::X509SubjectName(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"X509SubjectName",
                            )))?;
                        }
                        self.content = Some(X509DataContent103TypeContent::X509SubjectName(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state =
                            X509DataContent103TypeDeserializerState::X509SubjectName(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (X509DataContent103TypeDeserializerState::X509Certificate(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"X509Certificate",
                            )))?;
                        }
                        self.content = Some(X509DataContent103TypeContent::X509Certificate(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state =
                            X509DataContent103TypeDeserializerState::X509Certificate(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
                (X509DataContent103TypeDeserializerState::X509Crl(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                b"X509CRL",
                            )))?;
                        }
                        self.content = Some(X509DataContent103TypeContent::X509Crl(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state =
                            X509DataContent103TypeDeserializerState::X509Crl(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::X509DataContent103Type, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::X509DataContent103Type {
                content: self
                    .content
                    .ok_or(xsd_parser::quick_xml::ErrorKind::MissingContent)?,
            })
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
        PgpkeyID(Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        PgpkeyPacket(Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl PgpdataContent113TypeDeserializer {}
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::PgpdataContent113Type>
        for PgpdataContent113TypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::PgpdataContent113Type, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::DeserializerOutput;
            let deserializer = Self {
                pgpkey_id: None,
                pgpkey_packet: None,
                state: Box::new(PgpdataContent113TypeDeserializerState::PgpkeyID(None)),
            };
            let mut out = deserializer.next(reader, event)?;
            if out.event.is_some() {
                out.deserializer = None;
            }
            Ok(out)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::PgpdataContent113Type, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(
                        &mut *self.state,
                        PgpdataContent113TypeDeserializerState::Done__,
                    ),
                    event,
                ) {
                    (
                        PgpdataContent113TypeDeserializerState::PgpkeyID(Some(deserializer)),
                        event,
                    ) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.pgpkey_id.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"PGPKeyID",
                                )))?;
                            }
                            self.pgpkey_id = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    PgpdataContent113TypeDeserializerState::PgpkeyID(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                PgpdataContent113TypeDeserializerState::PgpkeyID(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.pgpkey_id.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"PGPKeyID",
                                )))?;
                            }
                            self.pgpkey_id = Some(data);
                        }
                        *self.state = PgpdataContent113TypeDeserializerState::PgpkeyID(None);
                        event
                    }
                    (PgpdataContent113TypeDeserializerState::PgpkeyID(None), event) => match &event
                    {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DS),
                                Some(b"PGPKeyID")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.pgpkey_id.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"PGPKeyID",
                                    )))?;
                                }
                                self.pgpkey_id = Some(data);
                            }
                            *self.state =
                                PgpdataContent113TypeDeserializerState::PgpkeyID(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state =
                                        PgpdataContent113TypeDeserializerState::PgpkeyPacket(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            PgpdataContent113TypeDeserializerState::PgpkeyID(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state =
                                PgpdataContent113TypeDeserializerState::PgpkeyPacket(None);
                            allow_any_fallback.get_or_insert(
                                PgpdataContent113TypeDeserializerState::PgpkeyID(None),
                            );
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = PgpdataContent113TypeDeserializerState::PgpkeyID(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (
                        PgpdataContent113TypeDeserializerState::PgpkeyPacket(Some(deserializer)),
                        event,
                    ) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.pgpkey_packet.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"PGPKeyPacket",
                                )))?;
                            }
                            self.pgpkey_packet = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = PgpdataContent113TypeDeserializerState::PgpkeyPacket(
                                    deserializer,
                                );
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                PgpdataContent113TypeDeserializerState::PgpkeyPacket(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.pgpkey_packet.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"PGPKeyPacket",
                                )))?;
                            }
                            self.pgpkey_packet = Some(data);
                        }
                        *self.state = PgpdataContent113TypeDeserializerState::PgpkeyPacket(None);
                        event
                    }
                    (PgpdataContent113TypeDeserializerState::PgpkeyPacket(None), event) => {
                        match &event {
                            Event::Start(x) | Event::Empty(x)
                                if matches!(
                                    reader.resolve_local_name(x.name(), NS_DS),
                                    Some(b"PGPKeyPacket")
                                ) =>
                            {
                                let DeserializerOutput {
                                    data,
                                    deserializer,
                                    event,
                                    allow_any,
                                } = <String as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                                if let Some(data) = data {
                                    if self.pgpkey_packet.is_some() {
                                        Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                            b"PGPKeyPacket",
                                        )))?;
                                    }
                                    self.pgpkey_packet = Some(data);
                                }
                                *self.state = PgpdataContent113TypeDeserializerState::PgpkeyPacket(
                                    deserializer,
                                );
                                match event {
                                    Some(event @ (Event::Start(_) | Event::End(_))) => {
                                        *self.state =
                                            PgpdataContent113TypeDeserializerState::Done__;
                                        if allow_any {
                                            allow_any_fallback . get_or_insert (PgpdataContent113TypeDeserializerState :: PgpkeyPacket (None)) ;
                                        }
                                        event
                                    }
                                    event @ (None | Some(_)) => {
                                        return Ok(DeserializerOutput {
                                            data: None,
                                            deserializer: Some(self),
                                            event,
                                            allow_any: false,
                                        })
                                    }
                                }
                            }
                            Event::Start(_) | Event::Empty(_) => {
                                *self.state = PgpdataContent113TypeDeserializerState::Done__;
                                allow_any_fallback.get_or_insert(
                                    PgpdataContent113TypeDeserializerState::PgpkeyPacket(None),
                                );
                                event
                            }
                            Event::End(_) => {
                                let data = self.finish(reader)?;
                                return Ok(DeserializerOutput {
                                    data: Some(data),
                                    deserializer: None,
                                    event: None,
                                    allow_any: false,
                                });
                            }
                            _ => {
                                *self.state =
                                    PgpdataContent113TypeDeserializerState::PgpkeyPacket(None);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: Some(event),
                                    allow_any: false,
                                });
                            }
                        }
                    }
                    (PgpdataContent113TypeDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::PgpdataContent113Type, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
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
        PgpkeyPacket(Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl PgpdataContent116TypeDeserializer {}
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::PgpdataContent116Type>
        for PgpdataContent116TypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::PgpdataContent116Type, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::DeserializerOutput;
            let deserializer = Self {
                pgpkey_packet: None,
                state: Box::new(PgpdataContent116TypeDeserializerState::PgpkeyPacket(None)),
            };
            let mut out = deserializer.next(reader, event)?;
            if out.event.is_some() {
                out.deserializer = None;
            }
            Ok(out)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::PgpdataContent116Type, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(
                        &mut *self.state,
                        PgpdataContent116TypeDeserializerState::Done__,
                    ),
                    event,
                ) {
                    (
                        PgpdataContent116TypeDeserializerState::PgpkeyPacket(Some(deserializer)),
                        event,
                    ) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.pgpkey_packet.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"PGPKeyPacket",
                                )))?;
                            }
                            self.pgpkey_packet = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = PgpdataContent116TypeDeserializerState::PgpkeyPacket(
                                    deserializer,
                                );
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                PgpdataContent116TypeDeserializerState::PgpkeyPacket(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.pgpkey_packet.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"PGPKeyPacket",
                                )))?;
                            }
                            self.pgpkey_packet = Some(data);
                        }
                        *self.state = PgpdataContent116TypeDeserializerState::PgpkeyPacket(None);
                        event
                    }
                    (PgpdataContent116TypeDeserializerState::PgpkeyPacket(None), event) => {
                        match &event {
                            Event::Start(x) | Event::Empty(x)
                                if matches!(
                                    reader.resolve_local_name(x.name(), NS_DS),
                                    Some(b"PGPKeyPacket")
                                ) =>
                            {
                                let DeserializerOutput {
                                    data,
                                    deserializer,
                                    event,
                                    allow_any,
                                } = <String as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                                if let Some(data) = data {
                                    if self.pgpkey_packet.is_some() {
                                        Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                            b"PGPKeyPacket",
                                        )))?;
                                    }
                                    self.pgpkey_packet = Some(data);
                                }
                                *self.state = PgpdataContent116TypeDeserializerState::PgpkeyPacket(
                                    deserializer,
                                );
                                match event {
                                    Some(event @ (Event::Start(_) | Event::End(_))) => {
                                        *self.state =
                                            PgpdataContent116TypeDeserializerState::Done__;
                                        if allow_any {
                                            allow_any_fallback . get_or_insert (PgpdataContent116TypeDeserializerState :: PgpkeyPacket (None)) ;
                                        }
                                        event
                                    }
                                    event @ (None | Some(_)) => {
                                        return Ok(DeserializerOutput {
                                            data: None,
                                            deserializer: Some(self),
                                            event,
                                            allow_any: false,
                                        })
                                    }
                                }
                            }
                            Event::Start(_) | Event::Empty(_) => {
                                *self.state = PgpdataContent116TypeDeserializerState::Done__;
                                allow_any_fallback.get_or_insert(
                                    PgpdataContent116TypeDeserializerState::PgpkeyPacket(None),
                                );
                                event
                            }
                            Event::End(_) => {
                                let data = self.finish(reader)?;
                                return Ok(DeserializerOutput {
                                    data: Some(data),
                                    deserializer: None,
                                    event: None,
                                    allow_any: false,
                                });
                            }
                            _ => {
                                *self.state =
                                    PgpdataContent116TypeDeserializerState::PgpkeyPacket(None);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: Some(event),
                                    allow_any: false,
                                });
                            }
                        }
                    }
                    (PgpdataContent116TypeDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::PgpdataContent116Type, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
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
        Next__,
        Xpath(<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl TransformTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut algorithm: Option<String> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(
                    reader.resolve_local_name(attrib.key, NS_DS),
                    Some(b"Algorithm")
                ) {
                    reader.read_attrib(&mut algorithm, b"Algorithm", &attrib.value)?;
                } else {
                    reader.err(ErrorKind::UnexpectedAttribute(
                        xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                    ))?;
                }
            }
            Ok(Self {
                algorithm: algorithm.ok_or(ErrorKind::MissingAttribute("Algorithm".into()))?,
                content: Vec::new(),
                state: Box::new(TransformTypeDeserializerState::Next__),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::TransformType>
        for TransformTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::TransformType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::TransformType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            match (
                core::mem::replace(&mut *self.state, TransformTypeDeserializerState::Next__),
                &event,
            ) {
                (TransformTypeDeserializerState::Next__, Event::Start(x) | Event::Empty(x)) => {
                    if matches!(reader.resolve_local_name(x.name(), NS_DS), Some(b"XPath")) {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        if let Some(data) = data {
                            self.content.push(TransformTypeContent::Xpath(data));
                        }
                        if let Some(deserializer) = deserializer {
                            *self.state = TransformTypeDeserializerState::Xpath(deserializer);
                        }
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event,
                            allow_any,
                        })
                    } else {
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any: false,
                        })
                    }
                }
                (TransformTypeDeserializerState::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                (TransformTypeDeserializerState::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                (TransformTypeDeserializerState::Xpath(deserializer), _) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = deserializer.next(reader, event)?;
                    if let Some(data) = data {
                        self.content.push(TransformTypeContent::Xpath(data));
                    }
                    if let Some(deserializer) = deserializer {
                        *self.state = TransformTypeDeserializerState::Xpath(deserializer);
                    }
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::TransformType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::TransformType {
                algorithm: self.algorithm,
                content: self.content,
            })
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
        P(Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Q(Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl DsakeyValueContent125TypeDeserializer {}
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::DsakeyValueContent125Type>
        for DsakeyValueContent125TypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::DsakeyValueContent125Type, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::DeserializerOutput;
            let deserializer = Self {
                p: None,
                q: None,
                state: Box::new(DsakeyValueContent125TypeDeserializerState::P(None)),
            };
            let mut out = deserializer.next(reader, event)?;
            if out.event.is_some() {
                out.deserializer = None;
            }
            Ok(out)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::DsakeyValueContent125Type, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(
                        &mut *self.state,
                        DsakeyValueContent125TypeDeserializerState::Done__,
                    ),
                    event,
                ) {
                    (DsakeyValueContent125TypeDeserializerState::P(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.p.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"P")))?;
                            }
                            self.p = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    DsakeyValueContent125TypeDeserializerState::P(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                DsakeyValueContent125TypeDeserializerState::P(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.p.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"P")))?;
                            }
                            self.p = Some(data);
                        }
                        *self.state = DsakeyValueContent125TypeDeserializerState::P(None);
                        event
                    }
                    (DsakeyValueContent125TypeDeserializerState::P(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(reader.resolve_local_name(x.name(), NS_DS), Some(b"P")) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.p.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"P")))?;
                                }
                                self.p = Some(data);
                            }
                            *self.state =
                                DsakeyValueContent125TypeDeserializerState::P(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state =
                                        DsakeyValueContent125TypeDeserializerState::Q(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            DsakeyValueContent125TypeDeserializerState::P(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = DsakeyValueContent125TypeDeserializerState::Q(None);
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = DsakeyValueContent125TypeDeserializerState::P(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (DsakeyValueContent125TypeDeserializerState::Q(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.q.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Q")))?;
                            }
                            self.q = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    DsakeyValueContent125TypeDeserializerState::Q(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                DsakeyValueContent125TypeDeserializerState::Q(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.q.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Q")))?;
                            }
                            self.q = Some(data);
                        }
                        *self.state = DsakeyValueContent125TypeDeserializerState::Q(None);
                        event
                    }
                    (DsakeyValueContent125TypeDeserializerState::Q(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(reader.resolve_local_name(x.name(), NS_DS), Some(b"Q")) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.q.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Q")))?;
                                }
                                self.q = Some(data);
                            }
                            *self.state =
                                DsakeyValueContent125TypeDeserializerState::Q(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state =
                                        DsakeyValueContent125TypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            DsakeyValueContent125TypeDeserializerState::Q(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = DsakeyValueContent125TypeDeserializerState::Done__;
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = DsakeyValueContent125TypeDeserializerState::Q(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (DsakeyValueContent125TypeDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::DsakeyValueContent125Type, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
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
        Seed(Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        PgenCounter(Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl DsakeyValueContent131TypeDeserializer {}
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::DsakeyValueContent131Type>
        for DsakeyValueContent131TypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::DsakeyValueContent131Type, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::DeserializerOutput;
            let deserializer = Self {
                seed: None,
                pgen_counter: None,
                state: Box::new(DsakeyValueContent131TypeDeserializerState::Seed(None)),
            };
            let mut out = deserializer.next(reader, event)?;
            if out.event.is_some() {
                out.deserializer = None;
            }
            Ok(out)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::DsakeyValueContent131Type, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(
                        &mut *self.state,
                        DsakeyValueContent131TypeDeserializerState::Done__,
                    ),
                    event,
                ) {
                    (
                        DsakeyValueContent131TypeDeserializerState::Seed(Some(deserializer)),
                        event,
                    ) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.seed.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Seed")))?;
                            }
                            self.seed = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    DsakeyValueContent131TypeDeserializerState::Seed(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                DsakeyValueContent131TypeDeserializerState::Seed(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.seed.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Seed")))?;
                            }
                            self.seed = Some(data);
                        }
                        *self.state = DsakeyValueContent131TypeDeserializerState::Seed(None);
                        event
                    }
                    (DsakeyValueContent131TypeDeserializerState::Seed(None), event) => match &event
                    {
                        Event::Start(x) | Event::Empty(x)
                            if matches!(
                                reader.resolve_local_name(x.name(), NS_DS),
                                Some(b"Seed")
                            ) =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.seed.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"Seed",
                                    )))?;
                                }
                                self.seed = Some(data);
                            }
                            *self.state =
                                DsakeyValueContent131TypeDeserializerState::Seed(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state =
                                        DsakeyValueContent131TypeDeserializerState::PgenCounter(
                                            None,
                                        );
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            DsakeyValueContent131TypeDeserializerState::Seed(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state =
                                DsakeyValueContent131TypeDeserializerState::PgenCounter(None);
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = DsakeyValueContent131TypeDeserializerState::Seed(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (
                        DsakeyValueContent131TypeDeserializerState::PgenCounter(Some(deserializer)),
                        event,
                    ) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.pgen_counter.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"PgenCounter",
                                )))?;
                            }
                            self.pgen_counter = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    DsakeyValueContent131TypeDeserializerState::PgenCounter(
                                        deserializer,
                                    );
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                DsakeyValueContent131TypeDeserializerState::PgenCounter(
                                    deserializer,
                                ),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.pgen_counter.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"PgenCounter",
                                )))?;
                            }
                            self.pgen_counter = Some(data);
                        }
                        *self.state = DsakeyValueContent131TypeDeserializerState::PgenCounter(None);
                        event
                    }
                    (DsakeyValueContent131TypeDeserializerState::PgenCounter(None), event) => {
                        match &event {
                            Event::Start(x) | Event::Empty(x)
                                if matches!(
                                    reader.resolve_local_name(x.name(), NS_DS),
                                    Some(b"PgenCounter")
                                ) =>
                            {
                                let DeserializerOutput {
                                    data,
                                    deserializer,
                                    event,
                                    allow_any,
                                } = <String as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                                if let Some(data) = data {
                                    if self.pgen_counter.is_some() {
                                        Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                            b"PgenCounter",
                                        )))?;
                                    }
                                    self.pgen_counter = Some(data);
                                }
                                *self.state =
                                    DsakeyValueContent131TypeDeserializerState::PgenCounter(
                                        deserializer,
                                    );
                                match event {
                                    Some(event @ (Event::Start(_) | Event::End(_))) => {
                                        *self.state =
                                            DsakeyValueContent131TypeDeserializerState::Done__;
                                        if allow_any {
                                            allow_any_fallback . get_or_insert (DsakeyValueContent131TypeDeserializerState :: PgenCounter (None)) ;
                                        }
                                        event
                                    }
                                    event @ (None | Some(_)) => {
                                        return Ok(DeserializerOutput {
                                            data: None,
                                            deserializer: Some(self),
                                            event,
                                            allow_any: false,
                                        })
                                    }
                                }
                            }
                            Event::Start(_) | Event::Empty(_) => {
                                *self.state = DsakeyValueContent131TypeDeserializerState::Done__;
                                event
                            }
                            Event::End(_) => {
                                let data = self.finish(reader)?;
                                return Ok(DeserializerOutput {
                                    data: Some(data),
                                    deserializer: None,
                                    event: None,
                                    allow_any: false,
                                });
                            }
                            _ => {
                                *self.state =
                                    DsakeyValueContent131TypeDeserializerState::PgenCounter(None);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: Some(event),
                                    allow_any: false,
                                });
                            }
                        }
                    }
                    (DsakeyValueContent131TypeDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::DsakeyValueContent131Type, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
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
        X509IssuerName(Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        X509SerialNumber(Option<<i32 as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl X509IssuerSerialTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                reader.err(ErrorKind::UnexpectedAttribute(
                    xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                ))?;
            }
            Ok(Self {
                x509_issuer_name: None,
                x509_serial_number: None,
                state: Box::new(X509IssuerSerialTypeDeserializerState::X509IssuerName(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::X509IssuerSerialType>
        for X509IssuerSerialTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::X509IssuerSerialType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::X509IssuerSerialType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_DS: &[u8] = b"http://www.w3.org/2000/09/xmldsig#";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(
                        &mut *self.state,
                        X509IssuerSerialTypeDeserializerState::Done__,
                    ),
                    event,
                ) {
                    (
                        X509IssuerSerialTypeDeserializerState::X509IssuerName(Some(deserializer)),
                        event,
                    ) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.x509_issuer_name.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"X509IssuerName",
                                )))?;
                            }
                            self.x509_issuer_name = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = X509IssuerSerialTypeDeserializerState::X509IssuerName(
                                    deserializer,
                                );
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                X509IssuerSerialTypeDeserializerState::X509IssuerName(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.x509_issuer_name.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"X509IssuerName",
                                )))?;
                            }
                            self.x509_issuer_name = Some(data);
                        }
                        *self.state = X509IssuerSerialTypeDeserializerState::X509IssuerName(None);
                        event
                    }
                    (X509IssuerSerialTypeDeserializerState::X509IssuerName(None), event) => {
                        match &event {
                            Event::Start(x) | Event::Empty(x)
                                if matches!(
                                    reader.resolve_local_name(x.name(), NS_DS),
                                    Some(b"X509IssuerName")
                                ) =>
                            {
                                let DeserializerOutput {
                                    data,
                                    deserializer,
                                    event,
                                    allow_any,
                                } = <String as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                                if let Some(data) = data {
                                    if self.x509_issuer_name.is_some() {
                                        Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                            b"X509IssuerName",
                                        )))?;
                                    }
                                    self.x509_issuer_name = Some(data);
                                }
                                *self.state = X509IssuerSerialTypeDeserializerState::X509IssuerName(
                                    deserializer,
                                );
                                match event {
                                    Some(event @ (Event::Start(_) | Event::End(_))) => {
                                        *self.state =
                                            X509IssuerSerialTypeDeserializerState::X509SerialNumber(
                                                None,
                                            );
                                        if allow_any {
                                            allow_any_fallback . get_or_insert (X509IssuerSerialTypeDeserializerState :: X509IssuerName (None)) ;
                                        }
                                        event
                                    }
                                    event @ (None | Some(_)) => {
                                        return Ok(DeserializerOutput {
                                            data: None,
                                            deserializer: Some(self),
                                            event,
                                            allow_any: false,
                                        })
                                    }
                                }
                            }
                            Event::Start(_) | Event::Empty(_) => {
                                *self.state =
                                    X509IssuerSerialTypeDeserializerState::X509SerialNumber(None);
                                event
                            }
                            Event::End(_) => {
                                let data = self.finish(reader)?;
                                return Ok(DeserializerOutput {
                                    data: Some(data),
                                    deserializer: None,
                                    event: None,
                                    allow_any: false,
                                });
                            }
                            _ => {
                                *self.state =
                                    X509IssuerSerialTypeDeserializerState::X509IssuerName(None);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: Some(event),
                                    allow_any: false,
                                });
                            }
                        }
                    }
                    (
                        X509IssuerSerialTypeDeserializerState::X509SerialNumber(Some(deserializer)),
                        event,
                    ) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.x509_serial_number.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"X509SerialNumber",
                                )))?;
                            }
                            self.x509_serial_number = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    X509IssuerSerialTypeDeserializerState::X509SerialNumber(
                                        deserializer,
                                    );
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                X509IssuerSerialTypeDeserializerState::X509SerialNumber(
                                    deserializer,
                                ),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.x509_serial_number.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"X509SerialNumber",
                                )))?;
                            }
                            self.x509_serial_number = Some(data);
                        }
                        *self.state = X509IssuerSerialTypeDeserializerState::X509SerialNumber(None);
                        event
                    }
                    (X509IssuerSerialTypeDeserializerState::X509SerialNumber(None), event) => {
                        match &event {
                            Event::Start(x) | Event::Empty(x)
                                if matches!(
                                    reader.resolve_local_name(x.name(), NS_DS),
                                    Some(b"X509SerialNumber")
                                ) =>
                            {
                                let DeserializerOutput {
                                    data,
                                    deserializer,
                                    event,
                                    allow_any,
                                } = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                                if let Some(data) = data {
                                    if self.x509_serial_number.is_some() {
                                        Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                            b"X509SerialNumber",
                                        )))?;
                                    }
                                    self.x509_serial_number = Some(data);
                                }
                                *self.state =
                                    X509IssuerSerialTypeDeserializerState::X509SerialNumber(
                                        deserializer,
                                    );
                                match event {
                                    Some(event @ (Event::Start(_) | Event::End(_))) => {
                                        *self.state = X509IssuerSerialTypeDeserializerState::Done__;
                                        if allow_any {
                                            allow_any_fallback . get_or_insert (X509IssuerSerialTypeDeserializerState :: X509SerialNumber (None)) ;
                                        }
                                        event
                                    }
                                    event @ (None | Some(_)) => {
                                        return Ok(DeserializerOutput {
                                            data: None,
                                            deserializer: Some(self),
                                            event,
                                            allow_any: false,
                                        })
                                    }
                                }
                            }
                            Event::Start(_) | Event::Empty(_) => {
                                *self.state = X509IssuerSerialTypeDeserializerState::Done__;
                                event
                            }
                            Event::End(_) => {
                                let data = self.finish(reader)?;
                                return Ok(DeserializerOutput {
                                    data: Some(data),
                                    deserializer: None,
                                    event: None,
                                    allow_any: false,
                                });
                            }
                            _ => {
                                *self.state =
                                    X509IssuerSerialTypeDeserializerState::X509SerialNumber(None);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: Some(event),
                                    allow_any: false,
                                });
                            }
                        }
                    }
                    (X509IssuerSerialTypeDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::X509IssuerSerialType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
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
