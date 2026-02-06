pub mod er {
    #[derive(Debug)]
    pub struct CatalogType {
        pub id: Option<String>,
        pub prefer: Option<SystemOrPublicType>,
        pub content: Vec<CatalogTypeContent>,
    }
    #[derive(Debug)]
    pub enum CatalogTypeContent {
        Public(PublicType),
        System(SystemType),
        Uri(UriType),
        RewriteSystem(RewriteSystemType),
        RewriteUri(RewriteUriType),
        UriSuffix(UriSuffixType),
        SystemSuffix(SystemSuffixType),
        DelegatePublic(DelegatePublicType),
        DelegateSystem(DelegateSystemType),
        DelegateUri(DelegateUriType),
        NextCatalog(NextCatalogType),
        Group(GroupType),
    }
    pub type Catalog = CatalogType;
    #[derive(Debug)]
    pub struct DelegatePublicType {
        pub public_id_start_string: String,
        pub catalog: String,
        pub id: Option<String>,
    }
    pub type DelegatePublic = DelegatePublicType;
    #[derive(Debug)]
    pub struct DelegateSystemType {
        pub system_id_start_string: String,
        pub catalog: String,
        pub id: Option<String>,
    }
    pub type DelegateSystem = DelegateSystemType;
    #[derive(Debug)]
    pub struct DelegateUriType {
        pub uri_start_string: String,
        pub catalog: String,
        pub id: Option<String>,
    }
    pub type DelegateUri = DelegateUriType;
    #[derive(Debug)]
    pub struct GroupType {
        pub prefer: Option<SystemOrPublicType>,
        pub id: Option<String>,
        pub content: Vec<GroupTypeContent>,
    }
    #[derive(Debug)]
    pub enum GroupTypeContent {
        Public(PublicType),
        System(SystemType),
        Uri(UriType),
        RewriteSystem(RewriteSystemType),
        RewriteUri(RewriteUriType),
        UriSuffix(UriSuffixType),
        SystemSuffix(SystemSuffixType),
        DelegatePublic(DelegatePublicType),
        DelegateSystem(DelegateSystemType),
        DelegateUri(DelegateUriType),
        NextCatalog(NextCatalogType),
    }
    pub type Group = GroupType;
    #[derive(Debug)]
    pub struct NextCatalogType {
        pub catalog: String,
        pub id: Option<String>,
    }
    pub type NextCatalog = NextCatalogType;
    pub type PartialPublicIdentifierType = String;
    pub type PubIdCharsType = String;
    #[derive(Debug)]
    pub struct PublicType {
        pub public_id: String,
        pub uri: String,
        pub id: Option<String>,
    }
    pub type Public = PublicType;
    pub type PublicIdentifierType = String;
    #[derive(Debug)]
    pub struct RewriteSystemType {
        pub system_id_start_string: String,
        pub rewrite_prefix: String,
        pub id: Option<String>,
    }
    pub type RewriteSystem = RewriteSystemType;
    #[derive(Debug)]
    pub struct RewriteUriType {
        pub uri_start_string: String,
        pub rewrite_prefix: String,
        pub id: Option<String>,
    }
    pub type RewriteUri = RewriteUriType;
    #[derive(Debug)]
    pub struct SystemType {
        pub system_id: String,
        pub uri: String,
        pub id: Option<String>,
    }
    pub type System = SystemType;
    #[derive(Debug)]
    pub enum SystemOrPublicType {
        System,
        Public,
    }
    #[derive(Debug)]
    pub struct SystemSuffixType {
        pub system_id_suffix: String,
        pub uri: String,
        pub id: Option<String>,
    }
    pub type SystemSuffix = SystemSuffixType;
    #[derive(Debug)]
    pub struct UriType {
        pub name: String,
        pub uri: String,
        pub id: Option<String>,
    }
    pub type Uri = UriType;
    #[derive(Debug)]
    pub struct UriSuffixType {
        pub uri_suffix: String,
        pub uri: String,
        pub id: Option<String>,
    }
    pub type UriSuffix = UriSuffixType;
}
pub mod xs {
    #[derive(Debug, Default)]
    pub struct EntitiesType(pub Vec<String>);
    pub type EntityType = String;
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
