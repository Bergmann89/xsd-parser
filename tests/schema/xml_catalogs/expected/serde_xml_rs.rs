pub mod er {
    use core::ops::{Deref, DerefMut};
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, Serialize)]
    pub struct CatalogType {
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
        #[serde(default, rename = "@prefer")]
        pub prefer: Option<SystemOrPublicType>,
        #[serde(rename = "#content")]
        pub content: Vec<CatalogTypeContent>,
    }
    #[derive(Debug, Deserialize, Serialize)]
    pub enum CatalogTypeContent {
        #[serde(rename = "er:public")]
        Public(PublicType),
        #[serde(rename = "er:system")]
        System(SystemType),
        #[serde(rename = "er:uri")]
        Uri(UriType),
        #[serde(rename = "er:rewriteSystem")]
        RewriteSystem(RewriteSystemType),
        #[serde(rename = "er:rewriteURI")]
        RewriteUri(RewriteUriType),
        #[serde(rename = "er:uriSuffix")]
        UriSuffix(UriSuffixType),
        #[serde(rename = "er:systemSuffix")]
        SystemSuffix(SystemSuffixType),
        #[serde(rename = "er:delegatePublic")]
        DelegatePublic(DelegatePublicType),
        #[serde(rename = "er:delegateSystem")]
        DelegateSystem(DelegateSystemType),
        #[serde(rename = "er:delegateURI")]
        DelegateUri(DelegateUriType),
        #[serde(rename = "er:nextCatalog")]
        NextCatalog(NextCatalogType),
        #[serde(rename = "er:group")]
        Group(GroupType),
    }
    pub type Catalog = CatalogType;
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DelegatePublicType {
        #[serde(rename = "@publicIdStartString")]
        pub public_id_start_string: String,
        #[serde(rename = "@catalog")]
        pub catalog: String,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
    }
    pub type DelegatePublic = DelegatePublicType;
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DelegateSystemType {
        #[serde(rename = "@systemIdStartString")]
        pub system_id_start_string: String,
        #[serde(rename = "@catalog")]
        pub catalog: String,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
    }
    pub type DelegateSystem = DelegateSystemType;
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DelegateUriType {
        #[serde(rename = "@uriStartString")]
        pub uri_start_string: String,
        #[serde(rename = "@catalog")]
        pub catalog: String,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
    }
    pub type DelegateUri = DelegateUriType;
    #[derive(Debug, Deserialize, Serialize)]
    pub struct GroupType {
        #[serde(default, rename = "@prefer")]
        pub prefer: Option<SystemOrPublicType>,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
        #[serde(rename = "#content")]
        pub content: Vec<GroupTypeContent>,
    }
    #[derive(Debug, Deserialize, Serialize)]
    pub enum GroupTypeContent {
        #[serde(rename = "er:public")]
        Public(PublicType),
        #[serde(rename = "er:system")]
        System(SystemType),
        #[serde(rename = "er:uri")]
        Uri(UriType),
        #[serde(rename = "er:rewriteSystem")]
        RewriteSystem(RewriteSystemType),
        #[serde(rename = "er:rewriteURI")]
        RewriteUri(RewriteUriType),
        #[serde(rename = "er:uriSuffix")]
        UriSuffix(UriSuffixType),
        #[serde(rename = "er:systemSuffix")]
        SystemSuffix(SystemSuffixType),
        #[serde(rename = "er:delegatePublic")]
        DelegatePublic(DelegatePublicType),
        #[serde(rename = "er:delegateSystem")]
        DelegateSystem(DelegateSystemType),
        #[serde(rename = "er:delegateURI")]
        DelegateUri(DelegateUriType),
        #[serde(rename = "er:nextCatalog")]
        NextCatalog(NextCatalogType),
    }
    pub type Group = GroupType;
    #[derive(Debug, Deserialize, Serialize)]
    pub struct NextCatalogType {
        #[serde(rename = "@catalog")]
        pub catalog: String,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
    }
    pub type NextCatalog = NextCatalogType;
    pub type PartialPublicIdentifierType = String;
    pub type PubIdCharsType = String;
    #[derive(Debug, Deserialize, Serialize)]
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
    #[derive(Debug, Deserialize, Serialize)]
    pub struct RewriteSystemType {
        #[serde(rename = "@systemIdStartString")]
        pub system_id_start_string: String,
        #[serde(rename = "@rewritePrefix")]
        pub rewrite_prefix: String,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
    }
    pub type RewriteSystem = RewriteSystemType;
    #[derive(Debug, Deserialize, Serialize)]
    pub struct RewriteUriType {
        #[serde(rename = "@uriStartString")]
        pub uri_start_string: String,
        #[serde(rename = "@rewritePrefix")]
        pub rewrite_prefix: String,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
    }
    pub type RewriteUri = RewriteUriType;
    #[derive(Debug, Deserialize, Serialize)]
    pub struct SystemType {
        #[serde(rename = "@systemId")]
        pub system_id: String,
        #[serde(rename = "@uri")]
        pub uri: String,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
    }
    pub type System = SystemType;
    #[derive(Debug, Deserialize, Serialize)]
    pub struct SystemOrPublicType {
        #[serde(rename = "#text")]
        pub value: SystemOrPublicValue,
    }
    impl From<SystemOrPublicValue> for SystemOrPublicType {
        fn from(value: SystemOrPublicValue) -> Self {
            Self { value }
        }
    }
    impl From<SystemOrPublicType> for SystemOrPublicValue {
        fn from(value: SystemOrPublicType) -> Self {
            value.value
        }
    }
    impl Deref for SystemOrPublicType {
        type Target = SystemOrPublicValue;
        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }
    impl DerefMut for SystemOrPublicType {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.value
        }
    }
    #[derive(Debug, Deserialize, Serialize)]
    pub enum SystemOrPublicValue {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "public")]
        Public,
    }
    #[derive(Debug, Deserialize, Serialize)]
    pub struct SystemSuffixType {
        #[serde(rename = "@systemIdSuffix")]
        pub system_id_suffix: String,
        #[serde(rename = "@uri")]
        pub uri: String,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
    }
    pub type SystemSuffix = SystemSuffixType;
    #[derive(Debug, Deserialize, Serialize)]
    pub struct UriType {
        #[serde(rename = "@name")]
        pub name: String,
        #[serde(rename = "@uri")]
        pub uri: String,
        #[serde(default, rename = "@id")]
        pub id: Option<String>,
    }
    pub type Uri = UriType;
    #[derive(Debug, Deserialize, Serialize)]
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
    #[derive(Debug, Default, Deserialize, Serialize)]
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
    #[derive(Debug, Deserialize, Serialize)]
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
