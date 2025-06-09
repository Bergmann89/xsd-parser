pub mod er {
    use core::ops::{Deref, DerefMut};
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CatalogType {
        #[serde(default, rename = "@er:id")]
        pub id: Option<String>,
        #[serde(default, rename = "@er:prefer")]
        pub prefer: Option<SystemOrPublicType>,
        #[serde(rename = "#content")]
        pub content: Vec<CatalogTypeContent>,
    }
    #[derive(Debug, Serialize, Deserialize)]
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
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DelegatePublicType {
        #[serde(rename = "@er:publicIdStartString")]
        pub public_id_start_string: String,
        #[serde(rename = "@er:catalog")]
        pub catalog: String,
        #[serde(default, rename = "@er:id")]
        pub id: Option<String>,
    }
    pub type DelegatePublic = DelegatePublicType;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DelegateSystemType {
        #[serde(rename = "@er:systemIdStartString")]
        pub system_id_start_string: String,
        #[serde(rename = "@er:catalog")]
        pub catalog: String,
        #[serde(default, rename = "@er:id")]
        pub id: Option<String>,
    }
    pub type DelegateSystem = DelegateSystemType;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DelegateUriType {
        #[serde(rename = "@er:uriStartString")]
        pub uri_start_string: String,
        #[serde(rename = "@er:catalog")]
        pub catalog: String,
        #[serde(default, rename = "@er:id")]
        pub id: Option<String>,
    }
    pub type DelegateUri = DelegateUriType;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GroupType {
        #[serde(default, rename = "@er:prefer")]
        pub prefer: Option<SystemOrPublicType>,
        #[serde(default, rename = "@er:id")]
        pub id: Option<String>,
        #[serde(rename = "#content")]
        pub content: Vec<GroupTypeContent>,
    }
    #[derive(Debug, Serialize, Deserialize)]
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
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NextCatalogType {
        #[serde(rename = "@er:catalog")]
        pub catalog: String,
        #[serde(default, rename = "@er:id")]
        pub id: Option<String>,
    }
    pub type NextCatalog = NextCatalogType;
    pub type PartialPublicIdentifierType = String;
    pub type PubIdCharsType = String;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PublicType {
        #[serde(rename = "@er:publicId")]
        pub public_id: String,
        #[serde(rename = "@er:uri")]
        pub uri: String,
        #[serde(default, rename = "@er:id")]
        pub id: Option<String>,
    }
    pub type Public = PublicType;
    pub type PublicIdentifierType = String;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RewriteSystemType {
        #[serde(rename = "@er:systemIdStartString")]
        pub system_id_start_string: String,
        #[serde(rename = "@er:rewritePrefix")]
        pub rewrite_prefix: String,
        #[serde(default, rename = "@er:id")]
        pub id: Option<String>,
    }
    pub type RewriteSystem = RewriteSystemType;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RewriteUriType {
        #[serde(rename = "@er:uriStartString")]
        pub uri_start_string: String,
        #[serde(rename = "@er:rewritePrefix")]
        pub rewrite_prefix: String,
        #[serde(default, rename = "@er:id")]
        pub id: Option<String>,
    }
    pub type RewriteUri = RewriteUriType;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SystemType {
        #[serde(rename = "@er:systemId")]
        pub system_id: String,
        #[serde(rename = "@er:uri")]
        pub uri: String,
        #[serde(default, rename = "@er:id")]
        pub id: Option<String>,
    }
    pub type System = SystemType;
    #[derive(Debug, Serialize, Deserialize)]
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
    #[derive(Debug, Serialize, Deserialize)]
    pub enum SystemOrPublicValue {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "public")]
        Public,
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SystemSuffixType {
        #[serde(rename = "@er:systemIdSuffix")]
        pub system_id_suffix: String,
        #[serde(rename = "@er:uri")]
        pub uri: String,
        #[serde(default, rename = "@er:id")]
        pub id: Option<String>,
    }
    pub type SystemSuffix = SystemSuffixType;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct UriType {
        #[serde(rename = "@er:name")]
        pub name: String,
        #[serde(rename = "@er:uri")]
        pub uri: String,
        #[serde(default, rename = "@er:id")]
        pub id: Option<String>,
    }
    pub type Uri = UriType;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct UriSuffixType {
        #[serde(rename = "@er:uriSuffix")]
        pub uri_suffix: String,
        #[serde(rename = "@er:uri")]
        pub uri: String,
        #[serde(default, rename = "@er:id")]
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
