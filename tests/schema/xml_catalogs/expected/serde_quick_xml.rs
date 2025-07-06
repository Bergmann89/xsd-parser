pub mod er {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CatalogType {
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
        #[serde(default, rename = "@prefer")]
        pub prefer: Option<SystemOrPublicType>,
        #[serde(rename = "$value")]
        pub content: Vec<CatalogTypeContent>,
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub enum CatalogTypeContent {
        #[serde(rename = "public")]
        Public(PublicType),
        #[serde(rename = "system")]
        System(SystemType),
        #[serde(rename = "uri")]
        Uri(UriType),
        #[serde(rename = "rewriteSystem")]
        RewriteSystem(RewriteSystemType),
        #[serde(rename = "rewriteURI")]
        RewriteUri(RewriteUriType),
        #[serde(rename = "uriSuffix")]
        UriSuffix(UriSuffixType),
        #[serde(rename = "systemSuffix")]
        SystemSuffix(SystemSuffixType),
        #[serde(rename = "delegatePublic")]
        DelegatePublic(DelegatePublicType),
        #[serde(rename = "delegateSystem")]
        DelegateSystem(DelegateSystemType),
        #[serde(rename = "delegateURI")]
        DelegateUri(DelegateUriType),
        #[serde(rename = "nextCatalog")]
        NextCatalog(NextCatalogType),
        #[serde(rename = "group")]
        Group(GroupType),
    }
    pub type Catalog = CatalogType;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DelegatePublicType {
        #[serde(rename = "@publicIdStartString")]
        pub public_id_start_string: String,
        #[serde(rename = "@catalog")]
        pub catalog: String,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
    }
    pub type DelegatePublic = DelegatePublicType;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DelegateSystemType {
        #[serde(rename = "@systemIdStartString")]
        pub system_id_start_string: String,
        #[serde(rename = "@catalog")]
        pub catalog: String,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
    }
    pub type DelegateSystem = DelegateSystemType;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DelegateUriType {
        #[serde(rename = "@uriStartString")]
        pub uri_start_string: String,
        #[serde(rename = "@catalog")]
        pub catalog: String,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
    }
    pub type DelegateUri = DelegateUriType;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GroupType {
        #[serde(default, rename = "@prefer")]
        pub prefer: Option<SystemOrPublicType>,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
        #[serde(rename = "$value")]
        pub content: Vec<GroupTypeContent>,
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub enum GroupTypeContent {
        #[serde(rename = "public")]
        Public(PublicType),
        #[serde(rename = "system")]
        System(SystemType),
        #[serde(rename = "uri")]
        Uri(UriType),
        #[serde(rename = "rewriteSystem")]
        RewriteSystem(RewriteSystemType),
        #[serde(rename = "rewriteURI")]
        RewriteUri(RewriteUriType),
        #[serde(rename = "uriSuffix")]
        UriSuffix(UriSuffixType),
        #[serde(rename = "systemSuffix")]
        SystemSuffix(SystemSuffixType),
        #[serde(rename = "delegatePublic")]
        DelegatePublic(DelegatePublicType),
        #[serde(rename = "delegateSystem")]
        DelegateSystem(DelegateSystemType),
        #[serde(rename = "delegateURI")]
        DelegateUri(DelegateUriType),
        #[serde(rename = "nextCatalog")]
        NextCatalog(NextCatalogType),
    }
    pub type Group = GroupType;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NextCatalogType {
        #[serde(rename = "@catalog")]
        pub catalog: String,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
    }
    pub type NextCatalog = NextCatalogType;
    pub type PartialPublicIdentifierType = String;
    pub type PubIdCharsType = String;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PublicType {
        #[serde(rename = "@publicId")]
        pub public_id: String,
        #[serde(rename = "@uri")]
        pub uri: String,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
    }
    pub type Public = PublicType;
    pub type PublicIdentifierType = String;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RewriteSystemType {
        #[serde(rename = "@systemIdStartString")]
        pub system_id_start_string: String,
        #[serde(rename = "@rewritePrefix")]
        pub rewrite_prefix: String,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
    }
    pub type RewriteSystem = RewriteSystemType;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RewriteUriType {
        #[serde(rename = "@uriStartString")]
        pub uri_start_string: String,
        #[serde(rename = "@rewritePrefix")]
        pub rewrite_prefix: String,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
    }
    pub type RewriteUri = RewriteUriType;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SystemType {
        #[serde(rename = "@systemId")]
        pub system_id: String,
        #[serde(rename = "@uri")]
        pub uri: String,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
    }
    pub type System = SystemType;
    #[derive(Debug, Serialize, Deserialize)]
    pub enum SystemOrPublicType {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "public")]
        Public,
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SystemSuffixType {
        #[serde(rename = "@systemIdSuffix")]
        pub system_id_suffix: String,
        #[serde(rename = "@uri")]
        pub uri: String,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
    }
    pub type SystemSuffix = SystemSuffixType;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct UriType {
        #[serde(rename = "@name")]
        pub name: String,
        #[serde(rename = "@uri")]
        pub uri: String,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
    }
    pub type Uri = UriType;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct UriSuffixType {
        #[serde(rename = "@uriSuffix")]
        pub uri_suffix: String,
        #[serde(rename = "@uri")]
        pub uri: String,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
    }
    pub type UriSuffix = UriSuffixType;
}
pub mod xs {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct EntitiesType(pub Vec<String>);
    pub type EntityType = EntitiesType;
    pub type IdType = String;
    pub type IdrefType = String;
    pub type IdrefsType = EntitiesType;
    pub type NcNameType = String;
    pub type NmtokenType = String;
    pub type NmtokensType = EntitiesType;
    pub type NotationType = String;
    pub type NameType = String;
    pub type QNameType = String;
    pub type AnySimpleType = String;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AnyType;
    pub type AnyUriType = String;
    pub type Base64BinaryType = String;
    pub type BooleanType = bool;
    pub type ByteType = i8;
    pub type DateType = String;
    pub type DateTimeType = String;
    pub type DecimalType = f64;
    pub type DoubleType = f64;
    pub type DurationType = String;
    pub type FloatType = f32;
    pub type GDayType = String;
    pub type GMonthType = String;
    pub type GMonthDayType = String;
    pub type GYearType = String;
    pub type GYearMonthType = String;
    pub type HexBinaryType = String;
    pub type IntType = i32;
    pub type IntegerType = i32;
    pub type LanguageType = String;
    pub type LongType = i64;
    pub type NegativeIntegerType = isize;
    pub type NonNegativeIntegerType = usize;
    pub type NonPositiveIntegerType = isize;
    pub type NormalizedStringType = String;
    pub type PositiveIntegerType = usize;
    pub type ShortType = i16;
    pub type StringType = String;
    pub type TimeType = String;
    pub type TokenType = String;
    pub type UnsignedByteType = u8;
    pub type UnsignedIntType = u32;
    pub type UnsignedLongType = u64;
    pub type UnsignedShortType = u16;
}
